#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::{Rc, Weak};

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
}
