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

const dylib = Deno.dlopen("../../build/lib."+libSuffix, {
    // "add": { parameters: ["isize", "isize"], result: "isize" },

});

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