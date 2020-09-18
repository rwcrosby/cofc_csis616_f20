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

// Define the type of an option containing a tree node on the heap
// This is to simplify the remainder of the code
type NodeOpt = Option<Box<TreeNode>>;

/// Definition a node in the tree
struct TreeNode {
    
    val: u64,
    left: NodeOpt,
    right: NodeOpt
    
}

#[allow(dead_code)]
impl TreeNode {
    
    /// Create a new tree node with the value specified
    fn new(v: u64) -> NodeOpt {
        
        return Some(Box::new(TreeNode{val: v, left: None, right: None}))
        
    }
    
}

#[allow(dead_code)]
/// Do an inorder tranversal of the tree returning a 
/// vector containing the values
fn inorder(tn: &NodeOpt) -> Vec<u64> {

    // Get the TreeNode out of the Option if there is one,
    // if there isn't just return an empty vector
    let node= match tn {
        Some(n) => n,
        None => return vec!()
    };

    // Return the left subtree concatenated with
    // the current node concatenated with
    // the right subtree
    let mut vec = inorder(&node.left);
    vec.push(node.val);
    vec.extend(inorder(&node.right));

    vec

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

    #[test]
    fn bin_tree3() {

        // Create a simple tree with 5 nodes
        let mut tree = TreeNode::new(4);

        let root_node = tree.as_deref_mut().unwrap();

        root_node.left = TreeNode::new(2);
        root_node.right = TreeNode::new(5);

        let left_node = root_node.left.as_deref_mut().unwrap();
        left_node.left = TreeNode::new(1);
        left_node.right = TreeNode::new(3);

        // Verify we can get the desired tree out of the inorder function
        assert!(inorder(&tree) == [1,2,3,4,5]);
    }

}
