use std::collections::HashMap;
use std::io::Read;
use sha1::{Digest, Sha1};

use crate::models;

pub fn scan_archives(path: String, roms_in_software_list: HashMap<String, models::Rom>) -> Vec<i32> {
    let mut matched_ids: Vec<i32> = Vec::new();
    // read the files in the directory
    let dir_entries = std::fs::read_dir(path).unwrap();
    for dir_entry in dir_entries {
        let path_buf = dir_entry.unwrap().path();
        let path_str = path_buf.to_str().unwrap();
        if path_buf.is_file() && path_str.ends_with(".zip"){
            // open the archive
            let mut archive = match zip::ZipArchive::new(std::fs::File::open(&path_buf).unwrap()) {
                Ok(archive) => archive,
                Err(_) => {
                    // TODO: collect erroneous files and return their file names as list 
                    println!("Error opening archive: {}", path_str);
                    continue;
                }
            };
            // read the files in the archive
            for i in 0..archive.len() {
                let mut file = archive.by_index(i).unwrap();
                let file_name = file.name();
                print!("{} ", file_name);

                // Calculate SHA1 hash of the file
                let mut hasher = Sha1::new();

                // buffer to read the file in chunks of 1024 bytes, 
                // initialize it with zeros
                let mut buffer = [0; 1024];
                loop {
                    // TODO: handle errors
                    let bytes_read = file.read(&mut buffer).unwrap();
                    // is the end of the file reached?
                    if bytes_read == 0 {
                        break;
                    }
                    // slice valid bytes from the buffer and update the hash
                    hasher.update(&buffer[..bytes_read]);
                } 
                let sha_1 = hasher.finalize();
                let sha_1_str = format!("{:x}", sha_1);
                println!("{:x}", sha_1);
                if roms_in_software_list.contains_key(&sha_1_str) {
                    matched_ids.push(roms_in_software_list.get(&sha_1_str).unwrap().id);
                }
            }
        }
    }
    matched_ids
}