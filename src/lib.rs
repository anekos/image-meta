
mod errors;
mod types;
mod loader;

pub use types::ImageMeta;
pub use loader::png::load;



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
