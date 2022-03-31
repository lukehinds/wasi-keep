use std::io::Error;

pub fn password_prompt() -> Result<String, Error> {
    let pass1 = rpassword::read_password_from_tty(Some("Enter a password: "))?;
    let pass2 = rpassword::read_password_from_tty(Some("Re-enter password: "))?;
    if pass1 != pass2 {
        return Err(Error::new(
            std::io::ErrorKind::Other,
            "Passwords do not match",
        ));
    }
    Ok(pass1)
}