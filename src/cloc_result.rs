use std::{path::PathBuf, time::Duration, collections::{HashSet, hash_set}};

// 代码统计结果结构体
pub struct ClocResult {
    pub each_file: Vec<PathBuf>,        // 存储每一个文件的路径
    pub dir_num: usize,                 // 目录的数量
    pub file_num: usize,                // 所有文件数量
    pub file_type_num: usize,           // 文件种类数量
    pub file_type: HashSet<String>,     // 文件类型 set
    pub ignored_file: Vec<String>,      // 忽略的文件
    pub ignored_file_num: usize,        // 忽略的文件数量
    pub total_lines: usize,             // 代码总行数
    pub total_blank: usize,             // 代码总空行数
    pub total_comment: usize,           // 代码注释行数
    pub time: Duration,                 // 统计总耗时
}

impl ClocResult {
    pub fn new() -> Self {
        ClocResult {
            each_file: vec![],
            dir_num: 0,
            file_num: 0,
            file_type_num: 0,
            file_type: HashSet::new(),
            ignored_file: vec![],
            ignored_file_num: 0,
            total_lines: 0,
            total_blank: 0,
            total_comment: 0,
            time: Duration::new(0, 0),
        }
    }
}
