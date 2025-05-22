#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aNormal;

out vec3 ambient;
out vec3 diffuse;
out vec3 specular;

uniform mat4 model;
uniform mat4 camMatrix;
uniform vec3 lightColor;
uniform vec3 lightPos;
uniform vec3 viewPos;

void main()
{
    gl_Position = camMatrix * model * vec4(aPos, 1.0);

    vec3 FragPos = vec3(model * vec4(aPos, 1.0));            // actual position to fragment in world coordinates
    vec3 Normal = mat3(transpose(inverse(model))) * aNormal; // using normal matrix in case we are applying a non-uniform scale

    // Ambient lighting
    float ambientStrength = 0.1;                  // Strength of ambient component
    ambient = ambientStrength * lightColor;
    // Diffuse lighting
    vec3 norm = normalize(Normal);
    vec3 lightDir = normalize(lightPos - FragPos);// calculate relative light position
    float diff = max(dot(norm, lightDir), 0.0);   // get angle between light and normal, we don't want values < 0
    diffuse = diff * lightColor;
    // Specular highlights
    float specularStrength = 0.5;                 // Strength of specular component 
    vec3 viewDir = normalize(viewPos - FragPos);  // calculate normal against view direction
    vec3 reflectDir = reflect(-lightDir, norm);   // reflect light direction along normal, -1 * since reflect expects vector to point from the source
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), 32); // 32 is the "shininess" value of the object
    specular = specularStrength * spec * lightColor;
}