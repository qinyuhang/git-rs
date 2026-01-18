use anyhow::Result;

pub struct Blob {
    pub contents: Vec<u8>,
}

impl Blob {
    fn load<T: std::io::Read>(handle: &mut T) -> Result<Blob> {
        let mut contents = Vec::new();
        handle.read_to_end(&mut contents)?;

        Ok(Blob { contents })
    }
}

#[cfg(test)]
mod test {
    use std::fs::OpenOptions;

    use crate::blob::Blob;

    #[test]
    fn test_blob_load() {
        let root = env!("CARGO_MANIFEST_DIR");
        println!("root {root}");
        let mut file = OpenOptions::new()
            .read(true)
            .open("./fixtures/blob.txt")
            .unwrap();
        let blob = Blob::load(&mut file).unwrap();
        assert_ne!(blob.contents.len(), 0);
    }
}
