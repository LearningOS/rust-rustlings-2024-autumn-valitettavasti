/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/

use std::cmp::Ordering;
use std::fmt::Debug;


#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord + Clone,
{

    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        //TODO
        let new_node = Box::new(TreeNode::new(value.clone()));
        let mut current = &mut self.root;
        while let Some(ref mut node) = current {
            match node.value.cmp(&value) {
                Ordering::Less => {
                    if node.right.is_none() {
                        node.right = Some(new_node);
                        return;
                    } else {
                        current = &mut node.right;
                    }
                },
                Ordering::Greater => {
                    if node.left.is_none() {
                        node.left = Some(new_node);
                        return;
                    } else {
                        current = &mut node.left;
                    }
                },
                Ordering::Equal => return, 
            }
        }
        *current = Some(new_node);
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        let mut current = self.root.as_ref();
        while let Some(ref mut node) = current{
            match node.value.cmp(&value){
                Ordering::Less=>{
                    if node.right.is_none(){
                        return false;
                    }else{
                        current = node.right.as_ref();
                    }
                },
                Ordering::Greater=>{
                    if node.left.is_none(){
                        return false;
                    }else{
                        current = node.left.as_ref();
                    }
                },
                Ordering::Equal=>{
                    return true;
                }
            }
        }
    false
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        //TODO
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        
        assert_eq!(bst.search(1), false);

        
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        
        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        
        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        
        bst.insert(1);
        bst.insert(1);

        
        assert_eq!(bst.search(1), true);

        
        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}    


