#![allow(unused)]
fn main() {
    extern crate async_std;
    use async_std::{
        net::{TcpListener, ToSocketAddrs},
        prelude::*,
        task,
    };

    type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

    use async_std::{
        io::BufReader,
        net::TcpStream,
    };

    async fn accept_loop(addr: impl ToSocketAddrs) -> Result<()> { // 1
        let listener = TcpListener::bind(addr).await?; // 2
        let mut incoming = listener.incoming();
        while let Some(stream) = incoming.next().await { // 3
            // TODO
        }
        Ok(())
    }

    // main
    fn run() -> Result<()> {
        let fut = accept_loop("127.0.0.1:8080");
        task::block_on(fut)
    }

    async fn connection_loop(stream: TcpStream) -> Result<()> {
        let reader = BufReader::new(&stream); // 2
        let mut lines = reader.lines();

        let name = match lines.next().await { // 3
            None => Err("peer disconnected immediately")?,
            Some(line) => line?,
        };
        println!("name = {}", name);

        while let Some(line) = lines.next().await { // 4
            let line = line?;
            let (dest, msg) = match line.find(':') { // 5
                None => continue,
                Some(idx) => (&line[..idx], line[idx + 1 ..].trim()),
            };
            let dest: Vec<String> = dest.split(',').map(|name| name.trim().to_string()).collect();
            let msg: String = msg.to_string();
        }
        Ok(())
    }

    let listener: std::net::TcpListener = unimplemented!();
    for stream in listener.incoming() {
    }
}
