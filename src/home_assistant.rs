use std::fs::File;
use std::io::Read;
use tar::Archive;

pub fn extract_file_name(file_path: &str) -> Result<String, String> {
    // Open the tar archive file
    let file = File::open(file_path).expect("Unable to open file");
    let mut archive = Archive::new(file);

    // Iterate over the entries in the tar archive
    for entry in archive.entries().expect("Failed to read tar entries") {
        let mut entry = entry.expect("Failed to read an entry");
        if let Ok(path) = entry.path() {
            if path.ends_with("backup.json") {
                // Parse and extract the "name" field from the JSON file
                let mut content = String::new();
                entry
                    .read_to_string(&mut content)
                    .expect("Failed to read file content");

                let json: serde_json::Value =
                    serde_json::from_str(&content).expect("Failed to parse JSON");
                if let Some(name) = json.get("name").and_then(|v| v.as_str()) {
                    return Ok(name.to_string()); // Return the "name" field
                }
            }
        }
    }

    // If no "name" field was found, return an error
    Err("No 'name' field found in the tar archive".to_string())
}
