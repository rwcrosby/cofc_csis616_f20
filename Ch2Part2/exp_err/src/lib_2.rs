//! CSIS-616 - Expressions and Errors Example
//! 
//! Ralph W. Crosby PhD.
//! 
//! 
//! # Usage
//! 
//!    ```shell
//!     cargo test
//!     ```
//! 

pub struct TreeNode {

    val: u64,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>

}

// Make a new method for the structure taking the value to use
impl TreeNode {

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

}
