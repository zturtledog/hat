use raylib::prelude::*;
use std::ffi::CStr;

static mut HASDREW: bool = false;
static mut DRAWLIST: Vec<Rst> = Vec::new();

struct Rst {
    typ:isize,
    x:i32,
    y:i32,
}

#[no_mangle]
pub extern "C" fn init(title: *const u8, width: i32, height: i32) {
    let (mut rl, thread) = raylib::init()
        .size(width, height)
        .title(u8tastr(title))
        .build();
     
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
         
        d.clear_background(Color::WHITE);
        d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
    }
}

fn u8tastr<'a>(req: *const u8) -> &'a str {
    return unsafe {
        CStr::from_ptr(req as *const i8)
            .to_str()
            .expect("No null bytes in parameter")
    }//.to_string();
}
