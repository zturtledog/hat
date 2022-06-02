export function register(hat) {
    let ool = opendll("./raylib",[
        ["InitWindow", ["i32","i32","pointer"]],
        ["SetTargetFPS",["i32"]],
        ["WindowShouldClose",[],"isize"]
    ])   

    hat.win = ool.auto

    hat.win.initwindow = (width, height, title)=>{
        ool.dylb.symbols.InitWindow(width,height,pfs(title))
    }
}

function snme(x) {
    let y = x.toLowerCase()

    for (let i = 0; i < x.toLowerCase().length; i++) {
        const e = x.toLowerCase().charAt(i);
        
        y = y.replace(e+e,e)
    }

    return y
}

function pfs(s) {//pointer from string
    return new TextEncoder().encode(s)
}
  
function sfp(p) {
    return new Deno.UnsafePointerView(p)
}
  
function i8tb(i) {
    return !!i;
}

//clamp