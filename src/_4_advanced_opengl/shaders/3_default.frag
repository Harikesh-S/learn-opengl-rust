#version 330 core
// Struct to hold material data
// These define the color of the material for each component
struct Material {
    sampler2D texture_diffuse0;
    sampler2D texture_specular0;
    sampler2D texture_emissive0;
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
uniform Light dirLight;
uniform vec3 viewPos;

vec4 calculateDirectionalLight(Light light, Material material, vec3 viewPos, vec3 Normal, vec3 FragPos, vec2 TexCoord);

void main()
{
    // !Note! texture unit 0 is expected to have a blank 1x1 texture with alpha 0
    vec4 result = calculateDirectionalLight(dirLight, material, viewPos, Normal, FragPos, TexCoord);
    
    FragColor = result;
}

vec4 calculateDirectionalLight(Light light, Material material, vec3 viewPos, vec3 Normal, vec3 FragPos, vec2 TexCoord) {
    // Diffuse lighting
    vec3 norm = normalize(Normal);
    vec3 lightDir = normalize(-light.direction);  // direction is expected to point towards the light
    float diff = max(dot(norm, lightDir), 0.0);   // get angle between light and normal, we don't want values < 0
    // Specular highlights
    vec3 viewDir = normalize(viewPos - FragPos);  // calculate normal against view direction
    vec3 reflectDir = reflect(-lightDir, norm);   // reflect light direction along normal, -1 * since reflect expects vector to point from the source
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), material.shininess);
    
    // Add everything together
    vec4 ambient = vec4(light.ambient,1.0) * texture(material.texture_diffuse0, TexCoord);
    vec4 diffuse = vec4(light.diffuse,1.0) * (diff * texture(material.texture_diffuse0, TexCoord));
    vec4 specular = vec4(light.specular,1.0) * (spec * texture(material.texture_specular0, TexCoord));
    vec4 emissive = texture(material.texture_emissive0, TexCoord);
    vec4 result = ambient + diffuse + specular + emissive;
    
    return result;
}