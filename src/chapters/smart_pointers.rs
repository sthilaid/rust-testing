use std::rc::Rc;

pub fn run() {
    println!("\n*****************************************************************");
    println!("smart pointers");
    println!("*****************************************************************");

    data_structure_tests();
}

struct LinkedListNode<T> {
    prev: LinkedList<T>,
    next: LinkedList<T>,
    value: T,
}

enum LinkedList<T> {
    Node(Rc<LinkedListNode<T>>),
    Empty,
}

struct LinkedListIterator<T> {
    node: LinkedList<T>,
    is_forward: bool,
}

impl<T> LinkedList<T> {
    fn new(v: T) -> LinkedList<T> {
        LinkedList::Node(Rc::new(LinkedListNode {
            prev: LinkedList::Empty,
            next: LinkedList::Empty,
            value: v,
        }))
    }

    fn push(self: &mut Self, val: T) -> LinkedList<T> {
        LinkedList::Node(Rc::new(LinkedListNode {
            prev: match self {
                LinkedList::Node(n) => LinkedList::Node(Rc::clone(n)),
                LinkedList::Empty => LinkedList::Empty,
            },
            next: LinkedList::Empty,
            value: val,
        }))
    }

    fn clone(self: &Self) -> Self {
        match self {
            LinkedList::Node(n) => LinkedList::Node(Rc::clone(n)),
            LinkedList::Empty => LinkedList::Empty,
        }
    }

    fn iter(self: &Self) -> LinkedListIterator<T> {
        LinkedListIterator {
            node: self.clone(),
            is_forward: true,
        }
    }

    fn back_iter(self: &Self) -> LinkedListIterator<T> {
        LinkedListIterator {
            node: self.clone(),
            is_forward: false,
        }
    }
}

impl<T: Copy> Iterator for LinkedListIterator<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        match &self.node {
            LinkedList::Node(n) => {
                let val = n.value;
                match if self.is_forward { &n.next } else { &n.prev } {
                    LinkedList::Node(next_node) => {
                        self.node = LinkedList::Node(Rc::clone(&next_node))
                    }
                    LinkedList::Empty => self.node = LinkedList::Empty,
                };
                Option::Some(val)
            }
            LinkedList::Empty => None,
        }
    }
}

fn data_structure_tests() {
    println!("\n--- heap allocting data structures: linked list example ---\n");

    let mut list = LinkedList::new(5);
    list = list.push(6).push(7).push(8).push(9).push(10);

    let vlist: Vec<u8> = list.iter().collect();
    let bvlist: Vec<u8> = list.back_iter().collect();
    println!("vlist {:?} bvlist: {:?}", vlist, bvlist);
}
