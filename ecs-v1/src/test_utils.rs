use dcl_common::Result;
use rmp_serde::encode::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::Read;

pub fn json_to_mp<U, R>(json: U) -> Result<Vec<u8>>
where
    U: AsRef<str>,
    R: for<'a> Deserialize<'a> + Serialize,
{
    let element: R = serde_json::from_str(json.as_ref())?;
    let mut result: Vec<u8> = Vec::new();
    element.serialize(&mut Serializer::new(&mut result))?;
    Ok(result)
}

pub fn load_json_fixture<U>(fixture: U) -> Result<String>
where
    U: AsRef<str>,
{
    let path = format!("fixtures/{}.json", fixture.as_ref());
    let mut f = File::open(&path).expect("no file found");
    let metadata = fs::metadata(&path).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read_exact(&mut buffer).expect("buffer overflow");

    Ok(String::from_utf8(buffer).unwrap())
}

pub fn load_mp_fixture<U>(fixture: U) -> Result<Vec<u8>>
where
    U: AsRef<str>,
{
    let path = format!("fixtures/{}.mp", fixture.as_ref());
    let mut f = File::open(&path).expect("no file found");
    let metadata = fs::metadata(&path).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read_exact(&mut buffer).expect("buffer overflow");

    Ok(buffer)
}

pub fn can_go_from_json_to_mp<T, U>(fixture: U)
where
    T: for<'a> Deserialize<'a> + Serialize,
    U: AsRef<str>,
{
    let fixture = fixture.as_ref();

    let json = load_json_fixture(fixture).unwrap();
    let result = json_to_mp::<&str, T>(&json).expect("json to mp failed");
    let expected = load_mp_fixture(fixture).unwrap();

    assert_eq!(result, expected);
}
