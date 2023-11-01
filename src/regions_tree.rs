use std::cmp::Ordering;

use avl::AvlTreeMap;

// Define the Leaf struct to store start, end, and id values
#[derive(Debug, Clone, Copy)]
struct Leaf {
    start: usize,
    end: usize,
    id: usize,
}


impl Leaf {
    // fn overlaps(&self, other: &Self) -> bool {
    //     self.start <= other.end && self.end >= other.start
    // }
}

impl Ord for Leaf {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.start > other.end {
            Ordering::Greater
        } else if self.end < other.start {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
}

impl PartialOrd for Leaf {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Leaf {}

impl PartialEq for Leaf {
    fn eq(&self, other: &Self) -> bool {
        // Define your custom equality logic here
        // Return true if your custom equality condition is met
        // Return false otherwise
        // For example:
        self.start < other.end && self.end > other.start
    }
}
// Define a custom RegionsTree struct to manage the tree
#[derive(Debug)]
pub struct RegionTree  {
    tree: AvlTreeMap<Leaf, usize>,
    leaf_store: Vec<Leaf>,
}

impl RegionTree  {
    pub fn new() -> Self {
        RegionTree{
            tree: AvlTreeMap::new(),
            leaf_store : Vec::<Leaf>::with_capacity(100_000),
        }
    }

    pub fn insert(&mut self, start: usize, end: usize) -> usize {
        //println!("Leaf store adding a new entry: {start}-{end} with id possible {}", self.leaf_store.len());
        let id = self.leaf_store.len();
        let mut new_leaf = Leaf { start, end, id};
        let mut modified = false;
        let id = match self.tree.get(&new_leaf) {
            Some( id ) => {
                //println!("We found a match");
                if new_leaf.start > self.leaf_store[*id].start{
                    new_leaf.start = self.leaf_store[*id].start;
                    modified = true
                }
                if new_leaf.end < self.leaf_store[*id].end{
                    new_leaf.end = self.leaf_store[*id].end;
                    modified = true
                }
                *id
            }
            None => {
                // No matching leaf, insert the new one
                self.tree.insert(new_leaf, new_leaf.id );
                self.leaf_store.push( new_leaf );
                new_leaf.id
            }
        };
        if modified {
            //println!("The leaf is updated!");
            if let Some(_value) = &self.tree.remove( &self.leaf_store[id.clone()] ) {
                //println!("old entry: {:?}", self.leaf_store[id.clone()]);
                new_leaf.id = self.leaf_store[id.clone()].id;
                //println!("new entry: {:?}", new_leaf);
                self.tree.insert(new_leaf.clone(), id.clone() );
                self.leaf_store[id.clone()] = new_leaf.clone();
            }
        }
        id
    }

    pub fn to_array(&self) -> Vec<(usize, usize)> {
        let mut ret = vec![(0, 0); self.leaf_store.len() ];

        for leaf in &self.leaf_store{
            //println!("Trying to export the data as vec: id: {}",leaf.id );
            ret[leaf.id] = ( leaf.start, leaf.end );
        }
        ret
    }

}

#[cfg(test)]
mod tests {

    use crate::regions_tree::Leaf;
    use crate::regions_tree::RegionTree;

    #[test]
    fn check_leaf() {
        let l1 = Leaf{ start:100, end:300, id:0};
        let l2 = Leaf{ start:50, end:150, id:1};
        assert!( l1 == l2, "region 100-300 overlaps with region 50-150");
        let l3 = Leaf{ start:500, end:510, id:3};
        assert!( l1 != l3, "region 100-300 does NOT overlap with region 500-510");
    }
    #[test]
    fn check_tree() {
        let mut tree = RegionTree::new();
        let mut id = tree.insert( 100, 300 );
        assert!(id == 0, "first entry gets id == 0");
        id = tree.insert( 500, 510 );
        assert!(id == 1, "second entry gets id == 1");
        id = tree.insert( 50, 150 );
        assert!(id == 0, "overlapping entry with read 1 gets id == 0 ({})", id);
    }


}
