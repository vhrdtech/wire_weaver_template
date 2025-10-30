use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{mpsc, oneshot, RwLock};
use wire_weaver_client_common::{Command, CommandSender, Error};
use wire_weaver_usb_host::{usb_worker, ConnectionInfo};
use wire_weaver::{ww_api, ProtocolInfo};
pub use wire_weaver_client_common::{OnError};
pub use wire_weaver_client_common::DeviceFilter;
pub use api::LedState;

pub struct MyDeviceDriver {
    args_scratch: [u8; 512],
    cmd_tx: CommandSender,
    _conn_state: Arc<RwLock<ConnectionInfo>>,
    timeout: Duration,
}

impl MyDeviceDriver {
    pub async fn connect(device: DeviceFilter, on_error: OnError) -> Result<Self, Error> {
        let (connected_tx, connected_rx) = oneshot::channel();
        let conn_state = Arc::new(RwLock::new(ConnectionInfo::default()));
        let cmd_tx = start_ws_worker(
            Some(connected_tx),
            device,
            on_error,
            conn_state.clone(),
        )?;
        let connection_result = connected_rx.await.map_err(|_| Error::EventLoopNotRunning)?;
        connection_result?;
        Ok(Self {
            args_scratch: [0; 512],
            cmd_tx: CommandSender::new(cmd_tx),
            _conn_state: conn_state,
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
    //debug_to_file = "./target/ww_no_alloc.rs"
);

fn start_ws_worker(
    connected_tx: Option<oneshot::Sender<Result<(), Error>>>,
    filter: DeviceFilter,
    on_error: OnError,
    conn_state: Arc<RwLock<ConnectionInfo>>,
) -> Result<mpsc::UnboundedSender<Command>, Error> {
    let (cmd_tx, cmd_rx) = mpsc::unbounded_channel();
    let user_protocol = ProtocolInfo {
        protocol_id: 13,
        major_version: 0,
        minor_version: 1,
    };
    tokio::spawn(async move {
        usb_worker(cmd_rx, conn_state, user_protocol, 64).await;
    });
    cmd_tx
        .send(Command::Connect {
            filter,
            user_protocol_version: api::DEVICE_API_ROOT_FULL_GID.into(),
            on_error: on_error.into(),
            connected_tx,
        })
        .map_err(|_| Error::EventLoopNotRunning)?;
    Ok(cmd_tx)
}