pub mod author;
pub mod blob;
pub mod commit;
pub mod tree;
pub trait Object {
    type Loaded;
    fn load() -> std::result::Result<Self::Loaded, String>;
}
