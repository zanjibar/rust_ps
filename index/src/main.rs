use std::time::{Instant};
//use std::io::{self, BufRead};

fn main() {
    // 実行時間の計測
    let start = Instant::now();

    println!("Hello, world!");

    let end = start.elapsed();
    println!();
    println!("{}.{:03}秒経過しました。", end.as_secs(), end.subsec_nanos() / 1_000_000);
}
