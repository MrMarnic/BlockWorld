#version 450

layout(location=0) out vec4 f_color;
layout(location=0) in vec2 tex_coords_out;

layout(set = 0, binding = 0) uniform texture2D t_diffuse;
layout(set = 0, binding = 1) uniform sampler s_diffuse;

layout(set=3, binding=0)
uniform Uniforms {
    vec4 text_color;
};

void main() {
    vec4 sampled = vec4(1.0, 1.0, 1.0, texture(sampler2D(t_diffuse, s_diffuse), tex_coords_out).a);
    f_color = text_color * sampled;
}