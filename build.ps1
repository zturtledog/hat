param($p1)
if ($p1) {
    clear
}
cargo build # --out-dir build
deno run --allow-ffi --unstable src\jsx\test.js