#[cfg(test)]
mod tests {
    #[test]
    fn mutable_and_let() {
        // 默认为不可变变量，需要使用mut关键字声明可变
        let mut _x = 5;
        _x = 6;
        println!("x: {}", _x);
        // 重复let同一个变量名可以隐藏(shadowing)之前的变量，并且改变类型
        // 实际上重新创建了变量，只是复用了变量名
        let _y = 10;
        let _y = "10";
        println!("y: {}", _y);
    }

    #[test]
    fn scalar() {
        // 整型
        let _a: i8 = -1;
        let _a: u8 = 0x01;
        let _a: i32 = 0b1111_0000;
        let _a: u32 = 98_222; // 可以用下划线增加可读性
                              // i64 u64 i128 u128 isize usize
        println!("integer: {}", _a);

        // 浮点
        let _b: f32 = 10.1;
        let _b: f64 = 20.2;
        println!("float: {}", _b);

        // 基础运算
        let sum = 5 + 10;
        let diff = 80.1 - 9.0;
        let product = 3 * 4;
        let quot = 30.0 / 10.0;
        let remainder = 43 % 5;
        println!(
            "base calculation: {},{},{},{},{}",
            sum, diff, product, quot, remainder
        );

        // 布尔
        let t = true;
        let f: bool = false;
        println!("bool: {}", t & f);

        // 字符
        let alpha = 'z'; // 四字节
        println!("char: {}", alpha);
    }

    #[test]
    fn compound_types() {
        // 元组
        // 类型不必一致
        let tup: (i32, f64, u8) = (500, 6.4, 1);
        let (x, y, _) = tup; // 解构
        println!("tuple: ({}, {}, {})", x, y, tup.2); // 通过.索引的方式访问

        // 数组
        // 分配在栈上，定长，类型必须一致
        let a = [1, 2, 3, 4, 5];
        // let index = 10;
        // let element = a[index]; // 在编译期直接检查出越界行为而终止
        another_function(a);
        println!("five is: {}", five());
        println!("six is: {}", six());
    }

    // 可以通过[type; length]的方式声明数组
    fn another_function(_a: [i32; 5]) {
        let _a = "another_string";
        println!("{}", _a);
    }

    fn five() -> u32 {
        5 // 带分号就是一个表达式，不带分号代表返回
    }

    // 整个函数也是个块表达式
    fn six() -> u32 {
        // 块表达式，最后返回a+1=2
        let y = {
            let a = 1;
            a + 1
        };
        y + 4
    }

    #[test]
    fn control_flow() {
        let condition = true;
        // 赋值时，if和else代码块中的返回类型必须是一样的
        let number = if condition { 5 } else { 6 };
        println!("the number is: {}", number);

        // 循环
        let mut loop_num = 0;
        loop {
            if loop_num > 3 {
                println!("loop number is: {}, break", loop_num);
                break;
            }
            println!("loop number is: {}", loop_num);
            loop_num = loop_num + 1;
        }

        let a = [1, 2, 3, 5];
        let mut index = 0;
        while index < a.len() {
            println!("while a[{}] = {}", index, a[index]);
            index = index + 1;
        }
        for elem in a.iter() {
            println!("for iter value is: {}", elem)
        }
        // 翻转
        // 这里如果要用a，那么a必须是mutable
        for elem in (1..4).rev() {
            println!("revers (1..4): {}", elem);
        }
    }
}
