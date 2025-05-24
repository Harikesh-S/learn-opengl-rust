#version 330 core
// Struct to hold material data
// These define the color of the material for each component
struct Material {
    sampler2D texture_diffuse1;
    sampler2D texture_specular1;
    float shininess;
};

// Struct to hold light data
// These define the color of the light for each component
struct Light {
    // vec3 position; Directional lights do not have positions
    vec3 direction; // direction of the light
  
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
};

// Data from vertex shader about position and normal
in vec3 Normal;
in vec3 FragPos;
in vec2 TexCoord;

out vec4 FragColor;

// Uniforms for light colors, material colors and positions for light and viewer
uniform Material material;
uniform Light light;
uniform vec3 viewPos;

vec3 calculateDirectionalLight(Light light, Material material, vec3 viewPos, vec3 Normal, vec3 FragPos, vec2 TexCoord);

void main()
{
    vec3 result = calculateDirectionalLight(light, material, viewPos, Normal, FragPos, TexCoord);
    
    FragColor = vec4(result, 1.0);
}

vec3 calculateDirectionalLight(Light light, Material material, vec3 viewPos, vec3 Normal, vec3 FragPos, vec2 TexCoord) {
    // Diffuse lighting
    vec3 norm = normalize(Normal);
    vec3 lightDir = normalize(-light.direction);  // direction is expected to point towards the light
    float diff = max(dot(norm, lightDir), 0.0);   // get angle between light and normal, we don't want values < 0
    // Specular highlights
    vec3 viewDir = normalize(viewPos - FragPos);  // calculate normal against view direction
    vec3 reflectDir = reflect(-lightDir, norm);   // reflect light direction along normal, -1 * since reflect expects vector to point from the source
    //float spec = pow(max(dot(viewDir, reflectDir), 0.0), material.shininess);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), 32);
    
    // Add everything together
    vec3 ambient = light.ambient * vec3(texture(material.texture_diffuse1, TexCoord));
    vec3 diffuse = light.diffuse * (diff * vec3(texture(material.texture_diffuse1, TexCoord)));
    vec3 specular = light.specular * (spec * vec3(texture(material.texture_specular1, TexCoord)));
    vec3 result = ambient + diffuse + specular;   // Removing objectColor since color is a part of the material
    
    return result;
}