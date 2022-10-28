use crate::cloc_result::ClocResult;

static DASHES: &str = "-------------------------------------------------------------------------------";

pub fn print_res(res: &ClocResult) {
    println!("有效代码共有：{} 行", res.total_lines);
    println!("文件数量为：{}", res.file_num);
    println!("被忽略文件数量为：{}", res.ignored_file_num);
    println!("文件：{:?}", res.each_file);

    print_head();
    print_body(res);
    print_tail(res);
}

fn print_head() {
    println!(
        "{:79}\n{:19}{:>15}{:>15}{:>15}{:>15}\n{:79}",
        DASHES,
        "Language", "files", "blank", "comment", "code",
        DASHES
    )
}

fn print_body(res: &ClocResult) {
    let code_type = &res.file_type;
    for (typee, type_result) in code_type {
        println!(
            "{:19}{:>15}{:>15}{:>15}{:>15}",
            typee, 
            type_result.file_nums,
            type_result.blank_lines,
            type_result.comment_lines,
            type_result.code_lines
        )
    }
}

fn print_tail(res: &ClocResult) {
    println!(
        "{:79}\n{:19}{:>15}{:>15}{:>15}{:>15}\n{:79}",
        DASHES,
        "SUM:",
        res.file_num,
        res.total_blank,
        res.total_comment,
        res.total_lines,
        DASHES
    )
}