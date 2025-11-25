use std::time::Duration;
use driver::{OnError, MyDeviceDriver, DeviceFilter, LedState};
use anyhow::Result;

#[tokio::main]
async fn main()-> Result<()> {
    let filter = DeviceFilter::UsbVidPid {
        vid: 0xc0de,
        pid: 0xcafe,
    };
    let mut driver = MyDeviceDriver::connect(filter, OnError::ExitImmediately).await?;

    println!("Turning LED on");
    driver.root().set_led_state(LedState::On).await?;

    tokio::time::sleep(Duration::from_secs(1)).await;

    println!("Turning LED off");
    driver.root().set_led_state(LedState::Off).await?;

    Ok(())
}
