#version 330 core
// Struct to hold material data
// These define the color of the material for each component
struct Material {
    sampler2D diffuse;
    sampler2D specular;
    float shininess;
};

// Struct to hold light data
// These define the color of the light for each component
struct DirLight {
    // vec3 position; Directional lights do not have positions
    vec3 direction; // direction of the light
  
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
};

struct PointLight {
    vec3 position;
  
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;

    float constant;
    float linear;
    float quadratic;
};

struct SpotLightSoft {
    vec3 position;
    vec3 direction;
    float cutOff;       // cos(angle) to cut off the light
    float outerCutOff; // to smooth the edge
  
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
uniform vec3 viewPos;

// Uniforms for all lights
#define NR_POINT_LIGHTS 4  
uniform DirLight dirLight;
uniform PointLight pointLights[NR_POINT_LIGHTS];
uniform SpotLightSoft spotLight;

// Declare functions to calculate each light type
vec3 calculateDirectionalLight(DirLight light, vec3 norm, vec3 viewDir);
vec3 calculatePointLight(PointLight light, vec3 fragPos, vec3 norm, vec3 viewDir);
vec3 calculateSpotLightSoft(SpotLightSoft light, vec3 fragPos, vec3 norm, vec3 viewDir);

void main()
{

    vec3 norm = normalize(Normal);
    vec3 viewDir = normalize(viewPos - FragPos);  // calculate normal against view direction

    vec3 result = vec3(0.0, 0.0, 0.0);

    // Step 1 - Directional light
    result += calculateDirectionalLight(dirLight, norm, viewDir);
    // Step 2 - Point lights
    // moving duplicated actions outside
    for(int i = 0; i < NR_POINT_LIGHTS; i++)
        result += calculatePointLight(pointLights[i], FragPos, norm, viewDir); 
    // Step 3 - spot light
    result += calculateSpotLightSoft(spotLight, FragPos, norm, viewDir);
    
    FragColor = vec4(result, 1.0);
}

vec3 calculateDirectionalLight(DirLight light, vec3 norm, vec3 viewDir) {
    // Diffuse lighting
    vec3 lightDir = normalize(-light.direction);  // direction is expected to point towards the light
    float diff = max(dot(norm, lightDir), 0.0);   // get angle between light and normal, we don't want values < 0
    // Specular highlights
    vec3 reflectDir = reflect(-lightDir, norm);   // reflect light direction along normal, -1 * since reflect expects vector to point from the source
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), material.shininess);
    
    // Add everything together
    vec3 ambient = light.ambient * vec3(texture(material.diffuse, TexCoord));
    vec3 diffuse = light.diffuse * (diff * vec3(texture(material.diffuse, TexCoord)));
    vec3 specular = light.specular * (spec * vec3(texture(material.specular, TexCoord)));
    vec3 result = ambient + diffuse + specular;   // Removing objectColor since color is a part of the material
    
    return result;
}

vec3 calculatePointLight(PointLight light, vec3 fragPos, vec3 norm, vec3 viewDir) {
    // Diffuse lighting
    vec3 lightDir = normalize(light.position - fragPos);  // calculate relative light position
    float diff = max(dot(norm, lightDir), 0.0);   // get angle between light and normal, we don't want values < 0
    // Specular highlights
    vec3 reflectDir = reflect(-lightDir, norm);   // reflect light direction along normal, -1 * since reflect expects vector to point from the source
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), material.shininess);

    // Attenuation for point source
    float dist = length(light.position - fragPos);
    float attenuation = 1.0 / (light.constant + light.linear * dist + light.quadratic * (dist * dist));
    
    // Add everything together
    vec3 ambient = light.ambient * vec3(texture(material.diffuse, TexCoord));
    vec3 diffuse = light.diffuse * (diff * vec3(texture(material.diffuse, TexCoord)));
    vec3 specular = light.specular * (spec * vec3(texture(material.specular, TexCoord)));
    vec3 result = (ambient + diffuse + specular) * attenuation; 
    
    return result;
}

vec3 calculateSpotLightSoft(SpotLightSoft light, vec3 fragPos, vec3 norm, vec3 viewDir) {
    // Spot light
    vec3 lightDir = normalize(light.position - fragPos);  // calculate relative light position
    float theta = dot(lightDir, normalize(-light.direction));
    float epsilon = light.cutOff - light.outerCutOff;   // - order is changed due to cos()
    float intensity = smoothstep(0.0, 1.0, (theta - light.outerCutOff) / epsilon); // replacing with smoothstep for smoother edges

    // Diffuse lighting
    float diff = max(dot(norm, lightDir), 0.0);   // get angle between light and normal, we don't want values < 0
    // Specular highlights
    vec3 reflectDir = reflect(-lightDir, norm);   // reflect light direction along normal, -1 * since reflect expects vector to point from the source
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), material.shininess);
    
    // Add everything together
    vec3 ambient = light.ambient * vec3(texture(material.diffuse, TexCoord));
    vec3 diffuse = light.diffuse * (diff * vec3(texture(material.diffuse, TexCoord))) * intensity;
    vec3 specular = light.specular * (spec * vec3(texture(material.specular, TexCoord))) * intensity;   // * intensity calculated by spot (start)
    vec3 result = ambient + diffuse + specular;
    
    return result;
}