require 'ffi'

module Raylib
    extend FFI::Library

    # We load the lib
    ffi_lib 'raylib.dll'


    







    



    

    

    # Music enum
    enum :MusicContextType, [:MUSIC_AUDIO_OGG, 0, 
                            :MUSIC_AUDIO_FLAC,
                            :MUSIC_AUDIO_MP3,
                            :MUSIC_MODULE_XM,
                            :MUSIC_MODULE_MOD ]

    CAMERA_PERSPECTIVE = 0
    CAMERA_ORTHOGRAPHIC = 1


    # Structs
    # Vector2 type
    class Vector2 < FFI::Struct
        layout 	:x, :float,
                :y, :float

        def x; self[:x]; end
        def y; self[:y]; end

        def x=(v); self[:x] = v; end
        def y=(v); self[:y] = v; end
    end
    # Vector3 type
    class Vector3 < FFI::Struct
        layout 	:x, :float,
                :y, :float,
                :z, :float

        def x; self[:x]; end
        def y; self[:y]; end
        def z; self[:z]; end

        def x=(v); self[:x] = v; end
        def y=(v); self[:y] = v; end
        def z=(v); self[:z] = v; end
    end
    # Vector4 type
    class Vector4 < FFI::Struct
        layout 	:x, :float,
                :y, :float,
                :z, :float,
                :w, :float

        def x; self[:x]; end
        def y; self[:y]; end
        def z; self[:z]; end
        def w; self[:w]; end

        def x=(v); self[:x] = v; end
        def y=(v); self[:y] = v; end
        def z=(v); self[:z] = v; end
        def w=(v); self[:w] = v; end
    end
    # Quaternion type, same as Vector4
    # Matrix type (OpenGL style 4x4 - right handed, column major)
    class Matrix < FFI::Struct
        layout 	:m0, :float,
                :m4, :float,
                :m8, :float,
                :m12, :float,
                :m1, :float,
                :m5, :float,
                :m9, :float,
                :m13, :float,
                :m2, :float,
                :m6, :float,
                :m10, :float,
                :m14, :float,
                :m3, :float,
                :m7, :float,
                :m11, :float,
                :m15, :float
    end
    # Color type, RGBA (32bit)
    class Color < FFI::Struct
        layout 	:r, :uchar,
                :g, :uchar,
                :b, :uchar,
                :a, :uchar
        def r; self[:r]; end
        def g; self[:g]; end
        def b; self[:b]; end
        def a; self[:a]; end

        def r=(v); self[:r] = v; end
        def g=(v); self[:g] = v; end
        def b=(v); self[:b] = v; end
        def a=(v); self[:a] = v; end
    end
    # Rectangle type
    class Rectangle < FFI::Struct
        layout 	:x, :float,
                :y, :float,
                :width, :float,
                :height, :float

        def x; self[:x]; end
        def y; self[:y]; end
        def width; self[:width]; end
        def height; self[:height]; end

        def x=(v); self[:x] = v; end
        def y=(v); self[:y] = v; end
        def width=(v); self[:width] = v; end
        def height=(v); self[:height] = v; end
    end
    # Image type, bpp always RGBA (32bit)
    # NOTE: Data stored in CPU memory (RAM)
    class Image < FFI::Struct
        layout 	:data, :pointer,
                :width, :int,
                :height, :int,
                :mipmaps, :int,
                :format, :int
    end
    # Texture2D type
    # NOTE: Data stored in GPU memory
    class Texture2D < FFI::Struct
        layout 	:id, :uint,
                :width, :int,
                :height, :int,
                :mipmaps, :int,
                :format, :int
    end
    # Texture type, same as Texture2D
    # RenderTexture2D type, for texture rendering
    class RenderTexture2D < FFI::Struct
        layout 	:id, :uint,
                :texture, Texture2D,
                :depth, Texture2D
    end
    # RenderTexture type, same as RenderTexture2D
    # Font character info
    class CharInfo < FFI::Struct
        layout 	:value, :int,
    # Character rectangle in sprite font
                :offsetX, :int,
                :offsetY, :int,
                :advanceX, :int,
                :data, :uchar
    end
    # Font type, includes texture and charSet array data
    class Font < FFI::Struct
        layout 	:texture, Texture2D.by_value,
                :baseSize, :int,
                :charsCount, :int,
                :chars, CharInfo
    end
    #define SpriteFont Font# SpriteFont type fallback, defaults to Font
    # Camera type, defines a camera position/orientation in 3d space
    class Camera < FFI::Struct
        layout 	:position, Vector3.by_value,
                :target, Vector3.by_value,
                :up, Vector3.by_value,
                :fovy, :float,
                :type, :int

        def position; self[:position]; end
        def target; self[:target]; end
        def up; self[:up]; end
        def fovy; self[:fovy]; end
        def type; self[:type]; end

        def position=(v); self[:position] = v; end
        def target=(v); self[:target] = v; end
        def up=(v); self[:up] = v; end
        def fovy=(v); self[:fovy] = v; end
        def type=(v); self[:type] = v; end
    end
    #define Camera Camera3D# Camera type fallback, defaults to Camera3D
    # Camera2D type, defines a 2d camera
    class Camera2D < FFI::Struct
        layout 	:offset, Vector2.by_value,
                :target, Vector2.by_value,
                :rotation, :float,
                :zoom, :float

        def offset; self[:offset]; end
        def target; self[:target]; end
        def rotation; self[:rotation]; end
        def zoom; self[:zoom]; end

        def offset=(v); self[:offset] = v; end
        def target=(v); self[:target] = v; end
        def rotation=(v); self[:rotation] = v; end
        def zoom=(v); self[:zoom] = v; end
    end
    # Bounding box type
    class BoundingBox < FFI::Struct
        layout 	:min, Vector3.by_value,
                :max, Vector3.by_value
    end
    # Vertex data definning a mesh
    # NOTE: Data stored in CPU memory (and GPU)
    class Mesh < FFI::Struct
        layout 	:vertexCount, :int,
                :triangleCount, :int,
                :vertices, :pointer,
                :texcoords, :pointer,
                :texcoords2, :pointer,
                :normals, :pointer,
                :tangents, :pointer,
                :colors, :pointer,
                :indices, :pointer,
    # Vertex indices (in case vertex data comes indexed)
                :vaoId, :uint,
                :vboId, [:uint, 7]
    end
    # Shader type (generic)
    class Shader < FFI::Struct
        layout 	:id, :uint,
                :locs, [:int, MAX_SHADER_LOCATIONS]
    end
    # Material texture map
    class MaterialMap < FFI::Struct
        layout 	:texture, Texture2D.by_value,
                :color, Color.by_value,
                :value, :float

        def texture; self[:texture]; end
        def color; self[:color]; end
        def value; self[:value]; end

        def texture=(v); self[:texture] = v; end
        def color=(v); self[:color] = v; end
        def value=(v); self[:value] = v; end
    end
    # Material type (generic)
    class Material < FFI::Struct
        layout 	:shader, Shader.by_value,
                :maps, [MaterialMap.by_value, MAX_MATERIAL_MAPS],
                :params, :pointer

        def shader; self[:shader]; end
        def maps; self[:maps]; end
        def value; self[:value]; end

        def shader=(v); self[:shader] = v; end
        def maps=(v); self[:maps] = v; end
        def value=(v); self[:value] = v; end
      end
    # Model type
    class Model < FFI::Struct
        layout 	:mesh, Mesh.by_value,
                :transform, Matrix.by_value,
                :material, Material.by_value

        def mesh; self[:mesh]; end
        def transform; self[:transform]; end
        def material; self[:material]; end

        def mesh=(v); self[:mesh] = v; end
        def transform=(v); self[:transform] = v; end
        def material=(v); self[:material] = v; end
    end
    # Ray type (useful for raycast)
    class Ray < FFI::Struct
        layout 	:position, Vector3.by_value,
                :direction, Vector3.by_value

        def position; self[:position]; end
        def direction; self[:direction]; end

        def position=(v); self[:position] = v; end
        def direction=(v); self[:direction] = v; end
    end
    # Raycast hit information
    class RayHitInfo < FFI::Struct
        layout 	:hit, :bool,
                :distance, :float,
                :position, Vector3.by_value,
                :normal, Vector3.by_value
    end
    # Wave type, defines audio wave data
    class Wave < FFI::Struct
        layout 	:sampleCount, :uint,
                :sampleRate, :uint,
                :sampleSize, :uint,
                :channels, :uint,
                :data, :pointer
    end
    # Sound source type
    class Sound < FFI::Struct
        layout 	:audioBuffer, :pointer,
                :source, :uint,
                :buffer, :uint,
                :format, :int
    end
    # Music type (file streaming from memory)
    # NOTE: Anything longer than ~10 seconds should be streamed
    # class Music < FFI::Struct # Audio stream type
    # NOTE: Useful to create custom audio streams not bound to a specific file
    class AudioStream < FFI::Struct
        layout 	:sampleRate, :uint,
                :sampleSize, :uint,
                :channels, :uint,
                :audioBuffer, :pointer,
                :format, :int,
                :source, :uint,
                :buffers, [:uint, 2]
    end
    # Head-Mounted-Display device parameters
    class VrDeviceInfo < FFI::Struct
        layout 	:hResolution, :int,
                :vResolution, :int,
                :hScreenSize, :float,
                :vScreenSize, :float,
                :vScreenCenter, :float,
                :eyeToScreenDistance, :float,
                :lensSeparationDistance, :float,
                :interpupillaryDistance, :float,
                :lensDistortionValues, [:float, 4],
                :chromaAbCorrection, [:float, 4]
    end

    ## SOME FUNCTIONS ##
    def Raylib.new_color(r, g, b, a)
        c = Color.new
        c.r = r
        c.g = g
        c.b = b
        c.a = a

        return c
    end

    def Raylib.new_rectangle(x, y, width, height)
        rect = Rectangle.new
        rect.x = x
        rect.y = y
        rect.width = width
        rect.height = height

        return rect
    end

    def Raylib.new_camera2d()
        cam = Camera2D.new
        return cam
    end

    def Raylib.new_camera()
        cam = Camera.new
        return cam
    end

    def Raylib.new_vector2(x, y)
        vec2 = Vector2.new
        vec2[:x] = x.to_f
        vec2[:y] = y.to_f

        return vec2
    end

    def Raylib.new_vector3(x, y, z)
        vec3 = Vector3.new
        vec3[:x] = x.to_f
        vec3[:y] = y.to_f
        vec3[:z] = z.to_f

        return vec3
    end

    def Raylib.new_bounding_box(min, max)
        bb = BoundingBox.new
        bb[:min] = min
        bb[:max] = max

        return bb
    end

    def Raylib.new_ray()
        ray = Ray.new
        return ray
    end
    
    ## SOME OTHER DEFINES ##

    # Colors
    LIGHTGRAY = new_color(200, 200, 200, 255)   # Light Gray
    GRAY      = new_color(130, 130, 130, 255)   # Gray
    DARKGRAY  = new_color(80, 80, 80, 255)      # Dark Gray
    YELLOW    = new_color(253, 249, 0, 255)     # Yellow
    GOLD      = new_color(255, 203, 0, 255)     # Gold
    ORANGE    = new_color(255, 161, 0, 255)     # Orange
    PINK      = new_color(255, 109, 194, 255)   # Pink
    RED       = new_color(230, 41, 55, 255)     # Red
    MAROON    = new_color(190, 33, 55, 255)     # Maroon
    GREEN     = new_color(0, 228, 48, 255)      # Green
    LIME      = new_color(0, 158, 47, 255)      # Lime
    DARKGREEN = new_color(0, 117, 44, 255)      # Dark Green
    SKYBLUE   = new_color(102, 191, 255, 255)   # Sky Blue
    BLUE      = new_color(0, 121, 241, 255)     # Blue
    DARKBLUE  = new_color(0, 82, 172, 255)      # Dark Blue
    PURPLE    = new_color(200, 122, 255, 255)   # Purple
    VIOLET    = new_color(135, 60, 190, 255)    # Violet
    DARKPURPLE= new_color(112, 31, 126, 255)    # Dark Purple
    BEIGE     = new_color(211, 176, 131, 255)   # Beige
    BROWN     = new_color(127, 106, 79, 255)    # Brown
    DARKBROWN = new_color(76, 63, 47, 255)      # Dark Brown

    WHITE     = new_color(255, 255, 255, 255)   # White
    BLACK     = new_color(0, 0, 0, 255)         # Black
    BLANK     = new_color(0, 0, 0, 0)           # Blank (Transparent)
    MAGENTA   = new_color(255, 0, 255, 255)     # Magenta
    RAYWHITE  = new_color(245, 245, 245, 255)   # My own White (raylib logo)

    

    

    
    

    
    
    

    

    

    
    
    

    

    

    

    

    

    

    

    # Window-related functions

    attach_function :InitWindow, [:int, :int, :string], :void
    attach_function :CloseWindow, [], :void
    attach_function :IsWindowReady, [], :bool
    attach_function :WindowShouldClose, [], :bool
    attach_function :IsWindowMinimized, [], :bool
    attach_function :ToggleFullscreen, [], :void
    attach_function :SetWindowIcon, [Image.by_value], :void
    attach_function :SetWindowTitle, [], :void
    attach_function :SetWindowPosition, [:int, :int], :void
    attach_function :SetWindowMonitor, [:int], :void
    attach_function :SetWindowMinSize, [:int, :int], :void
    attach_function :SetWindowSize, [:int, :int], :void
    attach_function :GetScreenWidth, [], :int
    attach_function :GetScreenHeight, [], :int

    # Cursor-related functions

    attach_function :ShowCursor, [], :void
    attach_function :HideCursor, [], :void
    attach_function :IsCursorHidden, [], :bool
    attach_function :EnableCursor, [], :void
    attach_function :DisableCursor, [], :void

    # Drawing-related functions

    attach_function :ClearBackground, [Color.by_value], :void
    attach_function :BeginDrawing, [], :void
    attach_function :EndDrawing, [], :void
    attach_function :BeginMode2D, [Camera2D.by_value], :void
    attach_function :EndMode2D, [], :void
    attach_function :BeginMode3D, [Camera.by_value], :void
    attach_function :EndMode3D, [], :void
    attach_function :BeginTextureMode, [], :void
    attach_function :EndTextureMode, [], :void

    # Screen-space-related functions

    attach_function :GetMouseRay, [Vector2.by_value, Camera.by_value], Ray.by_value
    attach_function :GetWorldToScreen, [Vector3.by_value, Camera.by_value], Vector2.by_value
    attach_function :GetCameraMatrix, [Camera.by_value], Matrix.by_value

    # Timing-related functions

    attach_function :SetTargetFPS, [:int], :void
    attach_function :GetFPS, [], :int
    attach_function :GetFrameTime, [], :float

    # Color-related functions

    attach_function :ColorToInt, [Color.by_value], :int
    attach_function :ColorNormalize, [Color.by_value], Vector4.by_value
    attach_function :ColorToHSV, [Color.by_value], Vector3.by_value
    attach_function :GetColor, [:int], Color.by_value
    attach_function :Fade, [Color.by_value, :float], Color.by_value

    # Misc. functions

    attach_function :ShowLogo, [], :void
    attach_function :SetConfigFlags, [], :void
    attach_function :SetTraceLog, [], :void
    attach_function :TraceLog, [:int], :void
    attach_function :TakeScreenshot, [], :void
    attach_function :GetRandomValue, [:int, :int], :int

    # Files management functions

    attach_function :IsFileExtension, [], :bool
    attach_function :ChangeDirectory, [], :bool
    attach_function :IsFileDropped, [], :bool
    attach_function :ClearDroppedFiles, [], :void

    # Persistent storage management

    attach_function :StorageSaveValue, [:int, :int], :void
    attach_function :StorageLoadValue, [:int], :int

    # Input-related functions: keyboard

    attach_function :IsKeyPressed, [:int], :bool
    attach_function :IsKeyDown, [:int], :bool
    attach_function :IsKeyReleased, [:int], :bool
    attach_function :IsKeyUp, [:int], :bool
    attach_function :GetKeyPressed, [], :int
    attach_function :SetExitKey, [:int], :void

    # Input-related functions: gamepads

    attach_function :IsGamepadAvailable, [:int], :bool
    attach_function :IsGamepadName, [:int], :bool
    attach_function :IsGamepadButtonPressed, [:int, :int], :bool
    attach_function :IsGamepadButtonDown, [:int, :int], :bool
    attach_function :IsGamepadButtonReleased, [:int, :int], :bool
    attach_function :IsGamepadButtonUp, [:int, :int], :bool
    attach_function :GetGamepadButtonPressed, [], :int
    attach_function :GetGamepadAxisCount, [:int], :int
    attach_function :GetGamepadAxisMovement, [:int, :int], :float

    # Input-related functions: mouse

    attach_function :IsMouseButtonPressed, [:int], :bool
    attach_function :IsMouseButtonDown, [:int], :bool
    attach_function :IsMouseButtonReleased, [:int], :bool
    attach_function :IsMouseButtonUp, [:int], :bool
    attach_function :GetMouseX, [], :int
    attach_function :GetMouseY, [], :int
    attach_function :GetMousePosition, [], Vector2.by_value
    attach_function :SetMousePosition, [Vector2.by_value], :void
    attach_function :GetMouseWheelMove, [], :int

    # Input-related functions: touch

    attach_function :GetTouchX, [], :int
    attach_function :GetTouchY, [], :int
    attach_function :GetTouchPosition, [:int], Vector2.by_value

    # Gestures-related functions

    attach_function :SetGesturesEnabled, [:int], :void
    attach_function :IsGestureDetected, [:int], :bool
    attach_function :GetGestureDetected, [], :int
    attach_function :GetTouchPointsCount, [], :int
    attach_function :GetGestureHoldDuration, [], :float
    attach_function :GetGestureDragVector, [], Vector2.by_value
    attach_function :GetGestureDragAngle, [], :float
    attach_function :GetGesturePinchVector, [], Vector2.by_value
    attach_function :GetGesturePinchAngle, [], :float

    # Camera-related functions

    attach_function :SetCameraMode, [Camera.by_value, :int], :void
    attach_function :UpdateCamera, [Camera], :void
    attach_function :SetCameraPanControl, [:int], :void
    attach_function :SetCameraAltControl, [:int], :void
    attach_function :SetCameraSmoothZoomControl, [:int], :void
    attach_function :SetCameraMoveControls, [:int, :int], :void

    # Basic shapes drawing functions

    attach_function :DrawPixel, [:int, :int, Color.by_value], :void
    attach_function :DrawPixelV, [Vector2.by_value, Color.by_value], :void
    attach_function :DrawLine, [:int, :int, :int, :int, Color.by_value], :void
    attach_function :DrawLineV, [Vector2.by_value, Vector2.by_value, Color.by_value], :void
    attach_function :DrawLineEx, [Vector2.by_value, Vector2.by_value, :float, Color.by_value], :void
    attach_function :DrawLineBezier, [Vector2.by_value, Vector2.by_value, :float, Color.by_value], :void
    attach_function :DrawCircle, [:int, :int, :float, Color.by_value], :void
    attach_function :DrawCircleGradient, [:int, :int, :float, Color.by_value, Color.by_value], :void
    attach_function :DrawCircleV, [Vector2.by_value, :float, Color.by_value], :void
    attach_function :DrawCircleLines, [:int, :int, :float, Color.by_value], :void
    attach_function :DrawRectangle, [:int, :int, :int, :int, Color.by_value], :void
    attach_function :DrawRectangleV, [Vector2.by_value, Vector2.by_value, Color.by_value], :void
    attach_function :DrawRectangleRec, [Rectangle.by_value, Color.by_value], :void
    attach_function :DrawRectanglePro, [Rectangle.by_value, Vector2.by_value, :float, Color.by_value], :void
    attach_function :DrawRectangleGradientV, [:int, :int, :int, :int, Color.by_value, Color.by_value], :void
    attach_function :DrawRectangleGradientH, [:int, :int, :int, :int, Color.by_value, Color.by_value], :void
    attach_function :DrawRectangleGradientEx, [Rectangle.by_value, Color.by_value, Color.by_value, Color.by_value, Color.by_value], :void
    attach_function :DrawRectangleLines, [:int, :int, :int, :int, Color.by_value], :void
    attach_function :DrawRectangleLinesEx, [Rectangle.by_value, :int, Color.by_value], :void
    attach_function :DrawTriangle, [Vector2.by_value, Vector2.by_value, Vector2.by_value, Color.by_value], :void
    attach_function :DrawTriangleLines, [Vector2.by_value, Vector2.by_value, Vector2.by_value, Color.by_value], :void
    attach_function :DrawPoly, [Vector2.by_value, :int, :float, :float, Color.by_value], :void
    attach_function :DrawPolyEx, [Vector2.by_value, :int, Color.by_value], :void
    attach_function :DrawPolyExLines, [Vector2.by_value, :int, Color.by_value], :void

    # Basic shapes collision detection functions

    attach_function :CheckCollisionRecs, [Rectangle.by_value, Rectangle.by_value], :bool
    attach_function :CheckCollisionCircles, [Vector2.by_value, :float, Vector2.by_value, :float], :bool
    attach_function :CheckCollisionCircleRec, [Vector2.by_value, :float, Rectangle.by_value], :bool
    attach_function :GetCollisionRec, [Rectangle.by_value, Rectangle.by_value], Rectangle.by_value
    attach_function :CheckCollisionPointRec, [Vector2.by_value, Rectangle.by_value], :bool
    attach_function :CheckCollisionPointCircle, [Vector2.by_value, Vector2.by_value, :float], :bool
    attach_function :CheckCollisionPointTriangle, [Vector2.by_value, Vector2.by_value, Vector2.by_value, Vector2.by_value], :bool

    # Image/Texture2D data loading/unloading/saving functions

    attach_function :LoadImage, [:string], Image.by_value
    attach_function :LoadImageEx, [Color.by_value, :int, :int], Image.by_value
    attach_function :LoadImagePro, [:int, :int, :int], Image.by_value
    attach_function :LoadImageRaw, [:int, :int, :int, :int], Image.by_value
    attach_function :ExportImage, [Image.by_value], :void
    attach_function :LoadTexture, [:string], Texture2D.by_value
    attach_function :LoadTextureFromImage, [Image.by_value], Texture2D.by_value
    attach_function :UnloadImage, [Image.by_value], :void
    attach_function :UnloadTexture, [Texture2D.by_value], :void
    attach_function :UnloadRenderTexture, [], :void
    attach_function :GetImageData, [Image.by_value], :pointer
    attach_function :GetImageDataNormalized, [Image.by_value], :pointer
    attach_function :GetPixelDataSize, [:int, :int, :int], :int
    attach_function :GetTextureData, [Texture2D.by_value], Image.by_value
    attach_function :UpdateTexture, [Texture2D.by_value], :void

    # Image manipulation functions

    attach_function :ImageCopy, [Image.by_value], Image.by_value
    attach_function :ImageToPOT, [Image.by_value, Color.by_value], :void
    attach_function :ImageFormat, [Image.by_value, :int], :void
    attach_function :ImageAlphaMask, [Image.by_value, Image.by_value], :void
    attach_function :ImageAlphaClear, [Image.by_value, Color.by_value, :float], :void
    attach_function :ImageAlphaCrop, [Image.by_value, :float], :void
    attach_function :ImageAlphaPremultiply, [Image.by_value], :void
    attach_function :ImageCrop, [Image.by_value, Rectangle.by_value], :void
    attach_function :ImageResize, [Image.by_value, :int, :int], :void
    attach_function :ImageResizeNN, [Image.by_value, :int], :void
    attach_function :ImageResizeCanvas, [Image.by_value, :int, :int], :void
    attach_function :ImageMipmaps, [Image.by_value], :void
    attach_function :ImageDither, [Image.by_value, :int, :int, :int, :int], :void
    attach_function :ImageText, [:int, Color.by_value], Image.by_value
    attach_function :ImageTextEx, [Font.by_value, :float, :float, Color.by_value], Image.by_value
    attach_function :ImageDraw, [Image.by_value, Image.by_value, Rectangle.by_value, Rectangle.by_value], :void
    attach_function :ImageDrawRectangle, [Image.by_value, Vector2.by_value, Rectangle.by_value, Color.by_value], :void
    attach_function :ImageDrawText, [Image.by_value, Vector2.by_value, :int, Color.by_value], :void
    attach_function :ImageDrawTextEx, [Image.by_value, Vector2.by_value, Font.by_value], :void
    attach_function :ImageFlipVertical, [Image.by_value], :void
    attach_function :ImageFlipHorizontal, [Image.by_value], :void
    attach_function :ImageRotateCW, [Image.by_value], :void
    attach_function :ImageRotateCCW, [Image.by_value], :void
    attach_function :ImageColorTint, [Image.by_value, Color.by_value], :void
    attach_function :ImageColorInvert, [Image.by_value], :void
    attach_function :ImageColorGrayscale, [Image.by_value], :void
    attach_function :ImageColorContrast, [Image.by_value, :float], :void
    attach_function :ImageColorBrightness, [Image.by_value, :int], :void
    attach_function :ImageColorReplace, [Image.by_value, Color.by_value, Color.by_value], :void

    # Image generation functions

    attach_function :GenImageColor, [:int, :int, Color.by_value], Image.by_value
    attach_function :GenImageGradientV, [:int, :int, Color.by_value, Color.by_value], Image.by_value
    attach_function :GenImageGradientH, [:int, :int, Color.by_value, Color.by_value], Image.by_value
    attach_function :GenImageGradientRadial, [:int, :int, :float, Color.by_value, Color.by_value], Image.by_value
    attach_function :GenImageChecked, [:int, :int, :int, :int, Color.by_value, Color.by_value], Image.by_value
    attach_function :GenImageWhiteNoise, [:int, :int, :float], Image.by_value
    attach_function :GenImagePerlinNoise, [:int, :int, :int, :int, :float], Image.by_value
    attach_function :GenImageCellular, [:int, :int, :int], Image.by_value

    # Texture2D configuration functions

    attach_function :GenTextureMipmaps, [Texture2D.by_value], :void
    attach_function :SetTextureFilter, [Texture2D.by_value, :int], :void
    attach_function :SetTextureWrap, [Texture2D.by_value, :int], :void

    # Texture2D drawing functions

    attach_function :DrawTexture, [Texture2D.by_value, :int, :int, Color.by_value], :void
    attach_function :DrawTextureV, [Texture2D.by_value, Vector2.by_value, Color.by_value], :void
    attach_function :DrawTextureEx, [Texture2D.by_value, Vector2.by_value, :float, :float, Color.by_value], :void
    attach_function :DrawTextureRec, [Texture2D.by_value, Rectangle.by_value, Vector2.by_value, Color.by_value], :void
    attach_function :DrawTexturePro, [Texture2D.by_value, Rectangle.by_value, Rectangle.by_value, Vector2.by_value], :void
    
    
    # Font loading/unloading functions

    attach_function :GetFontDefault, [], Font.by_value
    attach_function :LoadFont, [], Font.by_value
    attach_function :LoadFontEx, [:int, :int, :int], Font.by_value
    attach_function :LoadFontData, [:int, :int, :int, :bool], CharInfo
    attach_function :GenImageFontAtlas, [CharInfo.by_value, :int, :int, :int, :int], Image.by_value
    attach_function :UnloadFont, [Font.by_value], :void

    # Text drawing functions

    attach_function :DrawFPS, [:int, :int], :void
    attach_function :DrawText, [:string, :int, :int, :int, Color.by_value], :void
    attach_function :DrawTextEx, [:string, Font.by_value, :string, Vector2.by_value, :float, :float, Color.by_value], :void

    # Text misc. functions

    attach_function :MeasureText, [:string, :int], :int
    attach_function :MeasureTextEx, [:string, Font.by_value, :float, :float], Vector2.by_value
    attach_function :GetGlyphIndex, [Font.by_value, :int], :int

    # Basic geometric 3D shapes drawing functions

    attach_function :DrawLine3D, [Vector3.by_value, Vector3.by_value, Color.by_value], :void
    attach_function :DrawCircle3D, [Vector3.by_value, :float, Vector3.by_value], :void
    attach_function :DrawCube, [Vector3.by_value, :float, :float, :float, Color.by_value], :void
    attach_function :DrawCubeV, [Vector3.by_value, Vector3.by_value, Color.by_value], :void
    attach_function :DrawCubeWires, [Vector3.by_value, :float, :float, :float, Color.by_value], :void
    attach_function :DrawCubeTexture, [Vector3.by_value, :float], :void
    attach_function :DrawSphere, [Vector3.by_value, :float, Color.by_value], :void
    attach_function :DrawSphereEx, [Vector3.by_value, :float, :int, :int, Color.by_value], :void
    attach_function :DrawSphereWires, [Vector3.by_value, :float, :int, :int, Color.by_value], :void
    attach_function :DrawCylinder, [Vector3.by_value, :float, :float], :void
    attach_function :DrawCylinderWires, [Vector3.by_value, :float, :float], :void
    attach_function :DrawPlane, [Vector3.by_value, Vector2.by_value, Color.by_value], :void
    attach_function :DrawRay, [Ray.by_value, Color.by_value], :void
    attach_function :DrawGrid, [:int, :float], :void
    attach_function :DrawGizmo, [Vector3.by_value], :void

    # Model loading/unloading functions

    attach_function :LoadModel, [:string], Model.by_value
    attach_function :LoadModelFromMesh, [Mesh.by_value], Model.by_value
    attach_function :UnloadModel, [Model.by_value], :void

    # Mesh loading/unloading functions

    attach_function :LoadMesh, [], Mesh.by_value
    attach_function :UnloadMesh, [Mesh.by_value], :void
    attach_function :ExportMesh, [Mesh.by_value], :void

    # Mesh manipulation functions

    attach_function :MeshTangents, [Mesh.by_value], :void
    attach_function :MeshBinormals, [Mesh.by_value], :void

    # Mesh generation functions

    attach_function :GenMeshPlane, [:float, :float, :int, :int], Mesh.by_value
    attach_function :GenMeshCube, [:float, :float, :float], Mesh.by_value
    attach_function :GenMeshSphere, [:float, :int, :int], Mesh.by_value
    attach_function :GenMeshHemiSphere, [:float, :int, :int], Mesh.by_value
    attach_function :GenMeshCylinder, [:float, :float, :int], Mesh.by_value
    attach_function :GenMeshTorus, [:float, :float, :int, :int], Mesh.by_value
    attach_function :GenMeshKnot, [:float, :float, :int, :int], Mesh.by_value
    attach_function :GenMeshHeightmap, [Image.by_value, Vector3.by_value], Mesh.by_value
    attach_function :GenMeshCubicmap, [Image.by_value, Vector3.by_value], Mesh.by_value

    # Material loading/unloading functions

    attach_function :LoadMaterial, [], Material.by_value
    attach_function :LoadMaterialDefault, [], Material.by_value
    attach_function :UnloadMaterial, [Material.by_value], :void

    # Model drawing functions

    attach_function :DrawModel, [Model.by_value, Vector3.by_value, :float, Color.by_value], :void
    attach_function :DrawModelEx, [Model.by_value, Vector3.by_value, Vector3.by_value], :void
    attach_function :DrawModelWires, [Model.by_value, Vector3.by_value, :float, Color.by_value], :void
    attach_function :DrawModelWiresEx, [Model.by_value, Vector3.by_value, Vector3.by_value], :void
    attach_function :DrawBoundingBox, [Color.by_value], :void
    attach_function :DrawBillboard, [Camera.by_value, Vector3.by_value, :float, Color.by_value], :void
    attach_function :DrawBillboardRec, [Camera.by_value, Rectangle.by_value], :void

    # Collision detection functions

    attach_function :CheckCollisionSpheres, [Vector3.by_value, :float, Vector3.by_value, :float], :bool
    attach_function :CheckCollisionBoxes, [Vector3.by_value, Vector3.by_value, Vector3.by_value, Vector3.by_value], :bool
    attach_function :CheckCollisionBoxSphere, [Vector3.by_value, Vector3.by_value, Vector3.by_value, :float], :bool
    attach_function :CheckCollisionRaySphere, [Ray.by_value, Vector3.by_value, :float], :bool
    attach_function :CheckCollisionRaySphereEx, [Ray.by_value, Vector3.by_value, :float, Vector3.by_value], :bool
    attach_function :CheckCollisionRayBox, [Ray.by_value, BoundingBox.by_value], :bool
    attach_function :GetCollisionRayModel, [Ray.by_value, Model.by_value], RayHitInfo.by_value
    attach_function :GetCollisionRayTriangle, [Ray.by_value, Vector3.by_value, Vector3.by_value, Vector3.by_value], RayHitInfo.by_value
    attach_function :GetCollisionRayGround, [Ray.by_value, :float], RayHitInfo.by_value

    # Shader loading/unloading functions

    attach_function :LoadShader, [], Shader.by_value
    attach_function :LoadShaderCode, [], Shader.by_value
    attach_function :UnloadShader, [Shader.by_value], :void
    attach_function :GetShaderDefault, [], Shader.by_value

    # Shader access functions

    attach_function :GetShaderLocation, [Shader.by_value], :int
    attach_function :SetShaderValue, [Shader.by_value, :int, :float, :int], :void
    attach_function :SetShaderValuei, [Shader.by_value, :int, :int, :int], :void
    attach_function :SetShaderValueMatrix, [Shader.by_value, :int, Matrix.by_value], :void
    attach_function :SetMatrixProjection, [Matrix.by_value], :void
    attach_function :SetMatrixModelview, [Matrix.by_value], :void
    attach_function :GetMatrixModelview, [], Matrix.by_value

    # Shading beegin/end functions

    attach_function :BeginShaderMode, [Shader.by_value], :void
    attach_function :EndShaderMode, [], :void
    attach_function :BeginBlendMode, [:int], :void
    attach_function :EndBlendMode, [], :void

    # VR control functions

    attach_function :InitVrSimulator, [], :void
    attach_function :CloseVrSimulator, [], :void
    attach_function :IsVrSimulatorReady, [], :bool
    attach_function :UpdateVrTracking, [Camera.by_value], :void
    attach_function :ToggleVrMode, [], :void
    attach_function :BeginVrDrawing, [], :void
    attach_function :EndVrDrawing, [], :void

    # Audio device management functions

    attach_function :InitAudioDevice, [], :void
    attach_function :CloseAudioDevice, [], :void
    attach_function :IsAudioDeviceReady, [], :bool
    attach_function :SetMasterVolume, [:float], :void

    # Wave/Sound loading/unloading functions

    attach_function :LoadWave, [], Wave.by_value
    attach_function :LoadWaveEx, [:float, :int, :int], Wave.by_value
    attach_function :LoadSound, [], Sound.by_value
    attach_function :LoadSoundFromWave, [Wave.by_value], Sound.by_value
    attach_function :UpdateSound, [Sound.by_value, :int], :void
    attach_function :UnloadWave, [Wave.by_value], :void
    attach_function :UnloadSound, [Sound.by_value], :void

    # Wave/Sound management functions

    attach_function :PlaySound, [Sound.by_value], :void
    attach_function :PauseSound, [Sound.by_value], :void
    attach_function :ResumeSound, [Sound.by_value], :void
    attach_function :StopSound, [Sound.by_value], :void
    attach_function :IsSoundPlaying, [Sound.by_value], :bool
    attach_function :SetSoundVolume, [Sound.by_value, :float], :void
    attach_function :SetSoundPitch, [Sound.by_value, :float], :void
    attach_function :WaveFormat, [Wave.by_value, :int, :int, :int], :void
    attach_function :WaveCopy, [Wave.by_value], Wave.by_value
    attach_function :WaveCrop, [Wave.by_value, :int, :int], :void
    attach_function :GetWaveData, [Wave.by_value], :pointer

    # Music management functions
=begin
    attach_function :LoadMusicStream, [], Music.by_value
    attach_function :UnloadMusicStream, [Music.by_value], :void
    attach_function :PlayMusicStream, [Music.by_value], :void
    attach_function :UpdateMusicStream, [Music.by_value], :void
    attach_function :StopMusicStream, [Music.by_value], :void
    attach_function :PauseMusicStream, [Music.by_value], :void
    attach_function :ResumeMusicStream, [Music.by_value], :void
    attach_function :IsMusicPlaying, [Music.by_value], :bool
    attach_function :SetMusicVolume, [Music.by_value, :float], :void
    attach_function :SetMusicPitch, [Music.by_value, :float], :void
    attach_function :SetMusicLoopCount, [Music.by_value, :float], :void
    attach_function :GetMusicTimeLength, [Music.by_value], :float
    attach_function :GetMusicTimePlayed, [Music.by_value], :float
    # AudioStream management functions
    attach_function :InitAudioStream, [:int, :int], AudioStream.by_value
    attach_function :UpdateAudioStream, [AudioStream.by_value, :int], :void
    attach_function :CloseAudioStream, [AudioStream.by_value], :void
    attach_function :IsAudioBufferProcessed, [AudioStream.by_value], :bool
    attach_function :PlayAudioStream, [AudioStream.by_value], :void
    attach_function :PauseAudioStream, [AudioStream.by_value], :void
    attach_function :ResumeAudioStream, [AudioStream.by_value], :void
    attach_function :StopAudioStream, [AudioStream.by_value], :void
=end
    
    end
end
