// options1.rs
// Execute `rustlings hint options1` or use the `hint` watch subcommand for a hint.



// This function returns how much icecream there is left in the fridge.
// If it's before 10PM, there's 5 pieces left. At 10PM, someone eats them
// all, so there'll be no more left :(
// TODO: Return an Option!
fn maybe_icecream(time_of_day: u16) -> Option<u16> {
    // 这里我们使用二十四小时制，所以 10PM 的值是22并且 12AM 的值是0
    // Option 输出需要很好地处理 time_of_day > 23 的情况。
    // TODO: 完成函数体 - 记住要返回一个 Option！
    if time_of_day > 24 {
        None
    } else if time_of_day >= 22 {
        Some(0)
    } else {
        Some(5)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_icecream() {
        assert_eq!(maybe_icecream(9), Some(5));
        assert_eq!(maybe_icecream(10), Some(5));
        assert_eq!(maybe_icecream(23), Some(0));
        assert_eq!(maybe_icecream(22), Some(0));
        assert_eq!(maybe_icecream(25), None);
    }

    #[test]
    fn raw_value() {
        // TODO: 修复这个测试。你要怎么获取 Option 内含的值？
        if let Some(icecreams) = maybe_icecream(12) {
            assert_eq!(icecreams, 5);
        }

    }
}

