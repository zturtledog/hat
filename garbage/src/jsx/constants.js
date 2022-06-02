export let misc = {
    PI: 3.14159265358979323846,
    DEG2RAD: (3.14159265358979323846/180.0),
    RAD2DEG: (180.0/3.14159265358979323846)
}

export let rayconf = {
    SHOW_LOGO: 1,       // Set to show raylib logo at startup
    FULLSCREEN_MODE: 2,       // Set to run program in fullscreen
    WINDOW_RESIZABLE: 4,       // Set to allow resizable window
    WINDOW_UNDECORATED: 8,       // Set to disable window decoration (frame and buttons)
    WINDOW_TRANSPARENT: 16,       // Set to allow transparent window
    MSAA_4X_HINT: 32,       // Set to try enabling MSAA 4X
    VSYNC_HINT: 64       // Set to try enabling V//Sync on GPU
}

export let keyfunc = {
    KEY_SPACE: 32,
    ESCAPE: 256,
    ENTER: 257,
    TAB: 258,
    BACKSPACE: 259,
    INSERT: 260,
    DELETE: 261,
    RIGHT: 262,
    LEFT: 263,
    DOWN: 264,
    UP: 265,
    PAGE_UP: 266,
    PAGE_DOWN: 267,
    HOME: 268,
    END: 269,
    CAPS_LOCK: 280,
    SCROLL_LOCK: 281,
    NUM_LOCK: 282,
    PRINT_SCREEN: 283,
    PAUSE: 284,
    F1: 290,
    F2: 291,
    F3: 292,
    F4: 293,
    F5: 294,
    F6: 295,
    F7: 296,
    F8: 297,
    F9: 298,
    F10: 299,
    F11: 300,
    F12: 301,
    LEFT_SHIFT: 340,
    LEFT_CONTROL: 341,
    LEFT_ALT: 342,
    RIGHT_SHIFT: 344,
    RIGHT_CONTROL: 345,
    RIGHT_ALT: 346,
    GRAVE: 96,
    SLASH: 47,
    BACKSLASH: 92
}

export let keys = {
    KEY_ZERO: 48,
    ONE: 49,
    TWO: 50,
    THREE: 51,
    FOUR: 52,
    FIVE: 53,
    SIX: 54,
    SEVEN: 55,
    EIGHT: 56,
    NINE: 57,
    A: 65,
    B: 66,
    C: 67,
    D: 68,
    E: 69,
    F: 70,
    G: 71,
    H: 72,
    I: 73,
    J: 74,
    K: 75,
    L: 76,
    M: 77,
    N: 78,
    O: 79,
    P: 80,
    Q: 81,
    R: 82,
    S: 83,
    T: 84,
    U: 85,
    V: 86,
    W: 87,
    X: 88,
    Y: 89,
    Z: 90
}

export let androidbuttons = {
    BACK: 4,
    MENU: 82,
    VOLUME_UP: 24,
    VOLUME_DOWN: 25
}

export let mouse = {
    LEFT: 0,
    RIGHT: 1,
    MIDDLE: 2
}

export let ps3 = {
    TRIANGLE: 0,
    CIRCLE: 1,
    CROSS: 2,
    SQUARE: 3,
    L1: 6,
    R1: 7,
    L2: 4,
    R2: 5,
    START: 8,
    SELECT: 9,
    UP: 24,
    RIGHT: 25,
    DOWN: 26,
    LEFT: 27,
    PS: 12,
    LEFT_X: 0,
    LEFT_Y: 1,
    RIGHT_X: 2,
    RIGHT_Y: 5,
    L2: 3 ,
    R2: 4
}

export let xbox = {
    A: 0,
    B: 1,
    X: 2,
    Y: 3,
    LB: 4,
    RB: 5,
    SELECT: 6,
    START: 7,
    UP: 10,
    RIGHT: 11,
    DOWN: 12,
    LEFT: 13,
    HOME: 8
}

export let snesclasic = {
    DPAD_UP: 19,
    DPAD_DOWN: 20,
    DPAD_LEFT: 21,
    DPAD_RIGHT: 22,
    DPAD_CENTER: 23,
    BUTTON_A: 96,
    BUTTON_B: 97,
    BUTTON_C: 98,
    BUTTON_X: 99,
    BUTTON_Y: 100,
    BUTTON_Z: 101,
    BUTTON_L1: 102,
    BUTTON_R1: 103,
    BUTTON_L2: 104,
    BUTTON_R2: 105
}

export let snm = {
    MAX_SHADER_LOCATIONS: 32,      // Maximum number of predefined locations stored in shader struct
    MAX_MATERIAL_MAPS: 12          // Maximum number of texture maps stored in shader struct
}

export let tracelog = {
    INFO: 0,
    WARNING: 1,
    ERROR: 2,
    DEBUG: 3,
    OTHER: 4
}

export let shaderptyp = {    // Shader location point type
    VERTEX_POSITION:0,
    VERTEX_TEXCOORD01: 5,
    VERTEX_TEXCOORD02: 6,
    VERTEX_NORMAL: 7,
    VERTEX_TANGENT: 8,
    VERTEX_COLOR: 9,
    MATRIX_MVP: 10,
    MATRIX_MODEL: 11,
    MATRIX_VIEW: 12,
    MATRIX_PROJECTION: 13,
    VECTOR_VIEW: 14,
    COLOR_DIFFUSE: 15,
    COLOR_SPECULAR: 16,
    COLOR_AMBIENT: 17,
    MAP_ALBEDO: 18, // MAP_DIFFUSE
    MAP_METALNESS: 19, // MAP_SPECULAR
    MAP_NORMAL: 20,
    MAP_ROUGHNESS: 21,
    MAP_OCCUSION: 22,
    MAP_EMISSION: 23,
    MAP_HEIGHT: 24,
    MAP_CUBEMAP: 25,
    MAP_IRRADIANCE: 26,
    MAP_PREFILTER: 27,
    MAP_BRDF: 28,
    MAP_DIFFUSE: shaderptyp.MAP_ALBEDO,
    MAP_SPECULAR: shaderptyp.MAP_METALNESS
}

export let textype = {
    uncompressed: {
        GRAYSCALE: 1, // 8 bit per pixel (no alpha)
        GRAY_ALPHA: 2, // 16 bpp (2 channels)
        R5G6B5: 3, // 16 bpp
        R8G8B8: 4, // 24 bpp
        R5G5B5A1: 5, // 16 bpp (1 bit alpha)
        R4G4B4A4: 6, // 16 bpp (4 bit alpha)
        R8G8B8A8: 7, // 32 bpp
        R32G32B32: 8, // 32 bit per channel (float) - HDR
    },
    compressed: {
        DXT1_RGB: 9, // 4 bpp (no alpha)
        DXT1_RGBA: 10, // 4 bpp (1 bit alpha)
        DXT3_RGBA: 11, // 8 bpp
        DXT5_RGBA: 12, // 8 bpp
        ETC1_RGB: 13, // 4 bpp
        ETC2_RGB: 14, // 4 bpp
        ETC2_EAC_RGBA: 15, // 8 bpp
        PVRT_RGB: 16, // 4 bpp
        PVRT_RGBA: 17, // 4 bpp
        ASTC_4x4_RGBA: 18, // 8 bpp
        ASTC_8x8_RGBA: 19 // 2 bpp
    }
}

export let texfilter = {
    POINT: 0, // No filter = just pixel aproximation
    BILINEAR: 1, // Linear filtering
    TRILINEAR: 2, // Trilinear filtering (linear with mipmaps)
    ANISOTROPIC_4X: 3, // Anisotropic filtering 4x
    ANISOTROPIC_8X: 4, // Anisotropic filtering 8x
    ANISOTROPIC_16X: 5 // Anisotropic filtering 16x
}

export let texwrap = {
    REPEAT: 0,
    CLAMP: 1,
    MIRROR: 2
}

export let RRESData = {
    RAW: 0,
    IMAGE: 1 ,
    WAVE: 2,
    VERTEX: 3 ,
    TEXT: 4,
    FONT_IMAGE: 5,
    FONT_CHARDATA: 6,   // CharInfo data array
    DIRECTORY: 7
}

export let hmdd = {
    DEFAULT_DEVICE: 0,
    OCULUS_RIFT_DK2: 1,
    OCULUS_RIFT_CV1: 2,
    OCULUS_GO: 3,
    VALVE_HTC_VIVE: 4,
    SONY_PSVR: 5
}

export let cameramode = {
    CUSTOM: 0,
    FREE: 1,
    ORBITAL: 2,
    FIRST_PERSON: 3,
    THIRD_PERSON: 4
}

export let gestures = {
    NONE: 0,
    TAP: 1,
    DOUBLETAP: 2,
    HOLD: 4,
    DRAG: 8,
    SWIPE_RIGHT: 16,
    SWIPE_LEFT: 32,
    SWIPE_UP: 64,
    SWIPE_DOWN: 128,
    PINCH_IN: 256,
    PINCH_OUT: 512
}

export let colorblending = {
    ALPHA: 0,
    ADDITIVE: 1,
    MULTIPLIED: 2
}

export let materialmap = {
    ALBEDO: 0,
    METALNESS: 1,
    NORMAL: 2,
    ROUGHNESS: 3,
    OCCLUSION: 4,
    EMISSION: 5,
    HEIGHT: 6,
    CUBEMAP: 7,        // NOTE: Uses GL_TEXTURE_CUBE_MAP
    IRRADIANCE: 8,     // NOTE: Uses GL_TEXTURE_CUBE_MAP
    PREFILTER: 9,      // NOTE: Uses GL_TEXTURE_CUBE_MAP
    BRDF: 10,

    DIFFUSE: ALBEDO,
    SPECULAR: METALNESS
}