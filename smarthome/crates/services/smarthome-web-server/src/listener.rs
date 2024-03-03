use listenfd::ListenFd;
use std::io::Result;
use tokio::net::TcpListener;

pub async fn try_create() -> Result<TcpListener> {
    match ListenFd::from_env().take_tcp_listener(0)? {
        // if we are given a tcp listener on listen fd 0, we use that one
        Some(listener) => {
            listener.set_nonblocking(true)?;
            Ok(TcpListener::from_std(listener)?)
        }
        // otherwise fall back to local listening
        None => TcpListener::bind("127.0.0.1:3000").await,
    }
}
