use flexbuffers;
use serde::{Deserialize, Serialize};
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

#[derive(Debug, Deserialize, Serialize)]
struct Node {
    name: String,
    path: String,

    // #[serde(skip)]
    #[serde(default)]
    parents: Option<Vec<Weak<RefCell<Node>>>>,

    // #[serde(default)]
    #[serde(skip)]
    children: Option<Vec<Rc<RefCell<Node>>>>,
}

impl Eq for Node {}
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.path == other.path
    }
}

#[derive(Eq, PartialEq, Hash, Deserialize, Serialize)]
struct NodeKey {
    name: String,
    path: String,
}

impl Node {
    fn key(&self) -> NodeKey {
        NodeKey {
            name: self.name.to_owned(),
            path: self.path.to_owned(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn it_works() {
        //   __a__
        //   b   c

        let a = Rc::new(RefCell::new(Node {
            name: "a".to_string(),
            path: "src/a.js".to_string(),
            parents: None,
            children: None,
        }));

        let b = Rc::new(RefCell::new(Node {
            name: "b".to_string(),
            path: "src/some/where/b.js".to_string(),
            parents: None,
            children: None,
        }));

        let c = Rc::new(RefCell::new(Node {
            name: "c".to_string(),
            path: "src/some/where/c.js".to_string(),
            parents: None,
            children: None,
        }));

        a.borrow_mut().children = Some(vec![Rc::clone(&b), Rc::clone(&c)]);
        b.borrow_mut().parents = Some(vec![Rc::downgrade(&a)]);
        c.borrow_mut().parents = Some(vec![Rc::downgrade(&a)]);

        println!("{:#?}", a);
        println!("{:#?}", b);
        println!("{:#?}", c);

        assert_eq!(b, b);

        // let mut map: HashMap<NodeKey, Rc<RefCell<Node>>> = HashMap::new();
        // map.insert(a.borrow().key(), Rc::clone(&a));
        // map.insert(b.borrow().key(), Rc::clone(&b));
        // map.insert(c.borrow().key(), Rc::clone(&c));
        let mut map: HashMap<String, Rc<RefCell<Node>>> = HashMap::new();
        map.insert("a".to_string(), Rc::clone(&a));
        map.insert("b".to_string(), Rc::clone(&b));
        map.insert("c".to_string(), Rc::clone(&c));

        let mut s = flexbuffers::FlexbufferSerializer::new();
        map.serialize(&mut s).unwrap();
        let r = flexbuffers::Reader::get_root(s.view()).unwrap();

        println!("Map stored in {:?} bytes.", s.view().len());
        println!("{}", r);
    }
}
