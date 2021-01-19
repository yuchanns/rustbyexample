fn main() {
    vectors();
    fat_pointer();
}

fn fat_pointer() {
    // 胖指针携带内存地址以及长度信息的指针
    let arr1 = &[0; 5];
    let ptr1 = arr1.as_ptr();
    // 因为确定了类型和数组长度，所以arr1只携带了内存地址，是普通指针
    assert_eq!(std::mem::size_of_val(&arr1), 8);
    // 没有确定长度，arr2需要携带长度信息，是胖指针
    let arr2: &[i32] = &[0; 5];
    let ptr2 = arr2.as_ptr();
    assert_eq!(std::mem::size_of_val(&arr2), 16);
    // 指针指向的内存地址一样
    assert_eq!(format!("{:p}", ptr1), format!("{:p}", ptr2));
    // [i32]和[i32;5]不是同一个类型，他们携带信息不一样
    assert_eq!(std::mem::size_of::<&[i32]>(), 16);
    assert_eq!(std::mem::size_of::<&[i32; 5]>(), 8);
    let str = "hello world";
    // &str也是胖指针，无法确定字符串长度，需要携带长度信息
    assert_eq!(std::mem::size_of_val(&str), 16);
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
