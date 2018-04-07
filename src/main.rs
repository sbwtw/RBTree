
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

trait KeyTrait: Ord + fmt::Debug {}
impl<T: Ord + fmt::Debug> KeyTrait for T {}

trait ValueTrait: fmt::Debug {}
impl<T: fmt::Debug> ValueTrait for T {}

type OptNode<K, V> = Option<Box<RBNode<K, V>>>;
struct RBNode<K: KeyTrait, V: ValueTrait> {
    color: RBColor,
    key: K,
    value: V,
    left_child: OptNode<K, V>,
    right_child: OptNode<K, V>,
}

struct RBTree<K: KeyTrait, V: ValueTrait> {
    root: OptNode<K, V>
}

fn search_node<'a, K: KeyTrait, V: ValueTrait>(root: &'a mut RBNode<K, V>, key: &K) -> Option<&'a mut RBNode<K, V>> {
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

fn insert_node<K: KeyTrait, V: ValueTrait>(root: &mut RBNode<K, V>, key: K, value: V) -> Option<V> {
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

#[inline]
fn ensure_root_black<K: KeyTrait, V: ValueTrait>(root: &mut OptNode<K, V>) {
    assert!(root.is_some());

    root.as_mut().map(|x| x.set_color(RBColor::Black));
}

fn insert_n<K: KeyTrait, V: ValueTrait>(root: &mut OptNode<K, V>, key: K, value: V) -> Option<V> {
    if root.is_none() {
        *root = Some(Box::new(RBNode::new(key, value)));
        ensure_root_black(root);

        return None;
    }

    unimplemented!()
}

fn dump<K: KeyTrait, V: ValueTrait>(root: &RBNode<K, V>) {

    root.left_child.as_ref().map(|x| dump(&**x));
    println!("({:?}\t{:?}, \t{:?})", root.color, root.key, root.value);
    root.right_child.as_ref().map(|x| dump(&**x));
}

impl<K, V> RBNode<K, V>
    where K: KeyTrait, V: ValueTrait {

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

    fn set_color(&mut self, color: RBColor) {
        self.color = color;
    }
}

impl<K, V> RBTree<K, V>
    where K: KeyTrait, V: ValueTrait {
    pub fn new() -> Self {
        RBTree {
            root: None,
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.root.is_none() {
            self.root = Some(Box::new(RBNode::with_color(RBColor::Black, key, value)));
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
