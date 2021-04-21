#version 140

in vec2 ndc;
out vec2 pos;

uniform vec2 target_size;

void main() {
    gl_Position = vec4(ndc, 0.0, 1.0);
    pos = ndc * target_size / 2.0;
}
