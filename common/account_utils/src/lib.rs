use eth2_wallet::Wallet;
use rand::{distributions::Alphanumeric, Rng};
use std::fs::{self, File};
use std::io;
use std::io::prelude::*;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

pub use eth2_wallet::PlainText;

/// The `Alphanumeric` crate only generates a-z, A-Z, 0-9, therefore it has a range of 62
/// characters.
///
/// 62**48 is greater than 255**32, therefore this password has more bits of entropy than a byte
/// array of length 32.
const DEFAULT_PASSWORD_LEN: usize = 48;

/// Returns the "default" path where a wallet should store its password file.
pub fn default_wallet_password_path<P: AsRef<Path>>(wallet_name: &str, secrets_dir: P) -> PathBuf {
    secrets_dir.as_ref().join(format!("{}.pass", wallet_name))
}

/// Returns a password for a wallet, where that password is loaded from the "default" path.
pub fn default_wallet_password<P: AsRef<Path>>(
    wallet: &Wallet,
    secrets_dir: P,
) -> Result<PlainText, io::Error> {
    let path = default_wallet_password_path(wallet.name(), secrets_dir);
    fs::read(path).map(|bytes| PlainText::from(strip_off_newlines(bytes)))
}

/// Creates a file with `600 (-rw-------)` permissions.
// TODO: move to password_utils crate.
pub fn create_with_600_perms<P: AsRef<Path>>(path: P, bytes: &[u8]) -> Result<(), io::Error> {
    let path = path.as_ref();

    let mut file = File::create(&path)?;

    let mut perm = file.metadata()?.permissions();

    perm.set_mode(0o600);

    file.set_permissions(perm)?;

    file.write_all(bytes)?;

    Ok(())
}

// TODO: move to password_utils crate.
pub fn random_password() -> PlainText {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(DEFAULT_PASSWORD_LEN)
        .collect::<String>()
        .into_bytes()
        .into()
}

/// Remove any number of newline or carriage returns from the end of a vector of bytes.
pub fn strip_off_newlines(mut bytes: Vec<u8>) -> Vec<u8> {
    let mut strip_off = 0;
    for (i, byte) in bytes.iter().rev().enumerate() {
        if *byte == b'\n' || *byte == b'\r' {
            strip_off = i + 1;
        } else {
            break;
        }
    }
    bytes.truncate(bytes.len() - strip_off);
    bytes
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
