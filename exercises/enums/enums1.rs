// enums1.rs
// No hints this time! ;)



#[derive(Debug)]
enum Message {
    // TODO: 定义下面所用的消息类型
    Quit,
    Echo,
    Move,
    ChangeColor,
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}

