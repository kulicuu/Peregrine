#version 450
// TODO :  Use the runtime-shader example to make a studio situation for live editing of shaders.  The example commentary
// mentions shaders served over web-services, but it could just be a local file watch that triggers the switch.


layout(location = 0) in vec3 position;
layout(location = 1) in vec3 normal;

layout(location = 0) out vec3 v_normal;

layout(set = 0, binding = 0) uniform Data {
    mat4 world;
    mat4 view;
    mat4 proj;
} uniforms;

void main() {
    mat4 worldview = uniforms.view;
    v_normal = transpose(inverse(mat3(worldview))) * normal;

    mat4 scale = mat4(1, 0, 0, 0,     0, 1, 0, 0,     0, 0, 1, 0,      0, 0, 0, 1/500.0);
    mat4 translate = mat4(1, 0, 0, 0,   0, 1, 0, 0,   0, 0, 1, 0, -300, -200, -35, 1);
    mat4 rotate = mat4(1, 0, 0, 0,   0, 0, 1, 0,    0, -1, 0, 0,    0, 0, 0, 1);
    gl_Position = uniforms.proj * worldview * scale * rotate * translate * vec4(position, 1.0);
}
