use gltf_derive::Validate;
use serde_derive::{Deserialize, Serialize};

use crate::validation::Validate;

/// The root object of a glTF 2.0 asset.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct Root {
    #[cfg(feature = "KHR_lights_punctual")]
    #[serde(
        default,
        rename = "KHR_lights_punctual",
        skip_serializing_if = "Option::is_none"
    )]
    pub khr_lights_punctual: Option<KhrLightsPunctual>,
    #[serde(
        default,
        rename = "ASOBO_normal_map_convention",
        skip_serializing_if = "Option::is_none"
    )]
    pub asobo_normal_map_convention: Option<AsoboNormalMapConvention>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum TangentSpaceConvention {
    DirectX,
}

impl Validate for TangentSpaceConvention {}

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct AsoboNormalMapConvention {
    pub tangent_space_convention: TangentSpaceConvention,
}

#[cfg(feature = "KHR_lights_punctual")]
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct KhrLightsPunctual {
    /// Lights at this node.
    pub lights: Vec<crate::extensions::scene::khr_lights_punctual::Light>,
}

#[cfg(feature = "KHR_lights_punctual")]
impl crate::root::Get<crate::extensions::scene::khr_lights_punctual::Light> for crate::Root {
    fn get(
        &self,
        id: crate::Index<crate::extensions::scene::khr_lights_punctual::Light>,
    ) -> Option<&crate::extensions::scene::khr_lights_punctual::Light> {
        if let Some(extensions) = self.extensions.as_ref() {
            if let Some(khr_lights_punctual) = extensions.khr_lights_punctual.as_ref() {
                khr_lights_punctual.lights.get(id.value())
            } else {
                None
            }
        } else {
            None
        }
    }
}
