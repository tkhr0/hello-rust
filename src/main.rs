use std::io;  // ioライブラリ読み込み

fn main() {   // main がエントリーポイント
    println!("Guess the number!");

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
