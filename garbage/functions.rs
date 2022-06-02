extern "C" {
    pub fn InitWindow(
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        title: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn WindowShouldClose() -> bool;
}
extern "C" {
    pub fn CloseWindow();
}
extern "C" {
    pub fn IsWindowReady() -> bool;
}
extern "C" {
    pub fn IsWindowFullscreen() -> bool;
}
extern "C" {
    pub fn IsWindowHidden() -> bool;
}
extern "C" {
    pub fn IsWindowMinimized() -> bool;
}
extern "C" {
    pub fn IsWindowMaximized() -> bool;
}
extern "C" {
    pub fn IsWindowFocused() -> bool;
}
extern "C" {
    pub fn IsWindowResized() -> bool;
}
extern "C" {
    pub fn IsWindowState(flag: ::std::os::raw::c_uint) -> bool;
}
extern "C" {
    pub fn SetWindowState(flags: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn ClearWindowState(flags: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn ToggleFullscreen();
}
extern "C" {
    pub fn MaximizeWindow();
}
extern "C" {
    pub fn MinimizeWindow();
}
extern "C" {
    pub fn RestoreWindow();
}
extern "C" {
    pub fn SetWindowIcon(image: Image);
}
extern "C" {
    pub fn SetWindowTitle(title: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn SetWindowPosition(x: ::std::os::raw::c_int, y: ::std::os::raw::c_int);
}
extern "C" {
    pub fn SetWindowMonitor(monitor: ::std::os::raw::c_int);
}
extern "C" {
    pub fn SetWindowMinSize(width: ::std::os::raw::c_int, height: ::std::os::raw::c_int);
}
extern "C" {
    pub fn SetWindowSize(width: ::std::os::raw::c_int, height: ::std::os::raw::c_int);
}
extern "C" {
    pub fn GetWindowHandle() -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn GetScreenWidth() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GetScreenHeight() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GetMonitorCount() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GetCurrentMonitor() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GetMonitorPosition(monitor: ::std::os::raw::c_int) -> Vector2;
}
extern "C" {
    pub fn GetMonitorWidth(monitor: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GetMonitorHeight(monitor: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GetMonitorPhysicalWidth(monitor: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GetMonitorPhysicalHeight(monitor: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GetMonitorRefreshRate(monitor: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GetWindowPosition() -> Vector2;
}
extern "C" {
    pub fn GetWindowScaleDPI() -> Vector2;
}
extern "C" {
    pub fn GetMonitorName(monitor: ::std::os::raw::c_int) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn SetClipboardText(text: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn GetClipboardText() -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn ShowCursor();
}
extern "C" {
    pub fn HideCursor();
}
extern "C" {
    pub fn IsCursorHidden() -> bool;
}
extern "C" {
    pub fn EnableCursor();
}
extern "C" {
    pub fn DisableCursor();
}
extern "C" {
    pub fn IsCursorOnScreen() -> bool;
}
extern "C" {
    pub fn ClearBackground(color: Color);
}
extern "C" {
    pub fn BeginDrawing();
}
extern "C" {
    pub fn EndDrawing();
}
extern "C" {
    pub fn BeginMode2D(camera: Camera2D);
}
extern "C" {
    pub fn EndMode2D();
}
extern "C" {
    pub fn BeginMode3D(camera: Camera3D);
}
extern "C" {
    pub fn EndMode3D();
}
extern "C" {
    pub fn BeginTextureMode(target: RenderTexture2D);
}
extern "C" {
    pub fn EndTextureMode();
}
extern "C" {
    pub fn BeginShaderMode(shader: Shader);
}
extern "C" {
    pub fn EndShaderMode();
}
extern "C" {
    pub fn BeginBlendMode(mode: ::std::os::raw::c_int);
}
extern "C" {
    pub fn EndBlendMode();
}
extern "C" {
    pub fn BeginScissorMode(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn EndScissorMode();
}
extern "C" {
    pub fn BeginVrStereoMode(config: VrStereoConfig);
}
extern "C" {
    pub fn EndVrStereoMode();
}
extern "C" {
    pub fn LoadVrStereoConfig(device: VrDeviceInfo) -> VrStereoConfig;
}
extern "C" {
    pub fn UnloadVrStereoConfig(config: VrStereoConfig);
}
extern "C" {
    pub fn LoadShader(
        vsFileName: *const ::std::os::raw::c_char,
        fsFileName: *const ::std::os::raw::c_char,
    ) -> Shader;
}
extern "C" {
    pub fn LoadShaderFromMemory(
        vsCode: *const ::std::os::raw::c_char,
        fsCode: *const ::std::os::raw::c_char,
    ) -> Shader;
}
extern "C" {
    pub fn GetShaderLocation(
        shader: Shader,
        uniformName: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GetShaderLocationAttrib(
        shader: Shader,
        attribName: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SetShaderValue(
        shader: Shader,
        locIndex: ::std::os::raw::c_int,
        value: *const ::std::os::raw::c_void,
        uniformType: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn SetShaderValueV(
        shader: Shader,
        locIndex: ::std::os::raw::c_int,
        value: *const ::std::os::raw::c_void,
        uniformType: ::std::os::raw::c_int,
        count: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn SetShaderValueMatrix(shader: Shader, locIndex: ::std::os::raw::c_int, mat: Matrix);
}
extern "C" {
    pub fn SetShaderValueTexture(
        shader: Shader,
        locIndex: ::std::os::raw::c_int,
        texture: Texture2D,
    );
}
extern "C" {
    pub fn UnloadShader(shader: Shader);
}
extern "C" {
    pub fn GetMouseRay(mousePosition: Vector2, camera: Camera) -> Ray;
}
extern "C" {
    pub fn GetCameraMatrix(camera: Camera) -> Matrix;
}
extern "C" {
    pub fn GetCameraMatrix2D(camera: Camera2D) -> Matrix;
}
extern "C" {
    pub fn GetWorldToScreen(position: Vector3, camera: Camera) -> Vector2;
}
extern "C" {
    pub fn GetWorldToScreenEx(
        position: Vector3,
        camera: Camera,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    ) -> Vector2;
}
extern "C" {
    pub fn GetWorldToScreen2D(position: Vector2, camera: Camera2D) -> Vector2;
}
extern "C" {
    pub fn GetScreenToWorld2D(position: Vector2, camera: Camera2D) -> Vector2;
}
extern "C" {
    pub fn SetTargetFPS(fps: ::std::os::raw::c_int);
}
extern "C" {
    pub fn GetFPS() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GetFrameTime() -> f32;
}
extern "C" {
    pub fn GetTime() -> f64;
}
extern "C" {
    pub fn GetRandomValue(
        min: ::std::os::raw::c_int,
        max: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TakeScreenshot(fileName: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn SetConfigFlags(flags: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn TraceLog(logLevel: ::std::os::raw::c_int, text: *const ::std::os::raw::c_char, ...);
}
extern "C" {
    pub fn SetTraceLogLevel(logLevel: ::std::os::raw::c_int);
}
extern "C" {
    pub fn MemAlloc(size: ::std::os::raw::c_int) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn MemRealloc(
        ptr: *mut ::std::os::raw::c_void,
        size: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn MemFree(ptr: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn SetTraceLogCallback(callback: TraceLogCallback);
}
extern "C" {
    pub fn SetLoadFileDataCallback(callback: LoadFileDataCallback);
}
extern "C" {
    pub fn SetSaveFileDataCallback(callback: SaveFileDataCallback);
}
extern "C" {
    pub fn SetLoadFileTextCallback(callback: LoadFileTextCallback);
}
extern "C" {
    pub fn SetSaveFileTextCallback(callback: SaveFileTextCallback);
}
extern "C" {
    pub fn LoadFileData(
        fileName: *const ::std::os::raw::c_char,
        bytesRead: *mut ::std::os::raw::c_uint,
    ) -> *mut ::std::os::raw::c_uchar;
}
extern "C" {
    pub fn UnloadFileData(data: *mut ::std::os::raw::c_uchar);
}
extern "C" {
    pub fn SaveFileData(
        fileName: *const ::std::os::raw::c_char,
        data: *mut ::std::os::raw::c_void,
        bytesToWrite: ::std::os::raw::c_uint,
    ) -> bool;
}
extern "C" {
    pub fn LoadFileText(fileName: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn UnloadFileText(text: *mut ::std::os::raw::c_uchar);
}
extern "C" {
    pub fn SaveFileText(
        fileName: *const ::std::os::raw::c_char,
        text: *mut ::std::os::raw::c_char,
    ) -> bool;
}
extern "C" {
    pub fn FileExists(fileName: *const ::std::os::raw::c_char) -> bool;
}
extern "C" {
    pub fn DirectoryExists(dirPath: *const ::std::os::raw::c_char) -> bool;
}
extern "C" {
    pub fn IsFileExtension(
        fileName: *const ::std::os::raw::c_char,
        ext: *const ::std::os::raw::c_char,
    ) -> bool;
}
extern "C" {
    pub fn GetFileExtension(
        fileName: *const ::std::os::raw::c_char,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn GetFileName(filePath: *const ::std::os::raw::c_char) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn GetFileNameWithoutExt(
        filePath: *const ::std::os::raw::c_char,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn GetDirectoryPath(
        filePath: *const ::std::os::raw::c_char,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn GetPrevDirectoryPath(
        dirPath: *const ::std::os::raw::c_char,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn GetWorkingDirectory() -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn GetDirectoryFiles(
        dirPath: *const ::std::os::raw::c_char,
        count: *mut ::std::os::raw::c_int,
    ) -> *mut *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn ClearDirectoryFiles();
}
extern "C" {
    pub fn ChangeDirectory(dir: *const ::std::os::raw::c_char) -> bool;
}
extern "C" {
    pub fn IsFileDropped() -> bool;
}
extern "C" {
    pub fn GetDroppedFiles(count: *mut ::std::os::raw::c_int) -> *mut *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn ClearDroppedFiles();
}
extern "C" {
    pub fn GetFileModTime(fileName: *const ::std::os::raw::c_char) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn CompressData(
        data: *mut ::std::os::raw::c_uchar,
        dataLength: ::std::os::raw::c_int,
        compDataLength: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_uchar;
}
extern "C" {
    pub fn DecompressData(
        compData: *mut ::std::os::raw::c_uchar,
        compDataLength: ::std::os::raw::c_int,
        dataLength: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_uchar;
}
extern "C" {
    pub fn SaveStorageValue(position: ::std::os::raw::c_uint, value: ::std::os::raw::c_int)
        -> bool;
}
extern "C" {
    pub fn LoadStorageValue(position: ::std::os::raw::c_uint) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn OpenURL(url: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn IsKeyPressed(key: ::std::os::raw::c_int) -> bool;
}
extern "C" {
    pub fn IsKeyDown(key: ::std::os::raw::c_int) -> bool;
}
extern "C" {
    pub fn IsKeyReleased(key: ::std::os::raw::c_int) -> bool;
}
extern "C" {
    pub fn IsKeyUp(key: ::std::os::raw::c_int) -> bool;
}
extern "C" {
    pub fn SetExitKey(key: ::std::os::raw::c_int);
}
extern "C" {
    pub fn GetKeyPressed() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GetCharPressed() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn IsGamepadAvailable(gamepad: ::std::os::raw::c_int) -> bool;
}
extern "C" {
    pub fn IsGamepadName(
        gamepad: ::std::os::raw::c_int,
        name: *const ::std::os::raw::c_char,
    ) -> bool;
}
extern "C" {
    pub fn GetGamepadName(gamepad: ::std::os::raw::c_int) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn IsGamepadButtonPressed(
        gamepad: ::std::os::raw::c_int,
        button: ::std::os::raw::c_int,
    ) -> bool;
}
extern "C" {
    pub fn IsGamepadButtonDown(
        gamepad: ::std::os::raw::c_int,
        button: ::std::os::raw::c_int,
    ) -> bool;
}
extern "C" {
    pub fn IsGamepadButtonReleased(
        gamepad: ::std::os::raw::c_int,
        button: ::std::os::raw::c_int,
    ) -> bool;
}
extern "C" {
    pub fn IsGamepadButtonUp(gamepad: ::std::os::raw::c_int, button: ::std::os::raw::c_int)
        -> bool;
}
extern "C" {
    pub fn GetGamepadButtonPressed() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GetGamepadAxisCount(gamepad: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GetGamepadAxisMovement(
        gamepad: ::std::os::raw::c_int,
        axis: ::std::os::raw::c_int,
    ) -> f32;
}
extern "C" {
    pub fn SetGamepadMappings(mappings: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn IsMouseButtonPressed(button: ::std::os::raw::c_int) -> bool;
}
extern "C" {
    pub fn IsMouseButtonDown(button: ::std::os::raw::c_int) -> bool;
}
extern "C" {
    pub fn IsMouseButtonReleased(button: ::std::os::raw::c_int) -> bool;
}
extern "C" {
    pub fn IsMouseButtonUp(button: ::std::os::raw::c_int) -> bool;
}
extern "C" {
    pub fn GetMouseX() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GetMouseY() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GetMousePosition() -> Vector2;
}
extern "C" {
    pub fn SetMousePosition(x: ::std::os::raw::c_int, y: ::std::os::raw::c_int);
}
extern "C" {
    pub fn SetMouseOffset(offsetX: ::std::os::raw::c_int, offsetY: ::std::os::raw::c_int);
}
extern "C" {
    pub fn SetMouseScale(scaleX: f32, scaleY: f32);
}
extern "C" {
    pub fn GetMouseWheelMove() -> f32;
}
extern "C" {
    pub fn SetMouseCursor(cursor: ::std::os::raw::c_int);
}
extern "C" {
    pub fn GetTouchX() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GetTouchY() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GetTouchPosition(index: ::std::os::raw::c_int) -> Vector2;
}
extern "C" {
    pub fn SetGesturesEnabled(flags: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn IsGestureDetected(gesture: ::std::os::raw::c_int) -> bool;
}
extern "C" {
    pub fn GetGestureDetected() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GetTouchPointsCount() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GetGestureHoldDuration() -> f32;
}
extern "C" {
    pub fn GetGestureDragVector() -> Vector2;
}
extern "C" {
    pub fn GetGestureDragAngle() -> f32;
}
extern "C" {
    pub fn GetGesturePinchVector() -> Vector2;
}
extern "C" {
    pub fn GetGesturePinchAngle() -> f32;
}
extern "C" {
    pub fn SetCameraMode(camera: Camera, mode: ::std::os::raw::c_int);
}
extern "C" {
    pub fn UpdateCamera(camera: *mut Camera);
}
extern "C" {
    pub fn SetCameraPanControl(keyPan: ::std::os::raw::c_int);
}
extern "C" {
    pub fn SetCameraAltControl(keyAlt: ::std::os::raw::c_int);
}
extern "C" {
    pub fn SetCameraSmoothZoomControl(keySmoothZoom: ::std::os::raw::c_int);
}
extern "C" {
    pub fn SetCameraMoveControls(
        keyFront: ::std::os::raw::c_int,
        keyBack: ::std::os::raw::c_int,
        keyRight: ::std::os::raw::c_int,
        keyLeft: ::std::os::raw::c_int,
        keyUp: ::std::os::raw::c_int,
        keyDown: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn SetShapesTexture(texture: Texture2D, source: Rectangle);
}
extern "C" {
    pub fn DrawPixel(posX: ::std::os::raw::c_int, posY: ::std::os::raw::c_int, color: Color);
}
extern "C" {
    pub fn DrawPixelV(position: Vector2, color: Color);
}
extern "C" {
    pub fn DrawLine(
        startPosX: ::std::os::raw::c_int,
        startPosY: ::std::os::raw::c_int,
        endPosX: ::std::os::raw::c_int,
        endPosY: ::std::os::raw::c_int,
        color: Color,
    );
}
extern "C" {
    pub fn DrawLineV(startPos: Vector2, endPos: Vector2, color: Color);
}
extern "C" {
    pub fn DrawLineEx(startPos: Vector2, endPos: Vector2, thick: f32, color: Color);
}
extern "C" {
    pub fn DrawLineBezier(startPos: Vector2, endPos: Vector2, thick: f32, color: Color);
}
extern "C" {
    pub fn DrawLineBezierQuad(
        startPos: Vector2,
        endPos: Vector2,
        controlPos: Vector2,
        thick: f32,
        color: Color,
    );
}
extern "C" {
    pub fn DrawLineStrip(points: *mut Vector2, pointsCount: ::std::os::raw::c_int, color: Color);
}
extern "C" {
    pub fn DrawCircle(
        centerX: ::std::os::raw::c_int,
        centerY: ::std::os::raw::c_int,
        radius: f32,
        color: Color,
    );
}
extern "C" {
    pub fn DrawCircleSector(
        center: Vector2,
        radius: f32,
        startAngle: f32,
        endAngle: f32,
        segments: ::std::os::raw::c_int,
        color: Color,
    );
}
extern "C" {
    pub fn DrawCircleSectorLines(
        center: Vector2,
        radius: f32,
        startAngle: f32,
        endAngle: f32,
        segments: ::std::os::raw::c_int,
        color: Color,
    );
}
extern "C" {
    pub fn DrawCircleGradient(
        centerX: ::std::os::raw::c_int,
        centerY: ::std::os::raw::c_int,
        radius: f32,
        color1: Color,
        color2: Color,
    );
}
extern "C" {
    pub fn DrawCircleV(center: Vector2, radius: f32, color: Color);
}
extern "C" {
    pub fn DrawCircleLines(
        centerX: ::std::os::raw::c_int,
        centerY: ::std::os::raw::c_int,
        radius: f32,
        color: Color,
    );
}
extern "C" {
    pub fn DrawEllipse(
        centerX: ::std::os::raw::c_int,
        centerY: ::std::os::raw::c_int,
        radiusH: f32,
        radiusV: f32,
        color: Color,
    );
}
extern "C" {
    pub fn DrawEllipseLines(
        centerX: ::std::os::raw::c_int,
        centerY: ::std::os::raw::c_int,
        radiusH: f32,
        radiusV: f32,
        color: Color,
    );
}
extern "C" {
    pub fn DrawRing(
        center: Vector2,
        innerRadius: f32,
        outerRadius: f32,
        startAngle: f32,
        endAngle: f32,
        segments: ::std::os::raw::c_int,
        color: Color,
    );
}
extern "C" {
    pub fn DrawRingLines(
        center: Vector2,
        innerRadius: f32,
        outerRadius: f32,
        startAngle: f32,
        endAngle: f32,
        segments: ::std::os::raw::c_int,
        color: Color,
    );
}
extern "C" {
    pub fn DrawRectangle(
        posX: ::std::os::raw::c_int,
        posY: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        color: Color,
    );
}
extern "C" {
    pub fn DrawRectangleV(position: Vector2, size: Vector2, color: Color);
}
extern "C" {
    pub fn DrawRectangleRec(rec: Rectangle, color: Color);
}
extern "C" {
    pub fn DrawRectanglePro(rec: Rectangle, origin: Vector2, rotation: f32, color: Color);
}
extern "C" {
    pub fn DrawRectangleGradientV(
        posX: ::std::os::raw::c_int,
        posY: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        color1: Color,
        color2: Color,
    );
}
extern "C" {
    pub fn DrawRectangleGradientH(
        posX: ::std::os::raw::c_int,
        posY: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        color1: Color,
        color2: Color,
    );
}
extern "C" {
    pub fn DrawRectangleGradientEx(
        rec: Rectangle,
        col1: Color,
        col2: Color,
        col3: Color,
        col4: Color,
    );
}
extern "C" {
    pub fn DrawRectangleLines(
        posX: ::std::os::raw::c_int,
        posY: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        color: Color,
    );
}
extern "C" {
    pub fn DrawRectangleLinesEx(rec: Rectangle, lineThick: ::std::os::raw::c_int, color: Color);
}
extern "C" {
    pub fn DrawRectangleRounded(
        rec: Rectangle,
        roundness: f32,
        segments: ::std::os::raw::c_int,
        color: Color,
    );
}
extern "C" {
    pub fn DrawRectangleRoundedLines(
        rec: Rectangle,
        roundness: f32,
        segments: ::std::os::raw::c_int,
        lineThick: ::std::os::raw::c_int,
        color: Color,
    );
}
extern "C" {
    pub fn DrawTriangle(v1: Vector2, v2: Vector2, v3: Vector2, color: Color);
}
extern "C" {
    pub fn DrawTriangleLines(v1: Vector2, v2: Vector2, v3: Vector2, color: Color);
}
extern "C" {
    pub fn DrawTriangleFan(points: *mut Vector2, pointsCount: ::std::os::raw::c_int, color: Color);
}
extern "C" {
    pub fn DrawTriangleStrip(
        points: *mut Vector2,
        pointsCount: ::std::os::raw::c_int,
        color: Color,
    );
}
extern "C" {
    pub fn DrawPoly(
        center: Vector2,
        sides: ::std::os::raw::c_int,
        radius: f32,
        rotation: f32,
        color: Color,
    );
}
extern "C" {
    pub fn DrawPolyLines(
        center: Vector2,
        sides: ::std::os::raw::c_int,
        radius: f32,
        rotation: f32,
        color: Color,
    );
}
extern "C" {
    pub fn CheckCollisionRecs(rec1: Rectangle, rec2: Rectangle) -> bool;
}
extern "C" {
    pub fn CheckCollisionCircles(
        center1: Vector2,
        radius1: f32,
        center2: Vector2,
        radius2: f32,
    ) -> bool;
}
extern "C" {
    pub fn CheckCollisionCircleRec(center: Vector2, radius: f32, rec: Rectangle) -> bool;
}
extern "C" {
    pub fn CheckCollisionPointRec(point: Vector2, rec: Rectangle) -> bool;
}
extern "C" {
    pub fn CheckCollisionPointCircle(point: Vector2, center: Vector2, radius: f32) -> bool;
}
extern "C" {
    pub fn CheckCollisionPointTriangle(
        point: Vector2,
        p1: Vector2,
        p2: Vector2,
        p3: Vector2,
    ) -> bool;
}
extern "C" {
    pub fn CheckCollisionLines(
        startPos1: Vector2,
        endPos1: Vector2,
        startPos2: Vector2,
        endPos2: Vector2,
        collisionPoint: *mut Vector2,
    ) -> bool;
}
extern "C" {
    pub fn GetCollisionRec(rec1: Rectangle, rec2: Rectangle) -> Rectangle;
}
extern "C" {
    pub fn LoadImage(fileName: *const ::std::os::raw::c_char) -> Image;
}
extern "C" {
    pub fn LoadImageRaw(
        fileName: *const ::std::os::raw::c_char,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        format: ::std::os::raw::c_int,
        headerSize: ::std::os::raw::c_int,
    ) -> Image;
}
extern "C" {
    pub fn LoadImageAnim(
        fileName: *const ::std::os::raw::c_char,
        frames: *mut ::std::os::raw::c_int,
    ) -> Image;
}
extern "C" {
    pub fn LoadImageFromMemory(
        fileType: *const ::std::os::raw::c_char,
        fileData: *const ::std::os::raw::c_uchar,
        dataSize: ::std::os::raw::c_int,
    ) -> Image;
}
extern "C" {
    pub fn UnloadImage(image: Image);
}
extern "C" {
    pub fn ExportImage(image: Image, fileName: *const ::std::os::raw::c_char) -> bool;
}
extern "C" {
    pub fn ExportImageAsCode(image: Image, fileName: *const ::std::os::raw::c_char) -> bool;
}
extern "C" {
    pub fn GenImageColor(
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        color: Color,
    ) -> Image;
}
extern "C" {
    pub fn GenImageGradientV(
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        top: Color,
        bottom: Color,
    ) -> Image;
}
extern "C" {
    pub fn GenImageGradientH(
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        left: Color,
        right: Color,
    ) -> Image;
}
extern "C" {
    pub fn GenImageGradientRadial(
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        density: f32,
        inner: Color,
        outer: Color,
    ) -> Image;
}
extern "C" {
    pub fn GenImageChecked(
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        checksX: ::std::os::raw::c_int,
        checksY: ::std::os::raw::c_int,
        col1: Color,
        col2: Color,
    ) -> Image;
}
extern "C" {
    pub fn GenImageWhiteNoise(
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        factor: f32,
    ) -> Image;
}
extern "C" {
    pub fn GenImagePerlinNoise(
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        offsetX: ::std::os::raw::c_int,
        offsetY: ::std::os::raw::c_int,
        scale: f32,
    ) -> Image;
}
extern "C" {
    pub fn GenImageCellular(
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        tileSize: ::std::os::raw::c_int,
    ) -> Image;
}
extern "C" {
    pub fn ImageCopy(image: Image) -> Image;
}
extern "C" {
    pub fn ImageFromImage(image: Image, rec: Rectangle) -> Image;
}
extern "C" {
    pub fn ImageText(
        text: *const ::std::os::raw::c_char,
        fontSize: ::std::os::raw::c_int,
        color: Color,
    ) -> Image;
}
extern "C" {
    pub fn ImageTextEx(
        font: Font,
        text: *const ::std::os::raw::c_char,
        fontSize: f32,
        spacing: f32,
        tint: Color,
    ) -> Image;
}
extern "C" {
    pub fn ImageFormat(image: *mut Image, newFormat: ::std::os::raw::c_int);
}
extern "C" {
    pub fn ImageToPOT(image: *mut Image, fill: Color);
}
extern "C" {
    pub fn ImageCrop(image: *mut Image, crop: Rectangle);
}
extern "C" {
    pub fn ImageAlphaCrop(image: *mut Image, threshold: f32);
}
extern "C" {
    pub fn ImageAlphaClear(image: *mut Image, color: Color, threshold: f32);
}
extern "C" {
    pub fn ImageAlphaMask(image: *mut Image, alphaMask: Image);
}
extern "C" {
    pub fn ImageAlphaPremultiply(image: *mut Image);
}
extern "C" {
    pub fn ImageResize(
        image: *mut Image,
        newWidth: ::std::os::raw::c_int,
        newHeight: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn ImageResizeNN(
        image: *mut Image,
        newWidth: ::std::os::raw::c_int,
        newHeight: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn ImageResizeCanvas(
        image: *mut Image,
        newWidth: ::std::os::raw::c_int,
        newHeight: ::std::os::raw::c_int,
        offsetX: ::std::os::raw::c_int,
        offsetY: ::std::os::raw::c_int,
        fill: Color,
    );
}
extern "C" {
    pub fn ImageMipmaps(image: *mut Image);
}
extern "C" {
    pub fn ImageDither(
        image: *mut Image,
        rBpp: ::std::os::raw::c_int,
        gBpp: ::std::os::raw::c_int,
        bBpp: ::std::os::raw::c_int,
        aBpp: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn ImageFlipVertical(image: *mut Image);
}
extern "C" {
    pub fn ImageFlipHorizontal(image: *mut Image);
}
extern "C" {
    pub fn ImageRotateCW(image: *mut Image);
}
extern "C" {
    pub fn ImageRotateCCW(image: *mut Image);
}
extern "C" {
    pub fn ImageColorTint(image: *mut Image, color: Color);
}
extern "C" {
    pub fn ImageColorInvert(image: *mut Image);
}
extern "C" {
    pub fn ImageColorGrayscale(image: *mut Image);
}
extern "C" {
    pub fn ImageColorContrast(image: *mut Image, contrast: f32);
}
extern "C" {
    pub fn ImageColorBrightness(image: *mut Image, brightness: ::std::os::raw::c_int);
}
extern "C" {
    pub fn ImageColorReplace(image: *mut Image, color: Color, replace: Color);
}
extern "C" {
    pub fn LoadImageColors(image: Image) -> *mut Color;
}
extern "C" {
    pub fn LoadImagePalette(
        image: Image,
        maxPaletteSize: ::std::os::raw::c_int,
        colorsCount: *mut ::std::os::raw::c_int,
    ) -> *mut Color;
}
extern "C" {
    pub fn UnloadImageColors(colors: *mut Color);
}
extern "C" {
    pub fn UnloadImagePalette(colors: *mut Color);
}
extern "C" {
    pub fn GetImageAlphaBorder(image: Image, threshold: f32) -> Rectangle;
}
extern "C" {
    pub fn ImageClearBackground(dst: *mut Image, color: Color);
}
extern "C" {
    pub fn ImageDrawPixel(
        dst: *mut Image,
        posX: ::std::os::raw::c_int,
        posY: ::std::os::raw::c_int,
        color: Color,
    );
}
extern "C" {
    pub fn ImageDrawPixelV(dst: *mut Image, position: Vector2, color: Color);
}
extern "C" {
    pub fn ImageDrawLine(
        dst: *mut Image,
        startPosX: ::std::os::raw::c_int,
        startPosY: ::std::os::raw::c_int,
        endPosX: ::std::os::raw::c_int,
        endPosY: ::std::os::raw::c_int,
        color: Color,
    );
}
extern "C" {
    pub fn ImageDrawLineV(dst: *mut Image, start: Vector2, end: Vector2, color: Color);
}
extern "C" {
    pub fn ImageDrawCircle(
        dst: *mut Image,
        centerX: ::std::os::raw::c_int,
        centerY: ::std::os::raw::c_int,
        radius: ::std::os::raw::c_int,
        color: Color,
    );
}
extern "C" {
    pub fn ImageDrawCircleV(
        dst: *mut Image,
        center: Vector2,
        radius: ::std::os::raw::c_int,
        color: Color,
    );
}
extern "C" {
    pub fn ImageDrawRectangle(
        dst: *mut Image,
        posX: ::std::os::raw::c_int,
        posY: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        color: Color,
    );
}
extern "C" {
    pub fn ImageDrawRectangleV(dst: *mut Image, position: Vector2, size: Vector2, color: Color);
}
extern "C" {
    pub fn ImageDrawRectangleRec(dst: *mut Image, rec: Rectangle, color: Color);
}
extern "C" {
    pub fn ImageDrawRectangleLines(
        dst: *mut Image,
        rec: Rectangle,
        thick: ::std::os::raw::c_int,
        color: Color,
    );
}
extern "C" {
    pub fn ImageDraw(
        dst: *mut Image,
        src: Image,
        srcRec: Rectangle,
        dstRec: Rectangle,
        tint: Color,
    );
}
extern "C" {
    pub fn ImageDrawText(
        dst: *mut Image,
        text: *const ::std::os::raw::c_char,
        posX: ::std::os::raw::c_int,
        posY: ::std::os::raw::c_int,
        fontSize: ::std::os::raw::c_int,
        color: Color,
    );
}
extern "C" {
    pub fn ImageDrawTextEx(
        dst: *mut Image,
        font: Font,
        text: *const ::std::os::raw::c_char,
        position: Vector2,
        fontSize: f32,
        spacing: f32,
        tint: Color,
    );
}
extern "C" {
    pub fn LoadTexture(fileName: *const ::std::os::raw::c_char) -> Texture2D;
}
extern "C" {
    pub fn LoadTextureFromImage(image: Image) -> Texture2D;
}
extern "C" {
    pub fn LoadTextureCubemap(image: Image, layout: ::std::os::raw::c_int) -> TextureCubemap;
}
extern "C" {
    pub fn LoadRenderTexture(
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    ) -> RenderTexture2D;
}
extern "C" {
    pub fn UnloadTexture(texture: Texture2D);
}
extern "C" {
    pub fn UnloadRenderTexture(target: RenderTexture2D);
}
extern "C" {
    pub fn UpdateTexture(texture: Texture2D, pixels: *const ::std::os::raw::c_void);
}
extern "C" {
    pub fn UpdateTextureRec(
        texture: Texture2D,
        rec: Rectangle,
        pixels: *const ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn GetTextureData(texture: Texture2D) -> Image;
}
extern "C" {
    pub fn GetScreenData() -> Image;
}
extern "C" {
    pub fn GenTextureMipmaps(texture: *mut Texture2D);
}
extern "C" {
    pub fn SetTextureFilter(texture: Texture2D, filter: ::std::os::raw::c_int);
}
extern "C" {
    pub fn SetTextureWrap(texture: Texture2D, wrap: ::std::os::raw::c_int);
}
extern "C" {
    pub fn DrawTexture(
        texture: Texture2D,
        posX: ::std::os::raw::c_int,
        posY: ::std::os::raw::c_int,
        tint: Color,
    );
}
extern "C" {
    pub fn DrawTextureV(texture: Texture2D, position: Vector2, tint: Color);
}
extern "C" {
    pub fn DrawTextureEx(
        texture: Texture2D,
        position: Vector2,
        rotation: f32,
        scale: f32,
        tint: Color,
    );
}
extern "C" {
    pub fn DrawTextureRec(texture: Texture2D, source: Rectangle, position: Vector2, tint: Color);
}
extern "C" {
    pub fn DrawTextureQuad(
        texture: Texture2D,
        tiling: Vector2,
        offset: Vector2,
        quad: Rectangle,
        tint: Color,
    );
}
extern "C" {
    pub fn DrawTextureTiled(
        texture: Texture2D,
        source: Rectangle,
        dest: Rectangle,
        origin: Vector2,
        rotation: f32,
        scale: f32,
        tint: Color,
    );
}
extern "C" {
    pub fn DrawTexturePro(
        texture: Texture2D,
        source: Rectangle,
        dest: Rectangle,
        origin: Vector2,
        rotation: f32,
        tint: Color,
    );
}
extern "C" {
    pub fn DrawTextureNPatch(
        texture: Texture2D,
        nPatchInfo: NPatchInfo,
        dest: Rectangle,
        origin: Vector2,
        rotation: f32,
        tint: Color,
    );
}
extern "C" {
    pub fn DrawTexturePoly(
        texture: Texture2D,
        center: Vector2,
        points: *mut Vector2,
        texcoords: *mut Vector2,
        pointsCount: ::std::os::raw::c_int,
        tint: Color,
    );
}
extern "C" {
    pub fn Fade(color: Color, alpha: f32) -> Color;
}
extern "C" {
    pub fn ColorToInt(color: Color) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ColorNormalize(color: Color) -> Vector4;
}
extern "C" {
    pub fn ColorFromNormalized(normalized: Vector4) -> Color;
}
extern "C" {
    pub fn ColorToHSV(color: Color) -> Vector3;
}
extern "C" {
    pub fn ColorFromHSV(hue: f32, saturation: f32, value: f32) -> Color;
}
extern "C" {
    pub fn ColorAlpha(color: Color, alpha: f32) -> Color;
}
extern "C" {
    pub fn ColorAlphaBlend(dst: Color, src: Color, tint: Color) -> Color;
}
extern "C" {
    pub fn GetColor(hexValue: ::std::os::raw::c_int) -> Color;
}
extern "C" {
    pub fn GetPixelColor(
        srcPtr: *mut ::std::os::raw::c_void,
        format: ::std::os::raw::c_int,
    ) -> Color;
}
extern "C" {
    pub fn SetPixelColor(
        dstPtr: *mut ::std::os::raw::c_void,
        color: Color,
        format: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn GetPixelDataSize(
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        format: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GetFontDefault() -> Font;
}
extern "C" {
    pub fn LoadFont(fileName: *const ::std::os::raw::c_char) -> Font;
}
extern "C" {
    pub fn LoadFontEx(
        fileName: *const ::std::os::raw::c_char,
        fontSize: ::std::os::raw::c_int,
        fontChars: *mut ::std::os::raw::c_int,
        charsCount: ::std::os::raw::c_int,
    ) -> Font;
}
extern "C" {
    pub fn LoadFontFromImage(image: Image, key: Color, firstChar: ::std::os::raw::c_int) -> Font;
}
extern "C" {
    pub fn LoadFontFromMemory(
        fileType: *const ::std::os::raw::c_char,
        fileData: *const ::std::os::raw::c_uchar,
        dataSize: ::std::os::raw::c_int,
        fontSize: ::std::os::raw::c_int,
        fontChars: *mut ::std::os::raw::c_int,
        charsCount: ::std::os::raw::c_int,
    ) -> Font;
}
extern "C" {
    pub fn LoadFontData(
        fileData: *const ::std::os::raw::c_uchar,
        dataSize: ::std::os::raw::c_int,
        fontSize: ::std::os::raw::c_int,
        fontChars: *mut ::std::os::raw::c_int,
        charsCount: ::std::os::raw::c_int,
        type_: ::std::os::raw::c_int,
    ) -> *mut CharInfo;
}
extern "C" {
    pub fn GenImageFontAtlas(
        chars: *const CharInfo,
        recs: *mut *mut Rectangle,
        charsCount: ::std::os::raw::c_int,
        fontSize: ::std::os::raw::c_int,
        padding: ::std::os::raw::c_int,
        packMethod: ::std::os::raw::c_int,
    ) -> Image;
}
extern "C" {
    pub fn UnloadFontData(chars: *mut CharInfo, charsCount: ::std::os::raw::c_int);
}
extern "C" {
    pub fn UnloadFont(font: Font);
}
extern "C" {
    pub fn DrawFPS(posX: ::std::os::raw::c_int, posY: ::std::os::raw::c_int);
}
extern "C" {
    pub fn DrawText(
        text: *const ::std::os::raw::c_char,
        posX: ::std::os::raw::c_int,
        posY: ::std::os::raw::c_int,
        fontSize: ::std::os::raw::c_int,
        color: Color,
    );
}
extern "C" {
    pub fn DrawTextEx(
        font: Font,
        text: *const ::std::os::raw::c_char,
        position: Vector2,
        fontSize: f32,
        spacing: f32,
        tint: Color,
    );
}
extern "C" {
    pub fn DrawTextRec(
        font: Font,
        text: *const ::std::os::raw::c_char,
        rec: Rectangle,
        fontSize: f32,
        spacing: f32,
        wordWrap: bool,
        tint: Color,
    );
}
extern "C" {
    pub fn DrawTextRecEx(
        font: Font,
        text: *const ::std::os::raw::c_char,
        rec: Rectangle,
        fontSize: f32,
        spacing: f32,
        wordWrap: bool,
        tint: Color,
        selectStart: ::std::os::raw::c_int,
        selectLength: ::std::os::raw::c_int,
        selectTint: Color,
        selectBackTint: Color,
    );
}
extern "C" {
    pub fn DrawTextCodepoint(
        font: Font,
        codepoint: ::std::os::raw::c_int,
        position: Vector2,
        fontSize: f32,
        tint: Color,
    );
}
extern "C" {
    pub fn MeasureText(
        text: *const ::std::os::raw::c_char,
        fontSize: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn MeasureTextEx(
        font: Font,
        text: *const ::std::os::raw::c_char,
        fontSize: f32,
        spacing: f32,
    ) -> Vector2;
}
extern "C" {
    pub fn GetGlyphIndex(font: Font, codepoint: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TextCopy(
        dst: *mut ::std::os::raw::c_char,
        src: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TextIsEqual(
        text1: *const ::std::os::raw::c_char,
        text2: *const ::std::os::raw::c_char,
    ) -> bool;
}
extern "C" {
    pub fn TextLength(text: *const ::std::os::raw::c_char) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn TextFormat(text: *const ::std::os::raw::c_char, ...) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn TextSubtext(
        text: *const ::std::os::raw::c_char,
        position: ::std::os::raw::c_int,
        length: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn TextReplace(
        text: *mut ::std::os::raw::c_char,
        replace: *const ::std::os::raw::c_char,
        by: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn TextInsert(
        text: *const ::std::os::raw::c_char,
        insert: *const ::std::os::raw::c_char,
        position: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn TextJoin(
        textList: *mut *const ::std::os::raw::c_char,
        count: ::std::os::raw::c_int,
        delimiter: *const ::std::os::raw::c_char,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn TextSplit(
        text: *const ::std::os::raw::c_char,
        delimiter: ::std::os::raw::c_char,
        count: *mut ::std::os::raw::c_int,
    ) -> *mut *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn TextAppend(
        text: *mut ::std::os::raw::c_char,
        append: *const ::std::os::raw::c_char,
        position: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn TextFindIndex(
        text: *const ::std::os::raw::c_char,
        find: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TextToUpper(text: *const ::std::os::raw::c_char) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn TextToLower(text: *const ::std::os::raw::c_char) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn TextToPascal(text: *const ::std::os::raw::c_char) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn TextToInteger(text: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TextToUtf8(
        codepoints: *mut ::std::os::raw::c_int,
        length: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn GetCodepoints(
        text: *const ::std::os::raw::c_char,
        count: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_int;
}
extern "C" {
    pub fn GetCodepointsCount(text: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GetNextCodepoint(
        text: *const ::std::os::raw::c_char,
        bytesProcessed: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn CodepointToUtf8(
        codepoint: ::std::os::raw::c_int,
        byteLength: *mut ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn DrawLine3D(startPos: Vector3, endPos: Vector3, color: Color);
}
extern "C" {
    pub fn DrawPoint3D(position: Vector3, color: Color);
}
extern "C" {
    pub fn DrawCircle3D(
        center: Vector3,
        radius: f32,
        rotationAxis: Vector3,
        rotationAngle: f32,
        color: Color,
    );
}
extern "C" {
    pub fn DrawTriangle3D(v1: Vector3, v2: Vector3, v3: Vector3, color: Color);
}
extern "C" {
    pub fn DrawTriangleStrip3D(
        points: *mut Vector3,
        pointsCount: ::std::os::raw::c_int,
        color: Color,
    );
}
extern "C" {
    pub fn DrawCube(position: Vector3, width: f32, height: f32, length: f32, color: Color);
}
extern "C" {
    pub fn DrawCubeV(position: Vector3, size: Vector3, color: Color);
}
extern "C" {
    pub fn DrawCubeWires(position: Vector3, width: f32, height: f32, length: f32, color: Color);
}
extern "C" {
    pub fn DrawCubeWiresV(position: Vector3, size: Vector3, color: Color);
}
extern "C" {
    pub fn DrawCubeTexture(
        texture: Texture2D,
        position: Vector3,
        width: f32,
        height: f32,
        length: f32,
        color: Color,
    );
}
extern "C" {
    pub fn DrawSphere(centerPos: Vector3, radius: f32, color: Color);
}
extern "C" {
    pub fn DrawSphereEx(
        centerPos: Vector3,
        radius: f32,
        rings: ::std::os::raw::c_int,
        slices: ::std::os::raw::c_int,
        color: Color,
    );
}
extern "C" {
    pub fn DrawSphereWires(
        centerPos: Vector3,
        radius: f32,
        rings: ::std::os::raw::c_int,
        slices: ::std::os::raw::c_int,
        color: Color,
    );
}
extern "C" {
    pub fn DrawCylinder(
        position: Vector3,
        radiusTop: f32,
        radiusBottom: f32,
        height: f32,
        slices: ::std::os::raw::c_int,
        color: Color,
    );
}
extern "C" {
    pub fn DrawCylinderWires(
        position: Vector3,
        radiusTop: f32,
        radiusBottom: f32,
        height: f32,
        slices: ::std::os::raw::c_int,
        color: Color,
    );
}
extern "C" {
    pub fn DrawPlane(centerPos: Vector3, size: Vector2, color: Color);
}
extern "C" {
    pub fn DrawRay(ray: Ray, color: Color);
}
extern "C" {
    pub fn DrawGrid(slices: ::std::os::raw::c_int, spacing: f32);
}
extern "C" {
    pub fn LoadModel(fileName: *const ::std::os::raw::c_char) -> Model;
}
extern "C" {
    pub fn LoadModelFromMesh(mesh: Mesh) -> Model;
}
extern "C" {
    pub fn UnloadModel(model: Model);
}
extern "C" {
    pub fn UnloadModelKeepMeshes(model: Model);
}
extern "C" {
    pub fn UploadMesh(mesh: *mut Mesh, dynamic: bool);
}
extern "C" {
    pub fn UpdateMeshBuffer(
        mesh: Mesh,
        index: ::std::os::raw::c_int,
        data: *mut ::std::os::raw::c_void,
        dataSize: ::std::os::raw::c_int,
        offset: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn DrawMesh(mesh: Mesh, material: Material, transform: Matrix);
}
extern "C" {
    pub fn DrawMeshInstanced(
        mesh: Mesh,
        material: Material,
        transforms: *mut Matrix,
        instances: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn UnloadMesh(mesh: Mesh);
}
extern "C" {
    pub fn ExportMesh(mesh: Mesh, fileName: *const ::std::os::raw::c_char) -> bool;
}
extern "C" {
    pub fn LoadMaterials(
        fileName: *const ::std::os::raw::c_char,
        materialCount: *mut ::std::os::raw::c_int,
    ) -> *mut Material;
}
extern "C" {
    pub fn LoadMaterialDefault() -> Material;
}
extern "C" {
    pub fn UnloadMaterial(material: Material);
}
extern "C" {
    pub fn SetMaterialTexture(
        material: *mut Material,
        mapType: ::std::os::raw::c_int,
        texture: Texture2D,
    );
}
extern "C" {
    pub fn SetModelMeshMaterial(
        model: *mut Model,
        meshId: ::std::os::raw::c_int,
        materialId: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn LoadModelAnimations(
        fileName: *const ::std::os::raw::c_char,
        animsCount: *mut ::std::os::raw::c_int,
    ) -> *mut ModelAnimation;
}
extern "C" {
    pub fn UpdateModelAnimation(model: Model, anim: ModelAnimation, frame: ::std::os::raw::c_int);
}
extern "C" {
    pub fn UnloadModelAnimation(anim: ModelAnimation);
}
extern "C" {
    pub fn UnloadModelAnimations(animations: *mut ModelAnimation, count: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn IsModelAnimationValid(model: Model, anim: ModelAnimation) -> bool;
}
extern "C" {
    pub fn GenMeshPoly(sides: ::std::os::raw::c_int, radius: f32) -> Mesh;
}
extern "C" {
    pub fn GenMeshPlane(
        width: f32,
        length: f32,
        resX: ::std::os::raw::c_int,
        resZ: ::std::os::raw::c_int,
    ) -> Mesh;
}
extern "C" {
    pub fn GenMeshCube(width: f32, height: f32, length: f32) -> Mesh;
}
extern "C" {
    pub fn GenMeshSphere(
        radius: f32,
        rings: ::std::os::raw::c_int,
        slices: ::std::os::raw::c_int,
    ) -> Mesh;
}
extern "C" {
    pub fn GenMeshHemiSphere(
        radius: f32,
        rings: ::std::os::raw::c_int,
        slices: ::std::os::raw::c_int,
    ) -> Mesh;
}
extern "C" {
    pub fn GenMeshCylinder(radius: f32, height: f32, slices: ::std::os::raw::c_int) -> Mesh;
}
extern "C" {
    pub fn GenMeshTorus(
        radius: f32,
        size: f32,
        radSeg: ::std::os::raw::c_int,
        sides: ::std::os::raw::c_int,
    ) -> Mesh;
}
extern "C" {
    pub fn GenMeshKnot(
        radius: f32,
        size: f32,
        radSeg: ::std::os::raw::c_int,
        sides: ::std::os::raw::c_int,
    ) -> Mesh;
}
extern "C" {
    pub fn GenMeshHeightmap(heightmap: Image, size: Vector3) -> Mesh;
}
extern "C" {
    pub fn GenMeshCubicmap(cubicmap: Image, cubeSize: Vector3) -> Mesh;
}
extern "C" {
    pub fn MeshBoundingBox(mesh: Mesh) -> BoundingBox;
}
extern "C" {
    pub fn MeshTangents(mesh: *mut Mesh);
}
extern "C" {
    pub fn MeshBinormals(mesh: *mut Mesh);
}
extern "C" {
    pub fn DrawModel(model: Model, position: Vector3, scale: f32, tint: Color);
}
extern "C" {
    pub fn DrawModelEx(
        model: Model,
        position: Vector3,
        rotationAxis: Vector3,
        rotationAngle: f32,
        scale: Vector3,
        tint: Color,
    );
}
extern "C" {
    pub fn DrawModelWires(model: Model, position: Vector3, scale: f32, tint: Color);
}
extern "C" {
    pub fn DrawModelWiresEx(
        model: Model,
        position: Vector3,
        rotationAxis: Vector3,
        rotationAngle: f32,
        scale: Vector3,
        tint: Color,
    );
}
extern "C" {
    pub fn DrawBoundingBox(box_: BoundingBox, color: Color);
}
extern "C" {
    pub fn DrawBillboard(
        camera: Camera,
        texture: Texture2D,
        center: Vector3,
        size: f32,
        tint: Color,
    );
}
extern "C" {
    pub fn DrawBillboardRec(
        camera: Camera,
        texture: Texture2D,
        source: Rectangle,
        center: Vector3,
        size: f32,
        tint: Color,
    );
}
extern "C" {
    pub fn CheckCollisionSpheres(
        center1: Vector3,
        radius1: f32,
        center2: Vector3,
        radius2: f32,
    ) -> bool;
}
extern "C" {
    pub fn CheckCollisionBoxes(box1: BoundingBox, box2: BoundingBox) -> bool;
}
extern "C" {
    pub fn CheckCollisionBoxSphere(box_: BoundingBox, center: Vector3, radius: f32) -> bool;
}
extern "C" {
    pub fn CheckCollisionRaySphere(ray: Ray, center: Vector3, radius: f32) -> bool;
}
extern "C" {
    pub fn CheckCollisionRaySphereEx(
        ray: Ray,
        center: Vector3,
        radius: f32,
        collisionPoint: *mut Vector3,
    ) -> bool;
}
extern "C" {
    pub fn CheckCollisionRayBox(ray: Ray, box_: BoundingBox) -> bool;
}
extern "C" {
    pub fn GetCollisionRayMesh(ray: Ray, mesh: Mesh, transform: Matrix) -> RayHitInfo;
}
extern "C" {
    pub fn GetCollisionRayModel(ray: Ray, model: Model) -> RayHitInfo;
}
extern "C" {
    pub fn GetCollisionRayTriangle(ray: Ray, p1: Vector3, p2: Vector3, p3: Vector3) -> RayHitInfo;
}
extern "C" {
    pub fn GetCollisionRayGround(ray: Ray, groundHeight: f32) -> RayHitInfo;
}
extern "C" {
    pub fn InitAudioDevice();
}
extern "C" {
    pub fn CloseAudioDevice();
}
extern "C" {
    pub fn IsAudioDeviceReady() -> bool;
}
extern "C" {
    pub fn SetMasterVolume(volume: f32);
}
extern "C" {
    pub fn LoadWave(fileName: *const ::std::os::raw::c_char) -> Wave;
}
extern "C" {
    pub fn LoadWaveFromMemory(
        fileType: *const ::std::os::raw::c_char,
        fileData: *const ::std::os::raw::c_uchar,
        dataSize: ::std::os::raw::c_int,
    ) -> Wave;
}
extern "C" {
    pub fn LoadSound(fileName: *const ::std::os::raw::c_char) -> Sound;
}
extern "C" {
    pub fn LoadSoundFromWave(wave: Wave) -> Sound;
}
extern "C" {
    pub fn UpdateSound(
        sound: Sound,
        data: *const ::std::os::raw::c_void,
        samplesCount: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn UnloadWave(wave: Wave);
}
extern "C" {
    pub fn UnloadSound(sound: Sound);
}
extern "C" {
    pub fn ExportWave(wave: Wave, fileName: *const ::std::os::raw::c_char) -> bool;
}
extern "C" {
    pub fn ExportWaveAsCode(wave: Wave, fileName: *const ::std::os::raw::c_char) -> bool;
}
extern "C" {
    pub fn PlaySound(sound: Sound);
}
extern "C" {
    pub fn StopSound(sound: Sound);
}
extern "C" {
    pub fn PauseSound(sound: Sound);
}
extern "C" {
    pub fn ResumeSound(sound: Sound);
}
extern "C" {
    pub fn PlaySoundMulti(sound: Sound);
}
extern "C" {
    pub fn StopSoundMulti();
}
extern "C" {
    pub fn GetSoundsPlaying() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn IsSoundPlaying(sound: Sound) -> bool;
}
extern "C" {
    pub fn SetSoundVolume(sound: Sound, volume: f32);
}
extern "C" {
    pub fn SetSoundPitch(sound: Sound, pitch: f32);
}
extern "C" {
    pub fn WaveFormat(
        wave: *mut Wave,
        sampleRate: ::std::os::raw::c_int,
        sampleSize: ::std::os::raw::c_int,
        channels: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn WaveCopy(wave: Wave) -> Wave;
}
extern "C" {
    pub fn WaveCrop(
        wave: *mut Wave,
        initSample: ::std::os::raw::c_int,
        finalSample: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn LoadWaveSamples(wave: Wave) -> *mut f32;
}
extern "C" {
    pub fn UnloadWaveSamples(samples: *mut f32);
}
extern "C" {
    pub fn LoadMusicStream(fileName: *const ::std::os::raw::c_char) -> Music;
}
extern "C" {
    pub fn LoadMusicStreamFromMemory(
        fileType: *const ::std::os::raw::c_char,
        data: *mut ::std::os::raw::c_uchar,
        dataSize: ::std::os::raw::c_int,
    ) -> Music;
}
extern "C" {
    pub fn UnloadMusicStream(music: Music);
}
extern "C" {
    pub fn PlayMusicStream(music: Music);
}
extern "C" {
    pub fn IsMusicPlaying(music: Music) -> bool;
}
extern "C" {
    pub fn UpdateMusicStream(music: Music);
}
extern "C" {
    pub fn StopMusicStream(music: Music);
}
extern "C" {
    pub fn PauseMusicStream(music: Music);
}
extern "C" {
    pub fn ResumeMusicStream(music: Music);
}
extern "C" {
    pub fn SetMusicVolume(music: Music, volume: f32);
}
extern "C" {
    pub fn SetMusicPitch(music: Music, pitch: f32);
}
extern "C" {
    pub fn GetMusicTimeLength(music: Music) -> f32;
}
extern "C" {
    pub fn GetMusicTimePlayed(music: Music) -> f32;
}
extern "C" {
    pub fn InitAudioStream(
        sampleRate: ::std::os::raw::c_uint,
        sampleSize: ::std::os::raw::c_uint,
        channels: ::std::os::raw::c_uint,
    ) -> AudioStream;
}
extern "C" {
    pub fn UpdateAudioStream(
        stream: AudioStream,
        data: *const ::std::os::raw::c_void,
        samplesCount: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn CloseAudioStream(stream: AudioStream);
}
extern "C" {
    pub fn IsAudioStreamProcessed(stream: AudioStream) -> bool;
}
extern "C" {
    pub fn PlayAudioStream(stream: AudioStream);
}
extern "C" {
    pub fn PauseAudioStream(stream: AudioStream);
}
extern "C" {
    pub fn ResumeAudioStream(stream: AudioStream);
}
extern "C" {
    pub fn IsAudioStreamPlaying(stream: AudioStream) -> bool;
}
extern "C" {
    pub fn StopAudioStream(stream: AudioStream);
}
extern "C" {
    pub fn SetAudioStreamVolume(stream: AudioStream, volume: f32);
}
extern "C" {
    pub fn SetAudioStreamPitch(stream: AudioStream, pitch: f32);
}
extern "C" {
    pub fn SetAudioStreamBufferSizeDefault(size: ::std::os::raw::c_int);
}
extern "C" {
    pub fn __va_start(arg1: *mut *mut ::std::os::raw::c_char, ...);
}
extern "C" {
    pub fn __security_init_cookie();
}
extern "C" {
    pub fn __security_check_cookie(_StackCookie: usize);
}
extern "C" {
    pub fn __report_gsfailure(_StackCookie: usize);
}
extern "C" {
    pub static mut __security_cookie: usize;
}
extern "C" {
    pub fn _invalid_parameter_noinfo();
}
extern "C" {
    pub fn _invalid_parameter_noinfo_noreturn();
}
extern "C" {
    pub fn _invoke_watson(
        _Expression: *const wchar_t,
        _FunctionName: *const wchar_t,
        _FileName: *const wchar_t,
        _LineNo: ::std::os::raw::c_uint,
        _Reserved: usize,
    );
}
extern "C" {
    pub static _HUGE: f64;
}
extern "C" {
    pub fn _fperrraise(_Except: ::std::os::raw::c_int);
}
extern "C" {
    pub fn _dclass(_X: f64) -> ::std::os::raw::c_short;
}
extern "C" {
    pub fn _ldclass(_X: f64) -> ::std::os::raw::c_short;
}
extern "C" {
    pub fn _fdclass(_X: f32) -> ::std::os::raw::c_short;
}
extern "C" {
    pub fn _dsign(_X: f64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _ldsign(_X: f64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _fdsign(_X: f32) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _dpcomp(_X: f64, _Y: f64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _ldpcomp(_X: f64, _Y: f64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _fdpcomp(_X: f32, _Y: f32) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _dtest(_Px: *mut f64) -> ::std::os::raw::c_short;
}
extern "C" {
    pub fn _ldtest(_Px: *mut f64) -> ::std::os::raw::c_short;
}
extern "C" {
    pub fn _fdtest(_Px: *mut f32) -> ::std::os::raw::c_short;
}
extern "C" {
    pub fn _d_int(_Px: *mut f64, _Xexp: ::std::os::raw::c_short) -> ::std::os::raw::c_short;
}
extern "C" {
    pub fn _ld_int(_Px: *mut f64, _Xexp: ::std::os::raw::c_short) -> ::std::os::raw::c_short;
}
extern "C" {
    pub fn _fd_int(_Px: *mut f32, _Xexp: ::std::os::raw::c_short) -> ::std::os::raw::c_short;
}
extern "C" {
    pub fn _dscale(_Px: *mut f64, _Lexp: ::std::os::raw::c_long) -> ::std::os::raw::c_short;
}
extern "C" {
    pub fn _ldscale(_Px: *mut f64, _Lexp: ::std::os::raw::c_long) -> ::std::os::raw::c_short;
}
extern "C" {
    pub fn _fdscale(_Px: *mut f32, _Lexp: ::std::os::raw::c_long) -> ::std::os::raw::c_short;
}
extern "C" {
    pub fn _dunscale(_Pex: *mut ::std::os::raw::c_short, _Px: *mut f64) -> ::std::os::raw::c_short;
}
extern "C" {
    pub fn _ldunscale(_Pex: *mut ::std::os::raw::c_short, _Px: *mut f64)
        -> ::std::os::raw::c_short;
}
extern "C" {
    pub fn _fdunscale(_Pex: *mut ::std::os::raw::c_short, _Px: *mut f32)
        -> ::std::os::raw::c_short;
}
extern "C" {
    pub fn _dexp(_Px: *mut f64, _Y: f64, _Eoff: ::std::os::raw::c_long) -> ::std::os::raw::c_short;
}
extern "C" {
    pub fn _ldexp(_Px: *mut f64, _Y: f64, _Eoff: ::std::os::raw::c_long)
        -> ::std::os::raw::c_short;
}
extern "C" {
    pub fn _fdexp(_Px: *mut f32, _Y: f32, _Eoff: ::std::os::raw::c_long)
        -> ::std::os::raw::c_short;
}
extern "C" {
    pub fn _dnorm(_Ps: *mut ::std::os::raw::c_ushort) -> ::std::os::raw::c_short;
}
extern "C" {
    pub fn _fdnorm(_Ps: *mut ::std::os::raw::c_ushort) -> ::std::os::raw::c_short;
}
extern "C" {
    pub fn _dpoly(_X: f64, _Tab: *const f64, _N: ::std::os::raw::c_int) -> f64;
}
extern "C" {
    pub fn _ldpoly(_X: f64, _Tab: *const f64, _N: ::std::os::raw::c_int) -> f64;
}
extern "C" {
    pub fn _fdpoly(_X: f32, _Tab: *const f32, _N: ::std::os::raw::c_int) -> f32;
}
extern "C" {
    pub fn _dlog(_X: f64, _Baseflag: ::std::os::raw::c_int) -> f64;
}
extern "C" {
    pub fn _ldlog(_X: f64, _Baseflag: ::std::os::raw::c_int) -> f64;
}
extern "C" {
    pub fn _fdlog(_X: f32, _Baseflag: ::std::os::raw::c_int) -> f32;
}
extern "C" {
    pub fn _dsin(_X: f64, _Qoff: ::std::os::raw::c_uint) -> f64;
}
extern "C" {
    pub fn _ldsin(_X: f64, _Qoff: ::std::os::raw::c_uint) -> f64;
}
extern "C" {
    pub fn _fdsin(_X: f32, _Qoff: ::std::os::raw::c_uint) -> f32;
}
extern "C" {
    pub static _Denorm_C: _float_const;
}
extern "C" {
    pub static _Inf_C: _float_const;
}
extern "C" {
    pub static _Nan_C: _float_const;
}
extern "C" {
    pub static _Snan_C: _float_const;
}
extern "C" {
    pub static _Hugeval_C: _float_const;
}
extern "C" {
    pub static _FDenorm_C: _float_const;
}
extern "C" {
    pub static _FInf_C: _float_const;
}
extern "C" {
    pub static _FNan_C: _float_const;
}
extern "C" {
    pub static _FSnan_C: _float_const;
}
extern "C" {
    pub static _LDenorm_C: _float_const;
}
extern "C" {
    pub static _LInf_C: _float_const;
}
extern "C" {
    pub static _LNan_C: _float_const;
}
extern "C" {
    pub static _LSnan_C: _float_const;
}
extern "C" {
    pub static _Eps_C: _float_const;
}
extern "C" {
    pub static _Rteps_C: _float_const;
}
extern "C" {
    pub static _FEps_C: _float_const;
}
extern "C" {
    pub static _FRteps_C: _float_const;
}
extern "C" {
    pub static _LEps_C: _float_const;
}
extern "C" {
    pub static _LRteps_C: _float_const;
}
extern "C" {
    pub static _Zero_C: f64;
}
extern "C" {
    pub static _Xbig_C: f64;
}
extern "C" {
    pub static _FZero_C: f32;
}
extern "C" {
    pub static _FXbig_C: f32;
}
extern "C" {
    pub static _LZero_C: f64;
}
extern "C" {
    pub static _LXbig_C: f64;
}
extern "C" {
    pub fn abs(_X: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn labs(_X: ::std::os::raw::c_long) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn llabs(_X: ::std::os::raw::c_longlong) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn acos(_X: f64) -> f64;
}
extern "C" {
    pub fn asin(_X: f64) -> f64;
}
extern "C" {
    pub fn atan(_X: f64) -> f64;
}
extern "C" {
    pub fn atan2(_Y: f64, _X: f64) -> f64;
}
extern "C" {
    pub fn cos(_X: f64) -> f64;
}
extern "C" {
    pub fn cosh(_X: f64) -> f64;
}
extern "C" {
    pub fn exp(_X: f64) -> f64;
}
extern "C" {
    pub fn fabs(_X: f64) -> f64;
}
extern "C" {
    pub fn fmod(_X: f64, _Y: f64) -> f64;
}
extern "C" {
    pub fn log(_X: f64) -> f64;
}
extern "C" {
    pub fn log10(_X: f64) -> f64;
}
extern "C" {
    pub fn pow(_X: f64, _Y: f64) -> f64;
}
extern "C" {
    pub fn sin(_X: f64) -> f64;
}
extern "C" {
    pub fn sinh(_X: f64) -> f64;
}
extern "C" {
    pub fn sqrt(_X: f64) -> f64;
}
extern "C" {
    pub fn tan(_X: f64) -> f64;
}
extern "C" {
    pub fn tanh(_X: f64) -> f64;
}
extern "C" {
    pub fn acosh(_X: f64) -> f64;
}
extern "C" {
    pub fn asinh(_X: f64) -> f64;
}
extern "C" {
    pub fn atanh(_X: f64) -> f64;
}
extern "C" {
    pub fn atof(_String: *const ::std::os::raw::c_char) -> f64;
}
extern "C" {
    pub fn _atof_l(_String: *const ::std::os::raw::c_char, _Locale: _locale_t) -> f64;
}
extern "C" {
    pub fn _cabs(_Complex_value: _complex) -> f64;
}
extern "C" {
    pub fn cbrt(_X: f64) -> f64;
}
extern "C" {
    pub fn ceil(_X: f64) -> f64;
}
extern "C" {
    pub fn _chgsign(_X: f64) -> f64;
}
extern "C" {
    pub fn copysign(_Number: f64, _Sign: f64) -> f64;
}
extern "C" {
    pub fn _copysign(_Number: f64, _Sign: f64) -> f64;
}
extern "C" {
    pub fn erf(_X: f64) -> f64;
}
extern "C" {
    pub fn erfc(_X: f64) -> f64;
}
extern "C" {
    pub fn exp2(_X: f64) -> f64;
}
extern "C" {
    pub fn expm1(_X: f64) -> f64;
}
extern "C" {
    pub fn fdim(_X: f64, _Y: f64) -> f64;
}
extern "C" {
    pub fn floor(_X: f64) -> f64;
}
extern "C" {
    pub fn fma(_X: f64, _Y: f64, _Z: f64) -> f64;
}
extern "C" {
    pub fn fmax(_X: f64, _Y: f64) -> f64;
}
extern "C" {
    pub fn fmin(_X: f64, _Y: f64) -> f64;
}
extern "C" {
    pub fn frexp(_X: f64, _Y: *mut ::std::os::raw::c_int) -> f64;
}
extern "C" {
    pub fn hypot(_X: f64, _Y: f64) -> f64;
}
extern "C" {
    pub fn _hypot(_X: f64, _Y: f64) -> f64;
}
extern "C" {
    pub fn ilogb(_X: f64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ldexp(_X: f64, _Y: ::std::os::raw::c_int) -> f64;
}
extern "C" {
    pub fn lgamma(_X: f64) -> f64;
}
extern "C" {
    pub fn llrint(_X: f64) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn llround(_X: f64) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn log1p(_X: f64) -> f64;
}
extern "C" {
    pub fn log2(_X: f64) -> f64;
}
extern "C" {
    pub fn logb(_X: f64) -> f64;
}
extern "C" {
    pub fn lrint(_X: f64) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn lround(_X: f64) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn _matherr(_Except: *mut _exception) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn modf(_X: f64, _Y: *mut f64) -> f64;
}
extern "C" {
    pub fn nan(_X: *const ::std::os::raw::c_char) -> f64;
}
extern "C" {
    pub fn nearbyint(_X: f64) -> f64;
}
extern "C" {
    pub fn nextafter(_X: f64, _Y: f64) -> f64;
}
extern "C" {
    pub fn nexttoward(_X: f64, _Y: f64) -> f64;
}
extern "C" {
    pub fn remainder(_X: f64, _Y: f64) -> f64;
}
extern "C" {
    pub fn remquo(_X: f64, _Y: f64, _Z: *mut ::std::os::raw::c_int) -> f64;
}
extern "C" {
    pub fn rint(_X: f64) -> f64;
}
extern "C" {
    pub fn round(_X: f64) -> f64;
}
extern "C" {
    pub fn scalbln(_X: f64, _Y: ::std::os::raw::c_long) -> f64;
}
extern "C" {
    pub fn scalbn(_X: f64, _Y: ::std::os::raw::c_int) -> f64;
}
extern "C" {
    pub fn tgamma(_X: f64) -> f64;
}
extern "C" {
    pub fn trunc(_X: f64) -> f64;
}
extern "C" {
    pub fn _j0(_X: f64) -> f64;
}
extern "C" {
    pub fn _j1(_X: f64) -> f64;
}
extern "C" {
    pub fn _jn(_X: ::std::os::raw::c_int, _Y: f64) -> f64;
}
extern "C" {
    pub fn _y0(_X: f64) -> f64;
}
extern "C" {
    pub fn _y1(_X: f64) -> f64;
}
extern "C" {
    pub fn _yn(_X: ::std::os::raw::c_int, _Y: f64) -> f64;
}
extern "C" {
    pub fn acoshf(_X: f32) -> f32;
}
extern "C" {
    pub fn asinhf(_X: f32) -> f32;
}
extern "C" {
    pub fn atanhf(_X: f32) -> f32;
}
extern "C" {
    pub fn cbrtf(_X: f32) -> f32;
}
extern "C" {
    pub fn _chgsignf(_X: f32) -> f32;
}
extern "C" {
    pub fn copysignf(_Number: f32, _Sign: f32) -> f32;
}
extern "C" {
    pub fn _copysignf(_Number: f32, _Sign: f32) -> f32;
}
extern "C" {
    pub fn erff(_X: f32) -> f32;
}
extern "C" {
    pub fn erfcf(_X: f32) -> f32;
}
extern "C" {
    pub fn expm1f(_X: f32) -> f32;
}
extern "C" {
    pub fn exp2f(_X: f32) -> f32;
}
extern "C" {
    pub fn fdimf(_X: f32, _Y: f32) -> f32;
}
extern "C" {
    pub fn fmaf(_X: f32, _Y: f32, _Z: f32) -> f32;
}
extern "C" {
    pub fn fmaxf(_X: f32, _Y: f32) -> f32;
}
extern "C" {
    pub fn fminf(_X: f32, _Y: f32) -> f32;
}
extern "C" {
    pub fn _hypotf(_X: f32, _Y: f32) -> f32;
}
extern "C" {
    pub fn ilogbf(_X: f32) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lgammaf(_X: f32) -> f32;
}
extern "C" {
    pub fn llrintf(_X: f32) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn llroundf(_X: f32) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn log1pf(_X: f32) -> f32;
}
extern "C" {
    pub fn log2f(_X: f32) -> f32;
}
extern "C" {
    pub fn logbf(_X: f32) -> f32;
}
extern "C" {
    pub fn lrintf(_X: f32) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn lroundf(_X: f32) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn nanf(_X: *const ::std::os::raw::c_char) -> f32;
}
extern "C" {
    pub fn nearbyintf(_X: f32) -> f32;
}
extern "C" {
    pub fn nextafterf(_X: f32, _Y: f32) -> f32;
}
extern "C" {
    pub fn nexttowardf(_X: f32, _Y: f64) -> f32;
}
extern "C" {
    pub fn remainderf(_X: f32, _Y: f32) -> f32;
}
extern "C" {
    pub fn remquof(_X: f32, _Y: f32, _Z: *mut ::std::os::raw::c_int) -> f32;
}
extern "C" {
    pub fn rintf(_X: f32) -> f32;
}
extern "C" {
    pub fn roundf(_X: f32) -> f32;
}
extern "C" {
    pub fn scalblnf(_X: f32, _Y: ::std::os::raw::c_long) -> f32;
}
extern "C" {
    pub fn scalbnf(_X: f32, _Y: ::std::os::raw::c_int) -> f32;
}
extern "C" {
    pub fn tgammaf(_X: f32) -> f32;
}
extern "C" {
    pub fn truncf(_X: f32) -> f32;
}
extern "C" {
    pub fn _logbf(_X: f32) -> f32;
}
extern "C" {
    pub fn _nextafterf(_X: f32, _Y: f32) -> f32;
}
extern "C" {
    pub fn _finitef(_X: f32) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _isnanf(_X: f32) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _fpclassf(_X: f32) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _set_FMA3_enable(_Flag: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _get_FMA3_enable() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn acosf(_X: f32) -> f32;
}
extern "C" {
    pub fn asinf(_X: f32) -> f32;
}
extern "C" {
    pub fn atan2f(_Y: f32, _X: f32) -> f32;
}
extern "C" {
    pub fn atanf(_X: f32) -> f32;
}
extern "C" {
    pub fn ceilf(_X: f32) -> f32;
}
extern "C" {
    pub fn cosf(_X: f32) -> f32;
}
extern "C" {
    pub fn coshf(_X: f32) -> f32;
}
extern "C" {
    pub fn expf(_X: f32) -> f32;
}
extern "C" {
    pub fn floorf(_X: f32) -> f32;
}
extern "C" {
    pub fn fmodf(_X: f32, _Y: f32) -> f32;
}
extern "C" {
    pub fn log10f(_X: f32) -> f32;
}
extern "C" {
    pub fn logf(_X: f32) -> f32;
}
extern "C" {
    pub fn modff(_X: f32, _Y: *mut f32) -> f32;
}
extern "C" {
    pub fn powf(_X: f32, _Y: f32) -> f32;
}
extern "C" {
    pub fn sinf(_X: f32) -> f32;
}
extern "C" {
    pub fn sinhf(_X: f32) -> f32;
}
extern "C" {
    pub fn sqrtf(_X: f32) -> f32;
}
extern "C" {
    pub fn tanf(_X: f32) -> f32;
}
extern "C" {
    pub fn tanhf(_X: f32) -> f32;
}
extern "C" {
    pub fn acoshl(_X: f64) -> f64;
}
extern "C" {
    pub fn asinhl(_X: f64) -> f64;
}
extern "C" {
    pub fn atanhl(_X: f64) -> f64;
}
extern "C" {
    pub fn cbrtl(_X: f64) -> f64;
}
extern "C" {
    pub fn copysignl(_Number: f64, _Sign: f64) -> f64;
}
extern "C" {
    pub fn erfl(_X: f64) -> f64;
}
extern "C" {
    pub fn erfcl(_X: f64) -> f64;
}
extern "C" {
    pub fn exp2l(_X: f64) -> f64;
}
extern "C" {
    pub fn expm1l(_X: f64) -> f64;
}
extern "C" {
    pub fn fdiml(_X: f64, _Y: f64) -> f64;
}
extern "C" {
    pub fn fmal(_X: f64, _Y: f64, _Z: f64) -> f64;
}
extern "C" {
    pub fn fmaxl(_X: f64, _Y: f64) -> f64;
}
extern "C" {
    pub fn fminl(_X: f64, _Y: f64) -> f64;
}
extern "C" {
    pub fn ilogbl(_X: f64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lgammal(_X: f64) -> f64;
}
extern "C" {
    pub fn llrintl(_X: f64) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn llroundl(_X: f64) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn log1pl(_X: f64) -> f64;
}
extern "C" {
    pub fn log2l(_X: f64) -> f64;
}
extern "C" {
    pub fn logbl(_X: f64) -> f64;
}
extern "C" {
    pub fn lrintl(_X: f64) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn lroundl(_X: f64) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn nanl(_X: *const ::std::os::raw::c_char) -> f64;
}
extern "C" {
    pub fn nearbyintl(_X: f64) -> f64;
}
extern "C" {
    pub fn nextafterl(_X: f64, _Y: f64) -> f64;
}
extern "C" {
    pub fn nexttowardl(_X: f64, _Y: f64) -> f64;
}
extern "C" {
    pub fn remainderl(_X: f64, _Y: f64) -> f64;
}
extern "C" {
    pub fn remquol(_X: f64, _Y: f64, _Z: *mut ::std::os::raw::c_int) -> f64;
}
extern "C" {
    pub fn rintl(_X: f64) -> f64;
}
extern "C" {
    pub fn roundl(_X: f64) -> f64;
}
extern "C" {
    pub fn scalblnl(_X: f64, _Y: ::std::os::raw::c_long) -> f64;
}
extern "C" {
    pub fn scalbnl(_X: f64, _Y: ::std::os::raw::c_int) -> f64;
}
extern "C" {
    pub fn tgammal(_X: f64) -> f64;
}
extern "C" {
    pub fn truncl(_X: f64) -> f64;
}
extern "C" {
    pub static mut HUGE: f64;
}
extern "C" {
    pub fn j0(_X: f64) -> f64;
}
extern "C" {
    pub fn j1(_X: f64) -> f64;
}
extern "C" {
    pub fn jn(_X: ::std::os::raw::c_int, _Y: f64) -> f64;
}
extern "C" {
    pub fn y0(_X: f64) -> f64;
}
extern "C" {
    pub fn y1(_X: f64) -> f64;
}
extern "C" {
    pub fn yn(_X: ::std::os::raw::c_int, _Y: f64) -> f64;
}
extern "C" {
    pub fn rlMatrixMode(mode: ::std::os::raw::c_int);
}
extern "C" {
    pub fn rlPushMatrix();
}
extern "C" {
    pub fn rlPopMatrix();
}
extern "C" {
    pub fn rlLoadIdentity();
}
extern "C" {
    pub fn rlTranslatef(x: f32, y: f32, z: f32);
}
extern "C" {
    pub fn rlRotatef(angleDeg: f32, x: f32, y: f32, z: f32);
}
extern "C" {
    pub fn rlScalef(x: f32, y: f32, z: f32);
}
extern "C" {
    pub fn rlMultMatrixf(matf: *mut f32);
}
extern "C" {
    pub fn rlFrustum(left: f64, right: f64, bottom: f64, top: f64, znear: f64, zfar: f64);
}
extern "C" {
    pub fn rlOrtho(left: f64, right: f64, bottom: f64, top: f64, znear: f64, zfar: f64);
}
extern "C" {
    pub fn rlViewport(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn rlBegin(mode: ::std::os::raw::c_int);
}
extern "C" {
    pub fn rlEnd();
}
extern "C" {
    pub fn rlVertex2i(x: ::std::os::raw::c_int, y: ::std::os::raw::c_int);
}
extern "C" {
    pub fn rlVertex2f(x: f32, y: f32);
}
extern "C" {
    pub fn rlVertex3f(x: f32, y: f32, z: f32);
}
extern "C" {
    pub fn rlTexCoord2f(x: f32, y: f32);
}
extern "C" {
    pub fn rlNormal3f(x: f32, y: f32, z: f32);
}
extern "C" {
    pub fn rlColor4ub(
        r: ::std::os::raw::c_uchar,
        g: ::std::os::raw::c_uchar,
        b: ::std::os::raw::c_uchar,
        a: ::std::os::raw::c_uchar,
    );
}
extern "C" {
    pub fn rlColor3f(x: f32, y: f32, z: f32);
}
extern "C" {
    pub fn rlColor4f(x: f32, y: f32, z: f32, w: f32);
}
extern "C" {
    pub fn rlEnableVertexArray(vaoId: ::std::os::raw::c_uint) -> bool;
}
extern "C" {
    pub fn rlDisableVertexArray();
}
extern "C" {
    pub fn rlEnableVertexBuffer(id: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn rlDisableVertexBuffer();
}
extern "C" {
    pub fn rlEnableVertexBufferElement(id: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn rlDisableVertexBufferElement();
}
extern "C" {
    pub fn rlEnableVertexAttribute(index: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn rlDisableVertexAttribute(index: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn rlActiveTextureSlot(slot: ::std::os::raw::c_int);
}
extern "C" {
    pub fn rlEnableTexture(id: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn rlDisableTexture();
}
extern "C" {
    pub fn rlEnableTextureCubemap(id: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn rlDisableTextureCubemap();
}
extern "C" {
    pub fn rlTextureParameters(
        id: ::std::os::raw::c_uint,
        param: ::std::os::raw::c_int,
        value: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn rlEnableShader(id: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn rlDisableShader();
}
extern "C" {
    pub fn rlEnableFramebuffer(id: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn rlDisableFramebuffer();
}
extern "C" {
    pub fn rlEnableDepthTest();
}
extern "C" {
    pub fn rlDisableDepthTest();
}
extern "C" {
    pub fn rlEnableDepthMask();
}
extern "C" {
    pub fn rlDisableDepthMask();
}
extern "C" {
    pub fn rlEnableBackfaceCulling();
}
extern "C" {
    pub fn rlDisableBackfaceCulling();
}
extern "C" {
    pub fn rlEnableScissorTest();
}
extern "C" {
    pub fn rlDisableScissorTest();
}
extern "C" {
    pub fn rlScissor(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn rlEnableWireMode();
}
extern "C" {
    pub fn rlDisableWireMode();
}
extern "C" {
    pub fn rlSetLineWidth(width: f32);
}
extern "C" {
    pub fn rlGetLineWidth() -> f32;
}
extern "C" {
    pub fn rlEnableSmoothLines();
}
extern "C" {
    pub fn rlDisableSmoothLines();
}
extern "C" {
    pub fn rlEnableStereoRender();
}
extern "C" {
    pub fn rlDisableStereoRender();
}
extern "C" {
    pub fn rlIsStereoRenderEnabled() -> bool;
}
extern "C" {
    pub fn rlClearColor(
        r: ::std::os::raw::c_uchar,
        g: ::std::os::raw::c_uchar,
        b: ::std::os::raw::c_uchar,
        a: ::std::os::raw::c_uchar,
    );
}
extern "C" {
    pub fn rlClearScreenBuffers();
}
extern "C" {
    pub fn rlCheckErrors();
}
extern "C" {
    pub fn rlSetBlendMode(mode: ::std::os::raw::c_int);
}
extern "C" {
    pub fn rlSetBlendFactors(
        glSrcFactor: ::std::os::raw::c_int,
        glDstFactor: ::std::os::raw::c_int,
        glEquation: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn rlglInit(width: ::std::os::raw::c_int, height: ::std::os::raw::c_int);
}
extern "C" {
    pub fn rlglClose();
}
extern "C" {
    pub fn rlLoadExtensions(loader: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn rlGetVersion() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rlGetFramebufferWidth() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rlGetFramebufferHeight() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rlGetShaderDefault() -> Shader;
}
extern "C" {
    pub fn rlGetTextureDefault() -> Texture2D;
}
extern "C" {
    pub fn rlLoadRenderBatch(
        numBuffers: ::std::os::raw::c_int,
        bufferElements: ::std::os::raw::c_int,
    ) -> RenderBatch;
}
extern "C" {
    pub fn rlUnloadRenderBatch(batch: RenderBatch);
}
extern "C" {
    pub fn rlDrawRenderBatch(batch: *mut RenderBatch);
}
extern "C" {
    pub fn rlSetRenderBatchActive(batch: *mut RenderBatch);
}
extern "C" {
    pub fn rlDrawRenderBatchActive();
}
extern "C" {
    pub fn rlCheckRenderBatchLimit(vCount: ::std::os::raw::c_int) -> bool;
}
extern "C" {
    pub fn rlSetTexture(id: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn rlLoadVertexArray() -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn rlLoadVertexBuffer(
        buffer: *mut ::std::os::raw::c_void,
        size: ::std::os::raw::c_int,
        dynamic: bool,
    ) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn rlLoadVertexBufferElement(
        buffer: *mut ::std::os::raw::c_void,
        size: ::std::os::raw::c_int,
        dynamic: bool,
    ) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn rlUpdateVertexBuffer(
        bufferId: ::std::os::raw::c_int,
        data: *mut ::std::os::raw::c_void,
        dataSize: ::std::os::raw::c_int,
        offset: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn rlUnloadVertexArray(vaoId: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn rlUnloadVertexBuffer(vboId: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn rlSetVertexAttribute(
        index: ::std::os::raw::c_uint,
        compSize: ::std::os::raw::c_int,
        type_: ::std::os::raw::c_int,
        normalized: bool,
        stride: ::std::os::raw::c_int,
        pointer: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn rlSetVertexAttributeDivisor(
        index: ::std::os::raw::c_uint,
        divisor: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn rlSetVertexAttributeDefault(
        locIndex: ::std::os::raw::c_int,
        value: *const ::std::os::raw::c_void,
        attribType: ::std::os::raw::c_int,
        count: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn rlDrawVertexArray(offset: ::std::os::raw::c_int, count: ::std::os::raw::c_int);
}
extern "C" {
    pub fn rlDrawVertexArrayElements(
        offset: ::std::os::raw::c_int,
        count: ::std::os::raw::c_int,
        buffer: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn rlDrawVertexArrayInstanced(
        offset: ::std::os::raw::c_int,
        count: ::std::os::raw::c_int,
        instances: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn rlDrawVertexArrayElementsInstanced(
        offset: ::std::os::raw::c_int,
        count: ::std::os::raw::c_int,
        buffer: *mut ::std::os::raw::c_void,
        instances: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn rlLoadTexture(
        data: *mut ::std::os::raw::c_void,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        format: ::std::os::raw::c_int,
        mipmapCount: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn rlLoadTextureDepth(
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        useRenderBuffer: bool,
    ) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn rlLoadTextureCubemap(
        data: *mut ::std::os::raw::c_void,
        size: ::std::os::raw::c_int,
        format: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn rlUpdateTexture(
        id: ::std::os::raw::c_uint,
        offsetX: ::std::os::raw::c_int,
        offsetY: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        format: ::std::os::raw::c_int,
        data: *const ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn rlGetGlTextureFormats(
        format: ::std::os::raw::c_int,
        glInternalFormat: *mut ::std::os::raw::c_uint,
        glFormat: *mut ::std::os::raw::c_uint,
        glType: *mut ::std::os::raw::c_uint,
    );
}
extern "C" {
    pub fn rlUnloadTexture(id: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn rlGenerateMipmaps(texture: *mut Texture2D);
}
extern "C" {
    pub fn rlReadTexturePixels(texture: Texture2D) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn rlReadScreenPixels(
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_uchar;
}
extern "C" {
    pub fn rlLoadFramebuffer(
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn rlFramebufferAttach(
        fboId: ::std::os::raw::c_uint,
        texId: ::std::os::raw::c_uint,
        attachType: ::std::os::raw::c_int,
        texType: ::std::os::raw::c_int,
        mipLevel: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn rlFramebufferComplete(id: ::std::os::raw::c_uint) -> bool;
}
extern "C" {
    pub fn rlUnloadFramebuffer(id: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn rlLoadShaderCode(
        vsCode: *const ::std::os::raw::c_char,
        fsCode: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn rlCompileShader(
        shaderCode: *const ::std::os::raw::c_char,
        type_: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn rlLoadShaderProgram(
        vShaderId: ::std::os::raw::c_uint,
        fShaderId: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn rlUnloadShaderProgram(id: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn rlGetLocationUniform(
        shaderId: ::std::os::raw::c_uint,
        uniformName: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rlGetLocationAttrib(
        shaderId: ::std::os::raw::c_uint,
        attribName: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rlSetUniform(
        locIndex: ::std::os::raw::c_int,
        value: *const ::std::os::raw::c_void,
        uniformType: ::std::os::raw::c_int,
        count: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn rlSetUniformMatrix(locIndex: ::std::os::raw::c_int, mat: Matrix);
}
extern "C" {
    pub fn rlSetUniformSampler(locIndex: ::std::os::raw::c_int, textureId: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn rlSetShader(shader: Shader);
}
extern "C" {
    pub fn rlGetMatrixModelview() -> Matrix;
}
extern "C" {
    pub fn rlGetMatrixProjection() -> Matrix;
}
extern "C" {
    pub fn rlGetMatrixTransform() -> Matrix;
}
extern "C" {
    pub fn rlGetMatrixProjectionStereo(eye: ::std::os::raw::c_int) -> Matrix;
}
extern "C" {
    pub fn rlGetMatrixViewOffsetStereo(eye: ::std::os::raw::c_int) -> Matrix;
}
extern "C" {
    pub fn rlSetMatrixProjection(proj: Matrix);
}
extern "C" {
    pub fn rlSetMatrixModelview(view: Matrix);
}
extern "C" {
    pub fn rlSetMatrixProjectionStereo(right: Matrix, left: Matrix);
}
extern "C" {
    pub fn rlSetMatrixViewOffsetStereo(right: Matrix, left: Matrix);
}
extern "C" {
    pub fn rlLoadDrawCube();
}
extern "C" {
    pub fn rlLoadDrawQuad();
}
extern "C" {
    pub fn _calloc_base(_Count: size_t, _Size: size_t) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn calloc(
        _Count: ::std::os::raw::c_ulonglong,
        _Size: ::std::os::raw::c_ulonglong,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn _callnewh(_Size: size_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _expand(
        _Block: *mut ::std::os::raw::c_void,
        _Size: size_t,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn _free_base(_Block: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn free(_Block: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn _malloc_base(_Size: size_t) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn malloc(_Size: ::std::os::raw::c_ulonglong) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn _msize_base(_Block: *mut ::std::os::raw::c_void) -> size_t;
}
extern "C" {
    pub fn _msize(_Block: *mut ::std::os::raw::c_void) -> size_t;
}
extern "C" {
    pub fn _realloc_base(
        _Block: *mut ::std::os::raw::c_void,
        _Size: size_t,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn realloc(
        _Block: *mut ::std::os::raw::c_void,
        _Size: ::std::os::raw::c_ulonglong,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn _recalloc_base(
        _Block: *mut ::std::os::raw::c_void,
        _Count: size_t,
        _Size: size_t,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn _recalloc(
        _Block: *mut ::std::os::raw::c_void,
        _Count: size_t,
        _Size: size_t,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn _aligned_free(_Block: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn _aligned_malloc(_Size: size_t, _Alignment: size_t) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn _aligned_offset_malloc(
        _Size: size_t,
        _Alignment: size_t,
        _Offset: size_t,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn _aligned_msize(
        _Block: *mut ::std::os::raw::c_void,
        _Alignment: size_t,
        _Offset: size_t,
    ) -> size_t;
}
extern "C" {
    pub fn _aligned_offset_realloc(
        _Block: *mut ::std::os::raw::c_void,
        _Size: size_t,
        _Alignment: size_t,
        _Offset: size_t,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn _aligned_offset_recalloc(
        _Block: *mut ::std::os::raw::c_void,
        _Count: size_t,
        _Size: size_t,
        _Alignment: size_t,
        _Offset: size_t,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn _aligned_realloc(
        _Block: *mut ::std::os::raw::c_void,
        _Size: size_t,
        _Alignment: size_t,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn _aligned_recalloc(
        _Block: *mut ::std::os::raw::c_void,
        _Count: size_t,
        _Size: size_t,
        _Alignment: size_t,
    ) -> *mut ::std::os::raw::c_void;
}
pub type max_align_t = f64;
pub type _CoreCrtSecureSearchSortCompareFunction = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut ::std::os::raw::c_void,
        arg2: *const ::std::os::raw::c_void,
        arg3: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int,
>;
pub type _CoreCrtNonSecureSearchSortCompareFunction = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *const ::std::os::raw::c_void,
        arg2: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int,
>;
extern "C" {
    pub fn bsearch_s(
        _Key: *const ::std::os::raw::c_void,
        _Base: *const ::std::os::raw::c_void,
        _NumOfElements: rsize_t,
        _SizeOfElements: rsize_t,
        _CompareFunction: _CoreCrtSecureSearchSortCompareFunction,
        _Context: *mut ::std::os::raw::c_void,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn qsort_s(
        _Base: *mut ::std::os::raw::c_void,
        _NumOfElements: rsize_t,
        _SizeOfElements: rsize_t,
        _CompareFunction: _CoreCrtSecureSearchSortCompareFunction,
        _Context: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn bsearch(
        _Key: *const ::std::os::raw::c_void,
        _Base: *const ::std::os::raw::c_void,
        _NumOfElements: size_t,
        _SizeOfElements: size_t,
        _CompareFunction: _CoreCrtNonSecureSearchSortCompareFunction,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn qsort(
        _Base: *mut ::std::os::raw::c_void,
        _NumOfElements: size_t,
        _SizeOfElements: size_t,
        _CompareFunction: _CoreCrtNonSecureSearchSortCompareFunction,
    );
}
extern "C" {
    pub fn _lfind_s(
        _Key: *const ::std::os::raw::c_void,
        _Base: *const ::std::os::raw::c_void,
        _NumOfElements: *mut ::std::os::raw::c_uint,
        _SizeOfElements: size_t,
        _CompareFunction: _CoreCrtSecureSearchSortCompareFunction,
        _Context: *mut ::std::os::raw::c_void,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn _lfind(
        _Key: *const ::std::os::raw::c_void,
        _Base: *const ::std::os::raw::c_void,
        _NumOfElements: *mut ::std::os::raw::c_uint,
        _SizeOfElements: ::std::os::raw::c_uint,
        _CompareFunction: _CoreCrtNonSecureSearchSortCompareFunction,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn _lsearch_s(
        _Key: *const ::std::os::raw::c_void,
        _Base: *mut ::std::os::raw::c_void,
        _NumOfElements: *mut ::std::os::raw::c_uint,
        _SizeOfElements: size_t,
        _CompareFunction: _CoreCrtSecureSearchSortCompareFunction,
        _Context: *mut ::std::os::raw::c_void,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn _lsearch(
        _Key: *const ::std::os::raw::c_void,
        _Base: *mut ::std::os::raw::c_void,
        _NumOfElements: *mut ::std::os::raw::c_uint,
        _SizeOfElements: ::std::os::raw::c_uint,
        _CompareFunction: _CoreCrtNonSecureSearchSortCompareFunction,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn lfind(
        _Key: *const ::std::os::raw::c_void,
        _Base: *const ::std::os::raw::c_void,
        _NumOfElements: *mut ::std::os::raw::c_uint,
        _SizeOfElements: ::std::os::raw::c_uint,
        _CompareFunction: _CoreCrtNonSecureSearchSortCompareFunction,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn lsearch(
        _Key: *const ::std::os::raw::c_void,
        _Base: *mut ::std::os::raw::c_void,
        _NumOfElements: *mut ::std::os::raw::c_uint,
        _SizeOfElements: ::std::os::raw::c_uint,
        _CompareFunction: _CoreCrtNonSecureSearchSortCompareFunction,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn _itow_s(
        _Value: ::std::os::raw::c_int,
        _Buffer: *mut wchar_t,
        _BufferCount: size_t,
        _Radix: ::std::os::raw::c_int,
    ) -> errno_t;
}
extern "C" {
    pub fn _itow(
        _Value: ::std::os::raw::c_int,
        _Buffer: *mut wchar_t,
        _Radix: ::std::os::raw::c_int,
    ) -> *mut wchar_t;
}
extern "C" {
    pub fn _ltow_s(
        _Value: ::std::os::raw::c_long,
        _Buffer: *mut wchar_t,
        _BufferCount: size_t,
        _Radix: ::std::os::raw::c_int,
    ) -> errno_t;
}
extern "C" {
    pub fn _ltow(
        _Value: ::std::os::raw::c_long,
        _Buffer: *mut wchar_t,
        _Radix: ::std::os::raw::c_int,
    ) -> *mut wchar_t;
}
extern "C" {
    pub fn _ultow_s(
        _Value: ::std::os::raw::c_ulong,
        _Buffer: *mut wchar_t,
        _BufferCount: size_t,
        _Radix: ::std::os::raw::c_int,
    ) -> errno_t;
}
extern "C" {
    pub fn _ultow(
        _Value: ::std::os::raw::c_ulong,
        _Buffer: *mut wchar_t,
        _Radix: ::std::os::raw::c_int,
    ) -> *mut wchar_t;
}
extern "C" {
    pub fn wcstod(_String: *const wchar_t, _EndPtr: *mut *mut wchar_t) -> f64;
}
extern "C" {
    pub fn _wcstod_l(
        _String: *const wchar_t,
        _EndPtr: *mut *mut wchar_t,
        _Locale: _locale_t,
    ) -> f64;
}
extern "C" {
    pub fn wcstol(
        _String: *const wchar_t,
        _EndPtr: *mut *mut wchar_t,
        _Radix: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn _wcstol_l(
        _String: *const wchar_t,
        _EndPtr: *mut *mut wchar_t,
        _Radix: ::std::os::raw::c_int,
        _Locale: _locale_t,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn wcstoll(
        _String: *const wchar_t,
        _EndPtr: *mut *mut wchar_t,
        _Radix: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn _wcstoll_l(
        _String: *const wchar_t,
        _EndPtr: *mut *mut wchar_t,
        _Radix: ::std::os::raw::c_int,
        _Locale: _locale_t,
    ) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn wcstoul(
        _String: *const wchar_t,
        _EndPtr: *mut *mut wchar_t,
        _Radix: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn _wcstoul_l(
        _String: *const wchar_t,
        _EndPtr: *mut *mut wchar_t,
        _Radix: ::std::os::raw::c_int,
        _Locale: _locale_t,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn wcstoull(
        _String: *const wchar_t,
        _EndPtr: *mut *mut wchar_t,
        _Radix: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_ulonglong;
}
extern "C" {
    pub fn _wcstoull_l(
        _String: *const wchar_t,
        _EndPtr: *mut *mut wchar_t,
        _Radix: ::std::os::raw::c_int,
        _Locale: _locale_t,
    ) -> ::std::os::raw::c_ulonglong;
}
extern "C" {
    pub fn wcstold(_String: *const wchar_t, _EndPtr: *mut *mut wchar_t) -> f64;
}
extern "C" {
    pub fn _wcstold_l(
        _String: *const wchar_t,
        _EndPtr: *mut *mut wchar_t,
        _Locale: _locale_t,
    ) -> f64;
}
extern "C" {
    pub fn wcstof(_String: *const wchar_t, _EndPtr: *mut *mut wchar_t) -> f32;
}
extern "C" {
    pub fn _wcstof_l(
        _String: *const wchar_t,
        _EndPtr: *mut *mut wchar_t,
        _Locale: _locale_t,
    ) -> f32;
}
extern "C" {
    pub fn _wtof(_String: *const wchar_t) -> f64;
}
extern "C" {
    pub fn _wtof_l(_String: *const wchar_t, _Locale: _locale_t) -> f64;
}
extern "C" {
    pub fn _wtoi(_String: *const wchar_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _wtoi_l(_String: *const wchar_t, _Locale: _locale_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _wtol(_String: *const wchar_t) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn _wtol_l(_String: *const wchar_t, _Locale: _locale_t) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn _wtoll(_String: *const wchar_t) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn _wtoll_l(_String: *const wchar_t, _Locale: _locale_t) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn _i64tow_s(
        _Value: ::std::os::raw::c_longlong,
        _Buffer: *mut wchar_t,
        _BufferCount: size_t,
        _Radix: ::std::os::raw::c_int,
    ) -> errno_t;
}
extern "C" {
    pub fn _i64tow(
        _Value: ::std::os::raw::c_longlong,
        _Buffer: *mut wchar_t,
        _Radix: ::std::os::raw::c_int,
    ) -> *mut wchar_t;
}
extern "C" {
    pub fn _ui64tow_s(
        _Value: ::std::os::raw::c_ulonglong,
        _Buffer: *mut wchar_t,
        _BufferCount: size_t,
        _Radix: ::std::os::raw::c_int,
    ) -> errno_t;
}
extern "C" {
    pub fn _ui64tow(
        _Value: ::std::os::raw::c_ulonglong,
        _Buffer: *mut wchar_t,
        _Radix: ::std::os::raw::c_int,
    ) -> *mut wchar_t;
}
extern "C" {
    pub fn _wtoi64(_String: *const wchar_t) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn _wtoi64_l(_String: *const wchar_t, _Locale: _locale_t) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn _wcstoi64(
        _String: *const wchar_t,
        _EndPtr: *mut *mut wchar_t,
        _Radix: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn _wcstoi64_l(
        _String: *const wchar_t,
        _EndPtr: *mut *mut wchar_t,
        _Radix: ::std::os::raw::c_int,
        _Locale: _locale_t,
    ) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn _wcstoui64(
        _String: *const wchar_t,
        _EndPtr: *mut *mut wchar_t,
        _Radix: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_ulonglong;
}
extern "C" {
    pub fn _wcstoui64_l(
        _String: *const wchar_t,
        _EndPtr: *mut *mut wchar_t,
        _Radix: ::std::os::raw::c_int,
        _Locale: _locale_t,
    ) -> ::std::os::raw::c_ulonglong;
}
extern "C" {
    pub fn _wfullpath(
        _Buffer: *mut wchar_t,
        _Path: *const wchar_t,
        _BufferCount: size_t,
    ) -> *mut wchar_t;
}
extern "C" {
    pub fn _wmakepath_s(
        _Buffer: *mut wchar_t,
        _BufferCount: size_t,
        _Drive: *const wchar_t,
        _Dir: *const wchar_t,
        _Filename: *const wchar_t,
        _Ext: *const wchar_t,
    ) -> errno_t;
}
extern "C" {
    pub fn _wmakepath(
        _Buffer: *mut wchar_t,
        _Drive: *const wchar_t,
        _Dir: *const wchar_t,
        _Filename: *const wchar_t,
        _Ext: *const wchar_t,
    );
}
extern "C" {
    pub fn _wperror(_ErrorMessage: *const wchar_t);
}
extern "C" {
    pub fn _wsplitpath(
        _FullPath: *const wchar_t,
        _Drive: *mut wchar_t,
        _Dir: *mut wchar_t,
        _Filename: *mut wchar_t,
        _Ext: *mut wchar_t,
    );
}
extern "C" {
    pub fn _wsplitpath_s(
        _FullPath: *const wchar_t,
        _Drive: *mut wchar_t,
        _DriveCount: size_t,
        _Dir: *mut wchar_t,
        _DirCount: size_t,
        _Filename: *mut wchar_t,
        _FilenameCount: size_t,
        _Ext: *mut wchar_t,
        _ExtCount: size_t,
    ) -> errno_t;
}
extern "C" {
    pub fn _wdupenv_s(
        _Buffer: *mut *mut wchar_t,
        _BufferCount: *mut size_t,
        _VarName: *const wchar_t,
    ) -> errno_t;
}
extern "C" {
    pub fn _wgetenv(_VarName: *const wchar_t) -> *mut wchar_t;
}
extern "C" {
    pub fn _wgetenv_s(
        _RequiredCount: *mut size_t,
        _Buffer: *mut wchar_t,
        _BufferCount: size_t,
        _VarName: *const wchar_t,
    ) -> errno_t;
}
extern "C" {
    pub fn _wputenv(_EnvString: *const wchar_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _wputenv_s(_Name: *const wchar_t, _Value: *const wchar_t) -> errno_t;
}
extern "C" {
    pub fn _wsearchenv_s(
        _Filename: *const wchar_t,
        _VarName: *const wchar_t,
        _Buffer: *mut wchar_t,
        _BufferCount: size_t,
    ) -> errno_t;
}
extern "C" {
    pub fn _wsearchenv(
        _Filename: *const wchar_t,
        _VarName: *const wchar_t,
        _ResultPath: *mut wchar_t,
    );
}
extern "C" {
    pub fn _wsystem(_Command: *const wchar_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _swab(
        _Buf1: *mut ::std::os::raw::c_char,
        _Buf2: *mut ::std::os::raw::c_char,
        _SizeInBytes: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn exit(_Code: ::std::os::raw::c_int);
}
extern "C" {
    pub fn _exit(_Code: ::std::os::raw::c_int);
}
extern "C" {
    pub fn _Exit(_Code: ::std::os::raw::c_int);
}
extern "C" {
    pub fn quick_exit(_Code: ::std::os::raw::c_int);
}
extern "C" {
    pub fn abort();
}
extern "C" {
    pub fn _set_abort_behavior(
        _Flags: ::std::os::raw::c_uint,
        _Mask: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_uint;
}
pub type _onexit_t = ::std::option::Option<unsafe extern "C" fn() -> ::std::os::raw::c_int>;
extern "C" {
    pub fn atexit(arg1: ::std::option::Option<unsafe extern "C" fn()>) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _onexit(_Func: _onexit_t) -> _onexit_t;
}
extern "C" {
    pub fn at_quick_exit(
        arg1: ::std::option::Option<unsafe extern "C" fn()>,
    ) -> ::std::os::raw::c_int;
}
pub type _purecall_handler = ::std::option::Option<unsafe extern "C" fn()>;
pub type _invalid_parameter_handler = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *const wchar_t,
        arg2: *const wchar_t,
        arg3: *const wchar_t,
        arg4: ::std::os::raw::c_uint,
        arg5: usize,
    ),
>;
extern "C" {
    pub fn _set_purecall_handler(_Handler: _purecall_handler) -> _purecall_handler;
}
extern "C" {
    pub fn _get_purecall_handler() -> _purecall_handler;
}
extern "C" {
    pub fn _set_invalid_parameter_handler(
        _Handler: _invalid_parameter_handler,
    ) -> _invalid_parameter_handler;
}
extern "C" {
    pub fn _get_invalid_parameter_handler() -> _invalid_parameter_handler;
}
extern "C" {
    pub fn _set_thread_local_invalid_parameter_handler(
        _Handler: _invalid_parameter_handler,
    ) -> _invalid_parameter_handler;
}
extern "C" {
    pub fn _get_thread_local_invalid_parameter_handler() -> _invalid_parameter_handler;
}
extern "C" {
    pub fn _set_error_mode(_Mode: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _errno() -> *mut ::std::os::raw::c_int;
}
extern "C" {
    pub fn _set_errno(_Value: ::std::os::raw::c_int) -> errno_t;
}
extern "C" {
    pub fn _get_errno(_Value: *mut ::std::os::raw::c_int) -> errno_t;
}
extern "C" {
    pub fn __doserrno() -> *mut ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn _set_doserrno(_Value: ::std::os::raw::c_ulong) -> errno_t;
}
extern "C" {
    pub fn _get_doserrno(_Value: *mut ::std::os::raw::c_ulong) -> errno_t;
}
extern "C" {
    pub fn __sys_errlist() -> *mut *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn __sys_nerr() -> *mut ::std::os::raw::c_int;
}
extern "C" {
    pub fn perror(_ErrMsg: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn __p__pgmptr() -> *mut *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn __p__wpgmptr() -> *mut *mut wchar_t;
}
extern "C" {
    pub fn __p__fmode() -> *mut ::std::os::raw::c_int;
}
extern "C" {
    pub fn _get_pgmptr(_Value: *mut *mut ::std::os::raw::c_char) -> errno_t;
}
extern "C" {
    pub fn _get_wpgmptr(_Value: *mut *mut wchar_t) -> errno_t;
}
extern "C" {
    pub fn _set_fmode(_Mode: ::std::os::raw::c_int) -> errno_t;
}
extern "C" {
    pub fn _get_fmode(_PMode: *mut ::std::os::raw::c_int) -> errno_t;
}
extern "C" {
    pub fn _abs64(_Number: ::std::os::raw::c_longlong) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn _byteswap_ushort(_Number: ::std::os::raw::c_ushort) -> ::std::os::raw::c_ushort;
}
extern "C" {
    pub fn _byteswap_ulong(_Number: ::std::os::raw::c_ulong) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn _byteswap_uint64(_Number: ::std::os::raw::c_ulonglong) -> ::std::os::raw::c_ulonglong;
}
extern "C" {
    pub fn div(_Numerator: ::std::os::raw::c_int, _Denominator: ::std::os::raw::c_int) -> div_t;
}
extern "C" {
    pub fn ldiv(_Numerator: ::std::os::raw::c_long, _Denominator: ::std::os::raw::c_long)
        -> ldiv_t;
}
extern "C" {
    pub fn lldiv(
        _Numerator: ::std::os::raw::c_longlong,
        _Denominator: ::std::os::raw::c_longlong,
    ) -> lldiv_t;
}
extern "C" {
    pub fn _rotl(
        _Value: ::std::os::raw::c_uint,
        _Shift: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn _lrotl(
        _Value: ::std::os::raw::c_ulong,
        _Shift: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn _rotl64(
        _Value: ::std::os::raw::c_ulonglong,
        _Shift: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_ulonglong;
}
extern "C" {
    pub fn _rotr(
        _Value: ::std::os::raw::c_uint,
        _Shift: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn _lrotr(
        _Value: ::std::os::raw::c_ulong,
        _Shift: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn _rotr64(
        _Value: ::std::os::raw::c_ulonglong,
        _Shift: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_ulonglong;
}
extern "C" {
    pub fn srand(_Seed: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn rand() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn atoi(_String: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn atol(_String: *const ::std::os::raw::c_char) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn atoll(_String: *const ::std::os::raw::c_char) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn _atoi64(_String: *const ::std::os::raw::c_char) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn _atoi_l(
        _String: *const ::std::os::raw::c_char,
        _Locale: _locale_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _atol_l(
        _String: *const ::std::os::raw::c_char,
        _Locale: _locale_t,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn _atoll_l(
        _String: *const ::std::os::raw::c_char,
        _Locale: _locale_t,
    ) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn _atoi64_l(
        _String: *const ::std::os::raw::c_char,
        _Locale: _locale_t,
    ) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn _atoflt(
        _Result: *mut _CRT_FLOAT,
        _String: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _atodbl(
        _Result: *mut _CRT_DOUBLE,
        _String: *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _atoldbl(
        _Result: *mut _LDOUBLE,
        _String: *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _atoflt_l(
        _Result: *mut _CRT_FLOAT,
        _String: *const ::std::os::raw::c_char,
        _Locale: _locale_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _atodbl_l(
        _Result: *mut _CRT_DOUBLE,
        _String: *mut ::std::os::raw::c_char,
        _Locale: _locale_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _atoldbl_l(
        _Result: *mut _LDOUBLE,
        _String: *mut ::std::os::raw::c_char,
        _Locale: _locale_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn strtof(
        _String: *const ::std::os::raw::c_char,
        _EndPtr: *mut *mut ::std::os::raw::c_char,
    ) -> f32;
}
extern "C" {
    pub fn _strtof_l(
        _String: *const ::std::os::raw::c_char,
        _EndPtr: *mut *mut ::std::os::raw::c_char,
        _Locale: _locale_t,
    ) -> f32;
}
extern "C" {
    pub fn strtod(
        _String: *const ::std::os::raw::c_char,
        _EndPtr: *mut *mut ::std::os::raw::c_char,
    ) -> f64;
}
extern "C" {
    pub fn _strtod_l(
        _String: *const ::std::os::raw::c_char,
        _EndPtr: *mut *mut ::std::os::raw::c_char,
        _Locale: _locale_t,
    ) -> f64;
}
extern "C" {
    pub fn strtold(
        _String: *const ::std::os::raw::c_char,
        _EndPtr: *mut *mut ::std::os::raw::c_char,
    ) -> f64;
}
extern "C" {
    pub fn _strtold_l(
        _String: *const ::std::os::raw::c_char,
        _EndPtr: *mut *mut ::std::os::raw::c_char,
        _Locale: _locale_t,
    ) -> f64;
}
extern "C" {
    pub fn strtol(
        _String: *const ::std::os::raw::c_char,
        _EndPtr: *mut *mut ::std::os::raw::c_char,
        _Radix: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn _strtol_l(
        _String: *const ::std::os::raw::c_char,
        _EndPtr: *mut *mut ::std::os::raw::c_char,
        _Radix: ::std::os::raw::c_int,
        _Locale: _locale_t,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn strtoll(
        _String: *const ::std::os::raw::c_char,
        _EndPtr: *mut *mut ::std::os::raw::c_char,
        _Radix: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn _strtoll_l(
        _String: *const ::std::os::raw::c_char,
        _EndPtr: *mut *mut ::std::os::raw::c_char,
        _Radix: ::std::os::raw::c_int,
        _Locale: _locale_t,
    ) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn strtoul(
        _String: *const ::std::os::raw::c_char,
        _EndPtr: *mut *mut ::std::os::raw::c_char,
        _Radix: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn _strtoul_l(
        _String: *const ::std::os::raw::c_char,
        _EndPtr: *mut *mut ::std::os::raw::c_char,
        _Radix: ::std::os::raw::c_int,
        _Locale: _locale_t,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn strtoull(
        _String: *const ::std::os::raw::c_char,
        _EndPtr: *mut *mut ::std::os::raw::c_char,
        _Radix: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_ulonglong;
}
extern "C" {
    pub fn _strtoull_l(
        _String: *const ::std::os::raw::c_char,
        _EndPtr: *mut *mut ::std::os::raw::c_char,
        _Radix: ::std::os::raw::c_int,
        _Locale: _locale_t,
    ) -> ::std::os::raw::c_ulonglong;
}
extern "C" {
    pub fn _strtoi64(
        _String: *const ::std::os::raw::c_char,
        _EndPtr: *mut *mut ::std::os::raw::c_char,
        _Radix: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn _strtoi64_l(
        _String: *const ::std::os::raw::c_char,
        _EndPtr: *mut *mut ::std::os::raw::c_char,
        _Radix: ::std::os::raw::c_int,
        _Locale: _locale_t,
    ) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn _strtoui64(
        _String: *const ::std::os::raw::c_char,
        _EndPtr: *mut *mut ::std::os::raw::c_char,
        _Radix: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_ulonglong;
}
extern "C" {
    pub fn _strtoui64_l(
        _String: *const ::std::os::raw::c_char,
        _EndPtr: *mut *mut ::std::os::raw::c_char,
        _Radix: ::std::os::raw::c_int,
        _Locale: _locale_t,
    ) -> ::std::os::raw::c_ulonglong;
}
extern "C" {
    pub fn _itoa_s(
        _Value: ::std::os::raw::c_int,
        _Buffer: *mut ::std::os::raw::c_char,
        _BufferCount: size_t,
        _Radix: ::std::os::raw::c_int,
    ) -> errno_t;
}
extern "C" {
    pub fn _itoa(
        _Value: ::std::os::raw::c_int,
        _Buffer: *mut ::std::os::raw::c_char,
        _Radix: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn _ltoa_s(
        _Value: ::std::os::raw::c_long,
        _Buffer: *mut ::std::os::raw::c_char,
        _BufferCount: size_t,
        _Radix: ::std::os::raw::c_int,
    ) -> errno_t;
}
extern "C" {
    pub fn _ltoa(
        _Value: ::std::os::raw::c_long,
        _Buffer: *mut ::std::os::raw::c_char,
        _Radix: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn _ultoa_s(
        _Value: ::std::os::raw::c_ulong,
        _Buffer: *mut ::std::os::raw::c_char,
        _BufferCount: size_t,
        _Radix: ::std::os::raw::c_int,
    ) -> errno_t;
}
extern "C" {
    pub fn _ultoa(
        _Value: ::std::os::raw::c_ulong,
        _Buffer: *mut ::std::os::raw::c_char,
        _Radix: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn _i64toa_s(
        _Value: ::std::os::raw::c_longlong,
        _Buffer: *mut ::std::os::raw::c_char,
        _BufferCount: size_t,
        _Radix: ::std::os::raw::c_int,
    ) -> errno_t;
}
extern "C" {
    pub fn _i64toa(
        _Value: ::std::os::raw::c_longlong,
        _Buffer: *mut ::std::os::raw::c_char,
        _Radix: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn _ui64toa_s(
        _Value: ::std::os::raw::c_ulonglong,
        _Buffer: *mut ::std::os::raw::c_char,
        _BufferCount: size_t,
        _Radix: ::std::os::raw::c_int,
    ) -> errno_t;
}
extern "C" {
    pub fn _ui64toa(
        _Value: ::std::os::raw::c_ulonglong,
        _Buffer: *mut ::std::os::raw::c_char,
        _Radix: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn _ecvt_s(
        _Buffer: *mut ::std::os::raw::c_char,
        _BufferCount: size_t,
        _Value: f64,
        _DigitCount: ::std::os::raw::c_int,
        _PtDec: *mut ::std::os::raw::c_int,
        _PtSign: *mut ::std::os::raw::c_int,
    ) -> errno_t;
}
extern "C" {
    pub fn _ecvt(
        _Value: f64,
        _DigitCount: ::std::os::raw::c_int,
        _PtDec: *mut ::std::os::raw::c_int,
        _PtSign: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn _fcvt_s(
        _Buffer: *mut ::std::os::raw::c_char,
        _BufferCount: size_t,
        _Value: f64,
        _FractionalDigitCount: ::std::os::raw::c_int,
        _PtDec: *mut ::std::os::raw::c_int,
        _PtSign: *mut ::std::os::raw::c_int,
    ) -> errno_t;
}
extern "C" {
    pub fn _fcvt(
        _Value: f64,
        _FractionalDigitCount: ::std::os::raw::c_int,
        _PtDec: *mut ::std::os::raw::c_int,
        _PtSign: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn _gcvt_s(
        _Buffer: *mut ::std::os::raw::c_char,
        _BufferCount: size_t,
        _Value: f64,
        _DigitCount: ::std::os::raw::c_int,
    ) -> errno_t;
}
extern "C" {
    pub fn _gcvt(
        _Value: f64,
        _DigitCount: ::std::os::raw::c_int,
        _Buffer: *mut ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn ___mb_cur_max_func() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ___mb_cur_max_l_func(_Locale: _locale_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mblen(_Ch: *const ::std::os::raw::c_char, _MaxCount: size_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _mblen_l(
        _Ch: *const ::std::os::raw::c_char,
        _MaxCount: size_t,
        _Locale: _locale_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _mbstrlen(_String: *const ::std::os::raw::c_char) -> size_t;
}
extern "C" {
    pub fn _mbstrlen_l(_String: *const ::std::os::raw::c_char, _Locale: _locale_t) -> size_t;
}
extern "C" {
    pub fn _mbstrnlen(_String: *const ::std::os::raw::c_char, _MaxCount: size_t) -> size_t;
}
extern "C" {
    pub fn _mbstrnlen_l(
        _String: *const ::std::os::raw::c_char,
        _MaxCount: size_t,
        _Locale: _locale_t,
    ) -> size_t;
}
extern "C" {
    pub fn mbtowc(
        _DstCh: *mut wchar_t,
        _SrcCh: *const ::std::os::raw::c_char,
        _SrcSizeInBytes: size_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _mbtowc_l(
        _DstCh: *mut wchar_t,
        _SrcCh: *const ::std::os::raw::c_char,
        _SrcSizeInBytes: size_t,
        _Locale: _locale_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mbstowcs_s(
        _PtNumOfCharConverted: *mut size_t,
        _DstBuf: *mut wchar_t,
        _SizeInWords: size_t,
        _SrcBuf: *const ::std::os::raw::c_char,
        _MaxCount: size_t,
    ) -> errno_t;
}
extern "C" {
    pub fn mbstowcs(
        _Dest: *mut wchar_t,
        _Source: *const ::std::os::raw::c_char,
        _MaxCount: size_t,
    ) -> size_t;
}
extern "C" {
    pub fn _mbstowcs_s_l(
        _PtNumOfCharConverted: *mut size_t,
        _DstBuf: *mut wchar_t,
        _SizeInWords: size_t,
        _SrcBuf: *const ::std::os::raw::c_char,
        _MaxCount: size_t,
        _Locale: _locale_t,
    ) -> errno_t;
}
extern "C" {
    pub fn _mbstowcs_l(
        _Dest: *mut wchar_t,
        _Source: *const ::std::os::raw::c_char,
        _MaxCount: size_t,
        _Locale: _locale_t,
    ) -> size_t;
}
extern "C" {
    pub fn wctomb(_MbCh: *mut ::std::os::raw::c_char, _WCh: wchar_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _wctomb_l(
        _MbCh: *mut ::std::os::raw::c_char,
        _WCh: wchar_t,
        _Locale: _locale_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wctomb_s(
        _SizeConverted: *mut ::std::os::raw::c_int,
        _MbCh: *mut ::std::os::raw::c_char,
        _SizeInBytes: rsize_t,
        _WCh: wchar_t,
    ) -> errno_t;
}
extern "C" {
    pub fn _wctomb_s_l(
        _SizeConverted: *mut ::std::os::raw::c_int,
        _MbCh: *mut ::std::os::raw::c_char,
        _SizeInBytes: size_t,
        _WCh: wchar_t,
        _Locale: _locale_t,
    ) -> errno_t;
}
extern "C" {
    pub fn wcstombs_s(
        _PtNumOfCharConverted: *mut size_t,
        _Dst: *mut ::std::os::raw::c_char,
        _DstSizeInBytes: size_t,
        _Src: *const wchar_t,
        _MaxCountInBytes: size_t,
    ) -> errno_t;
}
extern "C" {
    pub fn wcstombs(
        _Dest: *mut ::std::os::raw::c_char,
        _Source: *const wchar_t,
        _MaxCount: size_t,
    ) -> size_t;
}
extern "C" {
    pub fn _wcstombs_s_l(
        _PtNumOfCharConverted: *mut size_t,
        _Dst: *mut ::std::os::raw::c_char,
        _DstSizeInBytes: size_t,
        _Src: *const wchar_t,
        _MaxCountInBytes: size_t,
        _Locale: _locale_t,
    ) -> errno_t;
}
extern "C" {
    pub fn _wcstombs_l(
        _Dest: *mut ::std::os::raw::c_char,
        _Source: *const wchar_t,
        _MaxCount: size_t,
        _Locale: _locale_t,
    ) -> size_t;
}
extern "C" {
    pub fn _fullpath(
        _Buffer: *mut ::std::os::raw::c_char,
        _Path: *const ::std::os::raw::c_char,
        _BufferCount: size_t,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn _makepath_s(
        _Buffer: *mut ::std::os::raw::c_char,
        _BufferCount: size_t,
        _Drive: *const ::std::os::raw::c_char,
        _Dir: *const ::std::os::raw::c_char,
        _Filename: *const ::std::os::raw::c_char,
        _Ext: *const ::std::os::raw::c_char,
    ) -> errno_t;
}
extern "C" {
    pub fn _makepath(
        _Buffer: *mut ::std::os::raw::c_char,
        _Drive: *const ::std::os::raw::c_char,
        _Dir: *const ::std::os::raw::c_char,
        _Filename: *const ::std::os::raw::c_char,
        _Ext: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn _splitpath(
        _FullPath: *const ::std::os::raw::c_char,
        _Drive: *mut ::std::os::raw::c_char,
        _Dir: *mut ::std::os::raw::c_char,
        _Filename: *mut ::std::os::raw::c_char,
        _Ext: *mut ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn _splitpath_s(
        _FullPath: *const ::std::os::raw::c_char,
        _Drive: *mut ::std::os::raw::c_char,
        _DriveCount: size_t,
        _Dir: *mut ::std::os::raw::c_char,
        _DirCount: size_t,
        _Filename: *mut ::std::os::raw::c_char,
        _FilenameCount: size_t,
        _Ext: *mut ::std::os::raw::c_char,
        _ExtCount: size_t,
    ) -> errno_t;
}
extern "C" {
    pub fn getenv_s(
        _RequiredCount: *mut size_t,
        _Buffer: *mut ::std::os::raw::c_char,
        _BufferCount: rsize_t,
        _VarName: *const ::std::os::raw::c_char,
    ) -> errno_t;
}
extern "C" {
    pub fn __p___argc() -> *mut ::std::os::raw::c_int;
}
extern "C" {
    pub fn __p___argv() -> *mut *mut *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn __p___wargv() -> *mut *mut *mut wchar_t;
}
extern "C" {
    pub fn __p__environ() -> *mut *mut *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn __p__wenviron() -> *mut *mut *mut wchar_t;
}
extern "C" {
    pub fn getenv(_VarName: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn _dupenv_s(
        _Buffer: *mut *mut ::std::os::raw::c_char,
        _BufferCount: *mut size_t,
        _VarName: *const ::std::os::raw::c_char,
    ) -> errno_t;
}
extern "C" {
    pub fn system(_Command: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _putenv(_EnvString: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _putenv_s(
        _Name: *const ::std::os::raw::c_char,
        _Value: *const ::std::os::raw::c_char,
    ) -> errno_t;
}
extern "C" {
    pub fn _searchenv_s(
        _Filename: *const ::std::os::raw::c_char,
        _VarName: *const ::std::os::raw::c_char,
        _Buffer: *mut ::std::os::raw::c_char,
        _BufferCount: size_t,
    ) -> errno_t;
}
extern "C" {
    pub fn _searchenv(
        _Filename: *const ::std::os::raw::c_char,
        _VarName: *const ::std::os::raw::c_char,
        _Buffer: *mut ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn _seterrormode(_Mode: ::std::os::raw::c_int);
}
extern "C" {
    pub fn _beep(_Frequency: ::std::os::raw::c_uint, _Duration: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn _sleep(_Duration: ::std::os::raw::c_ulong);
}
extern "C" {
    pub fn ecvt(
        _Value: f64,
        _DigitCount: ::std::os::raw::c_int,
        _PtDec: *mut ::std::os::raw::c_int,
        _PtSign: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn fcvt(
        _Value: f64,
        _FractionalDigitCount: ::std::os::raw::c_int,
        _PtDec: *mut ::std::os::raw::c_int,
        _PtSign: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn gcvt(
        _Value: f64,
        _DigitCount: ::std::os::raw::c_int,
        _DstBuf: *mut ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn itoa(
        _Value: ::std::os::raw::c_int,
        _Buffer: *mut ::std::os::raw::c_char,
        _Radix: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn ltoa(
        _Value: ::std::os::raw::c_long,
        _Buffer: *mut ::std::os::raw::c_char,
        _Radix: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn swab(
        _Buf1: *mut ::std::os::raw::c_char,
        _Buf2: *mut ::std::os::raw::c_char,
        _SizeInBytes: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn ultoa(
        _Value: ::std::os::raw::c_ulong,
        _Buffer: *mut ::std::os::raw::c_char,
        _Radix: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn putenv(_EnvString: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn onexit(_Func: _onexit_t) -> _onexit_t;
}
extern "C" {
    pub fn memchr(
        _Buf: *const ::std::os::raw::c_void,
        _Val: ::std::os::raw::c_int,
        _MaxCount: ::std::os::raw::c_ulonglong,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn memcmp(
        _Buf1: *const ::std::os::raw::c_void,
        _Buf2: *const ::std::os::raw::c_void,
        _Size: ::std::os::raw::c_ulonglong,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn memcpy(
        _Dst: *mut ::std::os::raw::c_void,
        _Src: *const ::std::os::raw::c_void,
        _Size: ::std::os::raw::c_ulonglong,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn memmove(
        _Dst: *mut ::std::os::raw::c_void,
        _Src: *const ::std::os::raw::c_void,
        _Size: ::std::os::raw::c_ulonglong,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn memset(
        _Dst: *mut ::std::os::raw::c_void,
        _Val: ::std::os::raw::c_int,
        _Size: ::std::os::raw::c_ulonglong,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn strchr(
        _Str: *const ::std::os::raw::c_char,
        _Val: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strrchr(
        _Str: *const ::std::os::raw::c_char,
        _Ch: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strstr(
        _Str: *const ::std::os::raw::c_char,
        _SubStr: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn wcschr(
        _Str: *const ::std::os::raw::c_ushort,
        _Ch: ::std::os::raw::c_ushort,
    ) -> *mut ::std::os::raw::c_ushort;
}
extern "C" {
    pub fn wcsrchr(_Str: *const wchar_t, _Ch: wchar_t) -> *mut wchar_t;
}
extern "C" {
    pub fn wcsstr(_Str: *const wchar_t, _SubStr: *const wchar_t) -> *mut wchar_t;
}
extern "C" {
    pub fn _memicmp(
        _Buf1: *const ::std::os::raw::c_void,
        _Buf2: *const ::std::os::raw::c_void,
        _Size: size_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _memicmp_l(
        _Buf1: *const ::std::os::raw::c_void,
        _Buf2: *const ::std::os::raw::c_void,
        _Size: size_t,
        _Locale: _locale_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn memccpy(
        _Dst: *mut ::std::os::raw::c_void,
        _Src: *const ::std::os::raw::c_void,
        _Val: ::std::os::raw::c_int,
        _Size: size_t,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn memicmp(
        _Buf1: *const ::std::os::raw::c_void,
        _Buf2: *const ::std::os::raw::c_void,
        _Size: size_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wcscat_s(
        _Destination: *mut wchar_t,
        _SizeInWords: rsize_t,
        _Source: *const wchar_t,
    ) -> errno_t;
}
extern "C" {
    pub fn wcscpy_s(
        _Destination: *mut wchar_t,
        _SizeInWords: rsize_t,
        _Source: *const wchar_t,
    ) -> errno_t;
}
extern "C" {
    pub fn wcsncat_s(
        _Destination: *mut wchar_t,
        _SizeInWords: rsize_t,
        _Source: *const wchar_t,
        _MaxCount: rsize_t,
    ) -> errno_t;
}
extern "C" {
    pub fn wcsncpy_s(
        _Destination: *mut wchar_t,
        _SizeInWords: rsize_t,
        _Source: *const wchar_t,
        _MaxCount: rsize_t,
    ) -> errno_t;
}
extern "C" {
    pub fn wcstok_s(
        _String: *mut wchar_t,
        _Delimiter: *const wchar_t,
        _Context: *mut *mut wchar_t,
    ) -> *mut wchar_t;
}
extern "C" {
    pub fn _wcsdup(_String: *const wchar_t) -> *mut wchar_t;
}
extern "C" {
    pub fn wcscat(_Destination: *mut wchar_t, _Source: *const wchar_t) -> *mut wchar_t;
}
extern "C" {
    pub fn wcscmp(
        _String1: *const ::std::os::raw::c_ushort,
        _String2: *const ::std::os::raw::c_ushort,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wcscpy(_Destination: *mut wchar_t, _Source: *const wchar_t) -> *mut wchar_t;
}
extern "C" {
    pub fn wcscspn(_String: *const wchar_t, _Control: *const wchar_t) -> size_t;
}
extern "C" {
    pub fn wcslen(_String: *const ::std::os::raw::c_ushort) -> ::std::os::raw::c_ulonglong;
}
extern "C" {
    pub fn wcsnlen(_Source: *const wchar_t, _MaxCount: size_t) -> size_t;
}
extern "C" {
    pub fn wcsncat(
        _Destination: *mut wchar_t,
        _Source: *const wchar_t,
        _Count: size_t,
    ) -> *mut wchar_t;
}
extern "C" {
    pub fn wcsncmp(
        _String1: *const ::std::os::raw::c_ushort,
        _String2: *const ::std::os::raw::c_ushort,
        _MaxCount: ::std::os::raw::c_ulonglong,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wcsncpy(
        _Destination: *mut wchar_t,
        _Source: *const wchar_t,
        _Count: size_t,
    ) -> *mut wchar_t;
}
extern "C" {
    pub fn wcspbrk(_String: *const wchar_t, _Control: *const wchar_t) -> *mut wchar_t;
}
extern "C" {
    pub fn wcsspn(_String: *const wchar_t, _Control: *const wchar_t) -> size_t;
}
extern "C" {
    pub fn wcstok(
        _String: *mut wchar_t,
        _Delimiter: *const wchar_t,
        _Context: *mut *mut wchar_t,
    ) -> *mut wchar_t;
}
extern "C" {
    pub fn _wcserror(_ErrorNumber: ::std::os::raw::c_int) -> *mut wchar_t;
}
extern "C" {
    pub fn _wcserror_s(
        _Buffer: *mut wchar_t,
        _SizeInWords: size_t,
        _ErrorNumber: ::std::os::raw::c_int,
    ) -> errno_t;
}
extern "C" {
    pub fn __wcserror(_String: *const wchar_t) -> *mut wchar_t;
}
extern "C" {
    pub fn __wcserror_s(
        _Buffer: *mut wchar_t,
        _SizeInWords: size_t,
        _ErrorMessage: *const wchar_t,
    ) -> errno_t;
}
extern "C" {
    pub fn _wcsicmp(_String1: *const wchar_t, _String2: *const wchar_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _wcsicmp_l(
        _String1: *const wchar_t,
        _String2: *const wchar_t,
        _Locale: _locale_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _wcsnicmp(
        _String1: *const wchar_t,
        _String2: *const wchar_t,
        _MaxCount: size_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _wcsnicmp_l(
        _String1: *const wchar_t,
        _String2: *const wchar_t,
        _MaxCount: size_t,
        _Locale: _locale_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _wcsnset_s(
        _Destination: *mut wchar_t,
        _SizeInWords: size_t,
        _Value: wchar_t,
        _MaxCount: size_t,
    ) -> errno_t;
}
extern "C" {
    pub fn _wcsnset(_String: *mut wchar_t, _Value: wchar_t, _MaxCount: size_t) -> *mut wchar_t;
}
extern "C" {
    pub fn _wcsrev(_String: *mut wchar_t) -> *mut wchar_t;
}
extern "C" {
    pub fn _wcsset_s(_Destination: *mut wchar_t, _SizeInWords: size_t, _Value: wchar_t) -> errno_t;
}
extern "C" {
    pub fn _wcsset(_String: *mut wchar_t, _Value: wchar_t) -> *mut wchar_t;
}
extern "C" {
    pub fn _wcslwr_s(_String: *mut wchar_t, _SizeInWords: size_t) -> errno_t;
}
extern "C" {
    pub fn _wcslwr(_String: *mut wchar_t) -> *mut wchar_t;
}
extern "C" {
    pub fn _wcslwr_s_l(_String: *mut wchar_t, _SizeInWords: size_t, _Locale: _locale_t) -> errno_t;
}
extern "C" {
    pub fn _wcslwr_l(_String: *mut wchar_t, _Locale: _locale_t) -> *mut wchar_t;
}
extern "C" {
    pub fn _wcsupr_s(_String: *mut wchar_t, _Size: size_t) -> errno_t;
}
extern "C" {
    pub fn _wcsupr(_String: *mut wchar_t) -> *mut wchar_t;
}
extern "C" {
    pub fn _wcsupr_s_l(_String: *mut wchar_t, _Size: size_t, _Locale: _locale_t) -> errno_t;
}
extern "C" {
    pub fn _wcsupr_l(_String: *mut wchar_t, _Locale: _locale_t) -> *mut wchar_t;
}
extern "C" {
    pub fn wcsxfrm(
        _Destination: *mut wchar_t,
        _Source: *const wchar_t,
        _MaxCount: size_t,
    ) -> size_t;
}
extern "C" {
    pub fn _wcsxfrm_l(
        _Destination: *mut wchar_t,
        _Source: *const wchar_t,
        _MaxCount: size_t,
        _Locale: _locale_t,
    ) -> size_t;
}
extern "C" {
    pub fn wcscoll(_String1: *const wchar_t, _String2: *const wchar_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _wcscoll_l(
        _String1: *const wchar_t,
        _String2: *const wchar_t,
        _Locale: _locale_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _wcsicoll(_String1: *const wchar_t, _String2: *const wchar_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _wcsicoll_l(
        _String1: *const wchar_t,
        _String2: *const wchar_t,
        _Locale: _locale_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _wcsncoll(
        _String1: *const wchar_t,
        _String2: *const wchar_t,
        _MaxCount: size_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _wcsncoll_l(
        _String1: *const wchar_t,
        _String2: *const wchar_t,
        _MaxCount: size_t,
        _Locale: _locale_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _wcsnicoll(
        _String1: *const wchar_t,
        _String2: *const wchar_t,
        _MaxCount: size_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _wcsnicoll_l(
        _String1: *const wchar_t,
        _String2: *const wchar_t,
        _MaxCount: size_t,
        _Locale: _locale_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wcsdup(_String: *const wchar_t) -> *mut wchar_t;
}
extern "C" {
    pub fn wcsicmp(_String1: *const wchar_t, _String2: *const wchar_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wcsnicmp(
        _String1: *const wchar_t,
        _String2: *const wchar_t,
        _MaxCount: size_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wcsnset(_String: *mut wchar_t, _Value: wchar_t, _MaxCount: size_t) -> *mut wchar_t;
}
extern "C" {
    pub fn wcsrev(_String: *mut wchar_t) -> *mut wchar_t;
}
extern "C" {
    pub fn wcsset(_String: *mut wchar_t, _Value: wchar_t) -> *mut wchar_t;
}
extern "C" {
    pub fn wcslwr(_String: *mut wchar_t) -> *mut wchar_t;
}
extern "C" {
    pub fn wcsupr(_String: *mut wchar_t) -> *mut wchar_t;
}
extern "C" {
    pub fn wcsicoll(_String1: *const wchar_t, _String2: *const wchar_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn strcpy_s(
        _Destination: *mut ::std::os::raw::c_char,
        _SizeInBytes: rsize_t,
        _Source: *const ::std::os::raw::c_char,
    ) -> errno_t;
}
extern "C" {
    pub fn strcat_s(
        _Destination: *mut ::std::os::raw::c_char,
        _SizeInBytes: rsize_t,
        _Source: *const ::std::os::raw::c_char,
    ) -> errno_t;
}
extern "C" {
    pub fn strerror_s(
        _Buffer: *mut ::std::os::raw::c_char,
        _SizeInBytes: size_t,
        _ErrorNumber: ::std::os::raw::c_int,
    ) -> errno_t;
}
extern "C" {
    pub fn strncat_s(
        _Destination: *mut ::std::os::raw::c_char,
        _SizeInBytes: rsize_t,
        _Source: *const ::std::os::raw::c_char,
        _MaxCount: rsize_t,
    ) -> errno_t;
}
extern "C" {
    pub fn strncpy_s(
        _Destination: *mut ::std::os::raw::c_char,
        _SizeInBytes: rsize_t,
        _Source: *const ::std::os::raw::c_char,
        _MaxCount: rsize_t,
    ) -> errno_t;
}
extern "C" {
    pub fn strtok_s(
        _String: *mut ::std::os::raw::c_char,
        _Delimiter: *const ::std::os::raw::c_char,
        _Context: *mut *mut ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn _memccpy(
        _Dst: *mut ::std::os::raw::c_void,
        _Src: *const ::std::os::raw::c_void,
        _Val: ::std::os::raw::c_int,
        _MaxCount: size_t,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn strcat(
        _Destination: *mut ::std::os::raw::c_char,
        _Source: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strcmp(
        _Str1: *const ::std::os::raw::c_char,
        _Str2: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _strcmpi(
        _String1: *const ::std::os::raw::c_char,
        _String2: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn strcoll(
        _String1: *const ::std::os::raw::c_char,
        _String2: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _strcoll_l(
        _String1: *const ::std::os::raw::c_char,
        _String2: *const ::std::os::raw::c_char,
        _Locale: _locale_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn strcpy(
        _Destination: *mut ::std::os::raw::c_char,
        _Source: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strcspn(
        _Str: *const ::std::os::raw::c_char,
        _Control: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_ulonglong;
}
extern "C" {
    pub fn _strdup(_Source: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn _strerror(_ErrorMessage: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn _strerror_s(
        _Buffer: *mut ::std::os::raw::c_char,
        _SizeInBytes: size_t,
        _ErrorMessage: *const ::std::os::raw::c_char,
    ) -> errno_t;
}
extern "C" {
    pub fn strerror(_ErrorMessage: ::std::os::raw::c_int) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn _stricmp(
        _String1: *const ::std::os::raw::c_char,
        _String2: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _stricoll(
        _String1: *const ::std::os::raw::c_char,
        _String2: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _stricoll_l(
        _String1: *const ::std::os::raw::c_char,
        _String2: *const ::std::os::raw::c_char,
        _Locale: _locale_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _stricmp_l(
        _String1: *const ::std::os::raw::c_char,
        _String2: *const ::std::os::raw::c_char,
        _Locale: _locale_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn strlen(_Str: *const ::std::os::raw::c_char) -> ::std::os::raw::c_ulonglong;
}
extern "C" {
    pub fn _strlwr_s(_String: *mut ::std::os::raw::c_char, _Size: size_t) -> errno_t;
}
extern "C" {
    pub fn _strlwr(_String: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn _strlwr_s_l(
        _String: *mut ::std::os::raw::c_char,
        _Size: size_t,
        _Locale: _locale_t,
    ) -> errno_t;
}
extern "C" {
    pub fn _strlwr_l(
        _String: *mut ::std::os::raw::c_char,
        _Locale: _locale_t,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strncat(
        _Destination: *mut ::std::os::raw::c_char,
        _Source: *const ::std::os::raw::c_char,
        _Count: ::std::os::raw::c_ulonglong,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strncmp(
        _Str1: *const ::std::os::raw::c_char,
        _Str2: *const ::std::os::raw::c_char,
        _MaxCount: ::std::os::raw::c_ulonglong,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _strnicmp(
        _String1: *const ::std::os::raw::c_char,
        _String2: *const ::std::os::raw::c_char,
        _MaxCount: size_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _strnicmp_l(
        _String1: *const ::std::os::raw::c_char,
        _String2: *const ::std::os::raw::c_char,
        _MaxCount: size_t,
        _Locale: _locale_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _strnicoll(
        _String1: *const ::std::os::raw::c_char,
        _String2: *const ::std::os::raw::c_char,
        _MaxCount: size_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _strnicoll_l(
        _String1: *const ::std::os::raw::c_char,
        _String2: *const ::std::os::raw::c_char,
        _MaxCount: size_t,
        _Locale: _locale_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _strncoll(
        _String1: *const ::std::os::raw::c_char,
        _String2: *const ::std::os::raw::c_char,
        _MaxCount: size_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _strncoll_l(
        _String1: *const ::std::os::raw::c_char,
        _String2: *const ::std::os::raw::c_char,
        _MaxCount: size_t,
        _Locale: _locale_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __strncnt(_String: *const ::std::os::raw::c_char, _Count: size_t) -> size_t;
}
extern "C" {
    pub fn strncpy(
        _Destination: *mut ::std::os::raw::c_char,
        _Source: *const ::std::os::raw::c_char,
        _Count: ::std::os::raw::c_ulonglong,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strnlen(_String: *const ::std::os::raw::c_char, _MaxCount: size_t) -> size_t;
}
extern "C" {
    pub fn _strnset_s(
        _String: *mut ::std::os::raw::c_char,
        _SizeInBytes: size_t,
        _Value: ::std::os::raw::c_int,
        _MaxCount: size_t,
    ) -> errno_t;
}
extern "C" {
    pub fn _strnset(
        _Destination: *mut ::std::os::raw::c_char,
        _Value: ::std::os::raw::c_int,
        _Count: size_t,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strpbrk(
        _Str: *const ::std::os::raw::c_char,
        _Control: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn _strrev(_Str: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn _strset_s(
        _Destination: *mut ::std::os::raw::c_char,
        _DestinationSize: size_t,
        _Value: ::std::os::raw::c_int,
    ) -> errno_t;
}
extern "C" {
    pub fn _strset(
        _Destination: *mut ::std::os::raw::c_char,
        _Value: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strspn(
        _Str: *const ::std::os::raw::c_char,
        _Control: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_ulonglong;
}
extern "C" {
    pub fn strtok(
        _String: *mut ::std::os::raw::c_char,
        _Delimiter: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn _strupr_s(_String: *mut ::std::os::raw::c_char, _Size: size_t) -> errno_t;
}
extern "C" {
    pub fn _strupr(_String: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn _strupr_s_l(
        _String: *mut ::std::os::raw::c_char,
        _Size: size_t,
        _Locale: _locale_t,
    ) -> errno_t;
}
extern "C" {
    pub fn _strupr_l(
        _String: *mut ::std::os::raw::c_char,
        _Locale: _locale_t,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strxfrm(
        _Destination: *mut ::std::os::raw::c_char,
        _Source: *const ::std::os::raw::c_char,
        _MaxCount: ::std::os::raw::c_ulonglong,
    ) -> ::std::os::raw::c_ulonglong;
}
extern "C" {
    pub fn _strxfrm_l(
        _Destination: *mut ::std::os::raw::c_char,
        _Source: *const ::std::os::raw::c_char,
        _MaxCount: size_t,
        _Locale: _locale_t,
    ) -> size_t;
}
extern "C" {
    pub fn strdup(_String: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strcmpi(
        _String1: *const ::std::os::raw::c_char,
        _String2: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn stricmp(
        _String1: *const ::std::os::raw::c_char,
        _String2: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn strlwr(_String: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strnicmp(
        _String1: *const ::std::os::raw::c_char,
        _String2: *const ::std::os::raw::c_char,
        _MaxCount: size_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn strnset(
        _String: *mut ::std::os::raw::c_char,
        _Value: ::std::os::raw::c_int,
        _MaxCount: size_t,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strrev(_String: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strset(
        _String: *mut ::std::os::raw::c_char,
        _Value: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strupr(_String: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub static mut GLVersion: gladGLversionStruct;
}
extern "C" {
    pub fn gladLoadGLLoader(arg1: GLADloadproc) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn imaxabs(_Number: intmax_t) -> intmax_t;
}
extern "C" {
    pub fn imaxdiv(_Numerator: intmax_t, _Denominator: intmax_t) -> imaxdiv_t;
}
extern "C" {
    pub fn strtoimax(
        _String: *const ::std::os::raw::c_char,
        _EndPtr: *mut *mut ::std::os::raw::c_char,
        _Radix: ::std::os::raw::c_int,
    ) -> intmax_t;
}
extern "C" {
    pub fn _strtoimax_l(
        _String: *const ::std::os::raw::c_char,
        _EndPtr: *mut *mut ::std::os::raw::c_char,
        _Radix: ::std::os::raw::c_int,
        _Locale: _locale_t,
    ) -> intmax_t;
}
extern "C" {
    pub fn strtoumax(
        _String: *const ::std::os::raw::c_char,
        _EndPtr: *mut *mut ::std::os::raw::c_char,
        _Radix: ::std::os::raw::c_int,
    ) -> uintmax_t;
}
extern "C" {
    pub fn _strtoumax_l(
        _String: *const ::std::os::raw::c_char,
        _EndPtr: *mut *mut ::std::os::raw::c_char,
        _Radix: ::std::os::raw::c_int,
        _Locale: _locale_t,
    ) -> uintmax_t;
}
extern "C" {
    pub fn wcstoimax(
        _String: *const wchar_t,
        _EndPtr: *mut *mut wchar_t,
        _Radix: ::std::os::raw::c_int,
    ) -> intmax_t;
}
extern "C" {
    pub fn _wcstoimax_l(
        _String: *const wchar_t,
        _EndPtr: *mut *mut wchar_t,
        _Radix: ::std::os::raw::c_int,
        _Locale: _locale_t,
    ) -> intmax_t;
}
extern "C" {
    pub fn wcstoumax(
        _String: *const wchar_t,
        _EndPtr: *mut *mut wchar_t,
        _Radix: ::std::os::raw::c_int,
    ) -> uintmax_t;
}
extern "C" {
    pub fn _wcstoumax_l(
        _String: *const wchar_t,
        _EndPtr: *mut *mut wchar_t,
        _Radix: ::std::os::raw::c_int,
        _Locale: _locale_t,
    ) -> uintmax_t;
}
extern "C" {
    pub static mut GLAD_GL_VERSION_1_0: ::std::os::raw::c_int;
}
extern "C" {
    pub fn GuiEnable();
}
extern "C" {
    pub fn GuiDisable();
}
extern "C" {
    pub fn GuiLock();
}
extern "C" {
    pub fn GuiUnlock();
}
extern "C" {
    pub fn GuiFade(alpha: f32);
}
extern "C" {
    pub fn GuiSetState(state: ::std::os::raw::c_int);
}
extern "C" {
    pub fn GuiGetState() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GuiSetFont(font: Font);
}
extern "C" {
    pub fn GuiGetFont() -> Font;
}
extern "C" {
    pub fn GuiSetStyle(
        control: ::std::os::raw::c_int,
        property: ::std::os::raw::c_int,
        value: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn GuiGetStyle(
        control: ::std::os::raw::c_int,
        property: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GuiEnableTooltip();
}
extern "C" {
    pub fn GuiDisableTooltip();
}
extern "C" {
    pub fn GuiSetTooltip(tooltip: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn GuiClearTooltip();
}
extern "C" {
    pub fn GuiWindowBox(bounds: Rectangle, title: *const ::std::os::raw::c_char) -> bool;
}
extern "C" {
    pub fn GuiGroupBox(bounds: Rectangle, text: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn GuiLine(bounds: Rectangle, text: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn GuiPanel(bounds: Rectangle);
}
extern "C" {
    pub fn GuiScrollPanel(bounds: Rectangle, content: Rectangle, scroll: *mut Vector2)
        -> Rectangle;
}
extern "C" {
    pub fn GuiLabel(bounds: Rectangle, text: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn GuiButton(bounds: Rectangle, text: *const ::std::os::raw::c_char) -> bool;
}
extern "C" {
    pub fn GuiLabelButton(bounds: Rectangle, text: *const ::std::os::raw::c_char) -> bool;
}
extern "C" {
    pub fn GuiImageButton(
        bounds: Rectangle,
        text: *const ::std::os::raw::c_char,
        texture: Texture2D,
    ) -> bool;
}
extern "C" {
    pub fn GuiImageButtonEx(
        bounds: Rectangle,
        text: *const ::std::os::raw::c_char,
        texture: Texture2D,
        texSource: Rectangle,
    ) -> bool;
}
extern "C" {
    pub fn GuiToggle(bounds: Rectangle, text: *const ::std::os::raw::c_char, active: bool) -> bool;
}
extern "C" {
    pub fn GuiToggleGroup(
        bounds: Rectangle,
        text: *const ::std::os::raw::c_char,
        active: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GuiCheckBox(
        bounds: Rectangle,
        text: *const ::std::os::raw::c_char,
        checked: bool,
    ) -> bool;
}
extern "C" {
    pub fn GuiComboBox(
        bounds: Rectangle,
        text: *const ::std::os::raw::c_char,
        active: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GuiDropdownBox(
        bounds: Rectangle,
        text: *const ::std::os::raw::c_char,
        active: *mut ::std::os::raw::c_int,
        editMode: bool,
    ) -> bool;
}
extern "C" {
    pub fn GuiSpinner(
        bounds: Rectangle,
        text: *const ::std::os::raw::c_char,
        value: *mut ::std::os::raw::c_int,
        minValue: ::std::os::raw::c_int,
        maxValue: ::std::os::raw::c_int,
        editMode: bool,
    ) -> bool;
}
extern "C" {
    pub fn GuiValueBox(
        bounds: Rectangle,
        text: *const ::std::os::raw::c_char,
        value: *mut ::std::os::raw::c_int,
        minValue: ::std::os::raw::c_int,
        maxValue: ::std::os::raw::c_int,
        editMode: bool,
    ) -> bool;
}
extern "C" {
    pub fn GuiTextBox(
        bounds: Rectangle,
        text: *mut ::std::os::raw::c_char,
        textSize: ::std::os::raw::c_int,
        editMode: bool,
    ) -> bool;
}
extern "C" {
    pub fn GuiTextBoxMulti(
        bounds: Rectangle,
        text: *mut ::std::os::raw::c_char,
        textSize: ::std::os::raw::c_int,
        editMode: bool,
    ) -> bool;
}
extern "C" {
    pub fn GuiSlider(
        bounds: Rectangle,
        textLeft: *const ::std::os::raw::c_char,
        textRight: *const ::std::os::raw::c_char,
        value: f32,
        minValue: f32,
        maxValue: f32,
    ) -> f32;
}
extern "C" {
    pub fn GuiSliderBar(
        bounds: Rectangle,
        textLeft: *const ::std::os::raw::c_char,
        textRight: *const ::std::os::raw::c_char,
        value: f32,
        minValue: f32,
        maxValue: f32,
    ) -> f32;
}
extern "C" {
    pub fn GuiProgressBar(
        bounds: Rectangle,
        textLeft: *const ::std::os::raw::c_char,
        textRight: *const ::std::os::raw::c_char,
        value: f32,
        minValue: f32,
        maxValue: f32,
    ) -> f32;
}
extern "C" {
    pub fn GuiStatusBar(bounds: Rectangle, text: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn GuiDummyRec(bounds: Rectangle, text: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn GuiScrollBar(
        bounds: Rectangle,
        value: ::std::os::raw::c_int,
        minValue: ::std::os::raw::c_int,
        maxValue: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GuiGrid(bounds: Rectangle, spacing: f32, subdivs: ::std::os::raw::c_int) -> Vector2;
}
extern "C" {
    pub fn GuiListView(
        bounds: Rectangle,
        text: *const ::std::os::raw::c_char,
        scrollIndex: *mut ::std::os::raw::c_int,
        active: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GuiListViewEx(
        bounds: Rectangle,
        text: *mut *const ::std::os::raw::c_char,
        count: ::std::os::raw::c_int,
        focus: *mut ::std::os::raw::c_int,
        scrollIndex: *mut ::std::os::raw::c_int,
        active: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GuiMessageBox(
        bounds: Rectangle,
        title: *const ::std::os::raw::c_char,
        message: *const ::std::os::raw::c_char,
        buttons: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GuiTextInputBox(
        bounds: Rectangle,
        title: *const ::std::os::raw::c_char,
        message: *const ::std::os::raw::c_char,
        buttons: *const ::std::os::raw::c_char,
        text: *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GuiColorPicker(bounds: Rectangle, color: Color) -> Color;
}
extern "C" {
    pub fn GuiColorPanel(bounds: Rectangle, color: Color) -> Color;
}
extern "C" {
    pub fn GuiColorBarAlpha(bounds: Rectangle, alpha: f32) -> f32;
}
extern "C" {
    pub fn GuiColorBarHue(bounds: Rectangle, value: f32) -> f32;
}
extern "C" {
    pub fn GuiLoadStyle(fileName: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn GuiLoadStyleDefault();
}
extern "C" {
    pub fn GuiIconText(
        iconId: ::std::os::raw::c_int,
        text: *const ::std::os::raw::c_char,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn GuiDrawIcon(
        iconId: ::std::os::raw::c_int,
        position: Vector2,
        pixelSize: ::std::os::raw::c_int,
        color: Color,
    );
}
extern "C" {
    pub fn GuiGetIcons() -> *mut ::std::os::raw::c_uint;
}
extern "C" {
    pub fn GuiGetIconData(iconId: ::std::os::raw::c_int) -> *mut ::std::os::raw::c_uint;
}
extern "C" {
    pub fn GuiSetIconData(iconId: ::std::os::raw::c_int, data: *mut ::std::os::raw::c_uint);
}
extern "C" {
    pub fn GuiSetIconPixel(
        iconId: ::std::os::raw::c_int,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn GuiClearIconPixel(
        iconId: ::std::os::raw::c_int,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn GuiCheckIconPixel(
        iconId: ::std::os::raw::c_int,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
    ) -> bool;
}
extern "C" {
    pub static mut guiIcons: [::std::os::raw::c_uint; 2048usize];
}
extern "C" {
    pub static mut guiState: GuiControlState;
}
extern "C" {
    pub static mut guiFont: Font;
}
extern "C" {
    pub static mut guiLocked: bool;
}
pub const guiAlpha: f32 = 1.0;
extern "C" {
    pub static mut guiStyle: [::std::os::raw::c_uint; 384usize];
}
extern "C" {
    pub static mut guiStyleLoaded: bool;
}
extern "C" {
    pub static mut guiTooltip: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut guiTooltipEnabled: bool;
}
extern "C" {
    pub fn GuiSliderPro(
        bounds: Rectangle,
        textLeft: *const ::std::os::raw::c_char,
        textRight: *const ::std::os::raw::c_char,
        value: f32,
        minValue: f32,
        maxValue: f32,
        sliderWidth: ::std::os::raw::c_int,
    ) -> f32;
}
extern "C" {
    pub fn GuiColorPanelEx(bounds: Rectangle, color: Color, hue: f32) -> Color;
}
extern "C" {
    pub fn GuiLoadIcons(
        fileName: *const ::std::os::raw::c_char,
        loadIconsName: bool,
    ) -> *mut *mut ::std::os::raw::c_char;
}
extern "C" {
    pub static mut RLGL: rlglData;
}
extern "C" {
    pub static mut exts_i: *mut *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn __acrt_iob_func(_Ix: ::std::os::raw::c_uint) -> *mut FILE;
}
extern "C" {
    pub fn fgetwc(_Stream: *mut FILE) -> wint_t;
}
extern "C" {
    pub fn _fgetwchar() -> wint_t;
}
extern "C" {
    pub fn fputwc(_Character: wchar_t, _Stream: *mut FILE) -> wint_t;
}
extern "C" {
    pub fn _fputwchar(_Character: wchar_t) -> wint_t;
}
extern "C" {
    pub fn getwc(_Stream: *mut FILE) -> wint_t;
}
extern "C" {
    pub fn getwchar() -> wint_t;
}
extern "C" {
    pub fn fgetws(
        _Buffer: *mut wchar_t,
        _BufferCount: ::std::os::raw::c_int,
        _Stream: *mut FILE,
    ) -> *mut wchar_t;
}
extern "C" {
    pub fn fputws(_Buffer: *const wchar_t, _Stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _getws_s(_Buffer: *mut wchar_t, _BufferCount: size_t) -> *mut wchar_t;
}
extern "C" {
    pub fn putwc(_Character: wchar_t, _Stream: *mut FILE) -> wint_t;
}
extern "C" {
    pub fn putwchar(_Character: wchar_t) -> wint_t;
}
extern "C" {
    pub fn _putws(_Buffer: *const wchar_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ungetwc(_Character: wint_t, _Stream: *mut FILE) -> wint_t;
}
extern "C" {
    pub fn _wfdopen(_FileHandle: ::std::os::raw::c_int, _Mode: *const wchar_t) -> *mut FILE;
}
extern "C" {
    pub fn _wfopen(_FileName: *const wchar_t, _Mode: *const wchar_t) -> *mut FILE;
}
extern "C" {
    pub fn _wfopen_s(
        _Stream: *mut *mut FILE,
        _FileName: *const wchar_t,
        _Mode: *const wchar_t,
    ) -> errno_t;
}
extern "C" {
    pub fn _wfreopen(
        _FileName: *const wchar_t,
        _Mode: *const wchar_t,
        _OldStream: *mut FILE,
    ) -> *mut FILE;
}
extern "C" {
    pub fn _wfreopen_s(
        _Stream: *mut *mut FILE,
        _FileName: *const wchar_t,
        _Mode: *const wchar_t,
        _OldStream: *mut FILE,
    ) -> errno_t;
}
extern "C" {
    pub fn _wfsopen(
        _FileName: *const wchar_t,
        _Mode: *const wchar_t,
        _ShFlag: ::std::os::raw::c_int,
    ) -> *mut FILE;
}
extern "C" {
    pub fn _wpopen(_Command: *const wchar_t, _Mode: *const wchar_t) -> *mut FILE;
}
extern "C" {
    pub fn _wremove(_FileName: *const wchar_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _wtempnam(_Directory: *const wchar_t, _FilePrefix: *const wchar_t) -> *mut wchar_t;
}
extern "C" {
    pub fn _wtmpnam_s(_Buffer: *mut wchar_t, _BufferCount: size_t) -> errno_t;
}
extern "C" {
    pub fn _wtmpnam(_Buffer: *mut wchar_t) -> *mut wchar_t;
}
extern "C" {
    pub fn _fgetwc_nolock(_Stream: *mut FILE) -> wint_t;
}
extern "C" {
    pub fn _fputwc_nolock(_Character: wchar_t, _Stream: *mut FILE) -> wint_t;
}
extern "C" {
    pub fn _getwc_nolock(_Stream: *mut FILE) -> wint_t;
}
extern "C" {
    pub fn _putwc_nolock(_Character: wchar_t, _Stream: *mut FILE) -> wint_t;
}
extern "C" {
    pub fn _ungetwc_nolock(_Character: wint_t, _Stream: *mut FILE) -> wint_t;
}
extern "C" {
    pub fn __stdio_common_vfwprintf(
        _Options: ::std::os::raw::c_ulonglong,
        _Stream: *mut FILE,
        _Format: *const wchar_t,
        _Locale: _locale_t,
        _ArgList: va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __stdio_common_vfwprintf_s(
        _Options: ::std::os::raw::c_ulonglong,
        _Stream: *mut FILE,
        _Format: *const wchar_t,
        _Locale: _locale_t,
        _ArgList: va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __stdio_common_vfwprintf_p(
        _Options: ::std::os::raw::c_ulonglong,
        _Stream: *mut FILE,
        _Format: *const wchar_t,
        _Locale: _locale_t,
        _ArgList: va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __stdio_common_vfwscanf(
        _Options: ::std::os::raw::c_ulonglong,
        _Stream: *mut FILE,
        _Format: *const wchar_t,
        _Locale: _locale_t,
        _ArgList: va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __stdio_common_vswprintf(
        _Options: ::std::os::raw::c_ulonglong,
        _Buffer: *mut wchar_t,
        _BufferCount: size_t,
        _Format: *const wchar_t,
        _Locale: _locale_t,
        _ArgList: va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __stdio_common_vswprintf_s(
        _Options: ::std::os::raw::c_ulonglong,
        _Buffer: *mut wchar_t,
        _BufferCount: size_t,
        _Format: *const wchar_t,
        _Locale: _locale_t,
        _ArgList: va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __stdio_common_vsnwprintf_s(
        _Options: ::std::os::raw::c_ulonglong,
        _Buffer: *mut wchar_t,
        _BufferCount: size_t,
        _MaxCount: size_t,
        _Format: *const wchar_t,
        _Locale: _locale_t,
        _ArgList: va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __stdio_common_vswprintf_p(
        _Options: ::std::os::raw::c_ulonglong,
        _Buffer: *mut wchar_t,
        _BufferCount: size_t,
        _Format: *const wchar_t,
        _Locale: _locale_t,
        _ArgList: va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __stdio_common_vswscanf(
        _Options: ::std::os::raw::c_ulonglong,
        _Buffer: *const wchar_t,
        _BufferCount: size_t,
        _Format: *const wchar_t,
        _Locale: _locale_t,
        _ArgList: va_list,
    ) -> ::std::os::raw::c_int;
}
pub type fpos_t = ::std::os::raw::c_longlong;
extern "C" {
    pub fn _get_stream_buffer_pointers(
        _Stream: *mut FILE,
        _Base: *mut *mut *mut ::std::os::raw::c_char,
        _Pointer: *mut *mut *mut ::std::os::raw::c_char,
        _Count: *mut *mut ::std::os::raw::c_int,
    ) -> errno_t;
}
extern "C" {
    pub fn clearerr_s(_Stream: *mut FILE) -> errno_t;
}
extern "C" {
    pub fn fopen_s(
        _Stream: *mut *mut FILE,
        _FileName: *const ::std::os::raw::c_char,
        _Mode: *const ::std::os::raw::c_char,
    ) -> errno_t;
}
extern "C" {
    pub fn fread_s(
        _Buffer: *mut ::std::os::raw::c_void,
        _BufferSize: size_t,
        _ElementSize: size_t,
        _ElementCount: size_t,
        _Stream: *mut FILE,
    ) -> size_t;
}
extern "C" {
    pub fn freopen_s(
        _Stream: *mut *mut FILE,
        _FileName: *const ::std::os::raw::c_char,
        _Mode: *const ::std::os::raw::c_char,
        _OldStream: *mut FILE,
    ) -> errno_t;
}
extern "C" {
    pub fn gets_s(
        _Buffer: *mut ::std::os::raw::c_char,
        _Size: rsize_t,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn tmpfile_s(_Stream: *mut *mut FILE) -> errno_t;
}
extern "C" {
    pub fn tmpnam_s(_Buffer: *mut ::std::os::raw::c_char, _Size: rsize_t) -> errno_t;
}
extern "C" {
    pub fn clearerr(_Stream: *mut FILE);
}
extern "C" {
    pub fn fclose(_Stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _fcloseall() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _fdopen(
        _FileHandle: ::std::os::raw::c_int,
        _Mode: *const ::std::os::raw::c_char,
    ) -> *mut FILE;
}
extern "C" {
    pub fn feof(_Stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ferror(_Stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fflush(_Stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fgetc(_Stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _fgetchar() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fgetpos(_Stream: *mut FILE, _Position: *mut fpos_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fgets(
        _Buffer: *mut ::std::os::raw::c_char,
        _MaxCount: ::std::os::raw::c_int,
        _Stream: *mut FILE,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn _fileno(_Stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _flushall() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fopen(
        _FileName: *const ::std::os::raw::c_char,
        _Mode: *const ::std::os::raw::c_char,
    ) -> *mut FILE;
}
extern "C" {
    pub fn fputc(_Character: ::std::os::raw::c_int, _Stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _fputchar(_Character: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fputs(
        _Buffer: *const ::std::os::raw::c_char,
        _Stream: *mut FILE,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fread(
        _Buffer: *mut ::std::os::raw::c_void,
        _ElementSize: ::std::os::raw::c_ulonglong,
        _ElementCount: ::std::os::raw::c_ulonglong,
        _Stream: *mut FILE,
    ) -> ::std::os::raw::c_ulonglong;
}
extern "C" {
    pub fn freopen(
        _FileName: *const ::std::os::raw::c_char,
        _Mode: *const ::std::os::raw::c_char,
        _Stream: *mut FILE,
    ) -> *mut FILE;
}
extern "C" {
    pub fn _fsopen(
        _FileName: *const ::std::os::raw::c_char,
        _Mode: *const ::std::os::raw::c_char,
        _ShFlag: ::std::os::raw::c_int,
    ) -> *mut FILE;
}
extern "C" {
    pub fn fsetpos(_Stream: *mut FILE, _Position: *const fpos_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fseek(
        _Stream: *mut FILE,
        _Offset: ::std::os::raw::c_long,
        _Origin: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _fseeki64(
        _Stream: *mut FILE,
        _Offset: ::std::os::raw::c_longlong,
        _Origin: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ftell(_Stream: *mut FILE) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn _ftelli64(_Stream: *mut FILE) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn fwrite(
        _Buffer: *const ::std::os::raw::c_void,
        _ElementSize: ::std::os::raw::c_ulonglong,
        _ElementCount: ::std::os::raw::c_ulonglong,
        _Stream: *mut FILE,
    ) -> ::std::os::raw::c_ulonglong;
}
extern "C" {
    pub fn getc(_Stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getchar() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _getmaxstdio() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _getw(_Stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _pclose(_Stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _popen(
        _Command: *const ::std::os::raw::c_char,
        _Mode: *const ::std::os::raw::c_char,
    ) -> *mut FILE;
}
extern "C" {
    pub fn putc(_Character: ::std::os::raw::c_int, _Stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn putchar(_Character: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn puts(_Buffer: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _putw(_Word: ::std::os::raw::c_int, _Stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn remove(_FileName: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rename(
        _OldFileName: *const ::std::os::raw::c_char,
        _NewFileName: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _unlink(_FileName: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn unlink(_FileName: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rewind(_Stream: *mut FILE);
}
extern "C" {
    pub fn _rmtmp() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setbuf(_Stream: *mut FILE, _Buffer: *mut ::std::os::raw::c_char);
}
extern "C" {
    pub fn _setmaxstdio(_Maximum: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setvbuf(
        _Stream: *mut FILE,
        _Buffer: *mut ::std::os::raw::c_char,
        _Mode: ::std::os::raw::c_int,
        _Size: size_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _tempnam(
        _DirectoryName: *const ::std::os::raw::c_char,
        _FilePrefix: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn tmpfile() -> *mut FILE;
}
extern "C" {
    pub fn tmpnam(_Buffer: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn ungetc(_Character: ::std::os::raw::c_int, _Stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _lock_file(_Stream: *mut FILE);
}
extern "C" {
    pub fn _unlock_file(_Stream: *mut FILE);
}
extern "C" {
    pub fn _fclose_nolock(_Stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _fflush_nolock(_Stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _fgetc_nolock(_Stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _fputc_nolock(
        _Character: ::std::os::raw::c_int,
        _Stream: *mut FILE,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _fread_nolock(
        _Buffer: *mut ::std::os::raw::c_void,
        _ElementSize: size_t,
        _ElementCount: size_t,
        _Stream: *mut FILE,
    ) -> size_t;
}
extern "C" {
    pub fn _fread_nolock_s(
        _Buffer: *mut ::std::os::raw::c_void,
        _BufferSize: size_t,
        _ElementSize: size_t,
        _ElementCount: size_t,
        _Stream: *mut FILE,
    ) -> size_t;
}
extern "C" {
    pub fn _fseek_nolock(
        _Stream: *mut FILE,
        _Offset: ::std::os::raw::c_long,
        _Origin: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _fseeki64_nolock(
        _Stream: *mut FILE,
        _Offset: ::std::os::raw::c_longlong,
        _Origin: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _ftell_nolock(_Stream: *mut FILE) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn _ftelli64_nolock(_Stream: *mut FILE) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn _fwrite_nolock(
        _Buffer: *const ::std::os::raw::c_void,
        _ElementSize: size_t,
        _ElementCount: size_t,
        _Stream: *mut FILE,
    ) -> size_t;
}
extern "C" {
    pub fn _getc_nolock(_Stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _putc_nolock(
        _Character: ::std::os::raw::c_int,
        _Stream: *mut FILE,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _ungetc_nolock(
        _Character: ::std::os::raw::c_int,
        _Stream: *mut FILE,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __p__commode() -> *mut ::std::os::raw::c_int;
}
extern "C" {
    pub fn __stdio_common_vfprintf(
        _Options: ::std::os::raw::c_ulonglong,
        _Stream: *mut FILE,
        _Format: *const ::std::os::raw::c_char,
        _Locale: _locale_t,
        _ArgList: va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __stdio_common_vfprintf_s(
        _Options: ::std::os::raw::c_ulonglong,
        _Stream: *mut FILE,
        _Format: *const ::std::os::raw::c_char,
        _Locale: _locale_t,
        _ArgList: va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __stdio_common_vfprintf_p(
        _Options: ::std::os::raw::c_ulonglong,
        _Stream: *mut FILE,
        _Format: *const ::std::os::raw::c_char,
        _Locale: _locale_t,
        _ArgList: va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _set_printf_count_output(_Value: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _get_printf_count_output() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __stdio_common_vfscanf(
        _Options: ::std::os::raw::c_ulonglong,
        _Stream: *mut FILE,
        _Format: *const ::std::os::raw::c_char,
        _Locale: _locale_t,
        _Arglist: va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __stdio_common_vsprintf(
        _Options: ::std::os::raw::c_ulonglong,
        _Buffer: *mut ::std::os::raw::c_char,
        _BufferCount: size_t,
        _Format: *const ::std::os::raw::c_char,
        _Locale: _locale_t,
        _ArgList: va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __stdio_common_vsprintf_s(
        _Options: ::std::os::raw::c_ulonglong,
        _Buffer: *mut ::std::os::raw::c_char,
        _BufferCount: size_t,
        _Format: *const ::std::os::raw::c_char,
        _Locale: _locale_t,
        _ArgList: va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __stdio_common_vsnprintf_s(
        _Options: ::std::os::raw::c_ulonglong,
        _Buffer: *mut ::std::os::raw::c_char,
        _BufferCount: size_t,
        _MaxCount: size_t,
        _Format: *const ::std::os::raw::c_char,
        _Locale: _locale_t,
        _ArgList: va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __stdio_common_vsprintf_p(
        _Options: ::std::os::raw::c_ulonglong,
        _Buffer: *mut ::std::os::raw::c_char,
        _BufferCount: size_t,
        _Format: *const ::std::os::raw::c_char,
        _Locale: _locale_t,
        _ArgList: va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __stdio_common_vsscanf(
        _Options: ::std::os::raw::c_ulonglong,
        _Buffer: *const ::std::os::raw::c_char,
        _BufferCount: size_t,
        _Format: *const ::std::os::raw::c_char,
        _Locale: _locale_t,
        _ArgList: va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tempnam(
        _Directory: *const ::std::os::raw::c_char,
        _FilePrefix: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn fcloseall() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fdopen(
        _FileHandle: ::std::os::raw::c_int,
        _Format: *const ::std::os::raw::c_char,
    ) -> *mut FILE;
}
extern "C" {
    pub fn fgetchar() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fileno(_Stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn flushall() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fputchar(_Ch: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getw(_Stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn putw(_Ch: ::std::os::raw::c_int, _Stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rmtmp() -> ::std::os::raw::c_int;
}
extern "C" {
    pub static mut max_loaded_major: ::std::os::raw::c_int;
}
extern "C" {
    pub static mut max_loaded_minor: ::std::os::raw::c_int;
}
extern "C" {
    pub static mut exts: *const ::std::os::raw::c_char;
}
