use std::error::Error;

use quinn::{ClientConfig, Endpoint, NewConnection, RecvStream, SendStream};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut endpoint = Endpoint::client("127.0.0.1:8081".parse()?)?;
    let mut certs = rustls::RootCertStore::empty();
    certs.add(&rustls::Certificate(certificate::load_ca_cert_der()?))?;
    endpoint.set_default_client_config(ClientConfig::with_root_certificates(certs));

    let NewConnection { connection, .. } = endpoint
        .connect("127.0.0.1:8080".parse()?, "localhost")?
        .await?;

    println!("id {}", connection.stable_id());

    let ts = [10, 3, 4, 2, 5];
    let mut hs = Vec::new();
    for (i, t) in ts.into_iter().enumerate() {
        let (send, recv) = connection.open_bi().await?;
        let h = tokio::spawn(async move {
            interact(send, recv, i as u8, t).await.unwrap();
        });
        hs.push(h);
    }

    for h in hs {
        h.await?;
    }
    Ok(())
}

async fn interact(
    mut send: SendStream,
    mut recv: RecvStream,
    id: u8,
    t: u8,
) -> Result<(), Box<dyn Error>> {
    println!("[{id}] open");
    send.write(&vec![id, t]).await?;
    let mut buf = vec![0; 1];
    recv.read_exact(&mut buf).await?;
    println!("[{id}] received");
    Ok(())
}
