use std::io;

fn main() {
    println!("摂氏から華氏へ、または華氏から摂氏へ変換を行えるプログラムです。");
    println!("摂氏から華氏へ変換する場合は「1」を、華氏から摂氏へ変換する場合は「2」を入力してください。");
    // 標準入力を受け取る
    // 受け取れる値はString型の 「1」 か 「2」 のみ
    // それ以外の値が入力された場合は再度入力を促す
    let mut input = String::new();
    loop {
        io::stdin().read_line(&mut input).expect("Failed to read line");
        if input.trim() == "1" || input.trim() == "2" {
            break;
        } else {
            println!("「1」か「2」を入力してください。");
            input = String::new();
        }
    }

    // 温度を入力してもらう
    // 受け取れる値はi32型の整数のみ
    // それ以外の値が入力された場合は再度入力を促す
    println!("温度を入力してください。");
    let mut input_temp = String::new();
    let temp: i32;
    loop {
        io::stdin().read_line(&mut input_temp).expect("Failed to read line");
        let _: i32 = match input_temp.trim().parse() {
            Ok(num) => {
                temp = num;
                break;
            }
            Err(_) => {
                println!("整数を入力してください。");
                input_temp.clear();
                continue;
            },
        };
    }

    // 入力された値によって処理を分岐
    // 「1」が入力された場合は摂氏から華氏へ変換
    // 「2」が入力された場合は華氏から摂氏へ変換

    if input.trim() == "1" {
        let fuel = calc_fuel(temp);
        println!("摂氏{}度は華氏{}度です。", temp, fuel);
    } else {
        let celsius = calc_celsius(temp);
        println!("華氏{}度は摂氏{}度です。", temp, celsius);
    }
}

fn calc_fuel(celsius: i32) -> i32 {
    // 摂氏から華氏への変換式
    // F = C * 1.8 + 32
    let fuel = celsius * 18 / 10 + 32;
    fuel
}

fn calc_celsius(fahrenheit: i32) -> i32 {
    // 華氏から摂氏への変換式
    // C = (F - 32) / 1.8
    let celsius = (fahrenheit - 32) * 10 / 18;
    celsius
}