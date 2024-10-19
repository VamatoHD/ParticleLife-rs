#version 100
attribute vec2 in_pos;
attribute vec4 in_color;

varying lowp vec4 color;
varying mediump vec2 pos;

void main() {
    gl_Position = vec4(in_pos, 0, 1);
    pos = in_pos;
    color = in_color;
}
