use crate::cloc_result::ClocResult;


pub fn print_res(res: &ClocResult) {
    println!("有效代码共有：{} 行", res.total_lines);
    println!("文件数量为：{}", res.file_num);
    println!("被忽略文件数量为：{}", res.ignored_file_num);
    println!("文件：{:?}", res.each_file)
}