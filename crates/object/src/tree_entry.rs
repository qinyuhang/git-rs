use std::{fmt, fmt::Display, str::FromStr};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TreeEntryType {
    Blob,
    Tree,
}

#[derive(Debug, PartialEq, Eq)]
pub struct TreeEntry {
    pub mode: u32,
    pub entry_type: TreeEntryType,
    pub oid: [u8; 20],
    pub name: String,
}

impl TreeEntry {}
impl FromStr for TreeEntry {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            mode: 40000,
            entry_type: TreeEntryType::Blob,
            oid: [0u8; 20],
            name: "s".to_string(),
        })
    }
}
impl Display for TreeEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "todo: TreeEntry")
    }
}

#[cfg(test)]
mod test {}
