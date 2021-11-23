//! implementation of tables that maps from array of Colors to u8 values
use crate::cube::{Color, NB_FACES};

//-----------------------------------------------------------------------------
// Type of tree

/// all types of tree
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
struct RadixTree<T>
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
            TreeType::Leaf { value } if key.is_empty() =>
            {
                // there was already an element at the key position, cancel the insertion
                false
            }
            TreeType::Node { children } if !key.is_empty() =>
            {
                // goes further in the tree
                let index_child = key[0] as usize;
                let child = &mut children[index_child];
                let new_key = &key[1..];
                child.insert(new_key, value)
            }
            _ => panic!("The keys appear to have different sizes!")
        }
    }

    /// applies a function to all keys, in order
    /// all key passed to f will be slices of color of length `key_length`
    /// if the tree contains longer keys, it will result in a crash
    pub fn for_each_key<F: FnMut(&[Color])>(&self, f: &mut F, key_length: usize)
    {
        let mut key: Vec<Color> = (0..key_length).map(|_| Color::Invalid).collect();
        let depth = 0;
        self.for_each_key_rec(f, &mut key, depth);
    }

    /// used recurcively to implement `for_each_key`
    fn for_each_key_rec<F: FnMut(&[Color])>(&self, f: &mut F, key: &mut [Color], depth: usize)
    {
        match &self.tree
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
                for (index_child, child) in children.iter().enumerate()
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

//-----------------------------------------------------------------------------
// type definitions

/// associate a length with a cube
type Table = RadixTree<u8>;

/// used to store a set of cubes
type CubeSet = RadixTree<()>;
