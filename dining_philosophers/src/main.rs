use std::sync::{Mutex, Arc};
use std::thread;
use std::time::Duration;

// 構造体
struct Philosopher {
    name: String,
    left: usize,  // forksのインデックス
    right: usize,
}

struct Table {
    // Mutexは並行処理を制御する機構
    // 1スレッドしかアクセスできない
    forks: Vec<Mutex<()>>,  // 中身は空タプル
}

// Philosopher構造体に関する定義
impl Philosopher {
    // 関連関数
    // new はただの慣習
    // Stringを受け付けると呼び出し元でメソッドを呼ばないとになる
    fn new(name: &str, left: usize, right: usize) -> Philosopher {
        // 最後の式が返す値が戻り値となる
        Philosopher {
            name: name.to_string(),  // &nameの参照先のコピーを作る
            left: left,
            right: right,
        }
    }

    fn eat(&self, table: &Table) {
        // フォークを使う（ロックする）
        let _left = table.forks[self.left].lock().unwrap();
        thread::sleep(Duration::from_millis(150));
        let _right = table.forks[self.right].lock().unwrap();

        println!("{} is eating.", self.name);

        thread::sleep(Duration::from_millis(1000));

        println!("{} is done eating.", self.name);

        // _left, _right がスコープから抜けると unlock される
    }
}

fn main() {
    // Arc = アトミック参照カウント
    // スレッドを跨いで参照をカウントするため
    let table = Arc::new(Table { forks: vec![
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
    ]});

    let philosophers = vec![
        Philosopher::new("Judith Butler",   0, 1),
        Philosopher::new("Gilles Deleuze",  1, 2),
        Philosopher::new("Karl Marx",       2, 3),
        Philosopher::new("Emma Goldman",    3, 4),
        Philosopher::new("Michel Foucault", 0, 4),
    ];

    let handles: Vec<_> = philosophers.into_iter()
        .map(|p| {  // クロージャ
            let table = table.clone();

            thread::spawn(move || {  // move アノテーション
                p.eat(&table);
            })  // セミコロンを置かずに式にしている
        }).collect();

    for h in handles {
        h.join().unwrap();
    }
}
