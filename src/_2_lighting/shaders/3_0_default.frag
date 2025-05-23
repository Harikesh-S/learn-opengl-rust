#version 330 core
// Struct to hold material data
// These define the color of the material for each component
struct Material {
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
    float shininess;
};

// Struct to hold light data
// These define the color of the light for each component
struct Light {
    vec3 position;
  
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
};

// Data from vertex shader about position and normal
in vec3 Normal;
in vec3 FragPos;

out vec4 FragColor;

// Uniforms for light colors, material colors and positions for light and viewer
uniform Material material;
uniform Light light;
uniform vec3 viewPos;

void main()
{
    // Ambient lighting
    vec3 ambient = light.ambient * material.ambient;
    // Diffuse lighting
    vec3 norm = normalize(Normal);
    vec3 lightDir = normalize(light.position - FragPos);// calculate relative light position
    float diff = max(dot(norm, lightDir), 0.0);   // get angle between light and normal, we don't want values < 0
    vec3 diffuse = light.diffuse * (diff * material.diffuse);
    // Specular highlights
    float specularStrength = 0.5;                 // Strength of specular component 
    vec3 viewDir = normalize(viewPos - FragPos);  // calculate normal against view direction
    vec3 reflectDir = reflect(-lightDir, norm);   // reflect light direction along normal, -1 * since reflect expects vector to point from the source
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), material.shininess);
    vec3 specular = light.specular * (spec * material.specular);
    // Add everything together
    vec3 result = ambient + diffuse + specular;   // Removing objectColor since color is a part of the material
    FragColor = vec4(result, 1.0);
} 