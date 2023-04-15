// primitive_types2.rs
// Fill in the rest of the line that has code missing!
// No hints, there's no tricks, just get used to typing these :)



fn main() {
    // Characters (`char`)

    // 注意 _单_ 引号，它们和双引号是不同的
    // 你可以在周围看到区别
    let my_first_initial = 'C';
    if my_first_initial.is_alphabetic() {
        println!("这是个字母！");
    } else if my_first_initial.is_numeric() {
        println!("这是个数字！");
    } else {
        println!("这不是字母也不是数字！");
    }

    let your_character = 'A';
    // 参照示例完成此行！这是你最喜欢的字符吗？
    // 试一下字母，试一下数字，试一下特殊字符，试一下
    // 你的语言之外其它语言的特殊字符，试一下 emoji ！
    if your_character.is_alphabetic() {
        println!("这是个字母！");
    } else if your_character.is_numeric() {
        println!("这是个数字！");
    } else {
        println!("这不是字母也不是数字！");
    }

    let your_number = '1';
    if your_number.is_numeric() {
        println!("this is a number");
    } else if your_number.is_alphabetic() {
        println!("this is a alphabetic");
    } else {
        println!("this is neither a alphabetic nor symbolic");
    }

    let your_symbol = '🍻';
    if your_symbol.is_alphabetic() {
        println!("this is a symbol");
    } else if your_symbol.is_numeric() {
        println!("this is a number");
    } else {
        println!("this is neither a number nor a alphabet");
    }
}
