#version 450

layout(location = 0) in vec2 position;
layout(location = 1) in vec3 color;

layout(location = 0) out vec3 v_color;

void main() {
    v_color = color;
    mat3 trans1 = mat3(1.0, 1.3, 0,  0.1, 1.3, 0.0,  0.0, 0.0, 1.0);
    vec3 x45 = vec3(position, 1.0);
    vec3 x46 = trans1 * x45;
    gl_Position = vec4(x46, 1.0);
    // gl_Position = vec4(position, 1.0, 1.0);
}
