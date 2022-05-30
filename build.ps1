rustc --crate-type cdylib --out-dir build src/lib.rs #--target x86_64-apple-darwin
# rustc --crate-type cdylib --out-dir build --target x86_64-pc-windows-gnu src/lib.rs
# rustc --crate-type cdylib --out-dir build --target x86_64-unknown-linux-gnu src/lib.rs
deno run -A src\jsx\test.js