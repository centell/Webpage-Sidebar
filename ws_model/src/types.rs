/// new type pattern for URL string!
/// - [see explanation](https://github.com/rust-unofficial/patterns/blob/master/patterns/newtype.md)
#[derive(
    Debug, Clone, PartialEq, 
    Serialize, Deserialize
)]
pub struct Url(pub String);

impl Url {
    pub fn new(url: String) -> Self {
        Url(url)
    }
    pub fn value(&self) -> &String {
        &self.0
    }
}