use std::net::IpAddr;
use std::sync::mpsc::Sender;
use tokio::net::TcpStream;

pub async fn scan(tx: Sender<u16>, port: u16, addr: IpAddr) {
    if TcpStream::connect((addr, port)).await.is_ok() {
        let _ = tx.send(port);
    }
}
