// #[derive(Debug)]
// pub enum Error {
//     Custom(String),
// }
pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = core::result::Result<T, Error>;
