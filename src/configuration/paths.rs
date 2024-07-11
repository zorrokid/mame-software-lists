pub const PATHS_CONFIG_PATH: &str = "configs/paths.json";

#[derive(serde::Deserialize, Debug, Clone)]
pub struct Paths {
    pub software_lists_data_files_folder: String,
    pub software_lists_roms_folder: String,
}

pub fn read_paths() -> Paths {
    let file = std::fs::File::open(PATHS_CONFIG_PATH).unwrap();
    let reader = std::io::BufReader::new(file);
    serde_json::from_reader(reader).unwrap()
}
