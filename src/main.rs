use std::{path::Path, time::Instant};

use crate::search::{search_key, SearchResult};

#[path = "./tools/file_tools.rs"]
mod file_tools;
#[path = "./tools/search.rs"]
mod search;

fn run_search() -> Instant {
    let now = Instant::now();
    let profiles = search::get_profiles().unwrap();
    let file_list = search::get_path_list(
        Path::new(&profiles.entry),
        &profiles.include_file,
        &profiles.exclude_file,
        &profiles.exclude_extension,
    );

    let mut result_object: Vec<SearchResult> = vec![];
    let mut searched_lines: usize = 0;
    let mut founded_key_file_num: usize = 0;
    for file_path in &file_list {
        // 排除output文件
        if file_path.contains(&profiles.output) {
            continue;
        }
        let mut result = search_key(&Path::new(file_path), &profiles.key);
        searched_lines += result.total_lines;
        if !result.search_result_list.is_empty() {
            founded_key_file_num += 1;
            result_object.append(&mut result.search_result_list);
        }
    }

    // file_tools::write_json_to(&Path::new(&profiles.output), &file_list);
    file_tools::write_json_to(&Path::new(&profiles.output), &result_object);

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
