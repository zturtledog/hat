// Determine library extension based on
// your OS.


//targets

//- x86_64-apple-darwin
//- x86_64-pc-windows-msvc
//- x86_64-unknown-linux-gnu


let libSuffix = "";
switch (Deno.build.os) {
  case "windows":
    libSuffix = "dll";
    break;
  case "darwin":
    libSuffix = "dylib";
    break;
  case "linux":
    libSuffix = "so";
    break;
}

const dylib = Deno.dlopen("D:/rusty programs/hat/target/debug/output.dll", {//"../../target/debug/output."+libSuffix
    // "add": { parameters: ["isize", "isize"], result: "isize" },
    "init":{
      parameters: ["pointer", "i32", "i32"], 
      result: "void",
      nonblocking: true,
    },
    "isdraw":{
      parameters: [], 
      result: "i8",
    },
    "redraw":{
      parameters: [], 
      result: "void",
    }
});

console.log(dylib.symbols)

//main

let llp = true;
let fps = {
  date: new Date(),
  ptime:0,
  avg:0,
  avgtime:0,
  fps:()=>{
    return fps.avg/fps.avgtime
  },
  upd:()=>{
    fps.avg += fps.date.getMilliseconds()-fps.ptime
    fps.avgtime++
    fps.ptime = fps.date.getMilliseconds();
  }
}
let o = (f)=>{f();llp?()=>{fps.upd();setTimeout(()=>{o(f)},6)}:console.log(fps.fps())}

export function init(title, width, height) {
  dylib.symbols.init(pfs(title),width,height);
  o(()=>{
    console.log(dylib.symbols.isdraw())
    if (i8tb(dylib.symbols.isdraw())) {
      dylib.symbols.redraw()
    }
    console.log("hello")
  });
}

//helper

function pfs(s) {//pointer from string
  return new TextEncoder().encode(s)
}

function sfp(p) {
  return new Deno.UnsafePointerView(p)
}

function i8tb(i) {
  return !!i;
}


/**
error: linker `cc` not found
  |
  = note: program not found

error: aborting due to previous error; 1 warning emitted

error: linker `cc` not found
  |
  = note: program not found

error: aborting due to previous error; 1 warning emitted
 */