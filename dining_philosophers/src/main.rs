// 構造体
struct Philosopher {
    name: String,
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
}

fn main() {
    let p1 = Philosopher::new("Judith Butler");
    let p2 = Philosopher::new("Gilles Deleuze");
    let p3 = Philosopher::new("Karl Marx");
    let p4 = Philosopher::new("Emma Goldman");
    let p5 = Philosopher::new("Michel Foucault");
}
