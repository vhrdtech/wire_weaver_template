use std::time::Duration;
use tokio::sync::mpsc;
use wire_weaver_client_common::{CommandSender, Error};
use wire_weaver_usb_host::{usb_worker};
use wire_weaver::ww_api;
pub use wire_weaver_client_common::{OnError};
pub use wire_weaver_client_common::DeviceFilter;
pub use api::LedState;

pub struct MyDeviceDriver {
    args_scratch: [u8; 512],
    cmd_tx: CommandSender,
    timeout: Duration,
}

impl MyDeviceDriver {
    pub async fn connect(filter: DeviceFilter, on_error: OnError) -> Result<Self, Error> {
        let (transport_cmd_tx, transport_cmd_rx) = mpsc::unbounded_channel();
        let (dispatcher_msg_tx, dispatcher_msg_rx) = mpsc::unbounded_channel();
        let mut cmd_tx = CommandSender::new(transport_cmd_tx, dispatcher_msg_rx);
        tokio::spawn(async move {
            usb_worker(transport_cmd_rx, dispatcher_msg_tx, api::DEVICE_API_ROOT_FULL_GID).await;
        });
        cmd_tx.connect(filter, api::DEVICE_API_ROOT_FULL_GID.into(), on_error).await?;
        Ok(Self {
            args_scratch: [0; 512],
            cmd_tx,
            timeout: Duration::from_secs(1),
        })
    }

    pub fn connect_blocking(filter: DeviceFilter, on_error: OnError) -> Result<Self, Error> {
        let (transport_cmd_tx, transport_cmd_rx) = mpsc::unbounded_channel();
        let (dispatcher_msg_tx, dispatcher_msg_rx) = mpsc::unbounded_channel();
        let mut cmd_tx = CommandSender::new(transport_cmd_tx, dispatcher_msg_rx);
        tokio::spawn(async move {
            usb_worker(transport_cmd_rx, dispatcher_msg_tx, api::DEVICE_API_ROOT_FULL_GID).await;
        });
        cmd_tx.connect_blocking(filter, api::DEVICE_API_ROOT_FULL_GID.into(), on_error)?;
        Ok(Self {
            args_scratch: [0; 512],
            cmd_tx,
            timeout: Duration::from_secs(1),
        })
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
