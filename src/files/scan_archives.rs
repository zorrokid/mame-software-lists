use sha1::{Digest, Sha1};
use std::io::Read;
use std::path::PathBuf;

pub struct ScanArchivesError {
    pub message: String,
}

pub struct ScanResult {
    pub found_checksums: Vec<String>,
    pub failed_files: Vec<String>,
}

pub fn scan_archives(path: PathBuf) -> Result<ScanResult, ScanArchivesError> {
    let mut scan_results = ScanResult {
        found_checksums: Vec::new(),
        failed_files: Vec::new(),
    };
    // read the files in the directory
    let dir_entries = std::fs::read_dir(path).unwrap();
    for dir_entry in dir_entries {
        let path_buf = dir_entry.unwrap().path();
        let path_str = path_buf.to_str().unwrap();
        if path_buf.is_file() && path_str.ends_with(".zip") {
            // open the archive
            let mut archive = match zip::ZipArchive::new(std::fs::File::open(&path_buf).unwrap()) {
                Ok(archive) => archive,
                Err(_) => {
                    scan_results.failed_files.push(path_str.to_string());
                    continue;
                }
            };
            // read the files in the archive
            for i in 0..archive.len() {
                let mut file = archive.by_index(i).map_err(|_| ScanArchivesError {
                    message: format!("Error reading file {} from archive", i),
                })?;
                let file_name = file.name();
                print!("{} ", file_name);

                // Calculate SHA1 hash of the file
                let mut hasher = Sha1::new();

                // buffer to read the file in chunks of 1024 bytes,
                // initialize it with zeros
                let mut buffer = [0; 1024];
                loop {
                    let bytes_read = file.read(&mut buffer).map_err(|_| ScanArchivesError {
                        message: "Error reading file".to_string(),
                    })?;
                    // is the end of the file reached?
                    if bytes_read == 0 {
                        break;
                    }
                    // slice valid bytes from the buffer and update the hash
                    hasher.update(&buffer[..bytes_read]);
                }
                let sha_1 = hasher.finalize();
                let sha_1_str = format!("{:x}", sha_1);
                scan_results.found_checksums.push(sha_1_str);
            }
        }
    }
    Ok(scan_results)
}
