use clap::{arg, Arg, ArgMatches, Command};
use std::{fs, io, path, result, str};

pub fn get_args<'a>() -> ArgMatches {
    Command::new("schlucker")
        .version("1.0.0")
        .arg(Arg::new("file").required(true).index(1))
        .arg(arg!(-k --key <KEY> "The secret key used to encrypt or decrypt a file"))
        .get_matches()
}

/// This function is like the Map function. It iterates over the entries in the given path and applies the given method to the entries. .
///
/// # Errors
///
/// This function will return an error if the path does not exist.
pub fn walk_directory(
    file_path: &path::PathBuf,
    func: &dyn Fn(path::PathBuf) -> Result<(), io::Error>,
) -> io::Result<()> {
    for entry in fs::read_dir(file_path)? {
        let entry = entry?;
        let path = entry.path();
        let metadata = fs::metadata(&path)?;

        if metadata.is_file() {
            let _ = func(path)?;
        } else if metadata.is_dir() {
            let _ = walk_directory(&path, &func);
        }
    }
    Ok(())
}

/// Encrypts a file.
///
/// # Panics
///
/// The function panics if something fails during encryption.
///
/// # Errors
///
/// The function returns an error if writing to the file fails.
pub fn run_operation(path: &path::PathBuf, key: &str) -> result::Result<(), io::Error> {
    let contents: Vec<u8> = fs::read(&path).expect("Error reading file");
    let output: Vec<u8> = contents
        .iter()
        .enumerate()
        .map(|(i, val)| val ^ key.chars().nth(i % key.len()).unwrap() as u8)
        .collect();

    fs::write(path, output)
}

/// Makes a request to the server to get the token.
///
/// # Errors
/// This function will return an error if some thing went wrong while making a request to the server.
pub async fn get_token(password: &str) -> Result<reqwest::Response, reqwest::Error> {
    let url = format!("http://localhost:8000/token/{}", password);
    return reqwest::get(&url).await;
}

/// Reads the token from the file and returns it.
/// # Panics
/// The function panics if the file can't be opened or the files content is not in UTF-8 format
pub fn get_stored_token(path: &path::PathBuf) -> String {
    let stored_token_bytes = fs::read(&path).unwrap_or_default();
    String::from_utf8(stored_token_bytes).unwrap_or("".to_string())
}
