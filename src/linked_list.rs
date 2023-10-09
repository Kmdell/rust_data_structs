use std::{cell::RefCell, rc::Rc};
type Link<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Debug, PartialEq, Eq)]
pub enum LinkedListError {
    OutOfBounds,
    EmptyList,
}

struct Node<T>
where
    T: Copy + std::cmp::PartialEq,
{
    pub value: T,
    pub next: Link<T>,
}

impl<T> Node<T>
where
    T: Copy + std::cmp::PartialEq,
{
    fn new(value: T) -> Self {
        Node { value, next: None }
    }
}

pub struct LinkedList<T>
where
    T: Copy + std::cmp::PartialEq,
{
    head: Link<T>,
    tail: Link<T>,
    length: usize,
}

impl<T> LinkedList<T>
where
    T: Copy + std::cmp::PartialEq,
{
    pub fn new() -> Self {
        LinkedList {
            head: None,
            tail: None,
            length: 0,
        }
    }

    pub fn length(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    pub fn append(&mut self, other: LinkedList<T>) {
        self.length += other.length;

        if self.length == 0 {
            self.head = other.head;
            self.tail = other.tail;
            return;
        }

        if let Some(tail) = self.tail.clone() {
            let mut tail = (*tail).borrow_mut();
            (*tail).next = other.head;
            self.tail = other.tail;
        }
    }

    pub fn clear(&mut self) {
        let mut node = self.head.clone();
        self.head = None;
        self.tail = None;

        while let Some(temp) = node {
            node = (*temp).borrow().next.clone();
            let mut temp = (*temp).borrow_mut();
            (*temp).next = None;
        }

        self.length = 0;
    }

    pub fn contains(&self, item: T) -> bool {
        if self.length == 0 {
            return false;
        }

        let mut node = self.head.clone();
        while let Some(temp) = node {
            if temp.borrow().value == item {
                return true;
            }
            node = temp.borrow().next.clone()
        }

        false
    }

    pub fn get(&self, idx: usize) -> Result<T, LinkedListError> {
        if self.head.is_none() {
            return Err(LinkedListError::EmptyList);
        }

        if idx + 1 > self.length {
            return Err(LinkedListError::OutOfBounds);
        }

        return Ok(self.get_node(idx).borrow().value);
    }

    pub fn remove(&mut self, idx: usize) -> Result<T, LinkedListError> {
        if self.length == 0 {
            return Err(LinkedListError::EmptyList);
        }

        if idx > self.length - 1 {
            return Err(LinkedListError::OutOfBounds);
        }

        self.length -= 1;

        if idx == 0 {
            let node = self.head.clone().unwrap();
            let node = (*node).borrow();
            self.head = node.next.clone();
            return Ok(node.value);
        }

        let node = self.get_node(idx - 1);

        if idx == self.length {
            let temp = node.clone();
            self.tail = Some(temp);
        }

        let next = node.borrow().next.clone().unwrap();
        let value = next.borrow().value;
        let next = next.borrow().next.clone();
        let mut node = (*node).borrow_mut();
        (*node).next = next;

        Ok(value)
    }

    pub fn front(&self) -> Result<T, LinkedListError> {
        if self.head.is_none() || self.length == 0 {
            return Err(LinkedListError::EmptyList);
        }

        Ok(self.head.clone().unwrap().borrow().value)
    }

    pub fn back(&self) -> Result<T, LinkedListError> {
        if self.tail.is_none() {
            return Err(LinkedListError::EmptyList);
        }

        Ok(self.tail.clone().unwrap().borrow().value)
    }

    pub fn pop_front(&mut self) -> Result<T, LinkedListError> {
        let value = self.front()?;

        self.length -= 1;
        if self.length == 0 {
            self.head = None;
            self.tail = None;
            return Ok(value);
        }

        let node = self.head.clone().unwrap();
        let node = node.borrow().next.clone();
        self.head = node;

        Ok(value)
    }

    pub fn pop_back(&mut self) -> Result<T, LinkedListError> {
        let value = self.back()?;

        self.length -= 1;
        if self.length == 0 {
            self.head = None;
            self.tail = None;
            return Ok(value);
        }

        let node = self.get_node(self.length - 1);
        let temp = node.clone();
        let mut node = (*node).borrow_mut();
        (*node).next = None;
        self.tail = Some(temp);

        Ok(value)
    }

    pub fn push_front(&mut self, item: T) {
        self.length += 1;

        let node = Rc::new(RefCell::new(Node {
            value: item,
            next: self.head.clone(),
        }));

        self.head = Some(node.clone());

        if self.tail.is_none() {
            self.tail = Some(node);
        }
    }

    pub fn push_back(&mut self, item: T) {
        self.length += 1;
        let node = Rc::new(RefCell::new(Node::new(item)));

        if self.head.is_none() {
            self.head = Some(node.clone());
            self.tail = Some(node);
            return;
        }

        if let Some(tail) = self.tail.clone() {
            let mut tail = (*tail).borrow_mut();
            (*tail).next = Some(node.clone());
        }

        self.tail = Some(node);
    }

    #[inline]
    fn get_node(&self, idx: usize) -> Rc<RefCell<Node<T>>> {
        let mut node = self.head.clone();
        let mut place: usize = 0;

        while idx != place {
            if let Some(temp) = node {
                node = temp.borrow().next.clone();
            }
            place += 1;
        }

        return node.unwrap().clone();
    }
}

impl<T, const N: usize> From<[T; N]> for LinkedList<T>
where
    T: Copy + std::cmp::PartialEq,
{
    fn from(value: [T; N]) -> Self {
        let mut list = LinkedList::new();
        for val in value {
            list.push_back(val);
        }
        list
    }
}

#[cfg(test)]
mod linked_list_tests {
    use crate::linked_list::LinkedListError;

    use super::LinkedList;

    #[test]
    fn test_empty() {
        let mut list: LinkedList<i32> = LinkedList::new();

        assert_eq!(list.is_empty(), true);

        list.push_front(9);

        assert_eq!(list.is_empty(), false);

        let mut list: LinkedList<i32> = LinkedList::new();

        list.push_back(9);

        assert_eq!(list.is_empty(), false);
    }

    #[test]
    fn test_length() {
        let mut list: LinkedList<i32> = LinkedList::new();

        assert_eq!(list.length(), 0);

        list.push_back(9);

        assert_eq!(list.length(), 1);

        list.push_front(8);

        assert_eq!(list.length(), 2);
    }

    #[test]
    fn test_get() {
        let mut list: LinkedList<i32> = LinkedList::new();

        assert_eq!(list.get(0), Err(LinkedListError::EmptyList));

        list.push_back(10);
        assert_eq!(list.get(0).unwrap(), 10);
        list.push_back(90);
        assert_eq!(list.get(1).unwrap(), 90);
        list.push_back(100);
        assert_eq!(list.get(2).unwrap(), 100);
        assert_eq!(list.get(0).unwrap(), 10);

        assert_eq!(list.get(4), Err(LinkedListError::OutOfBounds));
    }

    #[test]
    fn test_front_push() {
        let mut list: LinkedList<i32> = LinkedList::new();

        list.push_front(0);
        assert_eq!(list.get(0).unwrap(), 0);
        assert_eq!(list.length(), 1);
        list.push_front(1);
        assert_eq!(list.get(0).unwrap(), 1);
        assert_eq!(list.length(), 2);
        list.push_front(2);
        assert_eq!(list.get(0).unwrap(), 2);
        assert_eq!(list.length(), 3);
        list.push_front(3);
        assert_eq!(list.get(0).unwrap(), 3);
        assert_eq!(list.length(), 4);
    }

    #[test]
    fn test_back_push() {
        let mut list: LinkedList<i32> = LinkedList::new();

        list.push_back(0);
        assert_eq!(list.get(0).unwrap(), 0);
        assert_eq!(list.length(), 1);
        list.push_back(1);
        assert_eq!(list.get(1).unwrap(), 1);
        assert_eq!(list.length(), 2);
        list.push_back(2);
        assert_eq!(list.get(2).unwrap(), 2);
        assert_eq!(list.length(), 3);
        list.push_back(3);
        assert_eq!(list.get(3).unwrap(), 3);
        assert_eq!(list.length(), 4);
    }

    #[test]
    fn test_contains() {
        let mut list: LinkedList<i32> = LinkedList::new();

        assert!(!list.contains(4));

        list.push_front(4);
        list.push_front(34);

        assert!(list.contains(4));
        assert!(!list.contains(5));
        assert!(list.contains(34));
    }

    #[test]
    fn test_front() {
        let mut list: LinkedList<i32> = LinkedList::new();

        list.push_front(9);
        assert_eq!(list.front().unwrap(), 9);
        list.push_front(13);
        assert_eq!(list.front().unwrap(), 13);
        list.push_front(56);
        assert_eq!(list.front().unwrap(), 56);
        list.push_front(90);
        assert_eq!(list.front().unwrap(), 90);
    }

    #[test]
    fn test_back() {
        let mut list: LinkedList<i32> = LinkedList::new();

        list.push_back(9);
        assert_eq!(list.back().unwrap(), 9);
        list.push_back(13);
        assert_eq!(list.back().unwrap(), 13);
        list.push_back(56);
        assert_eq!(list.back().unwrap(), 56);
        list.push_back(90);
        assert_eq!(list.back().unwrap(), 90);
    }

    #[test]
    fn test_front_pop() {
        let mut list: LinkedList<i32> = LinkedList::new();

        list.push_front(1);
        assert_eq!(list.pop_front(), Ok(1));
        assert!(list.is_empty());

        list.push_front(20);
        list.push_front(30);
        list.push_back(10);
        assert_eq!(list.pop_front(), Ok(30));
        assert_eq!(list.pop_front(), Ok(20));
        assert_eq!(list.pop_front(), Ok(10));
        assert!(list.is_empty());
    }

    #[test]
    fn test_back_pop() {
        let mut list: LinkedList<i32> = LinkedList::new();

        list.push_back(10);
        assert_eq!(list.pop_back(), Ok(10));
        assert!(list.is_empty());

        list.push_back(20);
        list.push_back(30);
        list.push_front(10);
        assert_eq!(list.pop_back(), Ok(30));
        assert_eq!(list.pop_back(), Ok(20));
        assert_eq!(list.pop_back(), Ok(10));
        assert!(list.is_empty());
    }

    #[test]
    fn test_remove() {
        let mut list: LinkedList<i32> = LinkedList::new();

        list.push_back(10);
        list.push_back(20);
        list.push_back(30);
        assert_eq!(list.remove(1), Ok(20));
        assert_eq!(list.get(1), Ok(30));
        assert_eq!(list.remove(4), Err(LinkedListError::OutOfBounds));
        assert_eq!(list.remove(1), Ok(30));
        assert_eq!(list.remove(0), Ok(10));
        assert_eq!(list.remove(20), Err(LinkedListError::EmptyList));
        assert_eq!(list.remove(0), Err(LinkedListError::EmptyList));
    }

    #[test]
    fn test_clear() {
        let mut list = LinkedList::new();

        list.push_back(10);
        list.push_back(10);
        list.push_back(10);
        list.push_back(10);
        list.push_back(10);

        assert_eq!(list.length(), 5);
        list.clear();
        assert_eq!(list.get(0), Err(LinkedListError::EmptyList));
        assert_eq!(list.length(), 0);
    }

    #[test]
    fn test_append() {
        let mut list = LinkedList::new();
        let mut other_list = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        other_list.push_back(4);
        other_list.push_back(5);
        other_list.push_back(6);

        list.append(other_list);

        assert_eq!(list.length(), 6);

        assert_eq!(list.get(0).unwrap(), 1);
        assert_eq!(list.get(1).unwrap(), 2);
        assert_eq!(list.get(2).unwrap(), 3);
        assert_eq!(list.get(3).unwrap(), 4);
        assert_eq!(list.get(4).unwrap(), 5);
        assert_eq!(list.get(5).unwrap(), 6);
    }

    #[test]
    fn test_from_arr() {
        let arr = [1, 2, 3, 4, 5, 6];
        let list = LinkedList::from(arr);

        assert_eq!(list.length(), 6);

        assert_eq!(list.get(0).unwrap(), 1);
        assert_eq!(list.get(1).unwrap(), 2);
        assert_eq!(list.get(2).unwrap(), 3);
        assert_eq!(list.get(3).unwrap(), 4);
        assert_eq!(list.get(4).unwrap(), 5);
        assert_eq!(list.get(5).unwrap(), 6);
    }
}
