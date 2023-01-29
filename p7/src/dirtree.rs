/// Arena tree node representing a directory. Inspiration:
/// https://dev.to/deciduously/no-more-tears-no-more-knots-arena-allocated-trees-in-rust-44k6
///
/// Beware: The original tree from the article had a "node" function used to
/// create a new node or return its index depending on whether a node with the
/// provided name is already present in the tree.
/// This was not compatible with the example, however, since the names of the directories
/// (=nodes) are not unique! This caused wrong (even circular) parent-child relationships,
/// and an infinite recursion in the "add_subdir_sizes" function from the main file,
/// causing a stack overflow :-) The adapted version below works.

/// Tree node. Each node has an index which is used to identify it. Consequently, also parent
/// and child nodes are represented by their indices, stored in the "parent" Option or
/// the "children" vector - in contrast to "regular" trees, where parents and children are linked
/// by references, making up a linked list - which is apparently tricky to implement without
/// invoking the wrath of the borrow checker.
#[derive(Debug)]
pub struct Node {
    pub idx: usize,
    pub name: String,
    pub size: u32,
    pub parent: Option<usize>,
    pub children: Vec<usize>,
}
impl Node {
    /// Constructor.
    fn new(idx: usize, name: String, val: u32) -> Self {
        Self {
            idx,
            name,
            size: val,
            parent: None,
            children: vec![],
        }
    }
    /// This function is used to update the size corresponding to a directory node when
    /// encountering a file belonging to the directory in the input data.
    pub fn add_filesize(&mut self, filesize: u32) {
        self.size += filesize;
    }
}

/// "Arena" representing the directory tree. Basically just a flat vector with nodes, each
/// of them identified by its index.
#[derive(Debug, Default)]
pub struct DirTree {
    pub arena: Vec<Node>,
}
impl DirTree {
    /// Creates a new node and pushes it into the arena vector.
    pub fn create_node(&mut self, name: String, val: u32) -> usize {
        let idx = self.arena.len();
        self.arena.push(Node::new(idx, name, val));
        idx
    }
}
