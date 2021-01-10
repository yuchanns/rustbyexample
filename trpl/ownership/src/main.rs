// 分配在堆上和分配在栈上的区别：
// 栈中所有数据占用已知且固定的大小
// 堆上的数据大小未知，需要申请分配，然后标记已使用并返回指针
// 这个过程就是 在堆上分配内存
// 指针本身大小固定，可以放在栈上
// 入栈比入堆快(需要内存分配)
// 读取数据堆数据需要先访问指针再访问堆，速度也比访问栈数据慢
// 所有权(ownership)用于管理堆数据

// 所有权
// Rust 中的每一个值都有一个被称为其 所有者（owner）的变量。
// 值在任一时刻有且只有一个所有者。
// 当所有者（变量）离开作用域，这个值将被丢弃。
fn main() {
    let s = String::from("hello");
    let b = s;
    println!("{}", b);
    // println!("{}", s); // compile error: value borrowed here after move
    // move表现类似于浅拷贝，但实际上是将指针从一个栈移动到另一个栈上，即所有权转移
    // 设计理由：
    // 1.拷贝一份堆数据，可能会有性能影响
    // 2.拷贝指针，同时指向同一个堆，在释放内存时会出现二次释放的bug
    let s = b.clone(); // 深拷贝，同时拷贝一份堆数据
    println!("heap clone b: {} to s: {}", b, s);
    // 分配在栈上的数据都是深拷贝，拷贝迅速，不存在move问题
    let a = 2;
    let b = a;
    println!("stack clone a: {} to b: {}", a, b);
    // 函数也会转移所有权
    taken_ownership(s);
    // println!("{}", s); //compile error: value borrowed here after move
    let s = String::from("world");
    let s = takes_and_gives_back(s);
    println!("takes and gives back s: {}", s)
}

fn taken_ownership(s: String) {
    println!("s: {}", s);
}

fn takes_and_gives_back(s: String) -> String {
    // 所有权返回
    s
}
