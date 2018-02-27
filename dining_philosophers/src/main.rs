use std::sync::Mutex;
use std::thread;
use std::time::Duration;

// 構造体
struct Philosopher {
    name: String,
}

struct Table {
    // Mutexは並行処理を制御する機構
    forks: Vec<Mutex<()>>,  // 中身は空タプル
}

// Philosopher構造体に関する定義
impl Philosopher {
    // 関連関数
    // new はただの慣習
    // Stringを受け付けると呼び出し元でメソッドを呼ばないとになる
    fn new(name: &str) -> Philosopher {
        // 最後の式が返す値が戻り値となる
        Philosopher {
            name: name.to_string(),  // &nameの参照先のコピーを作る
        }
    }

    fn eat(&self) {
        println!("{} is eating.", self.name);

        thread::sleep(Duration::from_millis(1000));

        println!("{} is done eating.", self.name);
    }
}

fn main() {
    let philosophers = vec![
        Philosopher::new("Judith Butler"),
        Philosopher::new("Gilles Deleuze"),
        Philosopher::new("Karl Marx"),
        Philosopher::new("Emma Goldman"),
        Philosopher::new("Michel Foucault"),
    ];

    let handles: Vec<_> = philosophers.into_iter()
        .map(|p| {  // クロージャ
            thread::spawn(move || {  // move アノテーション
                p.eat();
            })  // セミコロンを置かずに式にしている
        }).collect();

    for h in handles {
        h.join().unwrap();
    }
}
