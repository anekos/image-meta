
mod errors;
mod types;
mod loader;

pub use loader::*;
pub use types::ImageMeta;



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
