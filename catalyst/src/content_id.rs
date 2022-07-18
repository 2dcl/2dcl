use serde::Deserialize;
use std::fmt;

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct ContentId(pub String);
impl ContentId {
    pub fn new<T>(id: T) -> ContentId
    where
        T: AsRef<str>,
    {
        ContentId(id.as_ref().to_string())
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
    fn it_implements_display() {
        let id = ContentId::new("id");
        let id_string = format!("{}", id);
        assert_eq!(id_string, "id");
    }
}
