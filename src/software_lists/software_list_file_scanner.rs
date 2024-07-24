use crate::data_access::data_access_provider::DataAccessTrait;
use crate::files::scan_archives::scan_archives;
use crate::models::SoftwareList;
use std::path::PathBuf;

pub struct SoftwareListScannerError {
    pub message: String,
}

pub struct SoftwareListFileScanner<'a> {
    data_access: &'a mut dyn DataAccessTrait,
    software_list_rom_folder: &'a PathBuf,
}

impl<'a> SoftwareListFileScanner<'a> {
    pub fn new(
        data_access: &'a mut dyn DataAccessTrait,
        software_list_rom_folder: &'a PathBuf,
    ) -> Self {
        Self {
            data_access,
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
    ) -> Result<(), SoftwareListScannerError> {
        let path = self.generate_path(&software_list);
        let result = scan_archives(path).map_err(|e| SoftwareListScannerError {
            message: format!("Error scanning archives: {}", e.message),
        })?;
        self.data_access
            .set_matched_roms(&software_list, &result.found_checksums)
            .map_err(|e| SoftwareListScannerError {
                message: format!("Error setting matched roms: {}", e.message),
            })?;
        Ok(())
    }
}
