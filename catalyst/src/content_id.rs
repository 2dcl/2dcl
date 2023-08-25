use serde::{Deserialize, Serialize};
use std::fmt;

use crate::HashId;

/// Used to represent a hash that is used in the context of a content file id.
///
/// This struct implements `Display` to simplify the formatting of urls and messages.
///
/// ```
/// let contentId = catalyst::ContentId::new("a-missing-content");
/// let message = format!("content missing: {}", contentId);
/// assert_eq!(message, "content missing: a-missing-content");
/// ```
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct ContentId(HashId);
impl ContentId {
    /// Creates a new instance of ContentId by receiving a string.
    ///
    /// # Example
    ///
    /// ```
    /// use catalyst::ContentId;
    /// let cid = ContentId::new("bafkreicq4z2rpf6fbxw5oxxrxcqntzbpl6chf2p3z2i7gryy5h4xwddybi");
    /// ```
    pub fn new<T>(id: T) -> ContentId
    where
        T: AsRef<str>,
    {
        ContentId(id.as_ref().to_string())
    }

    /// Returns the hash string for the `ContentId`
    ///
    /// # Example
    ///
    /// ```
    /// use catalyst::ContentId;
    /// let cid = ContentId::new("a-hash");
    /// assert_eq!(cid.hash(), "a-hash");
    /// ```
    pub fn hash(&self) -> &HashId {
        &self.0
    }
}

impl fmt::Display for ContentId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod test {
    use crate::ContentId;

    #[test]
    fn it_accepts_string() {
        let id = ContentId::new(String::from("test"));
        assert_eq!(id.0, "test");
    }

    #[test]
    fn it_has_accessor_for_hash() {
        let cid = ContentId::new("a-hash");
        assert_eq!(cid.hash(), "a-hash");
    }

    #[test]
    fn it_implements_display() {
        let id = ContentId::new("id");
        let id_string = format!("{}", id);
        assert_eq!(id_string, "id");
    }
}
