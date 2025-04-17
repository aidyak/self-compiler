use std::env;
use std::process;

fn main() {
    // コマンドライン引数を取得
    let args: Vec<String> = env::args().collect();

    // 引数が2つでなければエラー出力して終了
    if args.len() != 2 {
        eprintln!("引数の個数が正しくありません");
        process::exit(1);
    }

    // 引数を整数に変換（失敗したら終了）
    let num: i32 = match args[1].parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("数値のパースに失敗しました");
            process::exit(1);
        }
    };

    // アセンブリコードを出力
    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");
    println!("  mov rax, {}", num);
    println!("  ret");
}

