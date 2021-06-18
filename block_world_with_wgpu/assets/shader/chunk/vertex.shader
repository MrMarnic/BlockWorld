#version 450

layout(location=0) in vec3 a_position;
layout(location=1) in vec2 tex_coords;
layout(location=2) in vec3 normals;
layout(location=3) in float lightLevel;

layout(location=0) out vec2 tex_coords_out;
layout(location=1) out vec3 normals_out;
layout(location=2) out vec3 FragPos;
layout(location=3) out float lightLevelOut;

layout(set=1, binding=0) // 1.
uniform Uniforms {
    mat4 projection; // 2.
};
layout(set=1, binding=1) // 1.
uniform Uniforms2 {
    mat4 view; // 2.
};
layout(set=2, binding=0) // 1.
readonly buffer Uniforms3 {
    mat4 transform; // 2.
};

void main() {
    gl_Position = projection * view * transform * vec4(a_position, 1.0);
    tex_coords_out = tex_coords;
    normals_out = normals;
    FragPos = vec3(transform * vec4(a_position,1.0));
    lightLevelOut = lightLevel;
}