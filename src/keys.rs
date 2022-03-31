use openssl::nid::Nid;
use openssl::symm::Cipher;
use openssl::{ec::EcGroup, ec::EcKey};
use std::fs::File;
use std::io::Write;

pub fn generate_keys(passw: String) -> Result<(), Box<dyn std::error::Error>> {
    let group = EcGroup::from_curve_name(Nid::X9_62_PRIME256V1)?;
    let private_key = EcKey::generate(&group)?;

    let private_key_pem;

    match passw.is_empty() {
        true => {
            private_key_pem = private_key.private_key_to_pem()?;
        }
        false => {
            private_key_pem = private_key
                .private_key_to_pem_passphrase(Cipher::aes_128_cbc(), passw.as_bytes())?;
        }
    }

    let public_key = private_key.public_key();
    let ec_pub_key = EcKey::from_public_key(&group, public_key)?;
    let public_key_pem = &ec_pub_key.public_key_to_pem()?;

    let mut privkey_file = File::create("keep.key")?;
    privkey_file.write_all(String::from_utf8(private_key_pem)?.as_bytes())?;

    let mut pubkey_file = File::create("keep.pub")?;
    pubkey_file.write_all(String::from_utf8(public_key_pem.clone())?.as_bytes())?;

    Ok(())
}

#[test]
fn test_generate_keys() {
    let res = generate_keys(String::from("foo"));
    assert!(res.is_ok());
}