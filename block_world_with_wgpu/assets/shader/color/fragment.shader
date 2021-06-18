#version 450

layout(location=0) out vec4 f_color;
layout(set = 0, binding = 0)
uniform ColorUniform {
    vec4 color;
};

void main() {
    f_color = color;
}