use bpaf::Bpaf;
use std::io::{self, Write};
use std::net::{IpAddr, Ipv4Addr};
use std::sync::mpsc::{channel, Sender};
use tokio::net::TcpStream;
use tokio::task;

const MAX: u16 = 65535;
const IPFALLBACK: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options)]
pub struct Arguments {
    #[bpaf(
        long, 
        short,
        guard(is_ipv4, "Only IPv4 addresses are supported."),
        fallback(IPFALLBACK),
    )]
    /// The ip address that you want to lookup. Must be a valid ipv4 address. Falls back to 127.0.0.1
    pub address: IpAddr,
    #[bpaf(
        long("start"),
        short('s'),
        guard(start_port_guard, "Must be greater than 0."),
        fallback(1u16)
    )]
    /// The start port for the sniffer, it must be greater than 0.
    pub start_port: u16,
    #[bpaf(
        long("end"),
        short('e'),
        guard(end_port_guard, "Must be less than or equal to 65535."),
        fallback(MAX)
    )]
    /// The end port for the sniffer, it must be less than or equal to 65535.
    pub end_port: u16,
}

fn start_port_guard(input: &u16) -> bool {
    *input > 0
}

fn end_port_guard(input: &u16) -> bool {
    *input <= MAX
}

fn is_ipv4(addr: &IpAddr) -> bool {
    matches!(addr, IpAddr::V4(_))
}

async fn scan(tx: Sender<u16>, port: u16, addr: IpAddr) {
    match TcpStream::connect(format!("{}:{}", addr, port)).await {
        Ok(_) => {
            print!(".");
            io::stdout().flush().unwrap();
            tx.send(port).unwrap();
        }
        Err(_) => {}
    }
}

#[tokio::main]
async fn main() {
    let opts: Arguments = arguments().run();

    let (tx, rx) = channel();
    for i in opts.start_port..opts.end_port {
        let tx = tx.clone();

        task::spawn(async move {
            scan(tx, i, opts.address).await;
        });
    }

    let mut out = vec![];
    drop(tx);
    for p in rx {
        out.push(p);
    }

    println!("");
    out.sort();
    for v in out {
        println!("{} is open", v);
    }
}
