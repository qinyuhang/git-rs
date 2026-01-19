use crate::blob::Blob;
use crate::tree_entry::TreeEntry;
use anyhow::Result;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Default)]
pub struct Tree {
    pub children: Vec<TreeEntry>,
}
impl Tree {
    /// load from string
    fn load<T: std::io::Read>(handle: &mut T) -> Result<Self> {
        let mut content = String::new();
        handle.read_to_string(&mut content)?;
        let children = content
            .split("\n")
            .into_iter()
            .filter(|&line| line != "")
            .map(|line| TreeEntry::from_str(line))
            .collect::<Result<Vec<TreeEntry>>>()?;

        Ok(Self { children })
    }
}

#[cfg(test)]
mod test {
    use std::{fmt::Display, fs::OpenOptions, io::Read};

    use crate::tree::Tree;
    #[test]
    fn can_load_tree() {
        let mut file = OpenOptions::new()
            .read(true)
            .open("./fixtures/tree.txt")
            .unwrap();
        let tree = Tree::load(&mut file);
        assert!(tree.is_ok());
        let tree = tree.unwrap();
        assert_eq!(tree.children.len(), 7);
    }
}
