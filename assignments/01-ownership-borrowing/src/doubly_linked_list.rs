// Now try and implement a doubly linked version. Give an explanation
// for why this doesn't work.

struct Node {
    val: i32,
    next: Link,
    prev: Link,
}
struct Node {
    val: i32,
    next: Link,
    prev: Link,
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
        match &mut self.head {
            Some(head) => {
                let new_node = Box::new(Node {
                    val,
                    next: self.head.take(),
                    prev: None,
                });
                head.next = Some(new_node);
            }
            None => {
                self.head = Some(Box::new(Node {
                    val,
                    next: None,
                    prev: None,
                }));
            }
        }
    }

    fn pop(&mut self) -> Option<i32> {
        match &mut self.head {
            Some(node) => {
                if node.next.is_none() {
                    let val = node.val;
                    self.head = None;
                    return Some(val);
                }
                let val = node.val;
                self.head = node.next.take();
                self.head.as_mut().unwrap().prev = None;
                return Some(val);
            }
            None => None,
        }
    }
}
