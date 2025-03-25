use std::cell::RefCell;
use std::rc::{Rc, Weak};

struct ListNode<T: Clone> {
    value: T,
    previous: Option<Weak<RefCell<ListNode<T>>>>,
    next: Option<Rc<RefCell<ListNode<T>>>>,
}

impl<T: Clone> ListNode<T> {
    fn new(value: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            value,
            previous: None,
            next: None,
        }))
    }
}

pub struct DoublyLinkedList<T: Clone> {
    head: Option<Rc<RefCell<ListNode<T>>>>,
    tail: Option<Weak<RefCell<ListNode<T>>>>,
    len: usize,
}

impl<T: Clone> DoublyLinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            len: 0,
        }
    }

    pub fn is_empty(&self) -> bool { self.len == 0 }

    pub fn len(&self) -> usize { self.len }

    pub fn push_back(&mut self, value: T) {
        let new_node = ListNode::new(value);
        match self.tail.take() {
            Some(old_tail) => {
                if let Some(old_tail) = old_tail.upgrade() {
                    old_tail.borrow_mut().next = Some(new_node.clone());
                    new_node.borrow_mut().previous = Some(Rc::downgrade(&old_tail));
                }
                self.tail = Some(Rc::downgrade(&new_node));
            }
            None => {
                self.head = Some(new_node.clone());
                self.tail = Some(Rc::downgrade(&new_node));
            }
        }
        self.len += 1;
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().and_then(|tail| {
            tail.upgrade().map(|last_node| {
                let value = last_node.borrow().value.clone();
                if let Some(prev) = last_node.borrow().previous.clone() {
                    prev.upgrade().map(|prev| {
                        prev.borrow_mut().next = None;
                        self.tail = Some(Rc::downgrade(&prev));
                    });
                } else {
                    self.head = None;
                }
                self.len -= 1;
                value
            })
        })
    }

    pub fn push_front(&mut self, value: T) {
        let new_node = ListNode::new(value);
        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().previous = Some(Rc::downgrade(&new_node));
                new_node.borrow_mut().next = Some(old_head.clone());
                self.head = Some(new_node);
            }
            None => {
                self.head = Some(new_node.clone());
                self.tail = Some(Rc::downgrade(&new_node));
            }
        }
        self.len += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|first_node| {
            let value = first_node.borrow().value.clone();
            if let Some(next) = first_node.borrow_mut().next.clone() {
                next.borrow_mut().previous = None;
                self.head = Some(next);
            } else {
                self.tail = None;
            }
            self.len -= 1;
            value
        })
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            current: self.head.clone(),
        }
    }

    pub fn iter_reverse(&self) -> ReverseIter<T> {
        ReverseIter {
            current: self.tail.as_ref().and_then(|tail| tail.upgrade()),
        }
    }
}

pub struct Iter<T: Clone> { current: Option<Rc<RefCell<ListNode<T>>>>, }

impl<T: Clone> Iterator for Iter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.take().map(|current| {
            let current_ref = current.borrow();
            self.current = current_ref.next.clone();
            current_ref.value.clone()
        })
    }
}

pub struct ReverseIter<T: Clone> { current: Option<Rc<RefCell<ListNode<T>>>>, }

impl<T: Clone> Iterator for ReverseIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.take().map(|current| {
            let current_ref = current.borrow();
            self.current = current_ref.previous.as_ref().and_then(|prev| prev.upgrade());
            current_ref.value.clone()
        })
    }
}

fn main() {
    let mut list = DoublyLinkedList::new();
    list.push_back(11);
    list.push_back(2);
    list.push_back(33);

    println!("Итерация вперед:");
    for value in list.iter() { println!("{}", value); }
    println!();

    println!("Итерация назад:");
    for value in list.iter_reverse() { println!("{}", value); }
    println!();

    println!("Popped: {:?}", list.pop_back());
    for value in list.iter() { println!("{}", value); }
    println!();

    list.push_front(4);
    list.push_front(5);

    for value in list.iter() { println!("{}", value); }
    println!();

    println!("Popped from front: {:?}", list.pop_front());
    for value in list.iter() {println!("{}", value); }
    println!();

    println!("List is empty: {}", list.is_empty());
    println!("List length: {}", list.len());
}
