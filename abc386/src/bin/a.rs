fn main() {
    println!("整数を2つ入力してください");
    proconio::input! {
        a: i32,
        b: i32,
        c: i32,
        d: i32,
    }
    let mut l = Vec::new();
    for i in 0..13 {
        l.push(i);
    }

    println!("あいうえおd{}", a + b + c + d);

}
