use std::collections::{BTreeMap, btree_map::Entry};
use super::miniCube::MiniCube;

/// used to store distances to cubes
/// TODO: we might want to use a more memory efficient datastructure instead of the btree: https://crates.io/crates/btree-slab
pub struct Table
{
    table: BTreeMap<MiniCube, u8>
}

impl Table
{
    /// creates a new, empty, table
    pub fn new() -> Table
    {
        let table = BTreeMap::new();
        Table { table }
    }

    /// returns the number of elements in the table
    pub fn len(&self) -> usize
    {
        self.table.len()
    }

    /// adds a pair minicube/distance to the table
    /// returns false if the minicube was already known (in wihich case no insertion is performed)
    pub fn insert(&mut self, minicube: MiniCube, distance: u8) -> bool
    {
        match self.table.entry(minicube)
        {
            Entry::Vacant(entry) =>
            {
                entry.insert(distance);
                true
            }
            _ => false
        }
    }

    /// returns the distance associated with a minicube
    /// panics if the cube does not exist
    pub fn get(&self, minicube: MiniCube) -> u8
    {
        *self.table.get(&minicube).expect("You asked for a cube that is not recorded!")
    }
}

/// to enable collection into a table
impl FromIterator<(MiniCube, u8)> for Table
{
    fn from_iter<I: IntoIterator<Item = (MiniCube, u8)>>(iter: I) -> Self
    {
        let mut table = Table::new();

        for (minicube, distance) in iter
        {
            table.insert(minicube, distance);
        }

        table
    }
}
