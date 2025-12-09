use std::time::Duration;
use wire_weaver_client_common::{CommandSender, Error};
use wire_weaver::ww_api;
pub use wire_weaver_client_common::{OnError, DeviceFilter};
pub use api::LedState;

pub struct MyDeviceDriver {
    args_scratch: [u8; 4096],
    cmd_tx: CommandSender,
    timeout: Duration,
}

impl MyDeviceDriver {
    pub async fn connect(filter: DeviceFilter, on_error: OnError) -> Result<Self, Error> {
        MyDeviceDriver::connect_raw(filter, api::DEVICE_API_ROOT_FULL_GID, on_error, Duration::from_secs(1), [0u8; 4096]).await
    }

    pub fn connect_blocking(filter: DeviceFilter, on_error: OnError) -> Result<Self, Error> {
        MyDeviceDriver::connect_raw_blocking(filter, api::DEVICE_API_ROOT_FULL_GID, on_error, Duration::from_secs(1), [0u8; 4096])
    }
}

ww_api!(
    "../api/src/lib.rs" as api::DeviceApiRoot for MyDeviceDriver,
    client = "async_worker",
    no_alloc = true,
    use_async = true,
    //derive = "Debug",
    debug_to_file = "../target/generated_std_client.rs"
);
