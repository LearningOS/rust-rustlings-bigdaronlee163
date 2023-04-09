// move_semantics6.rs
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand for a hint.
// You can't change anything except adding or removing references.

fn main() {
    let data = "Rust is great!".to_string(); // 声明可变变量。

    get_char(&data); // 获取变量的可变引用

    string_uppercase(data);
}

// Should not take ownership
fn get_char(data: &String) -> char {
    data.chars().last().unwrap() // 获取最后一个字符串， 无更改只是读取。
}

// Should take ownership
fn string_uppercase(mut data: String) {
    // 获取所有权。
    data = data.to_uppercase();

    println!("{}", data);
}
