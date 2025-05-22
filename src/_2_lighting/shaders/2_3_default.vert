#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aNormal;

out vec3 Normal;
out vec3 FragPos;

out vec3 LightPos;  // output view space light pos to fragment shader

uniform vec3 lightPos; // used to get the view space Light pos
uniform mat4 model;
uniform mat4 camMatrix;
uniform mat4 view;

void main()
{
    gl_Position = camMatrix * model * vec4(aPos, 1.0);
    FragPos = vec3(view *model * vec4(aPos, 1.0));            // pass actual position to fragment in view space
    Normal = mat3(transpose(inverse(view * model))) * aNormal; // using normal matrix in case we are applying a non-uniform scale
    LightPos = vec3(view * vec4(lightPos, 1.0));    // light position in view space
}