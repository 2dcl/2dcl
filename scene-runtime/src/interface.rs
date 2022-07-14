use dcl_common::*;
use deno_core::op;
use deno_core::Extension;

// fn log() {}
// fn openExternalUrl(url: String) {}
// fn on_start() {}
// fn openNFTDialog(assetContractAddress: String, tokenId: String, comment: String)
// fn addEntity(entityId: String) {}
// fn removeEntity(entityId: String) {}
// fn onUpdate(cb: String) {}
// fn onEvent(cb: String) {}
// fn updateEntityComponent(entityId: String, componentName: String, classId: u64, json: String) {}
// fn attachEntityComponent(entityId: string, componentName: string, id: string): void {
// fn removeEntityComponent(entityId: string, componentName: string): void {
// fn setParent(entityId: string, parentId: string): void {

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
      //       Component = function () {}
      // dcl = {
      //   onStart: function(m) {},
      //   onUpdate: function(m) {},
      //   onEvent: function(m) {},
      //   subscribe: function(m) {},
      //   addEntity: function(m) {},
      //   loadModule:  function(m) { 
      //     return { 
      //       then: funtion(){}
      //     };
      //   }
      // };
      // this.dcl = dcl;

}
