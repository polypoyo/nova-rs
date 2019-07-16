//! Structs that represent shaderpack data

use std::{collections::HashMap, path::PathBuf};

#[derive(Debug, Clone)]
pub struct ShaderpackData {
    pipelines: Vec<PipelineCreationInfo>,
    /// All the renderpasses that this shaderpack needs, in submission order
    passes: Vec<RenderPassCreationInfo>,
    materials: Vec<MaterialData>,
    resources: ShaderpackResourceData,
}

#[derive(Debug, Clone)]
pub struct PipelineCreationInfo {
    /// The name of this pipeline
    name: String,
    /// The pipeline that this pipeline inherits from
    parent: Option<String>,
    /// The name of the pass that this pipeline belongs to
    pass: String,
    /// All of the symbols in the shader that are defined by this state
    defines: Vec<String>,
    /// Defines the rasterizer state that's active for this pipeline
    states: Vec<RasterizerState>,
    /// Sets up the vertex fields that Nova will bind to this pipeline
    vertex_fields: Vec<VertexFieldData>,
    /// The stencil buffer operations to perform on the front faces
    front_face: Option<StencilOpState>,
    /// The stencil buffer operations to perform on the back faces
    back_face: Option<StencilOpState>,
    /// The material to use if this one's shaders can't be found
    fallback: Option<String>,
    /// A bias to apply to the depth
    depth_bias: f32,
    /// The depth bias, scaled by slope I guess?
    slope_scaled_depth_bias: f32,
    /// The reference value to use for the stencil test
    stencil_ref: u32,
    /// The mask to use when reading from the stencil buffer
    stencil_read_mask: u32,
    /// The mask to use when writing to the stencil buffer
    stencil_write_mask: u32,
    /// How to handle MSAA for this state
    msaa_support: MSAASupport,
    /// Decides how the vertices are rendered
    primitive_mode: PrimitiveTopology,
    /// Where to get the blending factor for the soource
    src_blend_factor: BlendFactor,
    /// Where to get the blending factor for the destination
    dst_blend_factor: BlendFactor,
    /// How to get the source alpha in a blend
    alpha_src: BlendFactor,
    /// How to get the destination alpha in a blend
    alpha_dst: BlendFactor,
    /// The function to use for the depth test
    depth_func: CompareOp,
    /// The render queue that this pass belongs to
    /// This may or may not be removed depending on what is actually needed by Nova
    render_queue: RenderQueue,
    /// Vertex shader to use
    vertex_shader: ShaderSource,
    /// Geometry shader to use
    geometry_shader: Option<ShaderSource>,
    /// Tessellation Control shader to use
    tessellation_control_shader: Option<ShaderSource>,
    /// Tessellation Evaluation shader to use
    tessellation_evaluation_shader: Option<ShaderSource>,
    /// Fragment shader to use
    fragment_shader: Option<ShaderSource>,
}

impl PipelineCreationInfo {
    pub fn merge_with_parent(&self, other: &PipelineCreationInfo) -> Self {
        unimplimented!()
    }
}

/// A pass over the scene
///
/// A pass has a few things:
/// - What passes MUST be executed before this one
/// - What inputs this pass's shaders have
///      - What uniform buffers to use
///      - What vertex data to use
///      - Any textures that are needed
/// - What outputs this pass has
///      - Framebuffer attachments
///      - Write buffers
///
/// The inputs and outputs of a pass must be resources declared in the shaderpack's `resources.json` file (or the
/// default resources.json), or a resource that's internal to Nova. For example, Nova provides a UBO of uniforms that
/// change per frame, a UBO for per-model data like the model matrix, and the virtual texture atlases. The default
/// resources.json file sets up sixteen framebuffer color attachments for ping-pong buffers, a depth attachment,
/// some shadow maps, etc
#[derive(Debug, Clone)]
pub struct RenderPassCreationInfo {
    /// The name of this render pass
    name: String,
    /// The materials that MUST execute before this one
    dependencies: Vec<String>,
    /// The textures that this pass will read from
    texture_inputs: Vec<String>,
    /// The textures that this pass will write to
    texture_outputs: Vec<TextureAttachmentInfo>,
    /// The depth texture this pass will write to
    depth_texture: Option<TextureAttachmentInfo>,
    /// All the buffers that this renderpass reads from
    input_buffers: Vec<String>,
    /// All the buffers that this renderpass writes to
    output_buffers: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct MaterialData {
    name: String,
    passes: Vec<MaterialPass>,
    geometry_filter: String,
}

#[derive(Debug, Clone)]
pub struct ShaderpackResourceData {
    textures: Vec<TextureCreateInfo>,
    samplers: Vec<SamplerCreateInfo>,
}

#[derive(Debug, Clone)]
pub struct VertexFieldData {
    semantic_name: String,
    field: VertexField,
}

#[derive(Debug, Clone)]
pub struct StencilOpState {
    fail_op: StencilOp,
    pass_op: StencilOp,
    depth_fail_op: StencilOp,
    compare_op: StencilOp,
    compare_mask: u32,
    write_mask: u32,
}

#[derive(Debug, Clone)]
pub struct ShaderSource {
    filename: PathBuf,
    source: Vec<u32>,
}

///  A description of a texture that a render pass outputs to
#[derive(Debug, Clone, PartialEq)]
pub struct TextureAttachmentInfo {
    ///  The name of the texture
    name: String,
    /// Pixel format of the texture
    pixel_format: PixelFormat,
    ///  Whether to clear the texture
    ///
    /// If the texture is a depth buffer, it gets cleared to 1
    /// If the texture is a stencil buffer, it gets cleared to 0xFFFFFFFF
    /// If the texture is a color buffer, it gets cleared to (0, 0, 0, 0)
    clear: bool,
}

#[derive(Debug, Clone)]
pub struct MaterialPass {
    name: String,
    material_name: String,
    pipeline: String,
    bindings: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct TextureCreateInfo {
    ///  The name of the texture
    ///
    /// Nova implicitly defines a few textures for you to use:
    /// - ColorVirtualTexture
    ///      - Virtual texture atlas that holds color textures
    ///      - Textures which have the exact name as requested by Minecraft are in this atlas
    ///      - Things without a color texture get a pure white texture
    ///      - Always has a format of R8G8B8A8
    ///      - Can only be used as a pass's input
    /// - NormalVirtualTexture
    ///      - Virtual texture atlas that holds normal textures
    ///      - Textures which have `_n` after the name requested by Minecraft are in this atlas
    ///      - If no normal texture exists for a given object, a texture with RGBA of (0, 0, 1, 1) is used
    ///      - Always has a format of R8G8B8A8
    ///      - Can only be used as a pass's input
    /// - DataVirtualTexture
    ///      - Virtual texture atlas that holds data textures
    ///      - Textures which have a `_s` after the name requested by Minecraft are in this atlas
    ///      - If no data texture exists for a given object, a texture with an RGBA of (0, 0, 0, 0) is used
    ///      - Always has a format of R8G8B8A8
    ///      - Can only be used as a pass's input
    /// - Lightmap
    ///      - Lightmap, loaded from the current resourcepack
    ///      - Format of RGB8
    ///      - Can only be used as an input
    /// - Backbuffer
    ///      - The texture that gets presented to the screen
    ///      - Always has a format of RGB8
    ///      - Can only be used as a pass's output
    ///
    /// If you use one of the virtual textures, then all fields except the binding are ignored
    /// If you use `Backbuffer`, then all fields are ignored since the backbuffer is always bound to output location 0
    name: String,
    format: TextureFormat,
}

///  Defines a sampler to use for a texture
///
/// At the time of writing I'm not sure how this is corellated with a texture, but all well
#[derive(Debug, Clone)]
pub struct SamplerCreateInfo {
    name: String,
    ///  What kind of texture filter to use
    ///
    /// texel_aa does something that I don't want to figure out right now. Bilinear is your regular bilinear filter,
    /// and point is the point filter. Aniso isn't an option and I kinda hope it stays that way
    filter: TextureFilter,
    ///  How the texture should wrap at the edges
    wrap_mode: WrapMode,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TextureFormat {
    ///  The format of the texture
    pixel_format: PixelFormat,
    ///  How to interpret the dimensions of this texture
    dimension_type: TextureDimensionType,
    ///  The width, in pixels, of the texture
    width: f32,
    ///  The height, in pixels, of the texture
    height: f32,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum RasterizerState {
    /// Enable blending for this material state
    Blending,
    /// Render backfaces and cull frontfaces
    InvertCulling,
    /// Don't cull backfaces or frontfaces
    DisableCulling,
    /// Don't write to the depth buffer
    DisableDepthWrite,
    /// Don't perform a depth test
    DisableDepthTest,
    /// Perform the stencil test
    EnableStencilTest,
    /// Write to the stencil buffer
    StencilWrite,
    /// Don't write to the color buffer
    DisableColorWrite,
    /// Enable alpha to coverage
    EnableAlphaToCoverage,
    /// Don't write alpha
    DisableAlphaWrite,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum MSAASupport {
    MSAA,
    Both,
    None,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PrimitiveTopology {
    Triangles,
    Lines,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum BlendFactor {
    One,
    Zero,
    SrcColor,
    DstColor,
    OneMinusSrcColor,
    OneMinusDstColor,
    SrcAlpha,
    DstAlpha,
    OneMinusSrcAlpha,
    OneMinusDstAlpha,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum CompareOp {
    Never,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Equal,
    NotEqual,
    Always,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum RenderQueue {
    Transparent,
    Opaque,
    Cutout,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum VertexField {
    ///  The vertex position
    ///
    /// 12 bytes
    Position,
    ///  The vertex color
    ///
    /// 4 bytes
    Color,
    ///  The UV coordinate of this object
    ///
    /// Except not really, because Nova's virtual textures means that the UVs for a block or entity or whatever
    /// could change on the fly, so this is kinda more of a preprocessor define that replaces the UV with a lookup
    /// in the UV table
    ///
    /// 8 bytes (might try 4)
    UV0,
    ///  The UV coordinate in the lightmap texture
    ///
    /// This is a real UV and it doesn't change for no good reason
    ///
    /// 2 bytes
    UV1,
    ///  Vertex normal
    ///
    /// 12 bytes
    Normal,
    ///  Vertex tangents
    ///
    /// 12 bytes
    Tangent,
    ///  The texture coordinate of the middle of the quad
    ///
    /// 8 bytes
    MidTexCoord,
    ///  A uint32_t that's a unique identifier for the texture that this vertex uses
    ///
    /// This is generated at runtime by Nova, so it may change a lot depending on what resourcepacks are loaded and
    /// if they use CTM or random detail textures or whatever
    ///
    /// 4 bytes
    VirtualTextureId,
    ///  Some information about the current block/entity/whatever
    ///
    /// 12 bytes
    McEntityId,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum StencilOp {
    Keep,
    Zero,
    Replace,
    Incr,
    IncrWrap,
    Decr,
    DecrWrap,
    Invert,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PixelFormat {
    RGBA8,
    RGBA16F,
    RGBA32F,
    Depth,
    DepthStencil,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TextureFilter {
    TexelAA,
    Bilinear,
    Point,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum WrapMode {
    Repeat,
    Clamp,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TextureDimensionType {
    ScreenRelative,
    Absolute,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TextureLocation {
    ///  The texture is written to by a shader
    Dynamic,
    ///  The texture is loaded from the textures/ folder in the current shaderpack
    InUserPackage,
    ///  The texture is provided by Nova or by Minecraft
    InAppPackage,
}
