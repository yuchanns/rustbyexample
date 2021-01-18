fn main() {
    vectors();
}

fn vectors() {
    // 创建一个长度为0的vector
    // 这种方式需要加类型注解确定Vector的类型
    let mut v: Vec<i32> = Vec::new();
    v.push(10);
    v.push(12);
    println!("v: {:?}", v);
    // 实际上常用
    let mut v2 = vec![1, 2, 3, 4];
    // 通过索引访问vector元素,超出索引会panic
    let third = &v2[2];
    println!("the third elem of v2: {:?}", third);
    v2.push(5);
    println!("v2: {:?}", v2);
    // 这里不能再次访问third,会被编译器阻止
    // 因为上面push过，可能出现内存不足重新申请一块更大内存给v2的情况
    // 这时候third就会指向被释放的内存
    // println!("the third elem of v2: {:?}", third); // cannot borrow `v2` as mutable because it is also borrowed as immutable
    // 访问vector元素,通过get方法
    match v2.get(1) {
        Some(second) => println!("the second elem of v2: {:?}", second),
        None => println!("there is no second elem"),
    }
    // 遍历
    for i in &mut v2 {
        *i += 50;
    }
    println!("v2 after add by ergodic: {:?}", v2);
    // vec必须存储同一种类
    // 结合枚举体存储不同种类
    #[derive(Debug)]
    enum SpreadSheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadSheetCell::Int(1),
        SpreadSheetCell::Float(10.11),
        SpreadSheetCell::Text(String::from("hello world")),
    ];
    println!("row: {:?}", row);
    let first = &row[0];
    // 通过if let取值
    if let SpreadSheetCell::Int(value) = first {
        println!("the first elem of row is {:?}", value);
    }
    // 在未知vec元素的真实类型时，可以用match
    for elem in &row {
        match elem {
            SpreadSheetCell::Int(value) => println!("int value {:?}", value),
            SpreadSheetCell::Float(value) => println!("float value {:?}", value),
            SpreadSheetCell::Text(value) => println!("text value {:?}", value),
        }
    }
}
