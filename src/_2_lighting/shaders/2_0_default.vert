#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aNormal;

out vec3 Normal;
out vec3 FragPos;

uniform mat4 model;
uniform mat4 camMatrix;

void main()
{
    gl_Position = camMatrix * model * vec4(aPos, 1.0);
    FragPos = vec3(model * vec4(aPos, 1.0));            // pass actual position to fragment in world coordinates
    Normal = mat3(transpose(inverse(model))) * aNormal; // using normal matrix in case we are applying a non-uniform scale
}