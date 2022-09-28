use std::{
    env::args,
    process,
};

use mcloc::Config;

fn main() {
    let config = Config::new(args()).unwrap_or_else(|err| {
        println!("参数解析错误：{}", err);
        process::exit(0);
    });
    
    // println!("第一个参数：{:?}, 第二个参数：{:?}", args().nth(0), args().nth(1));
    if let Err(e) = mcloc::run(config) {
        println!("程序运行出错: {}", e);
        process::exit(1);
    }
}
