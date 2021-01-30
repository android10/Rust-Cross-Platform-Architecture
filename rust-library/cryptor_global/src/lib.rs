static CARGO_CONFIG_DIR_NAME: &str = ".cargo";
static CARGO_CONFIG_FILE_NAME: &str = "config";

pub mod io {
    use std::fs;
    use std::path::PathBuf;
    use std::fs::File;

    pub fn create_cargo_config_file(dir_path: &PathBuf) -> File {
        let config_dir = dir_path.join(super::CARGO_CONFIG_DIR_NAME);
        let config_file = config_dir.join(super::CARGO_CONFIG_FILE_NAME);

        match fs::create_dir(&config_dir) {
            Err(why) => panic!("Could not create cargo configuration directory: {}", why),
            Ok(_) => {
                match File::create(&config_file) {
                    Err(why) => panic!("Could not create cargo configuration file: {}", why),
                    Ok(file) => file,
                }
            }
        }
    }
}