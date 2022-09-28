// 用于统计单个文件的代码信息
pub struct FileInfo {
    pub file_name: String,
    pub file_type: String,
    pub lines: usize,
    pub blank: usize,
    pub comment: usize,
}