//! CSIS-616 - Expressions and Errors Example
//! 
//! Ralph W. Crosby PhD.
//! 
//! 
//! # Usage
//! 
//!    ```shell
//!    cargo test
//!    ```
//! 
//!    or, to see the print output:
//! 
//!    ```shell
//!    cargo test -- --nocapture
//!    ```
//! 

/// Definition a node in the tree
#[allow(dead_code)]
struct TreeNode {
    
    val: u64,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>
    
}

#[allow(dead_code)]
impl TreeNode {
    
    /// Create a new tree node with the value specified
    fn new(v: u64) -> Option<Box<TreeNode>> {
        
        return Some(Box::new(TreeNode{val: v, left: None, right: None}))
        
    }
    
}

/// The tests of it all
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn bin_tree1() {

        // Did we get a node containing the value we set?
        assert!(match TreeNode::new(23) {
                    Some(node) => node.val == 23,
                    None => false
                });
    }

    #[test]
    /// Create a root node and two attached children
    /// 
    /// To actually get to a mutable tree node we need to:
    /// 
    /// 1. Create a new Option containing a mutable reference to the 
    ///    TreeNode: `.as_deref_mut()`
    /// 2. Get the `Some` value out of the option: `.unwrap()`
    /// 
    /// This test prints the address of the root node as it goes along,
    /// All the values printed should be the same
    fn bin_tree2() {

        let mut tree = TreeNode::new(1);
        println!("{:p}", tree.as_deref_mut().unwrap());

        // tn1 will be an `Option::Some(&mut TreeNode)`
        let tn1 = tree.as_deref_mut();
        // tn will be an `&mut TreeNode`
        let tn = tn1.unwrap();

        // Now we can use the TreeNode reference and set the children pointers
        tn.left = TreeNode::new(2);
        tn.right = TreeNode::new(3);

        println!("{:p}", tn);

        // Didn't need a mutable tree node here
        println!("{:p}", tree.as_deref().unwrap());

        assert!(tree.as_deref().unwrap().val == 1);
    }

}
