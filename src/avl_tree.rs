use std::{
    cell::RefCell,
    cmp::{Ord, Ordering},
    rc::Rc,
};

type Link<K, V> = Option<Rc<RefCell<Node<K, V>>>>;

pub enum AVLTreeError {
    EmptyAVLTree,
    MaxNodesExists,
    NodeNotFound,
    AlreadyExists,
}

struct Node<K, V>
where
    K: PartialEq + Ord,
{
    pub key: K,
    pub value: V,
    pub left: Link<K, V>,
    pub right: Link<K, V>,
}

impl<K, V> Node<K, V>
where
    K: PartialEq + Ord,
{
    pub fn new(key: K, value: V) -> Self {
        Node {
            key,
            value,
            left: None,
            right: None,
        }
    }
}

struct AVLTree<K, V>
where
    K: PartialEq + Ord,
{
    head: Link<K, V>,
    size: usize,
}

impl<K, V> AVLTree<K, V>
where
    K: PartialEq + Ord,
{
    pub fn new() -> Self {
        AVLTree {
            head: None,
            size: 0,
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Result<(), AVLTreeError> {
        if self.size == usize::MAX {
            return Err(AVLTreeError::MaxNodesExists);
        }

        if self.size == 0 {
            self.head = Some(Rc::new(RefCell::new(Node::new(key, value))));
            self.size += 1;
            return Ok(());
        }

        self.insert_val(key, value, self.head.clone())
    }

    pub fn get(&self, key: K) -> Result<V, AVLTreeError> {
        if self.size == 0 {
            return Err(AVLTreeError::EmptyAVLTree);
        }
        let node = self.head.clone();

        return self.get_val(key, node);
    }

    #[inline]
    fn insert_val(&mut self, key: K, value: V, node: Link<K, V>) -> Result<(), AVLTreeError> {
        let cur_node = node.unwrap();
        let cur_key = &cur_node.borrow().key;

        return match key.cmp(cur_key) {
            Ordering::Equal => return Err(AVLTreeError::AlreadyExists),
            Ordering::Less => {
                let mut node = (*cur_node).borrow_mut();
                let left = (*node).left.clone();
                if left.is_some() {
                    return self.insert_val(key, value, left);
                }

                (*node).left = Some(Rc::new(RefCell::new(Node::new(key, value))));
                self.size += 1;
                Ok(())
            }
            Ordering::Greater => {
                let mut node = (*cur_node).borrow_mut();
                let right = (*node).right.clone();
                if right.is_some() {
                    return self.insert_val(key, value, right);
                }

                (*node).right = Some(Rc::new(RefCell::new(Node::new(key, value))));
                self.size += 1;
                Ok(())
            }
        };
    }

    fn get_val(&self, key: K, node: Link<K, V>) -> Result<V, AVLTreeError> {
        if node.is_none() {
            return Err(AVLTreeError::NodeNotFound);
        }
        let node = node.unwrap();
        let node = node.borrow();
        let cur_key = &node.key;

        return match key.cmp(cur_key) {
            Ordering::Equal => return Ok(node.value),
            Ordering::Less => {
                let left = node.left.clone();
                self.get_val(key, left)
            }
            Ordering::Greater => {
                let right = node.right.clone();
                self.get_val(key, right)
            }
        };
    }
}

#[cfg(test)]
mod avl_tree_tests {
    use super::AVLTree;

    #[test]
    fn test_get() {
        let mut tree = AVLTree::new();
        tree.insert(5, String::from("Hello World"));
    }
}
