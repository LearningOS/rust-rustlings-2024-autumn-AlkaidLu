// traits1.rs
//
// Time to implement some traits! Your task is to implement the trait
// `AppendBar` for the type `String`. The trait AppendBar has only one function,
// which appends "Bar" to any object implementing this trait.
//
// Execute `rustlings hint traits1` or use the `hint` watch subcommand for a
// hint.


trait AppendBar {
    fn append_bar(self) -> Self;
}
//self(小写)表示当前结构体或枚举的实例
//Self(大写)表示当前实现 trait 或方法的类型
//方法会返回一个与 self 相同类型的新实例。
impl AppendBar for String {
    // TODO: Implement `AppendBar` for type `String`.
    fn append_bar(self) ->Self{
        
        //self+"Bar"
        let mut new_string = self;  // 将 self 的所有权转移到可变的 new_string
        new_string.push_str("Bar"); // 使用 push_str 追加字符串
        new_string                 // 返回新的字符串
    }
}

fn main() {
    let s = String::from("Foo");
    let s = s.append_bar();
    println!("s: {}", s);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_foo_bar() {
        assert_eq!(String::from("Foo").append_bar(), String::from("FooBar"));
    }

    #[test]
    fn is_bar_bar() {
        assert_eq!(
            String::from("").append_bar().append_bar(),
            String::from("BarBar")
        );
    }
}
