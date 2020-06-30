pub const VERTEX_SOURCE: &str = r#"
#version 330 core

layout (location = 0) in vec3 Position;
layout (location = 1) in vec4 Color;

out VS_OUTPUT {
    vec3 Color;
} OUT;

void main() {
    gl_Position = vec4(Position, 1.0);
    OUT.Color = Color.xyz;
}
"#;

pub const FRAGMENT_SOURCE: &str = r#"
#version 330 core

in VS_OUTPUT {
    vec3 Color;
} IN;

out vec4 Color;

void main() {
    Color = vec4(IN.Color, 1.0f);
}
"#;

