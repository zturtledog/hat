export function register(hat) {
    let dlkey = {}
    let rexil = []

    //register

    rgis("InitWindow", ["i32", "i32", "pointer"])

    //helpers
    function rgis(name, parms, rtrn, bl) {
        dlkey[name] = {
            parameters: parms,
            result: rtrn || "void",
            nonblocking: bl || !true,
        }
        rexil.push(name)
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

    const dylib = Deno.dlopen("./raylib.dll", dlkey)

    for (let i = 0; i < rexil.length; i++) {
        hat[rexil[i].toLowerCase()] = dylib.symbols[rexil[i]]
    }

    return hat
}

//clamp