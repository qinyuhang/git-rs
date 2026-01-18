use crate::author::*;
use anyhow::Result;

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
}

#[cfg(test)]
mod test {
    use super::*;
}
