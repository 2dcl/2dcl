use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum BlendFactor {
    Zero,
    One,
    Src,
    OneMinusSrc,
    SrcAlpha,
    OneMinusSrcAlpha,
    Dst,
    OneMinusDst,
    DstAlpha,
    OneMinusDstAlpha,
    SrcAlphaSaturated,
    Constant,
    OneMinusConstant,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BlendOptions {
    pub src: BlendFactor,
    pub dst: BlendFactor,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum BlendMode {
    Add,
    AlphaBlend,
    Multiply,
    Custom {
        color: BlendOptions,
        alpha: BlendOptions,
    },
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_utils::*;

    #[test]
    fn can_be_serialized_from_json() {
        can_go_from_json_to_mp::<BlendMode, _>("blend_mode/add");
        can_go_from_json_to_mp::<BlendMode, _>("blend_mode/alpha_blend");
        can_go_from_json_to_mp::<BlendMode, _>("blend_mode/multiply");
    }

    #[test]
    fn can_serialize_custom_value_from_json() {
        can_go_from_json_to_mp::<BlendMode, _>("blend_mode/custom");
    }
}
