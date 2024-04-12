use std::path::Path;

#[path = "./tools/file_tools.rs"]
mod file_tools;
#[path = "./tools/search.rs"]
mod search;

fn main() {
    let profiles = search::get_profiles().unwrap();
    let result = search::get_path_list(
        Path::new(&profiles.entry),
        &profiles.include_file,
        &profiles.exclude_file,
        &profiles.exclude_extension,
    );

    file_tools::write_json_to(&Path::new(&profiles.output), &result);

    println!("{:#?}", result);
    println!(
        "在{:#?}目录下一共找到{:#?}个文件",
        profiles.entry,
        result.len()
    );
}
