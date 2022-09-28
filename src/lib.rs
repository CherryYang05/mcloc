use std::{error::Error, time::Instant};

pub mod error;
pub mod test;

// 命令行信息结构体
pub struct Config {
    file_path: Vec<String>,
    dir_path: Vec<String>,
}

impl Config {
    pub fn new(mut args_list: std::env::Args) -> Result<Self, &'static str> {
        // 跳过第一个参数(该程序的二进制文件名, arg[0])
        args_list.next();
        if args_list.len() == 0 {
            return Err("请输入至少一个文件路径");
        }
        let mut file_path: Vec<String> = vec![];
        let mut dir_path: Vec<String> = vec![];
        // 将参数中的路径分成文件和目录两类存放
        while args_list.len() > 0 {
            match args_list.next() {
                Some(path) => {
                    file_path.push(path);
                }
                None => return Err("无法获取到文件"),
            };
        }
        println!("文件路径为：{:?}", file_path);

        Ok(Config {
            file_path: file_path,
            dir_path: dir_path,
        })
    }
}

pub fn run(config: Config) -> Result<ClocResult, Box<dyn Error>> {
    let file_type: Vec<_> = config.file_path[0].split('.').collect();
    if file_type.len() < 2 {
        println!("忽略文件：{}", config.file_path[0]);
    } else {
        if file_type[file_type.len() - 1] == "rs" {
            println!("该文件为 Rust 文件");
        }
    }
    calc_file(config)
    // Ok(calc_file(config)?)
}

pub fn calc_file(config: Config) -> Result<ClocResult, Box<dyn Error>> {
    let mut cloc_result = ClocResult::new();
    let time_start = Instant::now();
    for (i, _) in config.file_path.iter().enumerate() {
        let file = std::fs::read_to_string(&config.file_path[i])?;
        // thread::sleep(Duration::new(1, 0));
        let lines = file
            .lines()
            .filter(|lines| !lines.trim().is_empty())
            .count();
        cloc_result.total_lines = cloc_result.total_lines + lines;
        println!("文件 {} 的有效代码行数为：{}", config.file_path[i], lines);
    }
    let time_duration = Instant::now() - time_start;
    println!("有效代码行数为：{}", cloc_result.total_lines);
    println!("总用时：{:.3?}", time_duration);
    Ok(cloc_result)
}

// 代码统计结果结构体
pub struct ClocResult {
    dir_num: usize,
    file_num: usize,
    file_type_num: usize,
    file_type: Vec<String>,
    total_lines: usize,
    total_blank: usize,
    total_comment: usize,
}

impl ClocResult {
    pub fn new() -> Self {
        ClocResult {
            dir_num: 0,
            file_num: 0,
            file_type_num: 0,
            file_type: vec![],
            total_lines: 0,
            total_blank: 0,
            total_comment: 0,
        }
    }
}
