use std::cell::{RefCell, RefMut, Ref};
use std::rc::{Rc, Weak};
use std::ptr;

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

    pub fn is_descendant_of(&self, other: &Node) -> bool {
        match self.get_parent() {
            Some(node) => {
                if ptr::eq(node.0.as_ptr(), other.0.as_ptr()) {
                    return true;
                } else {
                    return node.is_descendant_of(other);
                }
            }

            None => false
        }
    }

    pub fn set_parent(&self, other: &Node) {
        if ptr::eq(self.0.as_ptr(), other.0.as_ptr()) {
            panic!("attempt to set a node as its own parent");
        }

        if other.is_descendant_of(&self) {
            panic!("setting self's parent to other would result in a circular reference (other is a descendant of self)");
        }

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

    #[test]
    #[should_panic(expected = "circular reference")]
    fn tree_parents_cannot_have_their_parents_set_to_one_of_their_descendants() {
        let leaf = Node::new(10);
        let branch = Node::new(20);
        let stalk = Node::new(30);

        leaf.set_parent(&branch);
        branch.set_parent(&stalk);
        stalk.set_parent(&leaf);
    }

    #[test]
    #[should_panic(expected = "already borrowed")]
    fn tree_nodes_cannot_have_multiple_mutable_references() {
        let leaf = Node::new(10);
        let branch = Node::new(20);

        leaf.set_parent(&branch);

        let mut ref1 = leaf.get_mut_properties();
        let binding = branch.get_mut_properties();
        let mut ref2 = binding.children[0].get_mut_properties();
    }
}

//Get one-upped, Rustbook.
//Weak counts are OP.