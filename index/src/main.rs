//use std::env;
use std::time::{Instant};
use std::io::{self, BufRead};


fn main() {
    // 経過時間の取得
    let start = Instant::now();
    let stdin = io::stdin();

    let mut  _i =0;

    for _line in stdin.lock().lines() {
        _i = _i + 1;
       // println!("{}", _line.unwrap());
   }
        println!();
        println!("{}", _i);
        println!();

//    let args: Vec<String> = env::args().collect();
//    println!("{}",args[1]);
//    println!("");
//    println!("pseinfo -rfd");



    let end = start.elapsed();
    println!("{}.{:03}秒経過しました。", end.as_secs(), end.subsec_nanos() / 1_000_000);
}
