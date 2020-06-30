pub const VERTEX_SOURCE: &str = r#"
#version 330 core

layout (location = 0) in vec3 Position;
layout (location = 1) in vec2 TexCoord;

uniform mat4 TexCoordTransform;
uniform mat4 MVP;

out VS_OUTPUT {
    vec2 TexCoord;
} OUT;

void main() {
    gl_Position = MVP * vec4(Position, 1.0);

    OUT.TexCoord = vec2(TexCoordTransform * vec4(TexCoord, 1.0, 1.0));
}
"#;

pub const FRAGMENT_SOURCE: &str = r#"
#version 330 core

precision mediump float;

uniform sampler2D TexSampler;
uniform vec4 TexColor;

in VS_OUTPUT {
    vec2 TexCoord;
} IN;

out vec4 Color;

void main() {
    gl_FragColor = texture2D(TexSampler, IN.TexCoord) * vec4(TexColor.x, TexColor.y, TexColor.z, TexColor.w);
}
"#;

