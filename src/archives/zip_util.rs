use std::fs::File;
use std::io::copy;
use std::path::PathBuf;
use zip::read::ZipArchive;

pub struct CustomZipError {
    pub message: String,
}

pub fn extract_zip_file_to_tmp(
    zip_file_path: &str,
    output_path: &PathBuf,
) -> Result<Vec<String>, CustomZipError> {
    let file = std::fs::File::open(zip_file_path).map_err(|_| CustomZipError {
        message: format!("Failed to open the file {}", zip_file_path),
    })?;
    let mut archive = ZipArchive::new(file).map_err(|_| CustomZipError {
        message: format!("Failed to open the zip archive {}", zip_file_path),
    })?;
    let mut file_names = Vec::new();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).map_err(|_| CustomZipError {
            message: format!("Failed to get the file {} from zip archive", i),
        })?;
        let outpath = match file.enclosed_name() {
            Some(path) => output_path.join(path),
            None => continue,
        };

        file_names.push(outpath.to_str().unwrap().to_string());

        if file.name().ends_with('/') {
            std::fs::create_dir_all(&outpath).map_err(|_| CustomZipError {
                message: format!("Failed to create the directory {}", outpath.display()),
            })?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    std::fs::create_dir_all(&p).map_err(|_| CustomZipError {
                        message: format!("Failed to create the directory {}", p.display()),
                    })?;
                }
            }
            let mut outfile = File::create(&outpath).map_err(|_| CustomZipError {
                message: format!("Failed to create the file {})", outpath.display()),
            })?;
            copy(&mut file, &mut outfile).map_err(|_| CustomZipError {
                message: format!(
                    "Failed to copy the file {} to {}",
                    file.name(),
                    outpath.display()
                ),
            })?;
        }
    }
    Ok(file_names)
}
