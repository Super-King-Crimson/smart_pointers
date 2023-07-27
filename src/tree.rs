use std::cell::{RefCell, RefMut, Ref};
use std::rc::{Rc, Weak};

//with generics!
#[derive(Debug)]
pub struct NodeProperties {
    pub value: i32,
    children: Vec<Node>, 
    parent: WeakNode,
}
#[derive(Debug)]
struct WeakNode(Weak<RefCell<NodeProperties>>); 
#[derive(Debug)]
pub struct Node(Rc<RefCell<NodeProperties>>);

impl WeakNode {
    fn new() -> WeakNode {
        WeakNode(Weak::new())
    }

    fn try_upgrade(&self) -> Option<Node> {
        match self.0.upgrade() {
            Some(props) => Some(Node(props)),
            None => None,
        }
    }
}

impl Node {
    pub fn new(value: i32) -> Node {
        Node(Rc::new(RefCell::new(NodeProperties { 
            value, 
            children: Vec::new(), 
            parent: WeakNode::new(), 
        })))
    }

    pub fn get_properties(&self) -> Ref<'_, NodeProperties> {
        self.0.borrow()
    }

    pub fn get_mut_properties(&self) -> RefMut<'_, NodeProperties> {
        self.0.borrow_mut()
    }

    pub fn set_parent(&self, other: &Node) {
        if let Some(props) = self.get_properties().parent.0.upgrade() {
            props.borrow_mut().children.pop();
        }

        self.get_mut_properties().parent = WeakNode(Rc::downgrade(&other.0));
        other.get_mut_properties().children.push(Node(Rc::clone(&self.0)));
    }

    pub fn get_parent(&self) -> Option<Node> {
        Some(self.get_properties().parent.try_upgrade()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tree_parents_and_children_can_refer_to_each_other() {
        let leaf = Node::new(10);
        let branch = Node::new(20);

        leaf.set_parent(&branch);

        assert_eq!(
            leaf.get_properties().value, 
            branch.get_properties().children[0].get_properties().value
        );

        assert_eq!(
            branch.get_properties().value,
            leaf.get_parent().unwrap().get_properties().value
        );
    }

    #[test]
    fn tree_parents_and_children_refer_after_mutation() {
        let leaf = Node::new(10);
        let branch = Node::new(20);

        leaf.set_parent(&branch);

        branch.get_mut_properties().value = 100;
        
        assert_eq!(
            leaf.get_parent().unwrap().get_properties().value,
            branch.get_properties().value
        );

        assert_eq!(
            leaf.get_properties().value,
            branch.get_properties().children[0].get_properties().value
        );
    }


    #[test]
    fn tree_parents_and_children_can_mutate_each_other() {
        let leaf = Node::new(10);
        let branch = Node::new(20);

        leaf.set_parent(&branch);

        leaf.get_parent().unwrap().get_mut_properties().value = 100;

        assert_eq!(100, branch.get_properties().value);
    }

}

//Get one-upped, Rustbook.
//Weak counts are OP.