use proconio::input;
fn main(){
    println!("Hello, World");
    println!();
    println!("土");
    print!("土");
    print!("曜日");
    print!("だぞ");

    eprint!("An error");
    eprint!(" occurred");
    // エラー
    // 複数行は
    eprintln!("An error occurred");

    //整数リテラル
    println!("2{}", 3);

    //ok
    println!("{}", 2);

    //ng
    //println!(2);

    println!("2 + 3 = {}", 2 + 3);

    let length;
    length = 5;
    println!("lenght = {}", length);

    let width: i32 = 10;
    println!("width = {}", width);

    println!("\"Fool, \" said i, \"you do not know\"");

    println!(r"\\\\");

    println!(r#""Fool," said I, "you do not know ""#);
    let big_number;
    big_number = 22000000000_i64;
    println!("{}", big_number);

    println!("input dayo");
    input! {
        a: i32,
        b: i32,
        c: i32,
    }
    println!("a = {}, b = {}, c = {}", a, b, c);

}