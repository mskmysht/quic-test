use std::{
    fs::File,
    io::{self, Read},
};

pub const CA_CERT_DER: &'static str = "ca-cert.der";
pub const CA_PKEY_DER: &'static str = "ca-pkey.der";

fn load_buf(path: &str) -> io::Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;
    Ok(buf)
}

pub fn load_ca_cert_der() -> io::Result<Vec<u8>> {
    load_buf(CA_CERT_DER)
}

pub fn load_ca_pkey_der() -> io::Result<Vec<u8>> {
    load_buf(CA_PKEY_DER)
}
