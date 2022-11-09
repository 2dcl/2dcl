use dcl2d_ecs_v1::blend_mode::BlendFactor::One;
use dcl2d_ecs_v1::blend_mode::{BlendMode, BlendOptions};

fn main()
{
  let blend_mode = BlendMode::Custom {
    color: BlendOptions {
      src: One,
      dst: One
    },
    alpha: BlendOptions {
      src: One,
      dst: One
    }
  };

  let string = serde_json::to_string(&blend_mode).unwrap();
  println!("{}", string);
}
