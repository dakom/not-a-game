#version 300 es
precision mediump float;

layout (std140) uniform ubo_camera {
    uniform mat4 view;
    uniform mat4 projection;
} camera;

layout(location=0) in vec2 a_geom_vertex;
layout(location=1) in vec2 a_uv_vertex;


out vec2 v_uv;

void main() {
    mat4 view_projection = camera.projection * camera.view;


    gl_Position = view_projection * vec4(a_geom_vertex, 0.0, 1.0); 
    v_uv = a_uv_vertex;
}