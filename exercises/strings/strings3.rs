// strings3.rs
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a hint.



fn trim_me(input: &str) -> String {
    // TODO: 删除字符串两端的空格！
    String::from(input.trim())
}

fn compose_me(input: &str) -> String {
    // TODO: 添加 " world!" 到字符串！有好几种方法可以做到！
    let mut s = String::from(input);
    s.push_str(" world!");
    s
}

fn replace_me(input: &str) -> String {
    // TODO: 将字符串中的 "cars" 替换为 "balloons"！
    input.replace("cars","balloons")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
        assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    }
}

