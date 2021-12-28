pub struct Node<T> {
    // func: Fn() -> ()
    pub func: fn() -> T,
}

impl<T> Node<T> {
    pub fn call(&self) -> T {
        (self.func)()
    }
}

#[cfg(test)]
mod streaming_tests {
    use super::*;

    fn foo() -> &'static str {
        "test"
    }

    #[test]
    fn test_instance() {
        let n = Node { func: foo };
        assert_eq!(n.call(), "test");
    }
}
