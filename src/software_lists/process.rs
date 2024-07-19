use crate::data_access::data_access_provider::DataAccessTrait;
use crate::xml_parser::parse_file;

pub struct ProcessSoftwareListError {
    pub message: String,
}

pub fn process_from_datafile(
    data_access: &mut dyn DataAccessTrait,
    path: String,
) -> Result<(), ProcessSoftwareListError> {
    match parse_file(&path) {
        Ok(datafile) => {
            match data_access.software_list_exists(
                datafile.header.name.clone(),
                datafile.header.version.clone(),
            ) {
                true => Err(ProcessSoftwareListError {
                    message: "Software list already exists".to_string(),
                }),

                false => data_access.process_software_list(&datafile).map_err(|e| {
                    ProcessSoftwareListError {
                        message: format!("Error processing software list: {}", e.message),
                    }
                }),
            }
        }
        Err(e) => Err(ProcessSoftwareListError {
            message: format!("Error parsing file: {}", e),
        }),
    }
}
