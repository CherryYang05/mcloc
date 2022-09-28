// 创建一些文件
fn mk_file(t: &str, num: u32) {
    let mut path = "./test/";
    let p = 9;
    for i in 1..=num {
        let file_path = format!("{}{:0>3}.{}", path, i.to_string().as_str(), t);
        println!("file {} create done!", file_path);
        std::fs::write(file_path, "");
    }
}

// 写入文件
fn write_file() {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        mk_file("c", 5);
    }

    #[test]
    fn test_main() {
        
    }
}
