pub const VERTEX_SOURCE: &str = r#"
#version 330 core

layout (location = 0) in vec3 Position;

uniform vec4 Color;
uniform mat4 MVP;

out VS_OUTPUT {
    vec4 Color;
} OUT;

void main() {
    gl_Position = MVP * vec4(Position, 1.0);

    OUT.Color = Color;
}
"#;

pub const FRAGMENT_SOURCE: &str = r#"
#version 330 core

in VS_OUTPUT {
    vec4 Color;
} IN;

out vec4 Color;

void main() {
    Color = IN.Color;
}
"#;

