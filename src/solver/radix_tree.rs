//! implementation of tables that maps from array of Colors to unsigned integers
use crate::cube::{Color, NB_FACES};

//-----------------------------------------------------------------------------
// Type of tree

/// all types of tree
#[derive(Clone)]
enum TreeType
{
    Empty,
    Leaf
    {
        leftover_key: Box<[Color]>
    },
    /// a node with a branch per color, some of which might be empty
    /// it has size CubeSet*NB_FACES
    Node
    {
        children: Box<[CubeSet; NB_FACES]>
    } // a sparse node with only some colors
      // it has size len*(colour+CubeSet) = 2*len*CubeSet
      // so it is smaller when len <= NB_FACES/2
      /*SparseNode
      {
          children: Vec<(Color, CubeSet)>
      }*/
}

impl TreeType
{
    /// creates a new nodes with empty children
    pub fn new_node() -> TreeType
    {
        let children: [CubeSet; NB_FACES] = Default::default();
        TreeType::Node { children: Box::new(children) }
    }
}

//-----------------------------------------------------------------------------
// Set of Cubes

/// stores a tree and its operations
/// NOTE: how to improve the datastructure?
/// - one could introduce SparseNodes and DenseNodes
/// - one could try and normalize inputs before insertion in order to reduce the number of different elements
/// (as long as the normalization doesn't impact the time to solve)
/// - we could use the fact that the last face is entirely defined by the other faces and, thus, does not need to be included (at least if we don't need to reconstruct)
///   this brings some small improvements but is bug prone, the idea seem only worth using if it is combined with a wrapper
/// - we could replace colors with corners (there are 8 different corners, their respective position can be encoded by their order, we are now just missing their orientation)
/// - we could maybe just encode corners as unique integers (cutting the last nb_face elements as they are determined by the rest) and then use a Btree
#[derive(Clone)]
pub struct CubeSet
{
    tree: TreeType
}

impl CubeSet
{
    /// creates a new, empty, tree
    pub fn new() -> CubeSet
    {
        let tree = TreeType::Empty;
        CubeSet { tree }
    }

    /// returns true of the tree is empty
    /// note that this function might be fooled by a node full of empty subtrees
    pub fn is_empty(&self) -> bool
    {
        matches!(self.tree, TreeType::Empty)
    }

    /// returns true if the tree contains the key
    pub fn contains(&self, key: &[Color]) -> bool
    {
        match &self.tree
        {
            TreeType::Leaf { leftover_key } if key == &leftover_key[..] => true,
            TreeType::Node { children } if !key.is_empty() =>
            {
                // goes further in the tree
                let index_child = key[0] as usize;
                let child = &children[index_child];
                let new_key = &key[1..];
                child.contains(new_key)
            }
            _ => false
        }
    }

    /// inserts a new element in the tree
    /// returns true if the insertion suceeded
    /// and false if there was already a value in place (which will, then, not have been replaced)
    pub fn insert(&mut self, key: &[Color]) -> bool
    {
        match &mut self.tree
        {
            TreeType::Empty =>
            {
                // turns an empty node into a leaf
                let leftover_key = key.iter().cloned().collect();
                self.tree = TreeType::Leaf { leftover_key };
                true
            }
            TreeType::Leaf { leftover_key } if key == &leftover_key[..] =>
            {
                // there was already an element at the key position, cancels the insertion
                false
            }
            TreeType::Leaf { leftover_key } =>
            {
                // the leftoverkey is different, create a node and insert both keys in it
                let mut new_tree: CubeSet = CubeSet{tree:TreeType::new_node()};
                new_tree.insert(leftover_key);
                new_tree.insert(key);
                self.tree = new_tree.tree;
                true
            }
            TreeType::Node { children } /*if !key.is_empty()*/ =>
            {
                // goes further in the tree
                let index_child = key[0] as usize;
                let child = &mut children[index_child];
                let new_key = &key[1..];
                child.insert(new_key)
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
    pub fn for_each<F: FnMut(&[Color])>(self, key_length: usize, mut f: F)
    {
        let mut key: Vec<Color> = (0..key_length).map(|_| Color::Invalid).collect();
        let depth = 0;
        self.for_each_rec(&mut f, &mut key, depth);
    }

    /// used recurcively to implement `for_each_key`
    fn for_each_rec<F: FnMut(&[Color])>(self, f: &mut F, key: &mut [Color], depth: usize)
    {
        match self.tree
        {
            TreeType::Empty =>
            {
                // empty tree, we do nothing
            }
            TreeType::Leaf { leftover_key } =>
            {
                // we reached a leaf, completes the key and applies the function to it
                for (i, color) in leftover_key.iter().enumerate()
                {
                    key[depth + i] = *color;
                }
                f(key)
            }
            TreeType::Node { children } =>
            {
                // a node, go into each child one after the other
                for (index_child, child) in children.into_iter().enumerate()
                {
                    // sets the color in the key
                    let color = Color::ALL[index_child];
                    key[depth] = color;
                    // visit the child with the updated key
                    child.for_each_rec(f, key, depth + 1);
                }
            }
        }
    }
}

impl Default for CubeSet
{
    fn default() -> CubeSet
    {
        CubeSet::new()
    }
}
