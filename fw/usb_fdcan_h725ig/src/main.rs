#![no_std]
#![no_main]
#![feature(impl_trait_in_assoc_type)]

mod init;

use api::LedState;
use cortex_m_rt::exception;
use defmt::*;
use defmt_rtt as _;
use embassy_stm32::{
    Config, bind_interrupts,
    gpio::{Level, Output, Speed},
    peripherals::USB_OTG_HS,
    usb,
    usb::Driver,
};
use embassy_stm32::time::mhz;
use embassy_time::Timer;
use panic_probe as _;
use static_cell::StaticCell;
use wire_weaver::prelude::*;
use wire_weaver::{MessageSink, WireWeaverAsyncApiBackend};
use wire_weaver_usb_embassy::{UsbBuffers, UsbServer, UsbTimings, usb_init};
use ww_client_server::{StreamSidebandCommand, StreamSidebandEvent};

bind_interrupts!(struct Irqs {
    OTG_HS => usb::InterruptHandler<USB_OTG_HS>;
});

const MAX_USB_PACKET_LEN: usize = 1024; // 64 for FullSpeed, 1024 for HighSpeed
const EP_OUT_BUF_LEN: usize = MAX_USB_PACKET_LEN * wire_weaver_usb_embassy::ENDPOINTS_USED;
const MAX_MESSAGE_LEN: usize = 4096; // Maximum WireWeaver message length
static USB_BUFFERS: StaticCell<UsbBuffers<MAX_USB_PACKET_LEN, MAX_MESSAGE_LEN>> = StaticCell::new();

#[embassy_executor::task]
async fn usb_server_task(mut usb_server: UsbServer<'static, Driver<'static, USB_OTG_HS>, ServerState>) {
    usb_server.run().await;
}

impl WireWeaverAsyncApiBackend for ServerState {
    async fn process_bytes<'a>(
        &mut self,
        data: &[u8],
        scratch_args: &'a mut [u8],
        scratch_event: &'a mut [u8],
        scratch_err: &'a mut [u8],
    ) -> Result<&'a [u8], shrink_wrap::Error> {
        self.process_request_bytes(data, scratch_args, scratch_event, scratch_err)
            .await
    }

    async fn send_updates(
        &mut self,
        sink: &mut impl MessageSink,
        scratch_value: &mut [u8],
        scratch_event: &mut [u8],
    ) {
        let message = api_impl::stream_data_ser().usart_rx(
            &RefVec::new_bytes(&[0, 1, 2, 3, 4]),
            scratch_value,
            scratch_event,
        );
        _ = sink.send(message.unwrap()).await;
    }

    fn version(&self) -> FullVersion<'_> {
        api::DEVICE_API_ROOT_FULL_GID
    }
}

struct ServerState {
    leds: [Output<'static>; 2]
}

ww_api!(
    "../../api/src/lib.rs" as api::DeviceApiRoot for ServerState,
    server = true, no_alloc = true, use_async = true,
    method_model = "_=immediate",
    property_model = "_=get_set",
    debug_to_file = "./target/generated_no_std_server.rs" // uncomment if you want to see the resulting AST and generated code
);

impl ServerState {
    async fn set_led_state(&mut self, state: LedState) {
        info!("set_led state {:?}", state);
        match state {
            LedState::Off => {
                for led in &mut self.leds {
                    led.set_low();
                }
            },
            LedState::On => {
                for led in &mut self.leds {
                    led.set_high();
                }
            }
            LedState::Blinking => {}
        }
    }

    async fn usart_rx_sideband(
        &mut self,
        _cmd: StreamSidebandCommand,
    ) -> Option<StreamSidebandEvent> {
        None
    }

    async fn usart_tx_write(&mut self, data: &[u8]) {
        info!("tx: {:?}", data);
    }

    async fn usart_tx_sideband(
        &mut self,
        _cmd: StreamSidebandCommand,
    ) -> Option<StreamSidebandEvent> {
        None
    }
}

#[embassy_executor::main]
async fn main(spawner: embassy_executor::Spawner) {
    info!("cannify_micro_g0b1cetxn starting...");

    let mut config = Config::default();
    {
        use embassy_stm32::rcc::*;
        config.rcc.hsi = None;
        config.rcc.csi = false;
        config.rcc.hse = Some(Hse {
            freq: mhz(24),
            mode: HseMode::Bypass,
        });
        config.rcc.hsi48 = None;
        config.rcc.pll1 = Some(Pll {
            source: PllSource::HSE,
            prediv: PllPreDiv::DIV2,
            mul: PllMul::MUL45,
            divp: Some(PllDiv::DIV1),
            divq: Some(PllDiv::DIV4),
            divr: Some(PllDiv::DIV2),
        });
        config.rcc.pll3 = Some(Pll {
            source: PllSource::HSE,
            prediv: PllPreDiv::DIV2,
            mul: PllMul::MUL16,
            divp: Some(PllDiv::DIV2),
            divq: Some(PllDiv::DIV4), // 48MHz
            divr: Some(PllDiv::DIV4),
        });
        config.rcc.sys = Sysclk::PLL1_P; // 540MHz
        config.rcc.d1c_pre = AHBPrescaler::DIV2;
        config.rcc.ahb_pre = AHBPrescaler::DIV2;
        config.rcc.apb1_pre = APBPrescaler::DIV2;
        config.rcc.apb2_pre = APBPrescaler::DIV2;
        config.rcc.apb3_pre = APBPrescaler::DIV2;
        config.rcc.apb4_pre = APBPrescaler::DIV2;
        config.rcc.voltage_scale = VoltageScale::Scale0;
        config.rcc.supply_config = SupplyConfig::DirectSMPS;
        config.rcc.mux.fdcansel = mux::Fdcansel::PLL1_Q;
        config.rcc.mux.usbsel = mux::Usbsel::PLL3_Q;
        config.rcc.mux.adcsel = mux::Adcsel::PLL3_R;
        config.rcc.mux.sdmmcsel = mux::Sdmmcsel::PLL1_Q;
    }
    let p = embassy_stm32::init(config);
    init::reset_bkp_domain();
    info!("RCC and RAM init done");

    let led_b125 = Output::new(p.PF5, Level::Low, Speed::Low);
    let led_b135 = Output::new(p.PC6, Level::Low, Speed::Low);
    let state = ServerState { leds: [led_b125, led_b135] };

    let _ulpi_rst_n = Output::new(p.PH3, Level::High, Speed::Low); // do not drop
    let _usb_mux_n = Output::new(p.PH5, Level::Low, Speed::Low); // do not drop

    static EP_OUT_BUF: StaticCell<[u8; EP_OUT_BUF_LEN]> = StaticCell::new();
    let ep_out_buffer = EP_OUT_BUF.init([0u8; EP_OUT_BUF_LEN]);
    let config = usb::Config::default();
    let driver = Driver::new_hs_ulpi(
        p.USB_OTG_HS,
        Irqs,
        p.PA5,
        p.PC2,
        p.PC3,
        p.PC0,
        p.PA3,
        p.PB0,
        p.PB1,
        p.PB10,
        p.PB11,
        p.PB12,
        p.PB13,
        p.PB5,
        ep_out_buffer,
        config,
    );

    let buffers = USB_BUFFERS.init(UsbBuffers::default());
    let (usb_server, _tx) = usb_init(driver, buffers, state, UsbTimings::default_hs(), api::DEVICE_API_ROOT_FULL_GID, |config| {
        config.serial_number = Some(embassy_stm32::uid::uid_hex());
    });
    unwrap!(spawner.spawn(usb_server_task(usb_server)));

    info!("init done");
    loop {
        info!("loop");
        Timer::after_millis(2000).await;
        // _ = _tx.try_send(());
    }
}

#[exception]
unsafe fn DefaultHandler(irqn: i16) {
    error!("Unhandled exception (IRQn = {})", irqn);
}

#[exception]
unsafe fn HardFault(ef: &cortex_m_rt::ExceptionFrame) -> ! {
    error!("HardFault {}", Debug2Format(ef));

    loop {}
}
