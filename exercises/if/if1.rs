// if1.rs
// Execute `rustlings hint if1` or use the `hint` watch subcommand for a hint.



pub fn bigger(a: i32, b: i32) -> i32 {
    if a > b {
        a 
    } else {
        b 
    }
    // 完成这个函数使它返回一个较大值
    // 不要使用：
    // - 其它函数调用
    // - 额外的变量
}

// Don't mind this for now :)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(10, bigger(10, 8));
    }

    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, bigger(32, 42));
    }
}

