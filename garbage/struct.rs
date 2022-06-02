#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Matrix {
    pub m0: f32,
    pub m4: f32,
    pub m8: f32,
    pub m12: f32,
    pub m1: f32,
    pub m5: f32,
    pub m9: f32,
    pub m13: f32,
    pub m2: f32,
    pub m6: f32,
    pub m10: f32,
    pub m14: f32,
    pub m3: f32,
    pub m7: f32,
    pub m11: f32,
    pub m15: f32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub r: ::std::os::raw::c_uchar,
    pub g: ::std::os::raw::c_uchar,
    pub b: ::std::os::raw::c_uchar,
    pub a: ::std::os::raw::c_uchar,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Image {
    pub data: *mut ::std::os::raw::c_void,
    pub width: ::std::os::raw::c_int,
    pub height: ::std::os::raw::c_int,
    pub mipmaps: ::std::os::raw::c_int,
    pub format: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Texture {
    pub id: ::std::os::raw::c_uint,
    pub width: ::std::os::raw::c_int,
    pub height: ::std::os::raw::c_int,
    pub mipmaps: ::std::os::raw::c_int,
    pub format: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RenderTexture {
    pub id: ::std::os::raw::c_uint,
    pub texture: Texture,
    pub depth: Texture,
}
pub type RenderTexture2D = RenderTexture;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NPatchInfo {
    pub source: Rectangle,
    pub left: ::std::os::raw::c_int,
    pub top: ::std::os::raw::c_int,
    pub right: ::std::os::raw::c_int,
    pub bottom: ::std::os::raw::c_int,
    pub layout: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CharInfo {
    pub value: ::std::os::raw::c_int,
    pub offsetX: ::std::os::raw::c_int,
    pub offsetY: ::std::os::raw::c_int,
    pub advanceX: ::std::os::raw::c_int,
    pub image: Image,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Font {
    pub baseSize: ::std::os::raw::c_int,
    pub charsCount: ::std::os::raw::c_int,
    pub charsPadding: ::std::os::raw::c_int,
    pub texture: Texture2D,
    pub recs: *mut Rectangle,
    pub chars: *mut CharInfo,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Camera3D {
    pub position: Vector3,
    pub target: Vector3,
    pub up: Vector3,
    pub fovy: f32,
    pub projection: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Camera2D {
    pub offset: Vector2,
    pub target: Vector2,
    pub rotation: f32,
    pub zoom: f32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Mesh {
    pub vertexCount: ::std::os::raw::c_int,
    pub triangleCount: ::std::os::raw::c_int,
    pub vertices: *mut f32,
    pub texcoords: *mut f32,
    pub texcoords2: *mut f32,
    pub normals: *mut f32,
    pub tangents: *mut f32,
    pub colors: *mut ::std::os::raw::c_uchar,
    pub indices: *mut ::std::os::raw::c_ushort,
    pub animVertices: *mut f32,
    pub animNormals: *mut f32,
    pub boneIds: *mut ::std::os::raw::c_int,
    pub boneWeights: *mut f32,
    pub vaoId: ::std::os::raw::c_uint,
    pub vboId: *mut ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Shader {
    pub id: ::std::os::raw::c_uint,
    pub locs: *mut ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MaterialMap {
    pub texture: Texture2D,
    pub color: Color,
    pub value: f32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Material {
    pub shader: Shader,
    pub maps: *mut MaterialMap,
    pub params: [f32; 4usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Transform {
    pub translation: Vector3,
    pub rotation: Quaternion,
    pub scale: Vector3,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BoneInfo {
    pub name: [::std::os::raw::c_char; 32usize],
    pub parent: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Model {
    pub transform: Matrix,
    pub meshCount: ::std::os::raw::c_int,
    pub materialCount: ::std::os::raw::c_int,
    pub meshes: *mut Mesh,
    pub materials: *mut Material,
    pub meshMaterial: *mut ::std::os::raw::c_int,
    pub boneCount: ::std::os::raw::c_int,
    pub bones: *mut BoneInfo,
    pub bindPose: *mut Transform,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ModelAnimation {
    pub boneCount: ::std::os::raw::c_int,
    pub frameCount: ::std::os::raw::c_int,
    pub bones: *mut BoneInfo,
    pub framePoses: *mut *mut Transform,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub position: Vector3,
    pub direction: Vector3,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RayHitInfo {
    pub hit: bool,
    pub distance: f32,
    pub position: Vector3,
    pub normal: Vector3,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BoundingBox {
    pub min: Vector3,
    pub max: Vector3,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Wave {
    pub sampleCount: ::std::os::raw::c_uint,
    pub sampleRate: ::std::os::raw::c_uint,
    pub sampleSize: ::std::os::raw::c_uint,
    pub channels: ::std::os::raw::c_uint,
    pub data: *mut ::std::os::raw::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rAudioBuffer {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AudioStream {
    pub buffer: *mut rAudioBuffer,
    pub sampleRate: ::std::os::raw::c_uint,
    pub sampleSize: ::std::os::raw::c_uint,
    pub channels: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Sound {
    pub stream: AudioStream,
    pub sampleCount: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Music {
    pub stream: AudioStream,
    pub sampleCount: ::std::os::raw::c_uint,
    pub looping: bool,
    pub ctxType: ::std::os::raw::c_int,
    pub ctxData: *mut ::std::os::raw::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VrDeviceInfo {
    pub hResolution: ::std::os::raw::c_int,
    pub vResolution: ::std::os::raw::c_int,
    pub hScreenSize: f32,
    pub vScreenSize: f32,
    pub vScreenCenter: f32,
    pub eyeToScreenDistance: f32,
    pub lensSeparationDistance: f32,
    pub interpupillaryDistance: f32,
    pub lensDistortionValues: [f32; 4usize],
    pub chromaAbCorrection: [f32; 4usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VrStereoConfig {
    pub projection: [Matrix; 2usize],
    pub viewOffset: [Matrix; 2usize],
    pub leftLensCenter: [f32; 2usize],
    pub rightLensCenter: [f32; 2usize],
    pub leftScreenCenter: [f32; 2usize],
    pub rightScreenCenter: [f32; 2usize],
    pub scale: [f32; 2usize],
    pub scaleIn: [f32; 2usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct float3 {
    pub v: [f32; 3usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct float16 {
    pub v: [f32; 16usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __crt_locale_data_public {
    pub _locale_pctype: *const ::std::os::raw::c_ushort,
    pub _locale_mb_cur_max: ::std::os::raw::c_int,
    pub _locale_lc_codepage: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __crt_locale_pointers {
    pub locinfo: *mut __crt_locale_data,
    pub mbcinfo: *mut __crt_multibyte_data,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _Mbstatet {
    pub _Wchar: ::std::os::raw::c_ulong,
    pub _Byte: ::std::os::raw::c_ushort,
    pub _State: ::std::os::raw::c_ushort,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _exception {
    pub type_: ::std::os::raw::c_int,
    pub name: *mut ::std::os::raw::c_char,
    pub arg1: f64,
    pub arg2: f64,
    pub retval: f64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _complex {
    pub x: f64,
    pub y: f64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VertexBuffer {
    pub elementsCount: ::std::os::raw::c_int,
    pub vCounter: ::std::os::raw::c_int,
    pub tcCounter: ::std::os::raw::c_int,
    pub cCounter: ::std::os::raw::c_int,
    pub vertices: *mut f32,
    pub texcoords: *mut f32,
    pub colors: *mut ::std::os::raw::c_uchar,
    pub indices: *mut ::std::os::raw::c_uint,
    pub vaoId: ::std::os::raw::c_uint,
    pub vboId: [::std::os::raw::c_uint; 4usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DrawCall {
    pub mode: ::std::os::raw::c_int,
    pub vertexCount: ::std::os::raw::c_int,
    pub vertexAlignment: ::std::os::raw::c_int,
    pub textureId: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RenderBatch {
    pub buffersCount: ::std::os::raw::c_int,
    pub currentBuffer: ::std::os::raw::c_int,
    pub vertexBuffer: *mut VertexBuffer,
    pub draws: *mut DrawCall,
    pub drawsCounter: ::std::os::raw::c_int,
    pub currentDepth: f32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _div_t {
    pub quot: ::std::os::raw::c_int,
    pub rem: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _ldiv_t {
    pub quot: ::std::os::raw::c_long,
    pub rem: ::std::os::raw::c_long,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _lldiv_t {
    pub quot: ::std::os::raw::c_longlong,
    pub rem: ::std::os::raw::c_longlong,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _LDOUBLE {
    pub ld: [::std::os::raw::c_uchar; 10usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _CRT_DOUBLE {
    pub x: f64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _CRT_FLOAT {
    pub f: f32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _LONGDOUBLE {
    pub x: f64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _LDBL12 {
    pub ld12: [::std::os::raw::c_uchar; 12usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gladGLversionStruct {
    pub major: ::std::os::raw::c_int,
    pub minor: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _Lldiv_t {
    pub quot: intmax_t,
    pub rem: intmax_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __GLsync {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _cl_context {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _cl_event {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GuiStyleProp {
    pub controlId: ::std::os::raw::c_ushort,
    pub propertyId: ::std::os::raw::c_ushort,
    pub propertyValue: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __crt_locale_data {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __crt_multibyte_data {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rlglData__bindgen_ty_2 {
    pub vao: bool,
    pub instancing: bool,
    pub texNPOT: bool,
    pub texDepth: bool,
    pub texFloat32: bool,
    pub texCompDXT: bool,
    pub texCompETC1: bool,
    pub texCompETC2: bool,
    pub texCompPVRT: bool,
    pub texCompASTC: bool,
    pub texMirrorClamp: bool,
    pub texAnisoFilter: bool,
    pub maxAnisotropyLevel: f32,
    pub maxDepthBits: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rlglData__bindgen_ty_1 {
    pub currentMatrixMode: ::std::os::raw::c_int,
    pub currentMatrix: *mut Matrix,
    pub modelview: Matrix,
    pub projection: Matrix,
    pub transform: Matrix,
    pub transformRequired: bool,
    pub stack: [Matrix; 32usize],
    pub stackCounter: ::std::os::raw::c_int,
    pub defaultTextureId: ::std::os::raw::c_uint,
    pub activeTextureId: [::std::os::raw::c_uint; 4usize],
    pub defaultVShaderId: ::std::os::raw::c_uint,
    pub defaultFShaderId: ::std::os::raw::c_uint,
    pub defaultShader: Shader,
    pub currentShader: Shader,
    pub stereoRender: bool,
    pub projectionStereo: [Matrix; 2usize],
    pub viewOffsetStereo: [Matrix; 2usize],
    pub currentBlendMode: ::std::os::raw::c_int,
    pub glBlendSrcFactor: ::std::os::raw::c_int,
    pub glBlendDstFactor: ::std::os::raw::c_int,
    pub glad_glBlendEquation: ::std::os::raw::c_int,
    pub framebufferWidth: ::std::os::raw::c_int,
    pub framebufferHeight: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rlglData {
    pub currentBatch: *mut RenderBatch,
    pub defaultBatch: RenderBatch,
    pub State: rlglData__bindgen_ty_1,
    pub ExtSupported: rlglData__bindgen_ty_2,
}
