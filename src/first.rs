use std::mem;

pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
    // a "Foo" will store some int to indicate which variant of enum it represents
    // called the tag of the enum ^
    // will need enough space to store largest of T1, T2, etc
    // if enum Foo {A, B(nonnullpointer)}, then nullptr optimization eliminates space needed
    // for tag
    // this means &, &mut, Box, Rc, Arc, Vec, etc have no overhead when Option type
}

struct Node {
    elem: i32,
    next: List,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
        // self is an alias for "List"
        // create instance of struct similar to declaring, but provide fields
        // with values instead of types
        // refer to variants of enum using ::
        // most methods won't want just "self" as arg b/c u can't do anything
        // without the mutable reference
    }
    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            next: mem::replace(self, List { head: Link::Empty }), // replace self.head w/ new head
                                                                  // next: self.head doesn't work b/c self would be partially initialized
                                                                  // when we end the borrow and return to its owner
        });
        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}
