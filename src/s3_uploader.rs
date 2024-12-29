use s3::creds::Credentials;
use s3::{Bucket, Region};

pub fn initialize_bucket() -> Box<Bucket> {
    let credentials = Credentials::default().unwrap(); // Load credentials from env variables AWS_ACCESS_KEY_ID, AWS_SECRET_ACCESS_KEY, AWS_SESSION_TOKEN

    let region_name = std::env::var("AWS_REGION").expect("Missing AWS_REGION environment variable");
    let endpoint =
        std::env::var("AWS_ENDPOINT").unwrap_or_else(|_| "https://s3.amazonaws.com".to_string());

    let region = Region::Custom {
        region: region_name,
        endpoint,
    };

    let bucket_name = std::env::var("AWS_BUCKET").expect("Missing AWS_BUCKET environment variable");

    Bucket::new(&bucket_name, region, credentials)
        .unwrap()
        .with_path_style()
}


pub async fn upload_file_to_s3(bucket: &Bucket, file_path: &str, s3_key: &str) -> Result<(), Box<dyn std::error::Error>> {
    use std::fs::File;
    use std::io::Read;

    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    bucket.put_object(s3_key, &buffer).await.map_err(|e| {
        eprintln!("Failed to upload file to S3: {}", e);
        e
    })?;

    println!("File {} successfully uploaded to S3 with key: {}", file_path, s3_key);
    Ok(())
}