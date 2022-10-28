#[macro_use]
extern crate lazy_static;

use std::{
    collections::{HashMap, VecDeque},
    fs::{self, read_to_string},
    path::PathBuf,
    time::Instant,
};

use cloc_result::ClocResult;
use errors::ClocError;

use crate::cloc_result::TypeResult;

pub mod cloc_result;
pub mod errors;
pub mod file;
pub mod print;
pub mod test;

lazy_static! {
    pub static ref LANGUAGE_TYPE: HashMap<String, String> = HashMap::from([
        (String::from("rs"), String::from("Rust")),
        (String::from("c"), String::from("C")),
        (String::from("cpp"), String::from("C++")),
        (String::from("java"), String::from("Java")),
        (String::from("py"), String::from("Python")),
    ]);
}

// 命令行信息结构体，类型为 PathBuf
pub struct Config {
    pub file_path: Vec<PathBuf>,
    pub dir_path: Vec<PathBuf>,
    pub valid_file: Vec<PathBuf>,
}

impl Config {
    pub fn new<'a>(mut args_list: std::env::Args) -> Result<Self, &'a str> {
        // 跳过第一个参数(该程序的二进制文件名, arg[0])
        args_list.next();
        if args_list.len() == 0 {
            return Err("请输入至少一个文件路径");
        }
        let mut file_path: Vec<PathBuf> = vec![];
        let mut dir_path: Vec<PathBuf> = vec![];
        let valid_file: Vec<PathBuf> = vec![];
        // 将参数中的路径分成文件和目录两类存放
        while args_list.len() > 0 {
            match args_list.next() {
                Some(path) => {
                    let entry = PathBuf::from(path);
                    if entry.is_dir() {
                        dir_path.push(entry);
                    } else if entry.is_file() {
                        file_path.push(entry);
                    }
                }
                None => return Err("无法获取到文件"),
            };
        }
        println!("文件路径为：{:?}", file_path);
        println!("目录路径为：{:?}", dir_path);

        Ok(Config {
            file_path,
            dir_path,
            valid_file,
        })
    }
}

pub fn run(mut config: Config) -> Result<ClocResult, ClocError> {
    // 将目录递归展开，使用队列(crates.io 中有 "walkDir" 这个 crate，这里我们先手动实现)
    let mut queue: VecDeque<PathBuf> = VecDeque::new();
    for i in config.dir_path.iter() {
        queue.push_back(i.clone());
    }

    while !queue.is_empty() {
        let cur_dir = queue.pop_front().unwrap();
        for entry in fs::read_dir(cur_dir)? {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                queue.push_back(entry.path());
                config.dir_path.push(entry.path());
            } else if entry.file_type()?.is_file() {
                config.file_path.push(entry.path());
            }
        }
    }

    calc_file(config)
    // Ok(calc_file(config)?)
}

pub fn calc_file(config: Config) -> Result<ClocResult, ClocError> {
    let mut cloc_result = ClocResult::new();
    let time_start = Instant::now();

    for file in config.file_path.iter() {
        let file_name = file.to_string_lossy();
        let file_type: Vec<_> = file_name.split('.').collect();
        if file_type.len() < 2 {
            cloc_result.ignored_file_num += 1;
            println!("忽略文件：{:?}", file);
        } else {
            let language_type = file_type[file_type.len() - 1];
            // 如果是不支持的语言，返回 FileNotSupported 错误
            if !LANGUAGE_TYPE.contains_key(language_type) {
                return Err(ClocError::FileNotSupported);
            }
            // 如果 cloc_result 中第一次出现该类型语言，则新建对应的结构体
            if !cloc_result.file_type.contains_key(language_type) {
                let new_type_result = TypeResult::new(LANGUAGE_TYPE[language_type].clone());
                cloc_result
                    .file_type
                    .insert(language_type.to_string(), new_type_result);
            }
            // 处理该文件，统计代码信息，并将结果写入结构体
            add_to_result(file, language_type, &mut cloc_result);
        }
    }

    let time_duration = Instant::now() - time_start;
    cloc_result.time = time_duration;
    println!("总用时：{:.3?}", time_duration);
    Ok(cloc_result)
}

// 将遍历到的每一个文件进行统计，加入结果之中
pub fn add_to_result(file_name: &PathBuf, language_type: &str, cloc_result: &mut ClocResult) -> Result<(), ClocError>{
    println!("当前文件为 {:?}", file_name);
    let file = read_to_string(file_name)?;
    // thread::sleep(Duration::new(1, 0));
    cloc_result.each_file.push(file_name.to_path_buf());
    let lines = file
        .lines()
        .filter(|lines| !lines.trim().is_empty())
        .count();
        cloc_result.file_type.get_mut(language_type).unwrap().file_nums += 1;
    // FIXME
    cloc_result.file_type.get_mut(language_type).unwrap().code_lines = lines;
    cloc_result.total_lines += lines;
    println!("文件 {:?} 的有效代码行数为：{}", file_name, lines);
    Ok(())
}

// 处理文件后缀，包括检测语言
// pub fn check_suffix() -> Result<ClocResult, Box<dyn Error>> {

// }
