struct Node {
    val: i32,
    next: Link,
}

type Link = Option<Box<Node>>;

pub struct LinkedStack {
    head: Link,
}

impl LinkedStack {
    fn new() -> Self {
        Self { head: None }
    }

    fn push(&mut self, val: i32) {
        self.head = Some(Box::new(Node {
            val,
            next: self.head.take(),
        }));
    }

    fn pop(&mut self) -> Option<i32> {
        match &mut self.head {
            Some(node) => {
                if node.next.is_none() {
                    let val = node.val;
                    self.head = None;
                    return Some(val);
                } else {
                    let val = node.val;
                    self.head = node.next.take();
                    return Some(val);
                }
            }
            None => return None,
        }
    }
}

impl Drop for LinkedStack {
    fn drop(&mut self) {
        loop {
            match self.pop() {
                Some(_) => {}
                None => break,
            }
        }
    }
}

// DO NOT MODIFY BELOW THIS LINE

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_stack() {
        let mut stack = LinkedStack::new();
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn test_linked_stack() {
        let mut stack = LinkedStack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);

        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));

        stack.push(4);

        assert_eq!(stack.pop(), Some(4));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn test_big_stack() {
        let mut stack = LinkedStack::new();
        for i in 0..1_000_000 {
            stack.push(i);
        }

        for i in (0..1_000_000).rev() {
            assert_eq!(stack.pop(), Some(i));
        }

        assert_eq!(stack.pop(), None);
    }
}
