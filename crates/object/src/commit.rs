use std::str::FromStr;

use crate::author::*;
use anyhow::Result;

#[derive(Debug, PartialEq, Eq, Clone, Default)]
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
        let mut commit = Self::default();
        let mut contents = String::new();
        handle.read_to_string(&mut contents)?;
        let texts = contents.split("\n").collect::<Vec<&str>>();

        dbg!(&texts);

        for text in texts.iter() {
            match text {
                t if t.starts_with("tree ") => commit.tree = t.replace("tree ", "").to_string(),
                t if t.starts_with("parent ") => {
                    commit.parents.push(t.replace("parent ", "").to_string())
                }
                t if t.starts_with("author ") => commit.author = Author::from_str(t)?,
                t if t.starts_with("committer ") => commit.committer = Author::from_str(t)?,

                // TODO: parse
                t => commit.message.push_str(*t),
            }
        }

        Ok(commit)
    }
}

#[cfg(test)]
mod test {
    use std::fs::OpenOptions;

    use super::*;

    #[test]
    fn can_parse_commit0() {
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
    #[test]
    fn can_parse_commit1() {
        let root = env!("CARGO_MANIFEST_DIR");
        let mut handle = OpenOptions::new()
            .read(true)
            .open("./fixtures/commit1.txt")
            .unwrap();
        let commit = Commit::load(&mut handle);
        assert!(commit.is_ok());
        let commit = commit.unwrap();
        dbg!(&commit.tree);
    }
    #[test]
    fn can_parse_commit2() {
        let root = env!("CARGO_MANIFEST_DIR");
        let mut handle = OpenOptions::new()
            .read(true)
            .open("./fixtures/commit2.txt")
            .unwrap();
        let commit = Commit::load(&mut handle);
        assert!(commit.is_ok());
        let commit = commit.unwrap();
        dbg!(&commit);
    }
}
