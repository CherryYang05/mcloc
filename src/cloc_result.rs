use std::time::Duration;

use crate::file::FileInfo;


// 代码统计结果结构体
pub struct ClocResult {
    pub each_file: Vec<FileInfo>,
    pub dir_num: usize,
    pub file_num: usize,
    pub file_type_num: usize,
    pub file_type: Vec<String>,
    pub total_lines: usize,
    pub total_blank: usize,
    pub total_comment: usize,
    pub time: Duration,
}

impl ClocResult {
    pub fn new() -> Self {
        ClocResult {
            each_file: vec![],
            dir_num: 0,
            file_num: 0,
            file_type_num: 0,
            file_type: vec![],
            total_lines: 0,
            total_blank: 0,
            total_comment: 0,
            time: Duration::new(0, 0),
        }
    }
}