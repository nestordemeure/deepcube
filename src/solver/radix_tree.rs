//! implementation of tables that maps from array of Colors to unsigne dintegers
use std::borrow::Borrow;

use crate::cube::{Color, NB_FACES};

//-----------------------------------------------------------------------------
// Type of tree

/// all types of tree
#[derive(Clone)]
enum TreeType<T>
{
    Empty,
    Leaf
    {
        value: T
    },
    Node
    {
        children: Box<[RadixTree<T>]>
    }
}

impl<T> TreeType<T>
{
    /// creates a new nodes with empty children
    pub fn new_node() -> TreeType<T>
    {
        let children = (0..NB_FACES).map(|_| RadixTree::new()).collect();
        TreeType::Node { children }
    }
}

//-----------------------------------------------------------------------------
// Radix Tree

/// stores a tree and its operations
/// NOTE: there are several ways to improve teh datastructure
/// - increasing the arity of the nodes, consumming several colors at once
/// - storing raw leftover paths (once a value is alone in its subtree) instead of single color nodes
#[derive(Clone)]
pub struct RadixTree<T>
{
    tree: TreeType<T>
}

impl<T> RadixTree<T>
{
    /// creates a new, empty, tree
    pub fn new() -> RadixTree<T>
    {
        let tree = TreeType::Empty;
        RadixTree { tree }
    }

    /// returns true of the tree is empty
    /// note that this function might be fooled by a node full of empty subtrees
    pub fn is_empty(&self) -> bool
    {
        matches!(self.tree, TreeType::Empty)
    }

    /// get the value at the given key if it is present in the tree
    pub fn get(&self, key: &[Color]) -> Option<&T>
    {
        match &self.tree
        {
            TreeType::Leaf { value } if key.is_empty() => Some(value),
            TreeType::Node { children } if !key.is_empty() =>
            {
                // goes further in the tree
                let index_child = key[0] as usize;
                let child = &children[index_child];
                let new_key = &key[1..];
                child.get(new_key)
            }
            _ => None
        }
    }

    /// get the value at the given key if it is present in the tree
    /// panics or returns another value if it isn't
    pub fn get_unchecked(&self, key: &[Color]) -> &T
    {
        match &self.tree
        {
            TreeType::Leaf { value } => value,
            TreeType::Node { children } =>
            {
                // goes further in the tree
                let index_child = key[0] as usize;
                let child = &children[index_child];
                let new_key = &key[1..];
                child.get_unchecked(new_key)
            }
            _ => panic!("The value isn't in the tree!")
        }
    }

    /// returns true if the tree contains the key
    pub fn contains(&self, key: &[Color]) -> bool
    {
        !matches!(self.get(key), None)
    }

    /// inserts a new element in the tree
    /// returns true if the insertion suceeded
    /// and false if there was already a value in place (which will, then, not have been replaced)
    pub fn insert(&mut self, key: &[Color], value: T) -> bool
    {
        match &mut self.tree
        {
            TreeType::Empty if key.is_empty() =>
            {
                // turns an empty node into a leaf
                self.tree = TreeType::Leaf { value };
                true
            }
            TreeType::Empty =>
            {
                // expands the empty node and then inserts into it
                self.tree = TreeType::new_node();
                self.insert(key, value)
            }
            TreeType::Leaf { value } /*if key.is_empty()*/ =>
            {
                // there was already an element at the key position, cancel the insertion
                false
            }
            TreeType::Node { children } /*if !key.is_empty()*/ =>
            {
                // goes further in the tree
                let index_child = key[0] as usize;
                let child = &mut children[index_child];
                let new_key = &key[1..];
                child.insert(new_key, value)
            }
            //_ => panic!("The keys appear to have different sizes!")
        }
    }

    /// applies a function to all keys, in order
    /// all key passed to f will be slices of color of length `key_length`
    /// if the tree contains longer keys, it will result in a crash
    ///
    /// consumes the tree as we process it in order to free some memory
    ///
    /// NOTE: parallel version of this function could be build with a master-slaves architecture
    ///       where a single thread is going throug the tree and feeding the keys it builds to the slaves
    pub fn for_each_key<F: FnMut(&[Color])>(self, key_length: usize, mut f: F)
    {
        let mut key: Vec<Color> = (0..key_length).map(|_| Color::Invalid).collect();
        let depth = 0;
        self.for_each_key_rec(&mut f, &mut key, depth);
    }

    /// used recurcively to implement `for_each_key`
    fn for_each_key_rec<F: FnMut(&[Color])>(self, f: &mut F, key: &mut [Color], depth: usize)
    {
        match self.tree
        {
            TreeType::Empty =>
            {
                // empty tree, we do nothing
            }
            TreeType::Leaf { value } =>
            {
                // we reached a leaf, apply funciton to key
                f(key)
            }
            TreeType::Node { children } =>
            {
                // a node, go into each child one after the other
                for (index_child, child) in children.into_vec().into_iter().enumerate()
                {
                    // sets the color in the key
                    let color = Color::ALL[index_child];
                    key[depth] = color;
                    // visit the child with the updated key
                    child.for_each_key_rec(f, key, depth + 1);
                }
            }
        }
    }
}

impl<T: Copy + PartialEq> RadixTree<T>
{
    /// tries to reduce the size of the tree
    /// the resulting tree is meant to be used with `get_unchecked`
    /// as calls with `get` might panic or fail to find results
    pub fn compress(&mut self)
    {
        // only the nodes need to be compressed
        if let TreeType::Node { children } = &mut self.tree
        {
            // compresses the children
            let mut has_node_child = false;
            for child in children.iter_mut()
            {
                child.compress();
                // checks if the child is a node
                let child_is_node = matches!(&child.tree, TreeType::Node { children });
                has_node_child = has_node_child || child_is_node;
            }
            // try to fuse the children if they are all leafs or empty
            if !has_node_child
            {
                // finds the value that would be used for the leaf
                let leaf_value = children.iter()
                                         .find_map(|child| match &child.tree
                                         {
                                             TreeType::Leaf { value } => Some(value),
                                             _ => None
                                         })
                                         .expect("tried to compress a node will only empty children");
                // makes sure that all leafs share that same value
                let identical_values =
                    children.iter().all(|child| !matches!(&child.tree, TreeType::Leaf { value } if value != leaf_value));
                // replaces the tree with a leaf using the value
                if identical_values
                {
                    self.tree = TreeType::Leaf { value: *leaf_value };
                }
            }
        }
    }
}

//-----------------------------------------------------------------------------
// Table

/// goes from an array of color to an integer
/// NOTE: the optimal solution would be to have a way to index all (and only) legal positions into an array
pub type Table = RadixTree<u8>;

//-----------------------------------------------------------------------------
// Set

/// used to store a set of cubes
pub type CubeSet = RadixTree<()>;

impl CubeSet
{
    /// inserts a new key in the set
    /// returns true if the insertion suceeded
    /// and false if there was already an eement with that key
    pub fn insert_key(&mut self, key: &[Color]) -> bool
    {
        self.insert(key, ())
    }
}
