use crate::files::scan_archives::{scan_archives, ScanResult};
use crate::models::SoftwareList;
use std::path::PathBuf;

pub struct SoftwareListScannerError {
    pub message: String,
}

pub struct SoftwareListScannerResult {
    pub scan_result: ScanResult,
    pub software_list: SoftwareList,
}

pub struct SoftwareListFileScanner {
    software_list_rom_folder: PathBuf,
}

impl SoftwareListFileScanner {
    pub fn new(software_list_rom_folder: PathBuf) -> Self {
        Self {
            software_list_rom_folder,
        }
    }

    fn generate_path(&self, software_list: &SoftwareList) -> PathBuf {
        let mut path = self.software_list_rom_folder.clone();
        path.push(software_list.name.clone());
        path
    }

    pub fn scan_files(
        &mut self,
        software_list: &SoftwareList,
    ) -> Result<SoftwareListScannerResult, SoftwareListScannerError> {
        let path = self.generate_path(&software_list);
        let result = scan_archives(path).map_err(|e| SoftwareListScannerError {
            message: format!("Error scanning archives: {}", e.message),
        })?;
        Ok(SoftwareListScannerResult {
            scan_result: result,
            software_list: software_list.clone(),
        })
    }
}
