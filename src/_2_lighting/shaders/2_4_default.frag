#version 330 core
in vec3 ambient;
in vec3 diffuse;
in vec3 specular;

out vec4 FragColor;

uniform vec3 objectColor;

void main()
{
    // Add everything together
    vec3 result = (ambient + diffuse + specular) * objectColor;
    FragColor = vec4(result, 1.0);
} 