use std::{error::Error, fs::metadata, time::Instant};

use cloc_result::ClocResult;

pub mod cloc_result;
pub mod error;
pub mod file;
pub mod print;
pub mod test;

// 命令行信息结构体
pub struct Config {
    pub file_path: Vec<String>,
    pub dir_path: Vec<String>,
    pub valid_file: Vec<String>,
}

impl Config {
    pub fn new<'a>(mut args_list: std::env::Args) -> Result<Self, &'a str> {
        // 跳过第一个参数(该程序的二进制文件名, arg[0])
        args_list.next();
        if args_list.len() == 0 {
            return Err("请输入至少一个文件路径");
        }
        let mut file_path: Vec<String> = vec![];
        let mut dir_path: Vec<String> = vec![];
        let valid_file: Vec<String> = vec![];
        // 将参数中的路径分成文件和目录两类存放
        while args_list.len() > 0 {
            match args_list.next() {
                Some(path) => {
                    let file_metadata = match metadata(&path) {
                        Ok(metadata) => metadata,
                        Err(_) => {
                            return Err("文件读取失败");
                        }
                    };
                    if file_metadata.is_dir() {
                        dir_path.push(path);
                    } else if file_metadata.is_file() {
                        file_path.push(path);
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

pub fn run(config: Config) -> Result<ClocResult, Box<dyn Error>> {
    // 将目录递归展开

    for file in config.file_path.iter() {
        let file_type: Vec<_> = file.split('.').collect();
        if file_type.len() < 2 {
            println!("忽略文件：{}", file);
        } else {
            if file_type[file_type.len() - 1] == "rs" {
                println!("该文件为 Rust 文件");
            }
        }
    }
    calc_file(config)
    // Ok(calc_file(config)?)
}

pub fn calc_file(config: Config) -> Result<ClocResult, Box<dyn Error>> {
    let mut cloc_result = ClocResult::new();
    let time_start = Instant::now();
    for item in config.file_path.iter() {
        let file = std::fs::read_to_string(item)?;
        // thread::sleep(Duration::new(1, 0));
        let lines = file
            .lines()
            .filter(|lines| !lines.trim().is_empty())
            .count();
        cloc_result.total_lines += lines;
        println!("文件 {} 的有效代码行数为：{}", item, lines);
    }
    let time_duration = Instant::now() - time_start;
    cloc_result.time = time_duration;
    println!("总用时：{:.3?}", time_duration);
    Ok(cloc_result)
}
