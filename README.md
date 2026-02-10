# Terminal Renderer
An ASCII rasterizer that I created in Rust. This project is heavily inspired by donut.c. Able to load .obj files using a custom loader.

![Cow showcase](images/cow.gif)


## How to run
Run the command to build the project:
```
cargo build
```

Then, to start the renderer, run the following command: 
```
cargo run -- --scale 0.1 --fov 30 --model-path 'path/to/model.obj'
```

## Controls
WASD - Player Movement
Arrow keys - Camera Movement
