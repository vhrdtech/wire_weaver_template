#![no_std]
#![no_main]
#![feature(impl_trait_in_assoc_type)]

mod init;

use api::LedState;
use core::pin::pin;
use cortex_m_rt::exception;
use defmt::*;
use defmt_rtt as _;
use embassy_futures::select::{Either, select};
use embassy_stm32::{
    Config, bind_interrupts,
    gpio::{Level, Output, Speed},
    peripherals::USB,
    usb,
    usb::Driver,
};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::{Channel, Receiver};
use embassy_time::Timer;
use panic_probe as _;
use static_cell::StaticCell;
use wire_weaver::{WireWeaverAsyncApiBackend, shrink_wrap, ww_api, ww_version::FullVersion};
use wire_weaver_usb_embassy::{UsbBuffers, UsbServer, UsbTimings, usb_init};

bind_interrupts!(struct Irqs {
    USB_UCPD1_2 => usb::InterruptHandler<USB>;
});

const MAX_USB_PACKET_LEN: usize = 64; // 64 for FullSpeed, 1024 for HighSpeed
const MAX_MESSAGE_LEN: usize = 1024; // Maximum WireWeaver message length
static USB_BUFFERS: StaticCell<UsbBuffers<MAX_USB_PACKET_LEN, MAX_MESSAGE_LEN>> = StaticCell::new();

#[embassy_executor::task]
async fn usb_server_task(
    mut usb_server: UsbServer<'static, Driver<'static, USB>, ServerState>,
    rx: Receiver<'static, CriticalSectionRawMutex, u32, 8>,
) {
    let mut server_fut = pin!(usb_server.run());
    loop {
        let Either::Second(val) = select(&mut server_fut, rx.receive()).await;
        info!("got val = {}", val);
    }
}

struct ServerState {
    led: Output<'static>,
}

impl ServerState {
    async fn set_led_state(&mut self, state: LedState) {
        match state {
            LedState::Off => self.led.set_low(),
            LedState::On => self.led.set_high(),
            LedState::Blinking => {}
        }
    }
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

    fn version(&self) -> FullVersion<'_> {
        api::DEVICE_API_ROOT_FULL_GID
    }
}

ww_api!(
    "../../api/src/lib.rs" as api::DeviceApiRoot for ServerState,
    server = true, no_alloc = true, use_async = true,
    method_model = "_=immediate",
    property_model = "_=get_set",
    //debug_to_file = "./target/ws.rs" // uncomment if you want to see the resulting AST and generated code
);

#[embassy_executor::main]
async fn main(spawner: embassy_executor::Spawner) {
    info!("cannify_micro_g0b1cetxn starting...");

    let p = embassy_stm32::init(Config::default());
    init::reset_bkp_domain();
    info!("RCC and RAM init done");

    let led = Output::new(p.PB14, Level::Low, Speed::Low);
    let state = ServerState { led };

    let driver = Driver::new(p.USB, Irqs, p.PA12, p.PA11);
    let buffers = USB_BUFFERS.init(UsbBuffers::default());
    let usb_server = usb_init(driver, buffers, state, UsbTimings::default_fs(), |config| {
        config.serial_number = Some(embassy_stm32::uid::uid_hex());
    });
    static SHARED_CHANNEL: Channel<CriticalSectionRawMutex, u32, 8> = Channel::new();
    unwrap!(spawner.spawn(usb_server_task(usb_server, SHARED_CHANNEL.receiver())));

    info!("init done");
    let tx = SHARED_CHANNEL.sender();
    let mut i = 0;
    loop {
        info!("loop");
        Timer::after_millis(2000).await;
        tx.send(i).await;
        i = i.saturating_add(1);
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
