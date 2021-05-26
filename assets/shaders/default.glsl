#version 460 core

#if VERTEX_SHADER

layout(location = 0) in vec3 v_pos;

layout(location = 1) out vec2 v_uv;

void main() {
    gl_Position = vec4(v_pos, 1.0);
    v_uv = (vec2(v_pos.x, v_pos.y) + 1.0) / 2.0;
}

#elif FRAGMENT_SHADER

layout(binding = 0) uniform sampler2D u_texture;

layout(location = 1) in vec2 f_uv;

layout(location = 0) out vec4 f_color;

void main() {
    f_color = texture(u_texture, f_uv);
}

#endif