use std::{path::Path, time::Instant};

use crate::search::{search_key, Profiles, SearchResult};

#[path = "./tools/file_tools.rs"]
mod file_tools;
#[path = "./tools/search.rs"]
mod search;

fn run_search() -> Instant {
    let now = Instant::now();
    let profiles = search::get_profiles().unwrap_or(Profiles {
        key: None,
        entry: Some("./".to_owned()),
        output: None,
        include_file: Some(vec![]),
        exclude_file: Some(vec![]),
        exclude_extension: Some(vec![]),
    });
    let file_list = search::get_path_list(
        Some(Path::new(&profiles.entry.unwrap_or("./".to_owned()))),
        &profiles.include_file,
        &profiles.exclude_file,
        &profiles.exclude_extension,
    );

    let mut result_object: Vec<SearchResult> = vec![];
    let mut searched_lines: usize = 0;
    let mut founded_key_file_num: usize = 0;
    for file_path in &file_list {
        if let Some(output) = &profiles.output {
            // 排除output文件
            if file_path.contains(output) {
                continue;
            }
        }
        let mut result = search_key(&Path::new(file_path), &profiles.key);
        searched_lines += result.total_lines;
        if !result.search_result_list.is_empty() {
            founded_key_file_num += 1;
            result_object.append(&mut result.search_result_list);
        }
    }

    match profiles.output {
        Some(output) => {
            file_tools::write_json_to(&Path::new(&output), &result_object);
        }
        None => {
            println!("{:#?}", serde_json::to_string(&result_object));
        }
    }

    println!(
        "本次搜索 {:#?} 个文件，{:#?} 行, 在 {:#?} 个文件中发现结果",
        file_list.len(),
        searched_lines,
        founded_key_file_num
    );
    now
}

fn main() {
    println!("方法 run_search 执行时间：{:#?}", run_search().elapsed());
}
