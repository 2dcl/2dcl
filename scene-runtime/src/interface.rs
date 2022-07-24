
use serde::Deserialize;
use deno_core::op;
use deno_core::Extension;


#[derive(Debug, Deserialize)]
struct EntityId(String);

#[op]
fn op_log(text: String) {
  println!("{}", text);
}

#[op]
fn op_add_entity(entity_id: EntityId) {
  print!("EntityId: {:?}", entity_id);
}

pub fn ops() -> Vec<Extension> {
  let ext = Extension::builder()
  .ops(vec![
    op_log::decl(),
    op_add_entity::decl()
  ])
  .build();
  vec![ext]
}
