#version 300 es
precision highp float;

uniform sampler2D u_sampler;

uniform vec4 u_tint;
in vec2 v_uv;

out vec4 color;

void main() {
    vec4 tex_color = texture(u_sampler, v_uv);

    color = tex_color * u_tint;
}