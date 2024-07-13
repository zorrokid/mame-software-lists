use std::error::Error;
use std::fs::File;
use std::io::copy;
use std::path::PathBuf;
use zip::read::ZipArchive;

pub fn extract_zip_file_to_tmp(
    zip_file_path: &str,
    output_path: &PathBuf,
) -> Result<Vec<String>, Box<dyn Error>> {
    let file = std::fs::File::open(zip_file_path)?;
    let mut archive = ZipArchive::new(file)?;
    let mut file_names = Vec::new();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = match file.enclosed_name() {
            Some(path) => output_path.join(path),
            None => continue,
        };

        file_names.push(outpath.to_str().unwrap().to_string());

        if file.name().ends_with('/') {
            std::fs::create_dir_all(&outpath)?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    std::fs::create_dir_all(&p)?;
                }
            }
            let mut outfile = File::create(&outpath)?;
            copy(&mut file, &mut outfile)?;
        }
    }
    Ok(file_names)
}
