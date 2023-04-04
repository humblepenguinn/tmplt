use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;

use crate::error;

pub fn get_homedir() -> PathBuf {
    match dirs::home_dir() {
        Some(home) => home,
        None => {
            error!("Home directory not found");
            std::process::exit(1);
        }
    }
}

pub fn get_configdir() -> PathBuf {
    let homedir = get_homedir();
    homedir.join(".tmplt")
}

pub async fn download_file(url: &str, file_name: &str) {
    let client = Client::new();
    let mut resp = match client.get(url).send().await {
        Ok(resp) => resp,
        Err(e) => {
            error!(e.to_string());
            std::process::exit(1);
        }
    };

    let mut file = match File::create(file_name) {
        Ok(file) => file,
        Err(e) => {
            error!(e.to_string());
            std::process::exit(1);
        }
    };

    let mut content_length = match resp.content_length() {
        Some(length) => length,
        None => {
            error!("Can not get content length");
            std::process::exit(1);
        }
    };

    let pb = ProgressBar::new(content_length);

    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .unwrap()
        .progress_chars("#>-"));

    while let Some(chunk) = resp.chunk().await.unwrap() {
        let chunk_size = chunk.len();
        if let Err(e) = file.write_all(&chunk) {
            error!(e.to_string());
            std::process::exit(1);
        }

        pb.inc(chunk_size as u64);
        content_length -= chunk_size as u64;
    }

    pb.finish();
}
