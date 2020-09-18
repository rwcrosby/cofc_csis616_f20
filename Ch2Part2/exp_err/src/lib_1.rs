//! CSIS-616 - A simple binary tree
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
//! # First version
//! - Just a function to create a tree node

/// A node in the binary tree. Contains an unsigned integer value.
pub struct TreeNode {

    val: u64,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>

}

/// Create and return a tree node as a Option::Some 
pub fn bin_tree() -> Option<Box<TreeNode>> {

    return Some(Box::new(TreeNode{val: 1, left: None, right: None}))

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn bin_tree1() {

        // Just test that we created a node
        assert!(match bin_tree() {
            Some(_) => true,
            None => false
        });

    }

}
