// structs1.rs
// Address all the TODOs to make the tests pass!
// Execute `rustlings hint structs1` or use the `hint` watch subcommand for a hint.



struct ColorClassicStruct {
    // TODO: 这边有东西
    red: u8,
    green: u8,
    blue: u8,
}

struct ColorTupleStruct(u8,u8,u8/* TODO: 这边有东西 */);

#[derive(Debug)]
struct UnitLikeStruct;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_c_structs() {
        // TODO: 实例化一个经典的 c 结构！
        // let green =
        let green = ColorClassicStruct{
            red:0,
            green:255,
            blue:0,
        };
        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_structs() {
        // TODO: 实例化一个元组结构！
        // let green =
        let green = ColorTupleStruct(0,255,0);
        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_structs() {
        // TODO: 实例化一个类单元结构体！
        // let unit_like_struct =
        let mut unit_like_struct = UnitLikeStruct;
        let message = format!("{:?}s are fun!", unit_like_struct);

        assert_eq!(message, "UnitLikeStructs are fun!");
    }
}

