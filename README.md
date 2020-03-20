# YARGE
## Yet Another Rust Game Engine

Started as an experimental private repo to start learning Rust and OpenGL and it's still that for now.
It does seem to be growing and I would eventually like to make a game with it. It's probably wiser to use something
like Unreal Engine, but I want to go as low level as I can and understand how all these things work and what it takes
before diving deeper into using other games engines (although I have done some basic experimenting with several engines).

### Features
#### Currently
- create Textures Images/Sprites, Triangle, Rectangles
    - Position, Scale, Flip, Color
- Basic Delta Timer
- Basic Orthographic Camera for 2D
    - Supports basic panning (no zooming)
- Basic Sprite animations through Texture atlas/sprite sheets
- Basic Tilemap support loading from json
- Batch Rendering multiple sprites with one draw call (or currently one per about 1000 sprites)
- FrameBuffer for off screen rendering and full screen post-processing effects (currently implemented as RenderTarget in batch renderer)
#### Would like to have
 - Render text with fonts (`rusttype`?)

All discussions, advice, ideas, etc welcome!
