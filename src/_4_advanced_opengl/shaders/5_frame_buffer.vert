#version 330 core
layout (location = 0) in vec3 aPos;     // the position variable has attribute position 0
layout (location = 1) in vec3 aNormal;     // the texcoord variable has attribute position 1
layout (location = 2) in vec2 aTex;     // the texcoord variable has attribute position 2

out vec2 TexCoords;

void main()
{
    gl_Position = vec4(aPos.x, aPos.y, 0.0, 1.0); 
    TexCoords = aTex;
}  