use std::io;

fn main() {
    // 標準入力から1行読み込む
    // 数字のみを受け取り、その他の場合は再度入力を求める
    // 0番目は許容しない
    let mut input = String::new();
    let n: i32;
    println!("n番目のフィボナッチ数を求めます。");
    println!("nにあたる数字を入力してください。");
    loop {

        io::stdin().read_line(&mut input)
            .expect("Failed to read line");
        
        if input.trim() == "0" {
            println!("0は許容しません。");
            input.clear();
            continue;
        }

        let _: i32 = match input.trim().parse() {
            Ok(num) => {
                // 2147483647を超える場合は∞を返す
                if num > 46 {
                    println!("46番目までしか計算できません。");
                    input.clear();
                    continue;
                } else {
                    n = num;
                    break;
                
                }
            }
            Err(_) => {
                println!("数字のみを入力してください。");
                input.clear();
                continue;
            },
        };
    }
    println!("n番目の数はこちらです。: {}", calc_fibonacci(n));
}

fn calc_fibonacci(n: i32) -> i32 {
    // n番目のフィボナッチ数を求める
    // nが0または1の場合はnを返す
    // それ以外の場合は、n-1番目とn-2番目のフィボナッチ数の和を返す
    if n == 0 || n == 1 {
        return n;
    } else {
        return calc_fibonacci(n - 1) + calc_fibonacci(n - 2);
    }
}