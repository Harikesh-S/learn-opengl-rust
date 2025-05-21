#version 330 core
layout (location = 0) in vec3 aPos;     // the position variable has attribute position 0
layout (location = 1) in vec2 aTex;     // the texcoord variable has attribute position 1

out vec2 texCoord;      // specify texture coordinate output to the fragment shader

uniform mat4 transform;

void main()
{
    gl_Position = transform * vec4(aPos, 1.0);  // see how we directly give a vec3 to vec4's constructor
    texCoord = aTex;                // pass the texture coordinate to the fragment shader
}