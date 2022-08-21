pub mod error {
    pub enum Error {
        InvalidString(String),
        #[doc(hidden)]
        __Nonexhaustive,
    }
}
