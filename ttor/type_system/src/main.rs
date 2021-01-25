fn main() {
    associate_types();
}

fn associate_types() {
    let a = Test {
        name: "rust".to_string(),
    };
    println!("a.test: {}", test(a));
    let b = Test {
        name: "rust".to_string(),
    };
    println!("b.test: {}", test_b(b));
}

fn test<T: TestA>(t: T) -> T::Output {
    t.test()
}

// 未使用类型关联，需要在函数签名里指定trait成员的泛型类型，代码冗长
fn test_b<RHS, S, T: TestB<RHS, S>>(t: T) -> S {
    t.test()
}

trait TestA<RHS = Self> {
    type Output;
    fn test(self) -> Self::Output;
}

struct Test {
    name: String,
}

// 使用类型关联(associate types)，可以精简代码，免于指定成员类型
impl TestA for Test {
    type Output = String;
    fn test(self) -> Self::Output {
        format!("{}", self.name)
    }
}

trait TestB<RHS, S> {
    fn test(self) -> S;
}

impl TestB<Self, String> for Test {
    fn test(self) -> String {
        format!("{}", self.name)
    }
}
