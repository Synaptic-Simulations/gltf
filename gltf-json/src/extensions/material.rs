#[cfg(feature = "KHR_materials_pbrSpecularGlossiness")]
use crate::material::StrengthFactor;
use crate::texture;
use crate::validation::Validate;
#[cfg(any(
    feature = "KHR_materials_pbrSpecularGlossiness",
    feature = "KHR_materials_transmission",
    feature = "KHR_materials_ior"
))]
use crate::Extras;
use gltf_derive::Validate;
use serde_derive::{Deserialize, Serialize};

/// The material appearance of a primitive.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct Material {
    #[cfg(feature = "KHR_materials_pbrSpecularGlossiness")]
    #[serde(
        default,
        rename = "KHR_materials_pbrSpecularGlossiness",
        skip_serializing_if = "Option::is_none"
    )]
    pub pbr_specular_glossiness: Option<PbrSpecularGlossiness>,

    #[cfg(feature = "KHR_materials_unlit")]
    #[serde(
        default,
        rename = "KHR_materials_unlit",
        skip_serializing_if = "Option::is_none"
    )]
    pub unlit: Option<Unlit>,

    #[cfg(feature = "KHR_materials_transmission")]
    #[serde(
        default,
        rename = "KHR_materials_transmission",
        skip_serializing_if = "Option::is_none"
    )]
    pub transmission: Option<Transmission>,

    #[cfg(feature = "KHR_materials_ior")]
    #[serde(
        default,
        rename = "KHR_materials_ior",
        skip_serializing_if = "Option::is_none"
    )]
    pub ior: Option<Ior>,

    #[serde(
        default,
        rename = "ASOBO_material_alphamode_dither",
        skip_serializing_if = "Option::is_none"
    )]
    pub alpha_mode_dither: Option<AsoboMaterialAlphaModeDither>,

    #[serde(
        default,
        rename = "ASOBO_material_anisotropic",
        skip_serializing_if = "Option::is_none"
    )]
    pub anisotropic: Option<AsoboMaterialAnisotropic>,

    #[serde(
        default,
        rename = "ASOBO_material_blend_gbuffer",
        skip_serializing_if = "Option::is_none"
    )]
    pub decal: Option<AsoboMaterialDecal>,

    #[serde(
        default,
        rename = "ASOBO_material_clear_coat",
        skip_serializing_if = "Option::is_none"
    )]
    pub clear_coat: Option<AsoboMaterialClearCoat>,

    #[serde(
        default,
        rename = "ASOBO_material_detail_map",
        skip_serializing_if = "Option::is_none"
    )]
    pub detail_map: Option<AsoboMaterialDetailMap>,

    #[serde(
        default,
        rename = "ASOBO_material_draw_order",
        skip_serializing_if = "Option::is_none"
    )]
    pub draw_order: Option<AsoboMaterialDrawOrder>,

    #[serde(
        default,
        rename = "ASOBO_material_fake_terrain",
        skip_serializing_if = "Option::is_none"
    )]
    pub fake_terrain: Option<AsoboMaterialFakeTerrain>,

    #[serde(
        default,
        rename = "ASOBO_material_glass",
        skip_serializing_if = "Option::is_none"
    )]
    pub fresnel_face: Option<AsoboMaterialFresnelFade>,

    #[serde(
        default,
        rename = "ASOBO_material_glass",
        skip_serializing_if = "Option::is_none"
    )]
    pub glass: Option<AsoboMaterialGlass>,

    #[serde(
        default,
        rename = "ASOBO_material_invisible",
        skip_serializing_if = "Option::is_none"
    )]
    pub invisible: Option<AsoboMaterialInvisible>,

    #[serde(
        default,
        rename = "ASOBO_material_parallax_window",
        skip_serializing_if = "Option::is_none"
    )]
    pub parallax_window: Option<AsoboMaterialParallaxWindow>,

    #[serde(
        default,
        rename = "ASOBO_material_shadow_options",
        skip_serializing_if = "Option::is_none"
    )]
    pub shadow_options: Option<AsoboMaterialShadowOptions>,

    #[serde(
        default,
        rename = "ASOBO_material_UV_options",
        skip_serializing_if = "Option::is_none"
    )]
    pub uv_options: Option<AsoboMaterialUVOptions>,

    #[serde(
        default,
        rename = "ASOBO_material_SSS",
        skip_serializing_if = "Option::is_none"
    )]
    pub subsurface: Option<AsoboMaterialSubsurface>,

    #[serde(
        default,
        rename = "ASOBO_tags",
        skip_serializing_if = "Option::is_none"
    )]
    pub tags: Option<AsoboTags>,
}

fn bool_true() -> bool {
    true
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AsoboMaterialAlphaModeDither {
    #[serde(default = "bool_true")]
    enabled: bool,
}

impl Validate for AsoboMaterialAlphaModeDither {}

#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct AsoboMaterialAnisotropic {
    #[serde(
        default,
        rename = "wetnessAOTexture",
        skip_serializing_if = "Option::is_none"
    )]
    wetness_ao_texture: Option<texture::Info>,
}

#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub struct AsoboMaterialDecal {
    #[serde(default = "bool_true")]
    enabled: bool,
    #[serde(default, rename = "baseColorBlendFactor")]
    base_color_blend_factor: f32,
    #[serde(default, rename = "metallicBlendFactor")]
    metallic_blend_factor: f32,
    #[serde(default, rename = "roughnessBlendFactor")]
    roughness_blend_factor: f32,
    #[serde(default, rename = "normalBlendFactor")]
    normal_blend_factor: f32,
    #[serde(default, rename = "emissiveBlendFactor")]
    emissive_blend_factor: f32,
    #[serde(default, rename = "occlusionBlendFactor")]
    occlusion_blend_factor: f32,
}

impl Validate for AsoboMaterialDecal {}

#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct AsoboMaterialClearCoat {
    #[serde(
        default,
        rename = "dirtTexture",
        skip_serializing_if = "Option::is_none"
    )]
    dirt_texture: Option<texture::Info>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AsoboMaterialDetailMap {
    #[serde(default, rename = "UVScale")]
    uv_scale: f32,
    #[serde(default, rename = "UVOffset")]
    uv_offset: [f32; 2],
    #[serde(default, rename = "blendThreshold")]
    blend_threshold: f32,
    #[serde(
        default,
        rename = "detailColorTexture",
        skip_serializing_if = "Option::is_none"
    )]
    detail_color_texture: Option<texture::Info>,
    #[serde(
        default,
        rename = "detailNormalTexture",
        skip_serializing_if = "Option::is_none"
    )]
    detail_normal_texture: Option<crate::material::NormalTexture>,
    #[serde(
        default,
        rename = "detailMetalRoughAOTexture",
        skip_serializing_if = "Option::is_none"
    )]
    detail_metal_rough_ao_texture: Option<texture::Info>,
    #[serde(
        default,
        rename = "blendMaskTexture",
        skip_serializing_if = "Option::is_none"
    )]
    blend_mask_texture: Option<texture::Info>,
}

impl Validate for AsoboMaterialDetailMap {
    fn validate<P, R>(&self, root: &crate::Root, path: P, report: &mut R)
    where
        P: Fn() -> crate::Path,
        R: FnMut(&dyn Fn() -> crate::Path, crate::validation::Error),
    {
        self.detail_color_texture.validate(root, &path, report);
        self.detail_normal_texture.validate(root, &path, report);
        self.detail_metal_rough_ao_texture
            .validate(root, &path, report);
        self.blend_mask_texture.validate(root, &path, report);
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AsoboMaterialDrawOrder {
    #[serde(default, rename = "drawOrderOffset")]
    draw_order_offset: f32,
}

impl Validate for AsoboMaterialDrawOrder {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AsoboMaterialFakeTerrain {
    #[serde(default = "bool_true")]
    enabled: bool,
}

impl Validate for AsoboMaterialFakeTerrain {}

#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub struct AsoboMaterialFresnelFade {
    #[serde(default, rename = "fresnelFactor")]
    fresnel_factor: f32,
    #[serde(default, rename = "fresnelOpacityOffset")]
    fresnel_opacity_offset: f32,
}

impl Validate for AsoboMaterialFresnelFade {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AsoboMaterialGlass {
    #[serde(default, rename = "glassReflectionMaskFactor")]
    glass_reflection_mask_factor: f32,
    #[serde(default, rename = "glassDeformationFactor")]
    glass_deformation_factor: f32,
}

impl Validate for AsoboMaterialGlass {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AsoboMaterialInvisible {
    #[serde(default = "bool_true")]
    enabled: bool,
}

impl Validate for AsoboMaterialInvisible {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AsoboMaterialParallaxWindow {
    #[serde(default, rename = "parallaxScale")]
    parallax_scale: f32,
    #[serde(default, rename = "roomSizeXScale")]
    room_size_x_scale: f32,
    #[serde(default, rename = "roomSizeYScale")]
    room_size_y_scale: f32,
    #[serde(default, rename = "roomNumberXY")]
    room_number_xy: f32,
    #[serde(
        default,
        rename = "heightMapTexture",
        skip_serializing_if = "Option::is_none"
    )]
    height_map_texture: Option<texture::Info>,
    #[serde(
        default,
        rename = "behindWindowMapTexture",
        skip_serializing_if = "Option::is_none"
    )]
    behind_window_map_texture: Option<texture::Info>,
}

impl Validate for AsoboMaterialParallaxWindow {
    fn validate<P, R>(&self, root: &crate::Root, path: P, report: &mut R)
    where
        P: Fn() -> crate::Path,
        R: FnMut(&dyn Fn() -> crate::Path, crate::validation::Error),
    {
        self.height_map_texture.validate(root, &path, report);
        self.behind_window_map_texture.validate(root, &path, report);
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AsoboMaterialShadowOptions {
    #[serde(default = "bool_true", rename = "noCastShadow")]
    no_cast_shadow: bool,
}

impl Validate for AsoboMaterialShadowOptions {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AsoboMaterialSubsurface {
    #[serde(default, rename = "SSSColor")]
    sss_color: [f32; 4],
    #[serde(
        default,
        rename = "opacityTexture",
        skip_serializing_if = "Option::is_none"
    )]
    opacity_texture: Option<texture::Info>,
}

impl Validate for AsoboMaterialSubsurface {
    fn validate<P, R>(&self, root: &crate::Root, path: P, report: &mut R)
    where
        P: Fn() -> crate::Path,
        R: FnMut(&dyn Fn() -> crate::Path, crate::validation::Error),
    {
        self.opacity_texture.validate(root, path, report);
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AsoboMaterialUVOptions {
    #[serde(default = "bool_true", rename = "AOUseUV2")]
    ao_use_uv2: bool,
    #[serde(default = "bool_true", rename = "clampUVX")]
    clamp_uvx: bool,
    #[serde(default = "bool_true", rename = "clampUVY")]
    clamp_uvy: bool,
    #[serde(default = "bool_true", rename = "clampUVZ")]
    clamp_uvz: bool,
}

impl Validate for AsoboMaterialUVOptions {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AsoboTags {
    #[serde(default, rename = "Collision")]
    collision: bool,
}

impl Validate for AsoboTags {}

/// A set of parameter values that are used to define the metallic-roughness
/// material model from Physically-Based Rendering (PBR) methodology.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct PbrMetallicRoughness {}

/// A set of parameter values that are used to define the specular-glossiness
/// material model from Physically-Based Rendering (PBR) methodology.
///
/// This model supports more materials than metallic-roughness, at the cost of
/// increased memory use. When both are available, specular-glossiness should be
/// preferred.
#[cfg(feature = "KHR_materials_pbrSpecularGlossiness")]
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
#[serde(default, rename_all = "camelCase")]
pub struct PbrSpecularGlossiness {
    /// The material's diffuse factor.
    ///
    /// The RGBA components of the reflected diffuse color of the
    /// material. Metals have a diffuse value of `[0.0, 0.0, 0.0]`. The fourth
    /// component (A) is the alpha coverage of the material. The `alphaMode`
    /// property specifies how alpha is interpreted. The values are linear.
    pub diffuse_factor: PbrDiffuseFactor,

    /// The diffuse texture.
    ///
    /// This texture contains RGB(A) components of the reflected diffuse color
    /// of the material in sRGB color space. If the fourth component (A) is
    /// present, it represents the alpha coverage of the material. Otherwise, an
    /// alpha of 1.0 is assumed. The `alphaMode` property specifies how alpha is
    /// interpreted. The stored texels must not be premultiplied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diffuse_texture: Option<texture::Info>,

    /// The material's specular factor.
    pub specular_factor: PbrSpecularFactor,

    /// The glossiness or smoothness of the material.
    ///
    /// A value of 1.0 means the material has full glossiness or is perfectly
    /// smooth. A value of 0.0 means the material has no glossiness or is
    /// completely rough. This value is linear.
    pub glossiness_factor: StrengthFactor,

    /// The specular-glossiness texture.
    ///
    /// A RGBA texture, containing the specular color of the material (RGB
    /// components) and its glossiness (A component). The values are in sRGB
    /// space.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub specular_glossiness_texture: Option<texture::Info>,

    /// Optional application specific data.
    #[cfg_attr(feature = "extras", serde(skip_serializing_if = "Option::is_none"))]
    pub extras: Extras,
}

/// Defines the normal texture of a material.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct NormalTexture {}

/// Defines the occlusion texture of a material.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct OcclusionTexture {}

/// The diffuse factor of a material.
#[cfg(feature = "KHR_materials_pbrSpecularGlossiness")]
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct PbrDiffuseFactor(pub [f32; 4]);

#[cfg(feature = "KHR_materials_pbrSpecularGlossiness")]
impl Default for PbrDiffuseFactor {
    fn default() -> Self {
        PbrDiffuseFactor([1.0, 1.0, 1.0, 1.0])
    }
}

#[cfg(feature = "KHR_materials_pbrSpecularGlossiness")]
impl Validate for PbrDiffuseFactor {}

/// The specular factor of a material.
#[cfg(feature = "KHR_materials_pbrSpecularGlossiness")]
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct PbrSpecularFactor(pub [f32; 3]);

#[cfg(feature = "KHR_materials_pbrSpecularGlossiness")]
impl Default for PbrSpecularFactor {
    fn default() -> Self {
        PbrSpecularFactor([1.0, 1.0, 1.0])
    }
}

#[cfg(feature = "KHR_materials_pbrSpecularGlossiness")]
impl Validate for PbrSpecularFactor {}

/// Empty struct that should be present for primitives which should not be shaded with the PBR shading model.
#[cfg(feature = "KHR_materials_unlit")]
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct Unlit {}

/// A number in the inclusive range [0.0, 1.0] with a default value of 0.0.
#[cfg(feature = "KHR_materials_transmission")]
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct TransmissionFactor(pub f32);

#[cfg(feature = "KHR_materials_transmission")]
impl Default for TransmissionFactor {
    fn default() -> Self {
        TransmissionFactor(0.0)
    }
}

#[cfg(feature = "KHR_materials_transmission")]
impl Validate for TransmissionFactor {}

#[cfg(feature = "KHR_materials_transmission")]
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
#[serde(default, rename_all = "camelCase")]
pub struct Transmission {
    /// The base percentage of light that is transmitted through the surface.
    ///
    /// The amount of light that is transmitted by the surface rather than diffusely re-emitted.
    /// This is a percentage of all the light that penetrates a surface (i.e. isn’t specularly reflected)
    /// rather than a percentage of the total light that hits a surface.
    /// A value of 1.0 means that 100% of the light that penetrates the surface is transmitted through.
    pub transmission_factor: TransmissionFactor,

    /// The transmission texture.
    ///
    /// The R channel of this texture defines the amount of light that is transmitted by the surface
    /// rather than diffusely re-emitted. A value of 1.0 in the red channel means that 100% of the light
    /// that penetrates the surface (i.e. isn’t specularly reflected) is transmitted through.
    /// The value is linear and is multiplied by the transmissionFactor to determine the total transmission value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transmission_texture: Option<texture::Info>,

    /// Optional application specific data.
    #[cfg_attr(feature = "extras", serde(skip_serializing_if = "Option::is_none"))]
    pub extras: Extras,
}

/// A positive number with default value of 1.5
#[cfg(feature = "KHR_materials_ior")]
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct IndexOfRefraction(pub f32);

#[cfg(feature = "KHR_materials_ior")]
impl Default for IndexOfRefraction {
    fn default() -> Self {
        IndexOfRefraction(1.5)
    }
}

#[cfg(feature = "KHR_materials_ior")]
impl Validate for IndexOfRefraction {}

#[cfg(feature = "KHR_materials_ior")]
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
#[serde(default, rename_all = "camelCase")]
pub struct Ior {
    /// The index of refraction.
    ///
    /// Typical values for the index of refraction range from 1 to 2.
    /// In rare cases values greater than 2 are possible.
    /// For example, the ior of water is 1.33, and diamond is 2.42
    pub ior: IndexOfRefraction,

    /// Optional application specific data.
    #[cfg_attr(feature = "extras", serde(skip_serializing_if = "Option::is_none"))]
    pub extras: Extras,
}
