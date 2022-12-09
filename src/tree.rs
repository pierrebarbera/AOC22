use std::cell::RefCell;
use std::error;
use std::fmt;
use std::rc::Rc;

pub type NodeRef<T> = Rc<RefCell<Node<T>>>;

use std::sync::atomic::{AtomicUsize, Ordering};
fn get_unique_number() -> usize {
    static COUNTER: AtomicUsize = AtomicUsize::new(0);
    COUNTER.fetch_add(1, Ordering::Relaxed)
}

#[derive(Clone)]
pub struct Node<T>
where
    T: Clone,
{
    pub unique_id: usize,
    pub name: String,
    pub value: T,
    pub children: Vec<NodeRef<T>>,
    pub parent: Option<NodeRef<T>>,
}

impl<T> PartialEq for Node<T>
where
    T: Clone,
{
    fn eq(&self, other: &Self) -> bool {
        self.unique_id == other.unique_id
    }
}
impl<T> fmt::Display for Node<T>
where
    T: Clone + std::fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let id = &self.unique_id;
        let name = &self.name;
        let value = &self.value;
        write!(f, "- {} {}, ({})\n", id, name, value)
    }
}

#[derive(Debug)]
pub enum NodeError {
    Add(String),
    NotFound(String),
}
impl fmt::Display for NodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NodeError::Add(msg) => write!(f, "Could not add node: {}", msg),
            NodeError::NotFound(msg) => write!(f, "Node not found: {}", msg),
        }
    }
}
impl error::Error for NodeError {}

/* Member functions of Node */
impl<T> Node<T>
where
    T: Clone,
{
    pub fn new(name: String, value: T, parent: Option<NodeRef<T>>) -> Self {
        Node {
            unique_id: get_unique_number(),
            name: name,
            value: value,
            children: Vec::new(),
            parent: parent,
        }
    }
    pub fn add(&mut self, name: &str, value: T) -> Result<NodeRef<T>, NodeError> {
        if let Some(_) = self.get_child(name) {
            Err(NodeError::Add(format!(
                "'{}' already exists under node '{}'",
                name, self.name
            )))
        } else {
            let child = new_node_ref(name, value, None);
            self.children.push(Rc::clone(&child));
            Ok(child)
        }
    }

    pub fn get_child(&self, child_name: &str) -> Option<NodeRef<T>> {
        match self
            .children
            .iter()
            .find(|node| node.borrow().name == child_name)
        {
            Some(n) => Some(Rc::clone(&n)),
            None => None,
        }
    }

    pub fn down(&self, name: &str) -> Result<NodeRef<T>, NodeError> {
        match self.get_child(name) {
            Some(child) => Ok(Rc::clone(&child)),
            None => Err(NodeError::NotFound(format!("'{}' not found", name))),
        }
    }

    pub fn up(&self) -> Option<NodeRef<T>> {
        match &self.parent {
            Some(p) => Some(p.clone()),
            None => None,
        }
    }
}

pub fn new_node_ref<T>(name: &str, value: T, parent: Option<NodeRef<T>>) -> NodeRef<T>
where
    T: Clone,
{
    Rc::new(RefCell::new(Node::new(name.to_string(), value, parent)))
}

pub fn add_node_below<T>(current: NodeRef<T>, name: &str, value: T)
where
    T: Clone,
{
    if let Ok(child) = current.borrow_mut().add(name, value) {
        child.borrow_mut().parent = Some(current.clone());
    }
}

pub fn go_up<T>(current: NodeRef<T>) -> Option<NodeRef<T>>
where
    T: Clone,
{
    Some(current.borrow().up()?.clone())
}

pub fn go_down<T>(current: NodeRef<T>, dir: &str) -> Result<NodeRef<T>, NodeError>
where
    T: Clone,
{
    Ok(current.borrow().down(dir)?)
}

pub fn set_value<T>(node: NodeRef<T>, value: T)
where
    T: Clone,
{
    node.borrow_mut().value = value;
}

pub fn get_value<T>(node: &NodeRef<T>) -> T
where
    T: Clone,
{
    node.borrow().value.clone()
}
use std::io::{self, Write};
pub fn print_tree<T>(root: NodeRef<T>)
where
    T: Clone + std::fmt::Display,
{
    let mut level: usize = 0;
    let mut previous_node: NodeRef<T> = Rc::clone(&root);
    traverse_tree_apply_if(
        root,
        |_| true,
        |node| {
            // let parent = Rc::clone(node.borrow().parent.as_ref().unwrap());
            // if parent == previous_node {
            //     // level += 1;
            // } else if to_usize_iter(node.borrow().children.iter())
            //     .collect::<Vec<usize>>()
            //     .contains(&previous_node.borrow().unique_id)
            // {
            //     // level -= 1;
            // }
            let mut writer = Box::new(io::stdout()) as Box<dyn Write>;

            writer
                .write_fmt(format_args!(
                    "{} {}",
                    std::iter::repeat(" ").take(level).collect::<String>(),
                    node.borrow()
                ))
                .unwrap();
            previous_node = Rc::clone(&node);
        },
    );
}

pub fn traverse_tree_apply_if<T, UnaryPredicate, F>(root: NodeRef<T>, p: UnaryPredicate, f: F)
where
    UnaryPredicate: FnMut(NodeRef<T>) -> bool,
    F: FnMut(NodeRef<T>),
    T: Clone,
{
    postorder_apply_if(root, p, f);
}

fn to_usize_iter<'a, T, I>(iter: I) -> impl Iterator<Item = usize> + 'a
where
    I: Iterator<Item = &'a NodeRef<T>> + 'a,
    T: Clone + 'a,
{
    iter.map(|v| v.borrow().unique_id)
}

fn postorder_apply_if<T, UnaryPredicate, F>(root: NodeRef<T>, mut p: UnaryPredicate, mut f: F)
where
    UnaryPredicate: FnMut(NodeRef<T>) -> bool,
    F: FnMut(NodeRef<T>),
    T: Clone,
{
    let mut stack: Vec<NodeRef<T>> = Vec::new();
    stack.push(root.clone());
    let mut cur_node = root.borrow().children[0].clone();
    stack.extend(root.borrow().children[..].iter().rev().cloned());
    let mut coming_up = false;

    while cur_node != root {
        let next_node: NodeRef<T>;
        if cur_node.borrow().children.is_empty() || coming_up {
            // this is a leaf node
            // or we are on the way up
            // both cases mean we visit
            if p(cur_node.clone()) {
                f(cur_node.clone());
            }
            // we have visited this node, so we can pop it off the stack
            let popped_node = stack.pop().unwrap();
            assert!(cur_node.borrow().unique_id == popped_node.borrow().unique_id);

            // then we go to the next node, which is the next node on the stack
            next_node = stack.last().unwrap().clone();

            // lastly we check if the next node would be the current parent, in which case we are on the way up
            let parent = Rc::clone(cur_node.borrow().parent.as_ref().unwrap());
            coming_up = parent.borrow().unique_id == next_node.borrow().unique_id;
        } else {
            // if this is an interior node, and we came here from above or sideways, we push all children
            // on the stack and go down
            next_node = cur_node.borrow().children[0].clone();
            // if the next one down is a leaf, no put on stack pls
            stack.extend(cur_node.borrow().children[..].iter().rev().cloned());
            coming_up = false;
        }
        cur_node = Rc::clone(&next_node);
    }

    // finally, visit the root
    if p(root.clone()) {
        f(root.clone());
    }
}
