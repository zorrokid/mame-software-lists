use std::collections::HashMap;

use crate::models;

pub fn scan_archives(path: String, roms_in_software_list: HashMap<String, models::Rom>) {
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
                let file = archive.by_index(i).unwrap();
                let file_name = file.name();
                print!("{} ", file_name);
            }
        }
    }
}