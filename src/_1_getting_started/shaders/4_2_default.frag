#version 330 core
out vec4 FragColor;
  
in vec3 vertexColor;    // the input variable from the vertex shader (same name and same type)  
in vec2 texCoord;

uniform sampler2D tex0;  // Texture unit
uniform sampler2D tex1;

void main()
{
    FragColor = mix(texture(tex0, texCoord), texture(tex1, texCoord), 0.5);
    // using texture() to draw using texture
    // using mix() to mix multiple textures
} 