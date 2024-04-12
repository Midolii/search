use serde::{Deserialize, Serialize};
use std::{
    env,
    fs::{self, read, read_dir},
    io::BufRead,
    path::Path,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Profiles {
    pub key: Option<String>,
    pub entry: Option<String>,
    pub output: Option<String>,
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
            let mut _read_path: &Path = Path::new("./");
            match Path::new(DEFAULT_APPLICATION_YML).exists() {
                true => {
                    _read_path = Path::new(DEFAULT_APPLICATION_YML);
                }
                false => {
                    return None;
                }
            }
            let profiles: Profiles = serde_yaml::from_str(
                fs::read_to_string(_read_path)
                    .expect("没有找到配置文件，请前往 https://github.com/Midolii/search 查看文档")
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
    entry: Option<&Path>,
    include_file: &Option<Vec<String>>,
    exclude_file: &Option<Vec<String>>,
    exclude_extension: &Option<Vec<String>>,
) -> Vec<String> {
    let mut result: Vec<String> = vec![];
    match entry {
        Some(entry) => {
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
                    match &entry.extension() {
                        Some(extension) => {
                            if exclude.contains(&extension.to_str().unwrap().to_owned()) {
                                return result;
                            }
                        }
                        None => {
                            return result;
                        }
                    }
                }
                result.push(entry.to_str().unwrap().to_owned());
            }
            if entry.is_dir() {
                let files = read_dir(entry).unwrap();
                for file_name in files {
                    if let Ok(file) = file_name {
                        result.append(&mut get_path_list(
                            Some(&file.path()),
                            include_file,
                            exclude_file,
                            exclude_extension,
                        ));
                    }
                }
            }
        }
        None => {}
    }
    result
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchResult {
    path: String,
    line: usize,
    content: String,
}

pub struct SearchResultWithLines {
    pub search_result_list: Vec<SearchResult>,
    pub total_lines: usize,
}

/// 根据传入的文件路径和关键字在文件中进行查找对应行，最终返回所有匹配到的行以及文件行数，如果没有匹配到的结果则也会返回文件总行数
pub fn search_key(target: &Path, key: &Option<String>) -> SearchResultWithLines {
    let mut result: Vec<SearchResult> = vec![];
    let mut line_num = 0;
    match key {
        Some(key) => match read(target) {
            Ok(reader) => {
                reader.lines().for_each(|line| match line {
                    Ok(line) => {
                        line_num += 1;
                        if line.contains(key) {
                            result.push(SearchResult {
                                path: target.to_str().unwrap().to_owned(),
                                line: line_num,
                                content: line.trim().to_owned(),
                            })
                        }
                    }
                    Err(_) => {}
                });
            }
            Err(_) => {}
        },
        None => {}
    }
    SearchResultWithLines {
        search_result_list: result,
        total_lines: line_num,
    }
}
