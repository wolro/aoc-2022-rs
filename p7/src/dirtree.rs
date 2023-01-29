/// Arena tree node representing a directory. Inspiration:
/// https://dev.to/deciduously/no-more-tears-no-more-knots-arena-allocated-trees-in-rust-44k6
#[derive(Debug)]
pub struct Node
{
    pub idx: usize,
    pub name: String,
    pub size: u32,
    pub parent: Option<usize>,
    pub children: Vec<usize>,
}
impl Node
{
    fn new(idx: usize, name: String, val: u32) -> Self {
        Self {
            idx,
            name,
            size: val,
            parent: None,
            children: vec![],
        }
    }

    pub fn add_filesize(&mut self, filesize: u32) {
        self.size += filesize;
    }
}

/// Arena tree representing directory tree.
#[derive(Debug, Default)]
pub struct DirTree
{
    pub arena: Vec<Node>,
}
impl DirTree
{
    pub fn node(&mut self, name: String, val: u32) -> usize {
        //first see if it exists
        for node in &self.arena {
            if node.name == name {
                return node.idx;
            }
        }
        // Otherwise, add new node
        let idx = self.arena.len();
        self.arena.push(Node::new(idx, name, val));
        idx
    }
}