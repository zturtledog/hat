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
      // nonblocking: true,
    }
});

console.log(dylib.symbols)

//main

export function init(title, width, height) {
  dylib.symbols.init(pfs(title),width,height);
}

//helper

function pfs(s) {//pointer from string
  return new TextEncoder().encode(s)
}

function sfp(p) {
  return new Deno.UnsafePointerView(p)
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