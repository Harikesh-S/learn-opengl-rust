#version 330 core
in vec3 Normal;
in vec3 FragPos;
in vec3 LightPos;

out vec4 FragColor;

uniform vec3 objectColor;
uniform vec3 lightColor;

void main()
{
    // Ambient lighting
    float ambientStrength = 0.1;                  // Strength of ambient component
    vec3 ambient = ambientStrength * lightColor;
    // Diffuse lighting
    vec3 norm = normalize(Normal);
    vec3 lightDir = normalize(LightPos - FragPos);// calculate relative light position
    float diff = max(dot(norm, lightDir), 0.0);   // get angle between light and normal, we don't want values < 0
    vec3 diffuse = diff * lightColor;
    // Specular highlights
    float specularStrength = 0.5;                 // Strength of specular component 
    vec3 viewDir = normalize(-FragPos);           // calculate normal against view direction, in view space viewer is 0,0,0
    vec3 reflectDir = reflect(-lightDir, norm);   // reflect light direction along normal, -1 * since reflect expects vector to point from the source
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), 32); // 32 is the "shininess" value of the object
    vec3 specular = specularStrength * spec * lightColor;
    // Add everything together
    vec3 result = (ambient + diffuse + specular) * objectColor;
    FragColor = vec4(result, 1.0);
} 