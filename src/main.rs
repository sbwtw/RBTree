
use std::cmp::Ordering;
use std::fmt;

#[derive(Debug)]
enum RBColor {
    Red,
    Black,
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

type OptNode<K, V> = Option<Box<RBNode<K, V>>>;
struct RBNode<K: Ord + fmt::Debug, V: fmt::Debug> {
    color: RBColor,
    key: K,
    value: V,
    left_child: OptNode<K, V>,
    right_child: OptNode<K, V>,
}

struct RBTree<K: Ord + fmt::Debug, V: fmt::Debug> {
    root: OptNode<K, V>
}

fn search_node<'a, K: Ord + fmt::Debug, V: fmt::Debug>(root: &'a mut RBNode<K, V>, key: &K) -> Option<&'a mut RBNode<K, V>> {
    match key.cmp(&root.key) {
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

fn insert_node<K: Ord + fmt::Debug, V: fmt::Debug>(root: &mut RBNode<K, V>, key: K, value: V) -> Option<V> {
    match key.cmp(&root.key) {
        Ordering::Less => {
            if let Some(ref mut node) = root.left_child {
                insert_node(&mut *node, key, value)
            } else {
                root.left_child = Some(Box::new(RBNode::new(key, value)));
                None
            }
        },
        Ordering::Greater => {
            if let Some(ref mut node) = root.right_child {
                insert_node(&mut *node, key, value)
            } else {
                root.right_child = Some(Box::new(RBNode::new(key, value)));
                None
            }
        },
        Ordering::Equal => {
            Some(std::mem::replace(&mut root.value, value))
        }
    }
}

fn dump<K: Ord + fmt::Debug, V: fmt::Debug>(root: &RBNode<K, V>) {

    root.left_child.as_ref().map(|x| dump(&**x));
    println!("({:?}\t{:?}, \t{:?})", root.color, root.key, root.value);
    root.right_child.as_ref().map(|x| dump(&**x));
}

impl<K, V> RBNode<K, V>
    where K: Ord + fmt::Debug, V: fmt::Debug {

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
    where K: Ord + fmt::Debug, V: fmt::Debug {
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


///
///         123 -> "1234"
///        /          \
///  120 -> "120"   126 -> "126"
///
    assert_eq!(None, tree.get_mut(123));
    assert_eq!(None, tree.insert(123, "123".to_string()));
    assert_eq!(Some(&mut "123".to_string()), tree.get_mut(123));
    assert_eq!(Some("123".to_string()), tree.insert(123, "1234".to_string()));
    assert_eq!(Some(&mut "1234".to_string()), tree.get_mut(123));
    assert_eq!(None, tree.insert(120, "120".to_string()));
    assert_eq!(None, tree.insert(126, "126".to_string()));

    tree.root.map(|x| dump(&x));

    let mut tree = RBTree::<i32, ()>::new();
    tree.insert(8, ());
    tree.insert(3, ());
    tree.insert(5, ());
    tree.insert(23, ());
    tree.insert(6, ());
    tree.insert(12, ());
    tree.insert(3, ());

    tree.root.map(|x| dump(&x));
}
