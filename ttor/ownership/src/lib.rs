#[cfg(test)]
mod tests {
    #[derive(Debug)]
    struct TestA {}

    impl Drop for TestA {
        fn drop(&mut self) {
            println!("TestA dropped")
        }
    }

    #[derive(Debug)]
    struct TestB<'a> {
        // 成员作为引用类型，必须指定liftime specifier
        // 这可以帮助借用检查器检查是否存在非法借用
        // TestB的实例lifetime必须比它的任何成员实例lifetime短
        test_a: &'a TestA,
    }

    impl<'a> Drop for TestB<'a> {
        fn drop(&mut self) {
            println!("TestB dropped")
        }
    }

    #[test]
    fn test_lifetime() {
        // a lifetime start
        let test_a = TestA {};
        // b lifetime start
        let test_b = TestB { test_a: &test_a };
        println!("testA:{:?}, testB:{:?}", test_a, test_b);
        // b lifetime end
        // a lifetime end
    }
}
