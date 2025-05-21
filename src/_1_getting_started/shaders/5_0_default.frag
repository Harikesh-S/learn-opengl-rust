#version 330 core
out vec4 FragColor;
  
in vec2 texCoord;       // the input variable from the vertex shader (same name and same type)  

uniform sampler2D tex0;  // Texture unit
uniform sampler2D tex1;

void main()
{
    FragColor = mix(texture(tex0, texCoord), texture(tex1, texCoord), 0.5);
    // using texture() to draw using texture
    // using mix() to mix multiple textures
} 