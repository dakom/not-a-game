#version 300 es
precision mediump float;

layout(location=0) in vec2 a_geom_vertex;
layout(location=1) in vec2 a_uv_vertex;

layout (std140) uniform ubo_camera {
    uniform mat4 view;
    uniform mat4 projection;
} camera;

out vec2 v_uv;

uniform vec2 u_quad_scaler;
uniform vec2 u_uv_offset;
uniform mat4 u_model;

void main() {
    mat4 mvp = (camera.projection * (camera.view * u_model));

    mat4 quad_scaler = mat4(1.0);
    quad_scaler[0][0] = u_quad_scaler[0];
    quad_scaler[1][1] = u_quad_scaler[1];

    gl_Position = mvp * (quad_scaler * vec4(a_geom_vertex,0, 1));
    v_uv = a_uv_vertex + u_uv_offset;
}