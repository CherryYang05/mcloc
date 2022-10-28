use std::{
    collections::{HashMap},
    path::PathBuf,
    time::Duration, ops::{IndexMut, Index},
};

// 代码统计结果结构体
pub struct ClocResult {
    pub each_file: Vec<PathBuf>,                // 存储每一个文件的路径
    pub dir_num: usize,                         // 目录的数量
    pub file_num: usize,                        // 所有文件数量
    pub file_type_num: usize,                   // 文件种类数量
    pub file_type: HashMap<String, TypeResult>, // 语言类型 HashMap
    pub ignored_file: Vec<String>,              // 忽略的文件
    pub ignored_file_num: usize,                // 忽略的文件数量
    pub total_lines: usize,                     // 代码总行数
    pub total_blank: usize,                     // 代码总空行数
    pub total_comment: usize,                   // 代码总注释行数
    pub time: Duration,                         // 统计总耗时
}

// 每种语言的结构体统计结果
pub struct TypeResult {
    pub code_type: String,
    pub file_nums: usize,
    pub code_lines: usize,
    pub blank_lines: usize,
    pub comment_lines: usize,
}

impl ClocResult {
    pub fn new() -> Self {
        ClocResult {
            each_file: vec![],
            dir_num: 0,
            file_num: 0,
            file_type_num: 0,
            file_type: HashMap::new(),
            ignored_file: vec![],
            ignored_file_num: 0,
            total_lines: 0,
            total_blank: 0,
            total_comment: 0,
            time: Duration::new(0, 0),
        }
    }
}

impl TypeResult {
    pub fn new(language_type: String) -> Self {
        TypeResult {
            code_type: String::from(language_type),
            file_nums: 0,
            code_lines: 0,
            blank_lines: 0,
            comment_lines: 0,
        }
    }
}

