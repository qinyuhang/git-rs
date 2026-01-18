use std::str::FromStr;

use crate::author::*;
use anyhow::Result;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Commit {
    pub tree: String,
    pub parents: Vec<String>,
    pub author: Author,
    pub committer: Committer,
    pub message: String,
}

impl Commit {
    pub fn create(
        tree: String,
        parents: Vec<String>,
        author: Author,
        committer: Committer,
        message: String,
    ) -> Result<Self> {
        Ok(Self {
            tree,
            parents,
            author,
            committer,
            message,
        })
    }

    fn load<T: std::io::Read>(handle: &mut T) -> Result<Self> {
        let mut contents = String::new();
        handle.read_to_string(&mut contents)?;
        let texts = contents.split("\n").collect::<Vec<&str>>();

        dbg!(&texts);

        let tree = texts[0].replace("tree ", "").to_string();
        let (parents, hasParents) = if texts[1].starts_with("parent") {
            (vec![], true)
        } else {
            (vec![], false)
        };

        let author = if hasParents {
            Author::from_str(texts[2])?
        } else {
            Author::from_str(texts[1])?
        };

        let committer = if hasParents {
            Author::from_str(texts[3])?
        } else {
            Author::from_str(texts[2])?
        };

        let message = if hasParents {
            texts[5..].join("\n")
        } else {
            texts[4..].join("\n")
        };

        // tree
        // parents
        Ok(Self {
            tree,
            parents,
            author,
            committer,
            message,
        })
    }
}

#[cfg(test)]
mod test {
    use std::fs::OpenOptions;

    use super::*;

    #[test]
    fn can_parse() {
        let root = env!("CARGO_MANIFEST_DIR");
        let mut handle = OpenOptions::new()
            .read(true)
            .open("./fixtures/commit.txt")
            .unwrap();
        let commit = Commit::load(&mut handle);
        assert!(commit.is_ok());
        let commit = commit.unwrap();
        dbg!(&commit.tree);
    }
}
