pub type size_t = ::std::os::raw::c_ulonglong;
pub type __vcrt_bool = bool;
pub type wchar_t = ::std::os::raw::c_ushort;
pub type __crt_bool = bool;
pub type errno_t = ::std::os::raw::c_int;
pub type wint_t = ::std::os::raw::c_ushort;
pub type wctype_t = ::std::os::raw::c_ushort;
pub type __time32_t = ::std::os::raw::c_long;
pub type __time64_t = ::std::os::raw::c_longlong;
pub type _locale_t = *mut __crt_locale_pointers;
pub type mbstate_t = _Mbstatet;
pub type time_t = __time64_t;
pub type rsize_t = size_t;
pub type float_t = f32;
pub type double_t = f64;
pub type Camera = Camera3D;
pub type Texture2D = Texture;
pub type TextureCubemap = Texture;
pub type Quaternion = Vector4;
pub type __builtin_va_list = *mut ::std::os::raw::c_char;
pub type va_list = __builtin_va_list;
pub type __gnuc_va_list = __builtin_va_list;
pub type div_t = _div_t;
pub type ldiv_t = _ldiv_t;
pub type lldiv_t = _lldiv_t;
pub type int_least8_t = ::std::os::raw::c_schar;
pub type int_least16_t = ::std::os::raw::c_short;
pub type int_least32_t = ::std::os::raw::c_int;
pub type int_least64_t = ::std::os::raw::c_longlong;
pub type uint_least8_t = ::std::os::raw::c_uchar;
pub type uint_least16_t = ::std::os::raw::c_ushort;
pub type uint_least32_t = ::std::os::raw::c_uint;
pub type uint_least64_t = ::std::os::raw::c_ulonglong;
pub type int_fast8_t = ::std::os::raw::c_schar;
pub type int_fast16_t = ::std::os::raw::c_int;
pub type int_fast32_t = ::std::os::raw::c_int;
pub type int_fast64_t = ::std::os::raw::c_longlong;
pub type uint_fast8_t = ::std::os::raw::c_uchar;
pub type uint_fast16_t = ::std::os::raw::c_uint;
pub type uint_fast32_t = ::std::os::raw::c_uint;
pub type uint_fast64_t = ::std::os::raw::c_ulonglong;
pub type intmax_t = ::std::os::raw::c_longlong;
pub type uintmax_t = ::std::os::raw::c_ulonglong;
pub type imaxdiv_t = _Lldiv_t;
pub type GLenum = ::std::os::raw::c_uint;
pub type GLboolean = ::std::os::raw::c_uchar;
pub type GLbitfield = ::std::os::raw::c_uint;
pub type GLvoid = ::std::os::raw::c_void;
pub type GLbyte = ::std::os::raw::c_schar;
pub type GLshort = ::std::os::raw::c_short;
pub type GLint = ::std::os::raw::c_int;
pub type GLclampx = ::std::os::raw::c_int;
pub type GLubyte = ::std::os::raw::c_uchar;
pub type GLushort = ::std::os::raw::c_ushort;
pub type GLuint = ::std::os::raw::c_uint;
pub type GLsizei = ::std::os::raw::c_int;
pub type GLfloat = f32;
pub type GLclampf = f32;
pub type GLdouble = f64;
pub type GLclampd = f64;
pub type GLeglImageOES = *mut ::std::os::raw::c_void;
pub type GLchar = ::std::os::raw::c_char;
pub type GLcharARB = ::std::os::raw::c_char;
pub type GLhandleARB = ::std::os::raw::c_uint;
pub type GLhalfARB = ::std::os::raw::c_ushort;
pub type GLhalf = ::std::os::raw::c_ushort;
pub type GLfixed = GLint;
pub type GLintptr = isize;
pub type GLsizeiptr = isize;
pub type GLint64 = i64;
pub type GLuint64 = u64;
pub type GLintptrARB = isize;
pub type GLsizeiptrARB = isize;
pub type GLint64EXT = i64;
pub type GLuint64EXT = u64;
pub type GLsync = *mut __GLsync;
pub type GLhalfNV = ::std::os::raw::c_ushort;
pub type GLvdpauSurfaceNV = GLintptr;
pub type FILE = _iobuf;
pub type PFNGLCULLFACEPROC = ::std::option::Option<unsafe extern "C" fn(mode: GLenum)>;
pub type GLDEBUGPROC = ::std::option::Option<
    unsafe extern "C" fn(
        source: GLenum,
        type_: GLenum,
        id: GLuint,
        severity: GLenum,
        length: GLsizei,
        message: *const GLchar,
        userParam: *const ::std::os::raw::c_void,
    ),
>;
pub type GLDEBUGPROCARB = ::std::option::Option<
    unsafe extern "C" fn(
        source: GLenum,
        type_: GLenum,
        id: GLuint,
        severity: GLenum,
        length: GLsizei,
        message: *const GLchar,
        userParam: *const ::std::os::raw::c_void,
    ),
>;
pub type GLDEBUGPROCKHR = ::std::option::Option<
    unsafe extern "C" fn(
        source: GLenum,
        type_: GLenum,
        id: GLuint,
        severity: GLenum,
        length: GLsizei,
        message: *const GLchar,
        userParam: *const ::std::os::raw::c_void,
    ),
>;
pub type GLDEBUGPROCAMD = ::std::option::Option<
    unsafe extern "C" fn(
        id: GLuint,
        category: GLenum,
        severity: GLenum,
        length: GLsizei,
        message: *const GLchar,
        userParam: *mut ::std::os::raw::c_void,
    ),
>;
pub type GLADloadproc = ::std::option::Option<
    unsafe extern "C" fn(name: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_void,
>;
pub type TraceLogCallback = ::std::option::Option<
    unsafe extern "C" fn(
        logLevel: ::std::os::raw::c_int,
        text: *const ::std::os::raw::c_char,
        args: va_list,
    ),
>;
pub type LoadFileDataCallback = ::std::option::Option<
    unsafe extern "C" fn(
        fileName: *const ::std::os::raw::c_char,
        bytesRead: *mut ::std::os::raw::c_uint,
    ) -> *mut ::std::os::raw::c_uchar,
>;
pub type SaveFileDataCallback = ::std::option::Option<
    unsafe extern "C" fn(
        fileName: *const ::std::os::raw::c_char,
        data: *mut ::std::os::raw::c_void,
        bytesToWrite: ::std::os::raw::c_uint,
    ) -> bool,
>;
pub type LoadFileTextCallback = ::std::option::Option<
    unsafe extern "C" fn(fileName: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char,
>;
pub type SaveFileTextCallback = ::std::option::Option<
    unsafe extern "C" fn(
        fileName: *const ::std::os::raw::c_char,
        text: *mut ::std::os::raw::c_char,
    ) -> bool,
>;