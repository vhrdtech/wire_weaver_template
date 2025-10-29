#![no_std]
#![no_main]
#![feature(impl_trait_in_assoc_type)]

mod init;
mod init_ram;
use defmt::*;
use defmt_rtt as _;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_time::Timer;
use panic_probe as _;
use cortex_m_rt::exception;
use embassy_stm32::{Config, bind_interrupts, peripherals, usb};
use embassy_stm32::usb::Driver;

bind_interrupts!(struct Irqs {
    OTG_FS => usb::InterruptHandler<peripherals::USB_OTG_FS>;
});

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    init_ram::init_ram();
    info!("nucleo_h743zi2 starting...");
    
    let p = embassy_stm32::init(embassy_stm32::Config::default());
    init::init();
    init::reset_bkp_domain();
    info!("RCC and RAM init done");

    let mut cp = cortex_m::Peripherals::take().unwrap();
    cp.SCB.enable_icache();
    // Enable D-Cache only after verifying that no coherency issues will arise, e.g., when using DMAs
    // DMAs write/read to/from SRAM while cache continues to hold old data, can use cache invalidate to solve this
    // cp.SCB.enable_dcache(&mut cp.CPUID);
    let mut led = Output::new(p.PB14, Level::Low, Speed::Low);

    let mut config = embassy_stm32::usb::Config::default();
    // Do not enable vbus_detection. This is a safe default that works in all boards.
    // However, if your USB device is self-powered (can stay powered on if USB is unplugged), you need
    // to enable vbus_detection to comply with the USB spec. If you enable it, the board
    // has to support it or USB won't work at all. See docs on `vbus_detection` for details.
    config.vbus_detection = false;
    let driver = Driver::new_fs(p.USB_OTG_FS, Irqs, p.PA12, p.PA11, &mut ep_out_buffer, config);

    info!("Init done");
    loop {
        info!("LED ON");
        led.set_high();
        Timer::after_millis(2000).await;

        info!("LED OFF");
        led.set_low();
        Timer::after_millis(2000).await;
    }
}

#[exception]
unsafe fn DefaultHandler(irqn: i16) {
    error!("Unhandled exception (IRQn = {})", irqn);
}

#[exception]
unsafe fn HardFault(ef: &cortex_m_rt::ExceptionFrame) -> ! {
    error!("HardFault {}", defmt::Debug2Format(ef));

    loop {}
}

//NonMaskableInt (CSS?)
// NOTE that at this point we don't check if the exception is available on the target (e.g.
// MemoryManagement is not available on Cortex-M0)
// "MemoryManagement" | "BusFault" | "UsageFault" | "SecureFault" | "SVCall"
// | "DebugMonitor" | "PendSV" | "SysTick" => {
