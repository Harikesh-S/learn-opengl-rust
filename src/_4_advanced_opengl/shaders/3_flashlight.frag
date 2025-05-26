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
    vec3 position;
    vec3 direction;
    float cutOff;       // cos(angle) to cut off the light
  
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

//vec3 calculateDirectionalLight(Light light, Material material, vec3 viewPos, vec3 Normal, vec3 FragPos, vec2 TexCoord);
vec3 calculateSpotLight(Light light, Material material, vec3 viewPos, vec3 Normal, vec3 FragPos, vec2 TexCoord);

void main()
{
    //vec3 result = calculateDirectionalLight(light, material, viewPos, Normal, FragPos, TexCoord);
    vec3 result = calculateSpotLight(light, material, viewPos, Normal, FragPos, TexCoord);
    
    FragColor = vec4(result, 1.0);
}

// vec3 calculateDirectionalLight(Light light, Material material, vec3 viewPos, vec3 Normal, vec3 FragPos, vec2 TexCoord) {
//     // Diffuse lighting
//     vec3 norm = normalize(Normal);
//     vec3 lightDir = normalize(-light.direction);  // direction is expected to point towards the light
//     float diff = max(dot(norm, lightDir), 0.0);   // get angle between light and normal, we don't want values < 0
//     // Specular highlights
//     vec3 viewDir = normalize(viewPos - FragPos);  // calculate normal against view direction
//     vec3 reflectDir = reflect(-lightDir, norm);   // reflect light direction along normal, -1 * since reflect expects vector to point from the source
//     float spec = pow(max(dot(viewDir, reflectDir), 0.0), material.shininess);
    
//     // Add everything together
//     vec3 ambient = light.ambient * vec3(texture(material.texture_diffuse0, TexCoord));
//     vec3 diffuse = light.diffuse * (diff * vec3(texture(material.texture_diffuse0, TexCoord)));
//     vec3 specular = light.specular * (spec * vec3(texture(material.texture_specular0, TexCoord)));
//     vec3 emissive = vec3(texture(material.texture_emissive0, TexCoord));
//     vec3 result = ambient + diffuse + specular + emissive;   // Removing objectColor since color is a part of the material
    
//     return result;
// }


vec3 calculateSpotLight(Light light, Material material, vec3 viewPos, vec3 Normal, vec3 FragPos, vec2 TexCoord) {
    vec3 lightDir = normalize(light.position - FragPos);  // calculate relative light position
    float theta = dot(lightDir, normalize(-light.direction));

    if(theta > light.cutOff) {  // > cut off since this is cos(value)
        // Diffuse lighting
        vec3 norm = normalize(Normal);
        float diff = max(dot(norm, lightDir), 0.0);   // get angle between light and normal, we don't want values < 0
        // Specular highlights
        vec3 viewDir = normalize(viewPos - FragPos);  // calculate normal against view direction
        vec3 reflectDir = reflect(-lightDir, norm);   // reflect light direction along normal, -1 * since reflect expects vector to point from the source
        float spec = pow(max(dot(viewDir, reflectDir), 0.0), material.shininess);
        
        // Add everything together
        vec3 ambient = light.ambient * vec3(texture(material.texture_diffuse0, TexCoord));
        vec3 diffuse = light.diffuse * (diff * vec3(texture(material.texture_diffuse0, TexCoord)));
        vec3 specular = light.specular * (spec * vec3(texture(material.texture_specular0, TexCoord)));
        vec3 result = ambient + diffuse + specular;
        
        return result;
    }
    else {
        vec3 ambient = light.ambient * vec3(texture(material.texture_diffuse0, TexCoord));
        return ambient;
    }
}