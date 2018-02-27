extern crate rand;  // クレートライブラリ読み込み

use std::io;  // ioライブラリ読み込み
use std::cmp::Ordering;
use rand::Rng;  // メソッドを呼ぶためにトレイトを宣言する

fn main() {   // main がエントリーポイント
    println!("Guess the number!");

    let secret_number = rand::thread_rng()  // 現在のスレッドに乱数生成器をコピー
        .gen_range(1, 101);  // 1~100 の乱数

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        // 変数束縛
        // mut でミュータブル（可変）
        // 型は型推論で決まってる
        let mut guess = String::new();  // 空文字列を束縛する

        io::stdin()                          // 標準入力のハンドル
            .read_line(&mut guess)           // メソッド呼び出し
            .expect("Failed to read line");  // io::Result.expect()
        // 呼ばないとwarning

        // 変数を再定義 シャドーイング
        // 変数の型を変えるため
        let guess: u32 = guess.trim()  // 改行とかを削除
            .parse()                   // 文字列を数値にする
            .expect("Please type a number!");

        println!("You guessed: {}", guess);  // {} がプレースホルダ

        // 条件分岐
        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => println!("You win!"),
        }
    }
}
