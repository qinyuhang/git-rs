use std::fmt::{Display, Formatter};

use anyhow::Result;

pub struct Blob {
    pub contents: Vec<u8>,
}

impl Display for Blob {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            String::from_utf8(self.contents.clone()).unwrap_or("failed to get object".to_string())
        )
    }
}

impl Blob {
    fn load<T: std::io::Read>(handle: &mut T) -> Result<Self> {
        let mut contents = Vec::new();
        handle.read_to_end(&mut contents)?;

        Ok(Self { contents })
    }
}

#[cfg(test)]
mod test {
    use std::{fmt::Display, fs::OpenOptions, io::Read};

    use crate::blob::Blob;

    #[test]
    fn test_blob_load() {
        let root = env!("CARGO_MANIFEST_DIR");
        println!("root {root}");
        let mut file = OpenOptions::new()
            .read(true)
            .open("./fixtures/blob.txt")
            .unwrap();
        let mut file_clone = OpenOptions::new()
            .read(true)
            .open("./fixtures/blob.txt")
            .unwrap();
        let mut content = String::new();
        file_clone.read_to_string(&mut content).unwrap();
        let blob = Blob::load(&mut file).unwrap();
        assert_ne!(blob.contents.len(), 0);
        dbg!(&content);
        assert_eq!(blob.to_string(), content);
    }
}
