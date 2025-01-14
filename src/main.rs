/*
 * Copyright (c) 2024 Alexander Likhachev
 * Licensed under the MIT License. See LICENSE file for details.
 */

mod home_assistant;
mod s3_uploader;

use std::fs;
use std::path;
use tokio;

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <directory_to_upload>", args[0]);
        std::process::exit(1);
    }
    let directory_to_upload = &args[1];

    dotenv::dotenv().ok();

    let bucket = s3_uploader::initialize_bucket();

    let files = fs::read_dir(directory_to_upload).unwrap_or_else(|_| {
        let absolute_path = path::absolute(path::Path::new(directory_to_upload)).unwrap();
        eprintln!("Failed to read directory: {}", absolute_path.display());
        std::process::exit(1);
    });

    for entry in files {
        let entry = entry.unwrap();
        let path = entry.path();
        let s3_key = match home_assistant::extract_file_name(path.to_str().unwrap()) {
            Ok(name) => format!("{}.tar", name),
            Err(e) => {
                eprintln!("Failed to extract file name for key: {}", e);
                return; // Skip uploading if extraction fails
            }
        };

        if path.is_file() {
            match s3_uploader::upload_file_to_s3(&bucket, path.to_str().unwrap(), &s3_key).await {
                Ok(_) => {
                    if let Err(e) = fs::remove_file(&path) {
                        eprintln!(
                            "Failed to delete file {}: {}",
                            path::absolute(path).unwrap().display(),
                            e
                        );
                    } else {
                        println!(
                            "Successfully deleted file: {}",
                            path::absolute(path).unwrap().display()
                        );
                    }
                }
                Err(e) => eprintln!("Failed to upload {}: {}", path.display(), e),
            }
        }
    }
}
