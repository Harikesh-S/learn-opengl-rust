#version 330 core
in vec3 Normal;
in vec3 FragPos;

out vec4 FragColor;

uniform float ambientStrength;
uniform float diffuseStength;
uniform float specularStrength;
uniform int shininess;

uniform vec3 objectColor;
uniform vec3 lightColor;
uniform vec3 lightPos;
uniform vec3 viewPos;

void main()
{
    // Ambient lighting
    vec3 ambient = ambientStrength * lightColor;
    // Diffuse lighting
    vec3 norm = normalize(Normal);
    vec3 lightDir = normalize(lightPos - FragPos);// calculate relative light position
    float diff = max(dot(norm, lightDir), 0.0);   // get angle between light and normal, we don't want values < 0
    vec3 diffuse = diffuseStength * diff * lightColor;
    // Specular highlights
    vec3 viewDir = normalize(viewPos - FragPos);  // calculate normal against view direction
    vec3 reflectDir = reflect(-lightDir, norm);   // reflect light direction along normal, -1 * since reflect expects vector to point from the source
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), shininess); // 32 is the "shininess" value of the object
    vec3 specular = specularStrength * spec * lightColor;
    // Add everything together
    vec3 result = (ambient + diffuse + specular) * objectColor;
    FragColor = vec4(result, 1.0);
} 