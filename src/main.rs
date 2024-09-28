use std::{
    env::current_dir,
    error::Error,
    fs::{read_dir, read_to_string, DirEntry},
    path::PathBuf,
};

use config_parser::ServerConfig;

mod config_parser;

fn main() -> Result<(), Box<dyn Error>> {
    let current_directory = current_dir()?;

    fn is_config_file(dir_entry: &DirEntry) -> bool {
        let is_file = dir_entry.file_type().map(|f| f.is_file()).ok();
        let is_json = dir_entry.file_name().to_str().map(|s| s.ends_with(".json"));

        matches!((is_file, is_json), (Some(true), Some(true)))
    }

    for path in ["servers", "sharded_clusters", "replica_sets"] {
        let server_config_directory = PathBuf::from_iter([
            current_directory.clone(),
            "mongo-orchestration/mongo_orchestration/configurations".into(),
            path.into(),
        ]);

        for dir_entry in
            read_dir(server_config_directory.clone())?.filter_map(|r| r.ok().filter(is_config_file))
        {
            let name = PathBuf::from_iter([
                server_config_directory.clone(),
                dir_entry.file_name().into(),
            ]);
            let contents = read_to_string(name.clone())?;

            let result: Result<ServerConfig, _> = serde_json::from_str(&contents);
            if result.is_err() {
                println!("{} failed.", name.display());
                println!("--- {}", result.err().unwrap());
            }
        }
    }

    for path in ["servers", "sharded_clusters", "replica_sets"] {
        let server_config_directory = PathBuf::from_iter([
            current_directory.clone(),
            "drivers-evergreen-tools/.evergreen/orchestration/configs".into(),
            path.into(),
        ]);

        for dir_entry in
            read_dir(server_config_directory.clone())?.filter_map(|r| r.ok().filter(is_config_file))
        {
            let name = PathBuf::from_iter([
                server_config_directory.clone(),
                dir_entry.file_name().into(),
            ]);
            let contents = read_to_string(name.clone())?;

            let result: Result<ServerConfig, _> = serde_json::from_str(&contents);
            if result.is_err() {
                println!("{} failed.", name.display());
                println!("--- {}", result.err().unwrap());
            }
        }
    }

    Ok(())
}
