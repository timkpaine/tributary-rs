pub struct Node {
    // func: Fn() -> ()
    pub func: fn() -> &'static str,
}

impl Node {
    pub fn call(&self) -> &'static str {
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
