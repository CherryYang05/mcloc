use crate::cloc_result::ClocResult;


pub fn print_res(res: ClocResult) {
    println!("有效代码共有：{} 行", res.total_lines);
}