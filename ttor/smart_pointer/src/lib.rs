#[cfg(test)]
mod tests {
    use std::borrow::Cow;
    use std::cell::{Cell, RefCell};
    use std::rc::{Rc, Weak};
    use std::thread;

    /// Box<'a T> can be dereferred because of implementing std::op::Deref
    ///
    /// Box\<T\> is the only one of smart pointers that support deref
    /// ```
    /// # use std::ops::Deref;
    /// # use std::alloc::AllocRef;
    /// #[stable(feature = "rust1", since = "1.0.0")]
    /// impl<T: ?Sized, A: AllocRef> Deref for Box<T, A> {
    ///     type Target = T;
    ///
    ///     fn deref(&self) -> &T {
    ///         &**self
    ///     }
    /// }
    /// ```
    #[test]
    fn smart_pointer_box() {
        let x = Box::new("hello");
        //  - move occurs because `x` has type `Box<&str>`, which does not implement the `Copy` trait
        let y = x;
        //                - value moved here
        // println!("x: {:?}", x);
        //                  ^ value borrowed here after move
        let z = *y; // derefer `y` then copy the `&str` which implements the `Copy` trait
        println!("y: {:?}", y);
        println!("z: {:?}", z);
        let a = Box::new("Rust".to_string());
        // move occurs because `*a` has type `String`, which does not implement the `Copy` trait
        let b = *a;
        //             -- value moved here
        // println!("a: {:?}", a);
        //                     ^ value borrowed here after move
        println!("b: {:?}", b);
    }

    #[test]
    fn smart_pointer_rc() {
        let x = Rc::new("hello".to_string());
        // clone will share the ownership of a Rc pointer
        let _y1 = x.clone();
        let _y2 = x.clone();
        let x2 = &*x;
        println!("A value does not implement `Copy` trait in Rc pointer cannot be derefer but can be used as referred: {:?}", x2);
        println!("Rc strong count: {:?}", Rc::strong_count(&x));
        // a Rc pointer can downgrade to be a Weak pointer
        let z1 = Rc::downgrade(&x);
        println!("Rc weak count: {:?}", Rc::weak_count(&x));
        println!(
            "Weak pointers does not increase the strong count: {:?}",
            Rc::strong_count(&x)
        );
        // and the Weak pointer can upgrade to be a Rc pointer later
        let _y3 = z1.upgrade().unwrap();
        println!(
            "a Weak pointer can upgrade to be a Rc pointer anc increase the strong count: {:?}",
            Rc::strong_count(&x)
        );
        let _z2 = &*x;
        println!(
            "&*x increse neither the strong count: {:?}, nor the weak count: {:?}",
            Rc::strong_count(&x),
            Rc::weak_count(&x)
        );
        drop(x);
    }

    #[test]
    fn smart_pointer_weak_circular_reference() {
        struct Node {
            next: Option<Rc<RefCell<Node>>>,
            head: Option<Weak<RefCell<Node>>>,
            name: &'static str,
        }
        impl Drop for Node {
            fn drop(&mut self) {
                println!("{:?} is Dropping!", self.name)
            }
        }
        let first = Rc::new(RefCell::new(Node {
            next: None,
            head: None,
            name: "first",
        }));
        let second = Rc::new(RefCell::new(Node {
            next: None,
            head: None,
            name: "second",
        }));
        let third = Rc::new(RefCell::new(Node {
            next: None,
            head: None,
            name: "third",
        }));
        first.borrow_mut().next = Some(second.clone());
        second.borrow_mut().next = Some(third.clone());
        third.borrow_mut().head = Some(Rc::downgrade(&first));
    }

    #[test]
    fn memeory_leak() {
        struct Node {
            next: Option<Rc<RefCell<Node>>>,
            name: &'static str,
        }
        impl Drop for Node {
            fn drop(&mut self) {
                println!("{:?} is Dropping!", self.name)
            }
        }
        let first = Rc::new(RefCell::new(Node {
            next: None,
            name: "first",
        }));
        let second = Rc::new(RefCell::new(Node {
            next: None,
            name: "second",
        }));
        let third = Rc::new(RefCell::new(Node {
            next: None,
            name: "third",
        }));
        first.borrow_mut().next = Some(second.clone());
        second.borrow_mut().next = Some(third.clone());
        third.borrow_mut().next = Some(first.clone());
    }

    #[test]
    fn smart_pointer_cell() {
        struct Foo {
            x: u32,
            y: Cell<u32>,
        }
        let mut foo = Foo {
            x: 1,
            y: Cell::new(3),
        };
        assert_eq!(1, foo.x);
        // get copies the value in Cell<T> and returns it
        assert_eq!(3, foo.y.get());
        foo.y.set(5);
        assert_eq!(5, foo.y.get());
        // use get_mut to borrow a mutable value which does not implement `Copy` trait
        let y = foo.y.get_mut();
        *y = 10;
        assert_eq!(10, foo.y.get());
    }

    #[test]
    #[should_panic]
    fn smart_pointer_ref_cell() {
        let x = RefCell::new(vec![1, 2, 3, 4]);
        let mut mut_v1 = x.borrow_mut();
        mut_v1.push(5);
        x.borrow_mut().push(6); // runtime panic: already borrowed: BorrowMutError
        println!("{:?}", x);
    }

    #[test]
    fn smart_pointer_cow() {
        fn abs_all(input: &mut Cow<[i32]>) {
            for i in 0..input.len() {
                if input[i] < 0 {
                    input.to_mut()[i] = -input[i];
                }
            }
        }
        fn abs_sum(ns: &[i32]) -> i32 {
            let mut lst = Cow::from(ns);
            abs_all(&mut lst);
            lst.iter().fold(0, |acc, &n| acc + n)
        }

        let s1 = [1, 2, 3];
        let mut i1 = Cow::from(&s1[..]);
        // no write no copy
        abs_all(&mut i1);
        println!("IN: {:?}", s1);
        println!("OUT: {:?}", i1);
        let s2 = [1, 2, -3, -4];
        let mut i2 = Cow::from(&s2[..]);
        // copy due to write
        abs_all(&mut i2);
        println!("IN: {:?}", s2);
        println!("OUT: {:?}", i2);
        let mut v1 = Cow::from(vec![1, 2, -3, -4]);
        // no copy due to Vec<T> has the ownership
        // to_mut will get the ownership
        abs_all(&mut v1);
        println!("IN/OUT: {:?}", v1);
        let s3 = [1, 3, 5, 6];
        // no write no copy
        let sum1 = abs_sum(&s3);
        println!("{:?}", s3);
        println!("{:?}", sum1);
        let s4 = [1, 3, -5, -6];
        // write and copy
        let sum2 = abs_sum(&s4);
        println!("{:?}", s4);
        println!("{:?}", sum2);

        #[derive(Debug)]
        struct Token<'a> {
            raw: Cow<'a, str>,
        }
        impl<'a> Token<'a> {
            pub fn new<S>(raw: S) -> Token<'a>
            where
                S: Into<Cow<'a, str>>,
            {
                Token { raw: raw.into() }
            }
        }
        let token1 = Token::new("hello");
        let token2 = Token::new("Rust".to_string());
        // thread safe pointer
        thread::spawn(move || {
            println!("token1: {:?}", token1);
            println!("token2: {:?}", token2);
        })
        .join()
        .unwrap();
    }
}
