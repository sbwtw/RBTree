
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

fn search_node<'a, K: Ord, V>(root: &'a mut RBNode<K, V>, key: &K) -> Option<&'a mut RBNode<K, V>> {
    match root.key.cmp(key) {
        Ordering::Less => {
            root.left_child.as_mut().map_or(None, |x| search_node(x, key))
        },
        Ordering::Greater => {
            root.right_child.as_mut().map_or(None, |x| search_node(x, key))
        },
        Ordering::Equal => {
            if root.key == *key {
                Some(root)
            } else {
                None
            }
        },
    }
}

fn insert_node<K: Ord, V>(root: &mut RBNode<K, V>, key: K, value: V) -> Option<V> {
    match root.key.cmp(&key) {
        Ordering::Less => {
            unimplemented!()
        },
        Ordering::Greater => {
            unimplemented!()
        },
        Ordering::Equal => {
            Some(std::mem::replace(&mut root.value, value))
        }
    }
}

impl<K, V> RBNode<K, V>
    where K: Ord {

    fn new(key: K, value: V) -> Self {
        RBNode::with_color(RBColor::Black, key, value)
    }

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
            self.root.as_mut().and_then(|x| insert_node(x, key, value))
        }
    }

    pub fn get_mut(&mut self, key: K) -> Option<&mut V> {
        self.root.as_mut().and_then(|x| search_node(x, &key).map(|x| &mut (*x).value))
    }
}

fn main() {
    let mut tree = RBTree::<i32, String>::new();

    assert_eq!(None, tree.get_mut(123));
    assert_eq!(None, tree.insert(123, "123".to_string()));
    assert_eq!(Some(&mut "123".to_string()), tree.get_mut(123));
    assert_eq!(Some("123".to_string()), tree.insert(123, "1234".to_string()));
    assert_eq!(Some(&mut "1234".to_string()), tree.get_mut(123));
}
