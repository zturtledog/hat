//register



//helpers
let dlkey = {}
function rgis(name,parms,rtrn,bl) {
    dlkey[name] = {
        parameters: parms, 
        result: rtrn||"void",
        nonblocking: bl||!true,
    }
}

//load
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

console.log(dlkey)
const dylib = Deno.dlopen("./raylib.dll", dlkey)

//"../../target/debug/output."+libSuffix    
// "add": { parameters: ["isize", "isize"], result: "isize" },