use serde::{Deserialize, Serialize};
use std::{
    env,
    fs::{self, read_dir},
    path::Path,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Profiles {
    pub key: String,
    pub entry: String,
    pub output: String,
    pub include_file: Option<Vec<String>>,
    pub exclude_file: Option<Vec<String>>,
    pub exclude_extension: Option<Vec<String>>,
}

const DEFAULT_APPLICATION_YML: &str = "./application.yml";

/// 获取配置文件
pub fn get_profiles() -> Option<Profiles> {
    let args = env::args();
    match args.len() {
        1 => {
            let profiles: Profiles = serde_yaml::from_str(
                fs::read_to_string(Path::new(DEFAULT_APPLICATION_YML))
                    .unwrap()
                    .as_str(),
            )
            .unwrap();
            Some(profiles)
        }
        _ => None,
    }
}

/// 获取path下所有文件的名称
pub fn get_path_list(
    entry: &Path,
    include_file: &Option<Vec<String>>,
    exclude_file: &Option<Vec<String>>,
    exclude_extension: &Option<Vec<String>>,
) -> Vec<String> {
    let mut result: Vec<String> = vec![];
    // 如果是链接文件则略过
    if entry.is_symlink() {
        return result;
    }

    if let Some(exclude) = exclude_file {
        for key in exclude {
            if entry.to_str().unwrap().contains(key) {
                return result;
            }
        }
    }
    if entry.is_file() {
        if let Some(exclude) = exclude_extension {
            if exclude.contains(&entry.extension().unwrap().to_str().unwrap().to_string()) {
                return result;
            }
        }
        result.push(entry.to_str().unwrap().to_string());
    }
    if entry.is_dir() {
        let files = read_dir(entry).unwrap();
        for file_name in files {
            if let Ok(file) = file_name {
                result.append(&mut get_path_list(
                    &file.path(),
                    include_file,
                    exclude_file,
                    exclude_extension,
                ));
            }
        }
    }
    result
}
