#[repr(C)]
#[derive(Copy, Clone)]
pub union _double_val {
    pub _Sh: [::std::os::raw::c_ushort; 4usize],
    pub _Val: f64,
    _bindgen_union_align: u64,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union _float_val {
    pub _Sh: [::std::os::raw::c_ushort; 2usize],
    pub _Val: f32,
    _bindgen_union_align: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union _ldouble_val {
    pub _Sh: [::std::os::raw::c_ushort; 4usize],
    pub _Val: f64,
    _bindgen_union_align: u64,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union _float_const {
    pub _Word: [::std::os::raw::c_ushort; 4usize],
    pub _Float: f32,
    pub _Double: f64,
    pub _Long_double: f64,
    _bindgen_union_align: u64,
}
