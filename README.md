## Feature

Model loading using tobj. 
Sword Model from [free3d](https://free3d.com/3d-model/sting-sword-128810.html), mtl generated manually.
Car model from [kenney.nl](https://kenney.nl/) -  Toy Car Kit -> vehicle-vintage-racer

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