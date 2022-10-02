use futures_util::StreamExt;
use quinn::{Endpoint, RecvStream, SendStream, ServerConfig};
use std::{error::Error, net::SocketAddr, time::Duration};

fn load_der() -> Result<(rustls::Certificate, rustls::PrivateKey), Box<dyn Error>> {
    let cert = rustls::Certificate(certificate::load_ca_cert_der()?);
    let key = rustls::PrivateKey(certificate::load_ca_pkey_der()?);
    Ok((cert, key))
}

#[argopt::cmd]
#[tokio::main]
async fn main(addr: SocketAddr) -> Result<(), Box<dyn Error>> {
    let (cert, key) = load_der()?;
    let (_, mut incoming) =
        Endpoint::server(ServerConfig::with_single_cert(vec![cert], key)?, addr)?;
    while let Some(conn) = incoming.next().await {
        let mut new_conn = conn.await?;
        println!("incomming");
        while let Some(Ok((send, recv))) = new_conn.bi_streams.next().await {
            tokio::spawn(async move {
                interact(send, recv).await.unwrap();
            });
        }
    }
    Ok(())
}

async fn interact(mut send: SendStream, mut recv: RecvStream) -> Result<(), Box<dyn Error>> {
    let mut buf = vec![0; 2];
    recv.read_exact(&mut buf).await?;
    let id = buf[0];
    let t = buf[1];
    println!("[{id}] accepted with {t}");
    tokio::time::sleep(Duration::from_secs(t as u64)).await;
    send.write(&vec![1]).await?;
    println!("[{id}] sent");
    Ok(())
}
