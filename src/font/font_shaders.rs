pub const VERTEX_SOURCE: &str = r#"
#version 330 core

layout (location = 0) in vec3 Position;
layout (location = 1) in vec2 TexCoords;
layout (location = 2) in vec4 Color;

uniform mat4 MVP;

out VS_OUTPUT {
    vec2 TexCoords;
    vec4 Color;
} OUT;

void main() {
    //gl_Position = vec4(Position, 1.0);
    gl_Position = MVP * vec4(Position, 1.0);

    OUT.TexCoords = TexCoords;
    OUT.Color = Color;
}
"#;

pub const FRAGMENT_SOURCE: &str = r#"
#version 330 core

precision mediump float;

in VS_OUTPUT {
    vec2 TexCoords;
    vec4 Color;
} IN;

uniform sampler2D GlyphTexture;

out vec4 Color;

void main() {
    Color = IN.Color * vec4(1.0, 1.0, 1.0, texture(GlyphTexture, IN.TexCoords).r);
    //Color = texture2D(GlyphTexture, IN.TexCoords) * vec4(IN.Color.x, IN.Color.y, IN.Color.z, IN.Color.w);
    //float alpha = texture(GlyphTexture, IN.TexCoords).r;

    //if (alpha <= 0.0) {
    //    discard;
    //}

    //Color = IN.Color * vec4(1.0, 1.0, 1.0, alpha);
}
"#;

