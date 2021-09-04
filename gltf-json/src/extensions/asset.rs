use gltf_derive::Validate;
use serde_derive::{Deserialize, Serialize};

use crate::validation::Validate;

/// Metadata about the glTF asset.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct Asset {
    #[serde(default, rename = "ASOBO_normal_map_convention")]
    pub asobo_normal_map_convention: AsoboNormalMapConvention,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum TangentSpaceConvention {
    DirectX,
}

impl Default for TangentSpaceConvention {
    fn default() -> Self {
        Self::DirectX
    }
}

impl Validate for TangentSpaceConvention {}

#[derive(Clone, Default, Debug, Deserialize, Serialize, Validate)]
pub struct AsoboNormalMapConvention {
    pub tangent_space_convention: TangentSpaceConvention,
}
