## Feature

Using crate "assimp" for model loading. Crate seems to be very old (not updated in ~7 years)

- Required a older version of cmake, since with cmake 4.0.2 older versions of cmake <3.5 are not supported. I got around this by installing cmake 3.31 instead, but its probably easier to use tobj instead or use russimp.
- Model loading, with vertices and faces works. TODO Materials
- Model from [kenney](https://kenney.nl/)

## What is this?

Rust port of me following through [learn opengl](https://learnopengl.com/Introduction), including all chapters and exercises. I'm new to OpenGl and rust so this may not be the best way to implement any of this.  
  
In addition, i've followed [this excellent video tutorial series](https://www.youtube.com/playlist?list=PLPaoO-vpZnumdcb4tZc4x5Q-v7CkrQ6M-) by Victor Gordan. The series goes through learnopengl.com, and is in c++.
I've also referred [bwasty/learn-opengl-rs](https://github.com/bwasty/learn-opengl-rs) for rust versions of the functions/pointers.   

## How to run

This is directly from bwasty's implementation. Run any tutorial with `cargo run 1_7_1`.  
Refer main.rs for the complete list.

## Setup

Cargo.toml includes all dependencies. Unlike c++ downloading and linking glfw/glad is not required as it is managed by cargo.  

## Notes

- I've used nalgebra-glm instead of cgmath. Most methods from glm have a direct equivalent.

Flashlight texture from [80+ sprites for particles, light cookies and shaders (public domain)](https://www.reddit.com/r/gamedev/comments/8v3x2q/80_sprites_for_particles_light_cookies_and/)