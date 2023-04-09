// vecs2.rs
// A Vec of even numbers is given. Your task is to complete the loop
// so that each number in the Vec is multiplied by 2.
//
// Make me pass the test!
//
// Execute `rustlings hint vecs2` or use the `hint` watch subcommand for a hint.


fn vec_loop(mut v: Vec<i32>) -> Vec<i32> {
    for i in v.iter_mut() {
        // TODO: Fill this up so that each element in the Vec `v` is
        // multiplied by 2.
        *i *= 2;
    }

    // At this point, `v` should be equal to [4, 8, 12, 16, 20].
    v
}

// vec_map 接受一个 &Vec<i32> 类型的参数，使用 iter 方法获取到可迭代的 v.iter() 来遍历数组中每个元素，
// 对于每个元素进行 *2 操作后，使用 collect 方法将处理后的结果重新构造成一个新的 Vec<i32>。
fn vec_map(v: &Vec<i32>) -> Vec<i32> {
    v.iter()
        .map(|num| {
            // TODO: Do the same thing as above - but instead of mutating the
            // Vec, you can just return the new number!
            // num = num * 2
            num * 2 // 构造数组。
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_loop() {
        /*
        1.. 表示从数字 1 开始构造一个无限递增的迭代器。
        filter(|x| x % 2 == 0) 表示对该迭代器中每个元素进行过滤，只保留其中的偶数，即对于每个元素 x，只有在 x % 2 == 0 的情况下才会保留这个元素。
        take(5) 表示从过滤得到的结果中选择前五个元素。
        最后使用 collect() 将这五个元素保存到一个新的 Vec<i32> 中并返回。
        什么时候停止迭代？ 
            1. 在这段 Rust 代码中，迭代器的停止条件是 take(5) 方法，用于限制只迭代前 5 个偶数
            。因此，当取得了五个偶数后，迭代器就会自动停止迭代，不再继续处理后续的偶数。
        */
        let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
        let ans = vec_loop(v.clone());

        assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
    }

    #[test]
    fn test_vec_map() {
        let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
        let ans = vec_map(&v);

        assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
    }
}
