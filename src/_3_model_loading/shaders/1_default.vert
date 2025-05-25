#version 330 core
layout (location = 0) in vec3 aPos;     // the position variable has attribute position 0
layout (location = 1) in vec3 aNormal;     // the texcoord variable has attribute position 1
layout (location = 2) in vec2 aTex;     // the texcoord variable has attribute position 2

out vec3 vertexColor;
out vec3 Normal;
out vec3 FragPos;
out vec2 TexCoord;

uniform mat4 model;
uniform mat4 camMatrix;

void main()
{
    gl_Position = camMatrix * model * vec4(aPos, 1.0);  // see how we directly give a vec3 to vec4's constructor
    //vertexColor = vec3(1.0, 1.0, 1.0);
    //vertexColor = aNormal;
    vertexColor = vec3(aTex,0.0);
    FragPos = vec3(model * vec4(aPos, 1.0));            // pass actual position to fragment in world coordinates
    Normal = mat3(transpose(inverse(model))) * aNormal; // using normal matrix in case we are applying a non-uniform scale
    TexCoord = aTex;

}