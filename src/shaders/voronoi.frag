#version 140

in vec2 pos;

uniform isampler1D points_tex;
uniform sampler1D colors_tex;
uniform float point_distance;

out vec4 color;

int point_count = textureSize(points_tex, 0);

vec2 getXY(int i) {
    return vec2(texelFetch(points_tex, i, 0).xy);
}

vec4 getColor(int i) {
    return texelFetch(colors_tex, i, 0);
}

void main() {
    color = vec4(0.5, 0.5, 0.5, 1.0);

    float best_distance = 999999.0;
    for (int i = 0; i < point_count; i++) {
        vec2 delta = pos - getXY(i);
        vec2 tmp = pow(delta, vec2(2.0));
        float dist = tmp.x + tmp.y;

        if (dist <= point_distance) {
            color = getColor(i);
            return;
        }
        if (dist < best_distance) {
            best_distance = dist;
            color = getColor(i);
            color.rgb *= 0.5;
        }
    }
}
