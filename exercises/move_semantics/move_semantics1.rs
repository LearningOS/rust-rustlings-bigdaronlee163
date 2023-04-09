// move_semantics1.rs
// Execute `rustlings hint move_semantics1` or use the `hint` watch subcommand for a hint.


fn main() {
    let vec0 = Vec::new();

    let mut vec1 = fill_vec(vec0); // 错误发生在13行，但是需要在声明 vec1的地方更改。

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    // vec0.push(88);
}

// 这会将vec0的所有转转移到该函数内。
fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
