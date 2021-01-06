fn main() {
    // array
    let a = [1, 2, 3, 4, 5];
    // let index = 10;
    // let element = a[index]; // will abort due to index out of bounds
    another_function(a);
    println!("five is: {}", five());
}

fn another_function(_a: [i32; 5]) {
    let _a = "another_string";
    println!("{}", _a);
}

fn five() -> u32 {
    // 带分号就是一个表达式，不带分号代表返回
    5 // means return while without semicolon
}
