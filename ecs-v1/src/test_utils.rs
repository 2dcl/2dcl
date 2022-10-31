use std::fs::File;
use std::fs;
use serde::{Deserialize, Serialize};
use std::io::Read;
use rmp_serde::encode::*;
use dcl_common::Result;


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

pub fn load_mp_fixture<U>(path: U) -> Result<Vec<u8>> 
where
  U: AsRef<str>,
{
  let mut f = File::open(path.as_ref()).expect("no file found");
  let metadata = fs::metadata(path.as_ref()).expect("unable to read metadata");
  let mut buffer = vec![0; metadata.len() as usize];
  f.read(&mut buffer).expect("buffer overflow");

  Ok(buffer)
}
