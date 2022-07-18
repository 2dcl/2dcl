use serde::Serialize;
use serde::Serializer;
use std::fmt;

use serde::de::{self, Deserialize, Deserializer, Visitor};

#[derive(Debug, PartialEq, Eq)]
pub struct Parcel(pub i16, pub i16);

struct ParcelVisitor;

impl<'de> Visitor<'de> for ParcelVisitor {
    type Value = Parcel;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an integer between -2^31 and 2^31")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let mut split = s.split(',');
        let x = split.next();
        let y = split.next();
        if let (Some(x), Some(y)) = (x, y) {
            if let (Ok(x), Ok(y)) = (x.trim().parse::<i16>(), y.trim().parse::<i16>()) {
                Ok(Parcel(x, y))
            } else {
                Err(de::Error::invalid_value(de::Unexpected::Str(s), &self))
            }
        } else {
            Err(de::Error::invalid_value(de::Unexpected::Str(s), &self))
        }
    }
}

impl Serialize for Parcel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(format!("{},{}", self.0, self.1).as_str())
    }
}

impl<'de> Deserialize<'de> for Parcel {
    fn deserialize<D>(deserializer: D) -> Result<Parcel, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_string(ParcelVisitor)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_can_deserialize_parcel() {
        let parcel: Parcel = serde_json::from_str("\"-1,10\"").unwrap();
        assert_eq!(parcel, Parcel(-1, 10));
    }

    #[test]
    fn it_can_serialize_parcel() {
        let parcel = Parcel(-1, 10);
        let serialized = serde_json::to_string(&parcel).unwrap();
        assert_eq!(serialized, "\"-1,10\"");
    }
}
