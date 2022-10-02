use std::{error::Error, fs::File, io::Write};

use certificate::{CA_CERT_DER, CA_PKEY_DER};

#[argopt::cmd]
fn main(name: String) -> Result<(), Box<dyn Error>> {
    let cert = rcgen::generate_simple_self_signed(vec![name])?;
    let mut file = File::create(CA_CERT_DER)?;
    file.write_all(&cert.serialize_der()?)?;
    let mut file = File::create(CA_PKEY_DER)?;
    file.write_all(&cert.serialize_private_key_der())?;
    Ok(())
}
