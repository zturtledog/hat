import { hat } from "./hat.js";

hat.init(init,main)

console.log(hat)

function init() {
    hat.win.initwindow(800, 450, "raylib [core] example - basic window");

    hat.win.setargetfps(60);
}

function main() {
    // if (!hat.windowshouldclose()) {
    //     BeginDrawing();

    //     ClearBackground(RAYWHITE);
    //     DrawText("Congrats! You created your first window!", 190, 200, 20, LIGHTGRAY);

    //     EndDrawing();
    // }else {
    //     CloseWindow();
    // }
}

// ClearBackground(RAYWHITE);
// DrawText("Congrats! You created your first window!", 190, 200, 20, LIGHTGRAY);