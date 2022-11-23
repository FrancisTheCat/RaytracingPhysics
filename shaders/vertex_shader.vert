#version 330
in vec2 position;
out vec2 UV;

void main()
{    
    gl_Position = vec4(position, 0.0, 1.0);
    UV = (gl_Position.xy + 1.0) * 0.5;
}