use minikv_core::{Command, Response};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter};
use tokio::net::{TcpListener, TcpStream};
use tower::Service;
use tracing::{error, info};

pub async fn start_server<S>(addr: &str, service: S)
where
    S: Service<Command, Response = String, Error = minikv_core::storage::KvError>
        + Clone
        + Send
        + 'static,
    S::Future: Send + 'static,
{
    let listener = TcpListener::bind(addr).await.expect("bind failed");
    info!(%addr, "Server started");

    loop {
        let (socket, peer) = match listener.accept().await {
            Ok(pair) => pair,
            Err(e) => {
                error!("Accept failed: {e}");
                continue;
            }
        };

        let mut service = service.clone();
        tokio::spawn(async move {
            if let Err(e) = handle_connection(socket, &mut service).await {
                error!(?peer, error = ?e, "Connection error");
            }
        });
    }
}

async fn handle_connection<S>(
    socket: TcpStream,
    service: &mut S,
) -> Result<(), Box<dyn std::error::Error>>
where
    S: Service<Command, Response = String, Error = minikv_core::storage::KvError>,
    S::Future: Send + 'static,
{
    let peer = socket.peer_addr()?;
    let reader = BufReader::new(socket);
    let mut lines = reader.lines();

    while let Ok(Some(line)) = lines.next_line().await {
        let cmd: Command = match serde_json::from_str(&line) {
            Ok(c) => c,
            Err(e) => {
                error!(%peer, error = %e, "Failed to parse command");
                continue;
            }
        };

        let _ = service.poll_ready(&mut std::task::Context::from_waker(
            futures_util::task::noop_waker_ref(),
        ));

        let response = match service.call(cmd).await {
            Ok(val) => Response::Ok { value: val },
            Err(e) => Response::Error {
                message: format!("{:?}", e),
            },
        };

        let mut writer = BufWriter::new(lines.get_mut());
        let json = serde_json::to_string(&response)?;

        writer.write_all(json.as_bytes()).await?;
        writer.write_all(b"\n").await?;
        writer.flush().await?;
    }

    Ok(())
}
