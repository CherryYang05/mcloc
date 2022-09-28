// 创建一些文件
fn mk_file(t: &str, num: u32) {
    let path = "./test/";
    for i in 1..=num {
        let file_path = format!("{}{:0>3}.{}", path, i.to_string().as_str(), t);
        println!("file {} create done!", file_path);
        std::fs::write(file_path, "").unwrap();
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
        let data = std::fs::metadata("src/main.rs").unwrap();
        let file = std::fs::File::open("src/main.rs").unwrap();
        println!("{:?}", data);
        println!("{:?}", file);
        let file = std::fs::File::open("src/lib.rs").unwrap();
        println!("{:?}", file);
    }
}
