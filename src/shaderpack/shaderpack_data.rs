//! Structs that represent shaderpack data

use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone)]
struct ShaderpackData {
    pipelines: Vec<PipelineCreationInfo>,
    passes: Vec<RenderPassCreationInfo>,
    materials: Vec<MaterialData>,
    resources: ShaderpackResourceData,
}

#[derive(Debug, Clone)]
struct PipelineCreationInfo {
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

#[derive(Debug, Clone)]
struct RenderPassCreationInfo {
    name: String,
    dependencies: Vec<String>,
    texture_inputs: Vec<String>,
    texture_outputs: Vec<TextureAttachmentInfo>,
    depth_texture: Option<TextureAttachmentInfo>,
    input_buffers: Vec<String>,
    output_buffers: Vec<String>,
}

#[derive(Debug, Clone)]
struct MaterialData {
    name: String,
    passes: Vec<MaterialPass>,
    geometry_filter: String,
}

#[derive(Debug, Clone)]
struct ShaderpackResourceData {
    textures: Vec<TextureCreateInfo>,
    samplers: Vec<SamplerCreateInfo>,
}

#[derive(Debug, Clone)]
struct VertexFieldData {
    semantic_name: String,
    field: VertexFieldEnum,
}

#[derive(Debug, Clone)]
struct StencilOpState {
    fail_op: StencilOpEnum,
    pass_op: StencilOpEnum,
    depth_fail_op: StencilOpEnum,
    compare_op: StencilOpEnum,
    compare_mask: u32,
    write_mask: u32,
}

#[derive(Debug, Clone)]
struct ShaderSource {
    filename: PathBuf,
    source: Vec<u32>,
}

#[derive(Debug, Clone, PartialEq)]
struct TextureAttachmentInfo {
    name: String,
    pixel_format: PixelFormatEnum,
    clear: bool,
}

#[derive(Debug, Clone)]
struct MaterialPass {
    name: String,
    material_name: String,
    pipeline: String,
    bindings: HashMap<String, String>,
}

#[derive(Debug, Clone)]
struct TextureCreateInfo {
    name: String,
    format: TextureFormat,
}

#[derive(Debug, Clone)]
struct SamplerCreateInfo {
    name: String,
    filter: TextureFilterEnum,
    wrap_mode: WrapModeEnum,
}

#[derive(Debug, Clone, PartialEq)]
struct TextureFormat {
    pixel_format: PixelFormatEnum,
    dimension_type: TextureDimensionTypeEnum,
    width: f32,
    height: f32,
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum RasterizerState {
    Blending,
    InvertCulling,
    DisableCulling,
    DisableDepthWrite,
    DisableDepthTest,
    EnableStencilTest,
    StencilWrite,
    DisableColorWrite,
    EnableAlphaToCoverage,
    DisableAlphaWrite,
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum MSAASupport {
    MSAA,
    Both,
    None,
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum PrimitiveTopology {
    Triangles,
    Lines,
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum BlendFactor {
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
enum CompareOp {
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
enum RenderQueue {
    Transparent,
    Opaque,
    Cutout,
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum VertexFieldEnum {
    Position,
    Color,
    UV0,
    UV1,
    Normal,
    Tangent,
    MidTexCoord,
    VirtualTextureId,
    McEntityId,
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum StencilOpEnum {
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
enum PixelFormatEnum {
    RGBA8,
    RGBA16F,
    RGBA32F,
    Depth,
    DepthStencil,
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum TextureFilterEnum {
    TexelAA,
    Bilinear,
    Point,
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum WrapModeEnum {
    Repeat,
    Clamp,
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum TextureDimensionTypeEnum {
    ScreenRelative,
    Absolute,
}
