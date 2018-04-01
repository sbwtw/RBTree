
use std::cmp::Ordering;

enum RBColor {
    Red,
    Black,
}

struct RBNode<K: Ord, V> {
    color: RBColor,
    key: K,
    value: V,
    left_child: Option<Box<RBNode<K, V>>>,
    right_child: Option<Box<RBNode<K, V>>>,
}

struct RBTree<K: Ord, V> {
    root: Option<RBNode<K, V>>
}

fn search_node<'a, K: Ord, V>(root: &'a mut RBNode<K, V>, key: &K) -> Option<&'a mut V> {

    match root.key.cmp(key) {
        Ordering::Less => {
            root.left_child.as_mut().map_or(None, |x| search_node(x, key))
        },
        Ordering::Greater => {
            root.right_child.as_mut().map_or(None, |x| search_node(x, key))
        },
        Ordering::Equal => {
            if root.key == *key {
                Some(&mut root.value)
            } else {
                None
            }
        },
    }
}

impl<K, V> RBNode<K, V>
    where K: Ord {
    fn with_color(color: RBColor, key: K, value: V) -> Self {
        RBNode {
            color,
            key,
            value,
            left_child: None,
            right_child: None,
        }
    }
}

impl<K, V> RBTree<K, V>
    where K: Ord {
    pub fn new() -> Self {
        RBTree {
            root: None,
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.root.is_none() {
            self.root = Some(RBNode::with_color(RBColor::Black, key, value));
            None
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, key: K) -> Option<&mut V> {
        match self.root {
            Some(ref mut node) => search_node(node, &key),
            None => None,
        }
    }
}

fn main() {
    let mut tree = RBTree::<i32, String>::new();
}