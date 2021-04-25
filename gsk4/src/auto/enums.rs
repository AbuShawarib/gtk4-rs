// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use glib::error::ErrorDomain;
use glib::translate::*;
use glib::value::FromValue;
use glib::value::ToValue;
use glib::Quark;
use glib::StaticType;
use glib::Type;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "GskBlendMode")]
pub enum BlendMode {
    #[doc(alias = "GSK_BLEND_MODE_DEFAULT")]
    Default,
    #[doc(alias = "GSK_BLEND_MODE_MULTIPLY")]
    Multiply,
    #[doc(alias = "GSK_BLEND_MODE_SCREEN")]
    Screen,
    #[doc(alias = "GSK_BLEND_MODE_OVERLAY")]
    Overlay,
    #[doc(alias = "GSK_BLEND_MODE_DARKEN")]
    Darken,
    #[doc(alias = "GSK_BLEND_MODE_LIGHTEN")]
    Lighten,
    #[doc(alias = "GSK_BLEND_MODE_COLOR_DODGE")]
    ColorDodge,
    #[doc(alias = "GSK_BLEND_MODE_COLOR_BURN")]
    ColorBurn,
    #[doc(alias = "GSK_BLEND_MODE_HARD_LIGHT")]
    HardLight,
    #[doc(alias = "GSK_BLEND_MODE_SOFT_LIGHT")]
    SoftLight,
    #[doc(alias = "GSK_BLEND_MODE_DIFFERENCE")]
    Difference,
    #[doc(alias = "GSK_BLEND_MODE_EXCLUSION")]
    Exclusion,
    #[doc(alias = "GSK_BLEND_MODE_COLOR")]
    Color,
    #[doc(alias = "GSK_BLEND_MODE_HUE")]
    Hue,
    #[doc(alias = "GSK_BLEND_MODE_SATURATION")]
    Saturation,
    #[doc(alias = "GSK_BLEND_MODE_LUMINOSITY")]
    Luminosity,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for BlendMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "BlendMode::{}",
            match *self {
                BlendMode::Default => "Default",
                BlendMode::Multiply => "Multiply",
                BlendMode::Screen => "Screen",
                BlendMode::Overlay => "Overlay",
                BlendMode::Darken => "Darken",
                BlendMode::Lighten => "Lighten",
                BlendMode::ColorDodge => "ColorDodge",
                BlendMode::ColorBurn => "ColorBurn",
                BlendMode::HardLight => "HardLight",
                BlendMode::SoftLight => "SoftLight",
                BlendMode::Difference => "Difference",
                BlendMode::Exclusion => "Exclusion",
                BlendMode::Color => "Color",
                BlendMode::Hue => "Hue",
                BlendMode::Saturation => "Saturation",
                BlendMode::Luminosity => "Luminosity",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for BlendMode {
    type GlibType = ffi::GskBlendMode;

    fn to_glib(&self) -> ffi::GskBlendMode {
        match *self {
            BlendMode::Default => ffi::GSK_BLEND_MODE_DEFAULT,
            BlendMode::Multiply => ffi::GSK_BLEND_MODE_MULTIPLY,
            BlendMode::Screen => ffi::GSK_BLEND_MODE_SCREEN,
            BlendMode::Overlay => ffi::GSK_BLEND_MODE_OVERLAY,
            BlendMode::Darken => ffi::GSK_BLEND_MODE_DARKEN,
            BlendMode::Lighten => ffi::GSK_BLEND_MODE_LIGHTEN,
            BlendMode::ColorDodge => ffi::GSK_BLEND_MODE_COLOR_DODGE,
            BlendMode::ColorBurn => ffi::GSK_BLEND_MODE_COLOR_BURN,
            BlendMode::HardLight => ffi::GSK_BLEND_MODE_HARD_LIGHT,
            BlendMode::SoftLight => ffi::GSK_BLEND_MODE_SOFT_LIGHT,
            BlendMode::Difference => ffi::GSK_BLEND_MODE_DIFFERENCE,
            BlendMode::Exclusion => ffi::GSK_BLEND_MODE_EXCLUSION,
            BlendMode::Color => ffi::GSK_BLEND_MODE_COLOR,
            BlendMode::Hue => ffi::GSK_BLEND_MODE_HUE,
            BlendMode::Saturation => ffi::GSK_BLEND_MODE_SATURATION,
            BlendMode::Luminosity => ffi::GSK_BLEND_MODE_LUMINOSITY,
            BlendMode::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GskBlendMode> for BlendMode {
    unsafe fn from_glib(value: ffi::GskBlendMode) -> Self {
        skip_assert_initialized!();
        match value {
            0 => BlendMode::Default,
            1 => BlendMode::Multiply,
            2 => BlendMode::Screen,
            3 => BlendMode::Overlay,
            4 => BlendMode::Darken,
            5 => BlendMode::Lighten,
            6 => BlendMode::ColorDodge,
            7 => BlendMode::ColorBurn,
            8 => BlendMode::HardLight,
            9 => BlendMode::SoftLight,
            10 => BlendMode::Difference,
            11 => BlendMode::Exclusion,
            12 => BlendMode::Color,
            13 => BlendMode::Hue,
            14 => BlendMode::Saturation,
            15 => BlendMode::Luminosity,
            value => BlendMode::__Unknown(value),
        }
    }
}

impl StaticType for BlendMode {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gsk_blend_mode_get_type()) }
    }
}

impl glib::value::ValueType for BlendMode {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for BlendMode {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for BlendMode {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<BlendMode>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.to_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "GskCorner")]
pub enum Corner {
    #[doc(alias = "GSK_CORNER_TOP_LEFT")]
    TopLeft,
    #[doc(alias = "GSK_CORNER_TOP_RIGHT")]
    TopRight,
    #[doc(alias = "GSK_CORNER_BOTTOM_RIGHT")]
    BottomRight,
    #[doc(alias = "GSK_CORNER_BOTTOM_LEFT")]
    BottomLeft,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for Corner {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Corner::{}",
            match *self {
                Corner::TopLeft => "TopLeft",
                Corner::TopRight => "TopRight",
                Corner::BottomRight => "BottomRight",
                Corner::BottomLeft => "BottomLeft",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for Corner {
    type GlibType = ffi::GskCorner;

    fn to_glib(&self) -> ffi::GskCorner {
        match *self {
            Corner::TopLeft => ffi::GSK_CORNER_TOP_LEFT,
            Corner::TopRight => ffi::GSK_CORNER_TOP_RIGHT,
            Corner::BottomRight => ffi::GSK_CORNER_BOTTOM_RIGHT,
            Corner::BottomLeft => ffi::GSK_CORNER_BOTTOM_LEFT,
            Corner::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GskCorner> for Corner {
    unsafe fn from_glib(value: ffi::GskCorner) -> Self {
        skip_assert_initialized!();
        match value {
            0 => Corner::TopLeft,
            1 => Corner::TopRight,
            2 => Corner::BottomRight,
            3 => Corner::BottomLeft,
            value => Corner::__Unknown(value),
        }
    }
}

impl StaticType for Corner {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gsk_corner_get_type()) }
    }
}

impl glib::value::ValueType for Corner {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for Corner {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for Corner {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Corner>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.to_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "GskGLUniformType")]
pub enum GLUniformType {
    #[doc(alias = "GSK_GL_UNIFORM_TYPE_NONE")]
    None,
    #[doc(alias = "GSK_GL_UNIFORM_TYPE_FLOAT")]
    Float,
    #[doc(alias = "GSK_GL_UNIFORM_TYPE_INT")]
    Int,
    #[doc(alias = "GSK_GL_UNIFORM_TYPE_UINT")]
    Uint,
    #[doc(alias = "GSK_GL_UNIFORM_TYPE_BOOL")]
    Bool,
    #[doc(alias = "GSK_GL_UNIFORM_TYPE_VEC2")]
    Vec2,
    #[doc(alias = "GSK_GL_UNIFORM_TYPE_VEC3")]
    Vec3,
    #[doc(alias = "GSK_GL_UNIFORM_TYPE_VEC4")]
    Vec4,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for GLUniformType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "GLUniformType::{}",
            match *self {
                GLUniformType::None => "None",
                GLUniformType::Float => "Float",
                GLUniformType::Int => "Int",
                GLUniformType::Uint => "Uint",
                GLUniformType::Bool => "Bool",
                GLUniformType::Vec2 => "Vec2",
                GLUniformType::Vec3 => "Vec3",
                GLUniformType::Vec4 => "Vec4",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for GLUniformType {
    type GlibType = ffi::GskGLUniformType;

    fn to_glib(&self) -> ffi::GskGLUniformType {
        match *self {
            GLUniformType::None => ffi::GSK_GL_UNIFORM_TYPE_NONE,
            GLUniformType::Float => ffi::GSK_GL_UNIFORM_TYPE_FLOAT,
            GLUniformType::Int => ffi::GSK_GL_UNIFORM_TYPE_INT,
            GLUniformType::Uint => ffi::GSK_GL_UNIFORM_TYPE_UINT,
            GLUniformType::Bool => ffi::GSK_GL_UNIFORM_TYPE_BOOL,
            GLUniformType::Vec2 => ffi::GSK_GL_UNIFORM_TYPE_VEC2,
            GLUniformType::Vec3 => ffi::GSK_GL_UNIFORM_TYPE_VEC3,
            GLUniformType::Vec4 => ffi::GSK_GL_UNIFORM_TYPE_VEC4,
            GLUniformType::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GskGLUniformType> for GLUniformType {
    unsafe fn from_glib(value: ffi::GskGLUniformType) -> Self {
        skip_assert_initialized!();
        match value {
            0 => GLUniformType::None,
            1 => GLUniformType::Float,
            2 => GLUniformType::Int,
            3 => GLUniformType::Uint,
            4 => GLUniformType::Bool,
            5 => GLUniformType::Vec2,
            6 => GLUniformType::Vec3,
            7 => GLUniformType::Vec4,
            value => GLUniformType::__Unknown(value),
        }
    }
}

impl StaticType for GLUniformType {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gsk_gl_uniform_type_get_type()) }
    }
}

impl glib::value::ValueType for GLUniformType {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for GLUniformType {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for GLUniformType {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<GLUniformType>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.to_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "GskRenderNodeType")]
pub enum RenderNodeType {
    #[doc(alias = "GSK_NOT_A_RENDER_NODE")]
    NotARenderNode,
    #[doc(alias = "GSK_CONTAINER_NODE")]
    ContainerNode,
    #[doc(alias = "GSK_CAIRO_NODE")]
    CairoNode,
    #[doc(alias = "GSK_COLOR_NODE")]
    ColorNode,
    #[doc(alias = "GSK_LINEAR_GRADIENT_NODE")]
    LinearGradientNode,
    #[doc(alias = "GSK_REPEATING_LINEAR_GRADIENT_NODE")]
    RepeatingLinearGradientNode,
    #[doc(alias = "GSK_RADIAL_GRADIENT_NODE")]
    RadialGradientNode,
    #[doc(alias = "GSK_REPEATING_RADIAL_GRADIENT_NODE")]
    RepeatingRadialGradientNode,
    #[doc(alias = "GSK_CONIC_GRADIENT_NODE")]
    ConicGradientNode,
    #[doc(alias = "GSK_BORDER_NODE")]
    BorderNode,
    #[doc(alias = "GSK_TEXTURE_NODE")]
    TextureNode,
    #[doc(alias = "GSK_INSET_SHADOW_NODE")]
    InsetShadowNode,
    #[doc(alias = "GSK_OUTSET_SHADOW_NODE")]
    OutsetShadowNode,
    #[doc(alias = "GSK_TRANSFORM_NODE")]
    TransformNode,
    #[doc(alias = "GSK_OPACITY_NODE")]
    OpacityNode,
    #[doc(alias = "GSK_COLOR_MATRIX_NODE")]
    ColorMatrixNode,
    #[doc(alias = "GSK_REPEAT_NODE")]
    RepeatNode,
    #[doc(alias = "GSK_CLIP_NODE")]
    ClipNode,
    #[doc(alias = "GSK_ROUNDED_CLIP_NODE")]
    RoundedClipNode,
    #[doc(alias = "GSK_SHADOW_NODE")]
    ShadowNode,
    #[doc(alias = "GSK_BLEND_NODE")]
    BlendNode,
    #[doc(alias = "GSK_CROSS_FADE_NODE")]
    CrossFadeNode,
    #[doc(alias = "GSK_TEXT_NODE")]
    TextNode,
    #[doc(alias = "GSK_BLUR_NODE")]
    BlurNode,
    #[doc(alias = "GSK_DEBUG_NODE")]
    DebugNode,
    #[doc(alias = "GSK_GL_SHADER_NODE")]
    GlShaderNode,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for RenderNodeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "RenderNodeType::{}",
            match *self {
                RenderNodeType::NotARenderNode => "NotARenderNode",
                RenderNodeType::ContainerNode => "ContainerNode",
                RenderNodeType::CairoNode => "CairoNode",
                RenderNodeType::ColorNode => "ColorNode",
                RenderNodeType::LinearGradientNode => "LinearGradientNode",
                RenderNodeType::RepeatingLinearGradientNode => "RepeatingLinearGradientNode",
                RenderNodeType::RadialGradientNode => "RadialGradientNode",
                RenderNodeType::RepeatingRadialGradientNode => "RepeatingRadialGradientNode",
                RenderNodeType::ConicGradientNode => "ConicGradientNode",
                RenderNodeType::BorderNode => "BorderNode",
                RenderNodeType::TextureNode => "TextureNode",
                RenderNodeType::InsetShadowNode => "InsetShadowNode",
                RenderNodeType::OutsetShadowNode => "OutsetShadowNode",
                RenderNodeType::TransformNode => "TransformNode",
                RenderNodeType::OpacityNode => "OpacityNode",
                RenderNodeType::ColorMatrixNode => "ColorMatrixNode",
                RenderNodeType::RepeatNode => "RepeatNode",
                RenderNodeType::ClipNode => "ClipNode",
                RenderNodeType::RoundedClipNode => "RoundedClipNode",
                RenderNodeType::ShadowNode => "ShadowNode",
                RenderNodeType::BlendNode => "BlendNode",
                RenderNodeType::CrossFadeNode => "CrossFadeNode",
                RenderNodeType::TextNode => "TextNode",
                RenderNodeType::BlurNode => "BlurNode",
                RenderNodeType::DebugNode => "DebugNode",
                RenderNodeType::GlShaderNode => "GlShaderNode",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for RenderNodeType {
    type GlibType = ffi::GskRenderNodeType;

    fn to_glib(&self) -> ffi::GskRenderNodeType {
        match *self {
            RenderNodeType::NotARenderNode => ffi::GSK_NOT_A_RENDER_NODE,
            RenderNodeType::ContainerNode => ffi::GSK_CONTAINER_NODE,
            RenderNodeType::CairoNode => ffi::GSK_CAIRO_NODE,
            RenderNodeType::ColorNode => ffi::GSK_COLOR_NODE,
            RenderNodeType::LinearGradientNode => ffi::GSK_LINEAR_GRADIENT_NODE,
            RenderNodeType::RepeatingLinearGradientNode => ffi::GSK_REPEATING_LINEAR_GRADIENT_NODE,
            RenderNodeType::RadialGradientNode => ffi::GSK_RADIAL_GRADIENT_NODE,
            RenderNodeType::RepeatingRadialGradientNode => ffi::GSK_REPEATING_RADIAL_GRADIENT_NODE,
            RenderNodeType::ConicGradientNode => ffi::GSK_CONIC_GRADIENT_NODE,
            RenderNodeType::BorderNode => ffi::GSK_BORDER_NODE,
            RenderNodeType::TextureNode => ffi::GSK_TEXTURE_NODE,
            RenderNodeType::InsetShadowNode => ffi::GSK_INSET_SHADOW_NODE,
            RenderNodeType::OutsetShadowNode => ffi::GSK_OUTSET_SHADOW_NODE,
            RenderNodeType::TransformNode => ffi::GSK_TRANSFORM_NODE,
            RenderNodeType::OpacityNode => ffi::GSK_OPACITY_NODE,
            RenderNodeType::ColorMatrixNode => ffi::GSK_COLOR_MATRIX_NODE,
            RenderNodeType::RepeatNode => ffi::GSK_REPEAT_NODE,
            RenderNodeType::ClipNode => ffi::GSK_CLIP_NODE,
            RenderNodeType::RoundedClipNode => ffi::GSK_ROUNDED_CLIP_NODE,
            RenderNodeType::ShadowNode => ffi::GSK_SHADOW_NODE,
            RenderNodeType::BlendNode => ffi::GSK_BLEND_NODE,
            RenderNodeType::CrossFadeNode => ffi::GSK_CROSS_FADE_NODE,
            RenderNodeType::TextNode => ffi::GSK_TEXT_NODE,
            RenderNodeType::BlurNode => ffi::GSK_BLUR_NODE,
            RenderNodeType::DebugNode => ffi::GSK_DEBUG_NODE,
            RenderNodeType::GlShaderNode => ffi::GSK_GL_SHADER_NODE,
            RenderNodeType::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GskRenderNodeType> for RenderNodeType {
    unsafe fn from_glib(value: ffi::GskRenderNodeType) -> Self {
        skip_assert_initialized!();
        match value {
            0 => RenderNodeType::NotARenderNode,
            1 => RenderNodeType::ContainerNode,
            2 => RenderNodeType::CairoNode,
            3 => RenderNodeType::ColorNode,
            4 => RenderNodeType::LinearGradientNode,
            5 => RenderNodeType::RepeatingLinearGradientNode,
            6 => RenderNodeType::RadialGradientNode,
            7 => RenderNodeType::RepeatingRadialGradientNode,
            8 => RenderNodeType::ConicGradientNode,
            9 => RenderNodeType::BorderNode,
            10 => RenderNodeType::TextureNode,
            11 => RenderNodeType::InsetShadowNode,
            12 => RenderNodeType::OutsetShadowNode,
            13 => RenderNodeType::TransformNode,
            14 => RenderNodeType::OpacityNode,
            15 => RenderNodeType::ColorMatrixNode,
            16 => RenderNodeType::RepeatNode,
            17 => RenderNodeType::ClipNode,
            18 => RenderNodeType::RoundedClipNode,
            19 => RenderNodeType::ShadowNode,
            20 => RenderNodeType::BlendNode,
            21 => RenderNodeType::CrossFadeNode,
            22 => RenderNodeType::TextNode,
            23 => RenderNodeType::BlurNode,
            24 => RenderNodeType::DebugNode,
            25 => RenderNodeType::GlShaderNode,
            value => RenderNodeType::__Unknown(value),
        }
    }
}

impl StaticType for RenderNodeType {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gsk_render_node_type_get_type()) }
    }
}

impl glib::value::ValueType for RenderNodeType {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for RenderNodeType {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for RenderNodeType {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<RenderNodeType>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.to_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "GskScalingFilter")]
pub enum ScalingFilter {
    #[doc(alias = "GSK_SCALING_FILTER_LINEAR")]
    Linear,
    #[doc(alias = "GSK_SCALING_FILTER_NEAREST")]
    Nearest,
    #[doc(alias = "GSK_SCALING_FILTER_TRILINEAR")]
    Trilinear,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for ScalingFilter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ScalingFilter::{}",
            match *self {
                ScalingFilter::Linear => "Linear",
                ScalingFilter::Nearest => "Nearest",
                ScalingFilter::Trilinear => "Trilinear",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for ScalingFilter {
    type GlibType = ffi::GskScalingFilter;

    fn to_glib(&self) -> ffi::GskScalingFilter {
        match *self {
            ScalingFilter::Linear => ffi::GSK_SCALING_FILTER_LINEAR,
            ScalingFilter::Nearest => ffi::GSK_SCALING_FILTER_NEAREST,
            ScalingFilter::Trilinear => ffi::GSK_SCALING_FILTER_TRILINEAR,
            ScalingFilter::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GskScalingFilter> for ScalingFilter {
    unsafe fn from_glib(value: ffi::GskScalingFilter) -> Self {
        skip_assert_initialized!();
        match value {
            0 => ScalingFilter::Linear,
            1 => ScalingFilter::Nearest,
            2 => ScalingFilter::Trilinear,
            value => ScalingFilter::__Unknown(value),
        }
    }
}

impl StaticType for ScalingFilter {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gsk_scaling_filter_get_type()) }
    }
}

impl glib::value::ValueType for ScalingFilter {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for ScalingFilter {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for ScalingFilter {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<ScalingFilter>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.to_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "GskSerializationError")]
pub enum SerializationError {
    #[doc(alias = "GSK_SERIALIZATION_UNSUPPORTED_FORMAT")]
    UnsupportedFormat,
    #[doc(alias = "GSK_SERIALIZATION_UNSUPPORTED_VERSION")]
    UnsupportedVersion,
    #[doc(alias = "GSK_SERIALIZATION_INVALID_DATA")]
    InvalidData,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for SerializationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "SerializationError::{}",
            match *self {
                SerializationError::UnsupportedFormat => "UnsupportedFormat",
                SerializationError::UnsupportedVersion => "UnsupportedVersion",
                SerializationError::InvalidData => "InvalidData",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for SerializationError {
    type GlibType = ffi::GskSerializationError;

    fn to_glib(&self) -> ffi::GskSerializationError {
        match *self {
            SerializationError::UnsupportedFormat => ffi::GSK_SERIALIZATION_UNSUPPORTED_FORMAT,
            SerializationError::UnsupportedVersion => ffi::GSK_SERIALIZATION_UNSUPPORTED_VERSION,
            SerializationError::InvalidData => ffi::GSK_SERIALIZATION_INVALID_DATA,
            SerializationError::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GskSerializationError> for SerializationError {
    unsafe fn from_glib(value: ffi::GskSerializationError) -> Self {
        skip_assert_initialized!();
        match value {
            0 => SerializationError::UnsupportedFormat,
            1 => SerializationError::UnsupportedVersion,
            2 => SerializationError::InvalidData,
            value => SerializationError::__Unknown(value),
        }
    }
}

impl ErrorDomain for SerializationError {
    fn domain() -> Quark {
        skip_assert_initialized!();

        unsafe { from_glib(ffi::gsk_serialization_error_quark()) }
    }

    fn code(self) -> i32 {
        self.to_glib()
    }

    fn from(code: i32) -> Option<Self> {
        skip_assert_initialized!();
        match code {
            0 => Some(SerializationError::UnsupportedFormat),
            1 => Some(SerializationError::UnsupportedVersion),
            2 => Some(SerializationError::InvalidData),
            value => Some(SerializationError::__Unknown(value)),
        }
    }
}

impl StaticType for SerializationError {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gsk_serialization_error_get_type()) }
    }
}

impl glib::value::ValueType for SerializationError {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for SerializationError {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for SerializationError {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<SerializationError>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.to_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "GskTransformCategory")]
pub enum TransformCategory {
    #[doc(alias = "GSK_TRANSFORM_CATEGORY_UNKNOWN")]
    Unknown,
    #[doc(alias = "GSK_TRANSFORM_CATEGORY_ANY")]
    Any,
    #[doc(alias = "GSK_TRANSFORM_CATEGORY_3D")]
    _3d,
    #[doc(alias = "GSK_TRANSFORM_CATEGORY_2D")]
    _2d,
    #[doc(alias = "GSK_TRANSFORM_CATEGORY_2D_AFFINE")]
    _2dAffine,
    #[doc(alias = "GSK_TRANSFORM_CATEGORY_2D_TRANSLATE")]
    _2dTranslate,
    #[doc(alias = "GSK_TRANSFORM_CATEGORY_IDENTITY")]
    Identity,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for TransformCategory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "TransformCategory::{}",
            match *self {
                TransformCategory::Unknown => "Unknown",
                TransformCategory::Any => "Any",
                TransformCategory::_3d => "_3d",
                TransformCategory::_2d => "_2d",
                TransformCategory::_2dAffine => "_2dAffine",
                TransformCategory::_2dTranslate => "_2dTranslate",
                TransformCategory::Identity => "Identity",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for TransformCategory {
    type GlibType = ffi::GskTransformCategory;

    fn to_glib(&self) -> ffi::GskTransformCategory {
        match *self {
            TransformCategory::Unknown => ffi::GSK_TRANSFORM_CATEGORY_UNKNOWN,
            TransformCategory::Any => ffi::GSK_TRANSFORM_CATEGORY_ANY,
            TransformCategory::_3d => ffi::GSK_TRANSFORM_CATEGORY_3D,
            TransformCategory::_2d => ffi::GSK_TRANSFORM_CATEGORY_2D,
            TransformCategory::_2dAffine => ffi::GSK_TRANSFORM_CATEGORY_2D_AFFINE,
            TransformCategory::_2dTranslate => ffi::GSK_TRANSFORM_CATEGORY_2D_TRANSLATE,
            TransformCategory::Identity => ffi::GSK_TRANSFORM_CATEGORY_IDENTITY,
            TransformCategory::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GskTransformCategory> for TransformCategory {
    unsafe fn from_glib(value: ffi::GskTransformCategory) -> Self {
        skip_assert_initialized!();
        match value {
            0 => TransformCategory::Unknown,
            1 => TransformCategory::Any,
            2 => TransformCategory::_3d,
            3 => TransformCategory::_2d,
            4 => TransformCategory::_2dAffine,
            5 => TransformCategory::_2dTranslate,
            6 => TransformCategory::Identity,
            value => TransformCategory::__Unknown(value),
        }
    }
}

impl StaticType for TransformCategory {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gsk_transform_category_get_type()) }
    }
}

impl glib::value::ValueType for TransformCategory {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for TransformCategory {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for TransformCategory {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<TransformCategory>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.to_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}
