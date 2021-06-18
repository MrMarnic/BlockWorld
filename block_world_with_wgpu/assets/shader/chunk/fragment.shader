#version 450

layout(location=0) out vec4 f_color;

layout(location=0) in vec2 tex_coords_out;
layout(location=1) in vec3 normals_out;
layout(location=2) in vec3 FragPos;
layout(location=3) in float lightLevelOut;

layout(set = 0, binding = 0) uniform texture2D t_diffuse;
layout(set = 0, binding = 1) uniform sampler s_diffuse;

layout(set = 3, binding = 0) uniform lighting {
    vec3 light_pos;
};

void main() {

    vec3 norm = normalize(normals_out);
    vec3 lightDir = normalize(light_pos - FragPos);
    float diff = max(dot(norm, lightDir), 0.0);
    float ambient = 0.2;

    vec4 texel = texture(sampler2D(t_diffuse, s_diffuse), tex_coords_out) * vec4(lightLevelOut,lightLevelOut,lightLevelOut,1.0);

    //vec3 result = (diff + ambient) * texel.xyz;

    if(texel.a < 0.5)
    discard;

    //f_color = vec4(result,texel.a);
    f_color = texel;
}