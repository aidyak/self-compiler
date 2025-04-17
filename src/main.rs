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
    // 数式を文字列として取得
    let mut p = args[1].as_str();
    // アセンブリコードを出力
    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");

    // 引数を整数に変換（失敗したら終了）
    let (value, rest) = parse_number(p);
    println!("  mov rax, {}", value);
    p = rest;

    while !p.is_empty() {
        let mut chars = p.chars();
        match chars.next() {
            Some('+') => {
                let (num, rest) = parse_number(chars.as_str());
                println!("  add rax, {}", num);
                p = rest;
            }
            Some('-') => {
                let (num, rest) = parse_number(chars.as_str());
                println!("  sub rax, {}", num);
                p = rest;
            }
            Some(c) => {
                eprintln!("Unexpected operator: {}", c);
                process::exit(1)
            }
            None => break,
        }
    }

    println!("  ret");
}

fn parse_number(s: &str) -> (i64, &str) {
    let mut end = 0;
    for (i, c) in s.char_indices() {
        if !c.is_ascii_digit() {
            break;
        }
        end = i + c.len_utf8();
    }

    if end == 0 {
        eprintln!("Failed to get number: {}", s);
        process::exit(1);
    }

    let (num_str, rest) = s.split_at(end);
    let value = num_str.parse::<i64>().unwrap_or_else(|_| {
        eprintln!("Failed to parse number: {}", num_str);
        process::exit(1);
    });
    (value, rest)
}
