use crate as docify;

/// These are some docs
/// These are some more docs
#[doc = docify::embed!("tests/samples.rs", MyCoolStruct)]
/// even more here
#[allow(unused)]
pub struct SomeItem;
