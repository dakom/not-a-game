#version 300 es
precision mediump float;

uniform sampler2D u_sampler;

uniform vec4 u_tint;

in vec2 v_uv;

out vec4 color;

void main() {
    vec4 tex_color = texture(u_sampler, v_uv);


    if (tex_color.a < 0.5) {
        discard;
    }

    color = tex_color * u_tint;

}