extern crate rand;  // クレートライブラリ読み込み

use std::io;  // ioライブラリ読み込み
use rand::Rng;  // メソッドを呼ぶためにトレイトを宣言する

fn main() {   // main がエントリーポイント
    println!("Guess the number!");

    let secret_number = rand::thread_rng()  // 現在のスレッドに乱数生成器をコピー
        .gen_range(1, 101);  // 1~100 の乱数

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    // 変数束縛
    // mut でミュータブル（可変）
    let mut guess = String::new();  // 空文字列を束縛する

    io::stdin()                          // 標準入力のハンドル
        .read_line(&mut guess)           // メソッド呼び出し
        .expect("Failed to read line");  // io::Result.expect()
                                         // 呼ばないとwarning

    println!("You guessed: {}", guess);  // {} がプレースホルダ
}
