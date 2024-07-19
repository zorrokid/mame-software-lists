use crate::configuration::paths::read_paths;
use crate::data_access::data_access_provider::DataAccessTrait;
use crate::files::scan_archives::scan_archives;
use crate::models::SoftwareList;
use std::path::PathBuf;

pub struct SoftwareListScannerError {
    pub message: String,
}

pub struct SoftwareListFileScanner<'a> {
    data_access: &'a mut dyn DataAccessTrait,
}

impl<'a> SoftwareListFileScanner<'a> {
    pub fn new(data_access: &'a mut dyn DataAccessTrait) -> Self {
        Self { data_access }
    }

    fn generate_path(&self, software_list: &SoftwareList) -> PathBuf {
        let paths = read_paths();
        let mut path = PathBuf::from(paths.software_lists_roms_folder);
        path.push(software_list.name.clone());
        path
    }

    pub fn scan_files_for_software_list(
        &mut self,
        software_list: &SoftwareList,
    ) -> Result<(), SoftwareListScannerError> {
        let path = self.generate_path(&software_list);
        let roms_in_software_list = self
            .data_access
            .fetch_software_list_roms(software_list.id)
            .map_err(|e| SoftwareListScannerError {
                message: format!("Error fetching software list roms: {}", e.message),
            })?;
        let result =
            scan_archives(path, roms_in_software_list).map_err(|e| SoftwareListScannerError {
                message: format!("Error scanning archives: {}", e.message),
            })?;
        self.data_access
            .set_matched_roms(&result.matched_rom_ids)
            .map_err(|e| SoftwareListScannerError {
                message: format!("Error setting matched roms: {}", e.message),
            })?;
        Ok(())
    }
}
