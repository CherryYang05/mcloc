use std::{
    collections::{VecDeque},
    fs::{self},
};

// 创建一些文件
fn mk_file(t: &str, num: u32) {
    let path = "./test/";
    for i in 1..=num {
        let file_path = format!("{}{:0>3}.{}", path, i, t);
        println!("file {} create done!", file_path);
        std::fs::write(file_path, "").unwrap();
    }
}

// 创建一些目录
fn mk_dir(depth: u32, num: u32) {
    let mut path = String::from("./test/");
    for i in 0..depth {
        for j in 1..=num {
            let dir_path = format!("{}dir{}-{}", path, i, j);
            println!("dir {} create done!", dir_path);
            fs::create_dir(dir_path).unwrap();
        }
        path.insert_str(path.len(), format!("dir{}-{}/", i, 1).as_str());
    }
}

fn clear_mk() {
    let dir = "./test";
    fs::remove_dir_all(dir).unwrap();
}

// 写入文件
fn write_file() {
    todo!()
}

fn queue() {
    let mut queue: VecDeque<String> = VecDeque::new();

    let cur_dir = "test".to_string();
    // let mut dir = fs::read_dir(cur_dir).unwrap();
    queue.push_back(cur_dir);
    while !queue.is_empty() {
        let cur_dir = queue.pop_front().unwrap();
        let dir = fs::read_dir(cur_dir).unwrap();
        dir.for_each(|file| {
            let file = file.unwrap();
            if file.metadata().unwrap().is_dir() {
                queue.push_back(file.path().to_string_lossy().to_string());
                println!("{} is a dir", file.file_name().to_string_lossy().to_string());
            }
        });
    }
    for i in queue {
        println!("{}", i);
    }
}

#[cfg(test)]
mod test {
    use std::process::Command;

    use super::*;
    #[test]
    fn test() {
        mk_file("c", 5);
        mk_dir(3, 3);
    }

    #[test]
    fn clear_mk() {
        clear_mk();
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

    #[test]
    fn test_queue() {
        queue();
    }

    #[test]
    fn global_test() {
        // let output = Command::new("ls").output().expect("命令不合法");
        let output = Command::new("cargo run")
            .arg(" src/main.rs")
            .output()
            .expect("命令不合法");
        println!("{:?}", String::from_utf8(output.stdout).unwrap());
    }
}
