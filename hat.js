import { register } from "./wraper.js"

export let hat = {}
let backend = {}

hat.opendll = (path, regis)=>{
    let dlkey = {}
    let ato = {}
    let rexil = []
    let paxl = []

    //register

    for (let i = 0; i < regis.length; i++) {
        dlkey[regis[i][0]] = {
            parameters: regis[i][1],
            result: regis[i][2] || "void",
            nonblocking: regis[i][3] || !true,
        }
        if (!regis[i][1].includes("pointer")) {
            rexil.push(regis[i][0])
        }else {
            paxl.push(regis[i][0])
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
    const dylib = Deno.dlopen(path+"."+libSuffix, dlkey)

    for (let i = 0; i < rexil.length; i++) {
        ato[snme(rexil[i])] = dylib.symbols[rexil[i]]
    }

    return {
        auto:ato,
        dylb:dylib,
        unip:paxl
    }
}

hat.error = (title, message) => {
    message = message.replaceAll("\n", "\n    ")

    {//colorize
        let recordingstring = false
        let prstr = false
        let endpoint = ""
        for (let i = 0; i < message.length; i++) {
            if (message.charAt(i) == '"') {//message.charAt(i) == "'" || 
                if (recordingstring) {
                    endpoint += message.charAt(i)
                    endpoint += "\x1b[0m"
                    recordingstring = false
                } else {
                    endpoint += "\x1b[34m"
                    recordingstring = true
                    endpoint += message.charAt(i)
                }
            }
            if (!"1234567890".includes(message.charAt(i))) {
                if (i + 1 < message.length && "1234567890".includes(message.charAt(i + 1))) {
                    endpoint += "\x1b[33m"
                }
                if (i - 1 > -1 && "1234567890".includes(message.charAt(i - 1))) {
                    endpoint += "\x1b[0m"
                }
            }
            if (prstr == recordingstring)
                endpoint += message.charAt(i)

            prstr = recordingstring
        }
        message = endpoint;
    }

    console.error("\x1b[1m\x1b[31m" + title + "\x1b[0m\n  " + message + "\x1b[0m")
}

hat.init = (init, main) => {
    backend.dylib = register(hat);
    init();
    let o = () => {
        setTimeout(() => { o(); main(); hat.update(), 6 });
    }
}

hat.update = () => { }