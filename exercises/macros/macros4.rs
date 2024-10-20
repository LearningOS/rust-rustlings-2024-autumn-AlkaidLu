// macros4.rs
//
// Execute `rustlings hint macros4` or use the `hint` watch subcommand for a
// hint.

#[rustfmt::skip]
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };//宏规则的定义格式要求每个匹配规则都要以分号结束。
    ($val:tt) => {//tt 类型的片段可以匹配任意的令牌树，并且它是少数可以直接用于字面量匹配的类型之一。
        println!("Look at this other macro: {}", $val);
    }
}

fn main() {
    my_macro!();
    my_macro!(7777);
}
