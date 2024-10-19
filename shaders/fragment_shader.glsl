#version 100
varying lowp vec4 color;
varying mediump vec2 pos;  // Receive the position from vertex shader

precision mediump float; // Specify precision for float types

void main() {
    gl_FragColor = color; // Original color
}
