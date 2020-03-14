pub fn create_fragment_source(max_textures: i32) -> String {
    let mut switches = Vec::new();

    for i in 0..max_textures {
        switches.push(format!("
        case  {idx}:
            texColor = texture2D(Textures[{idx}u], IN.TexCoord) * vec4(IN.TexColor.x, IN.TexColor.y, IN.TexColor.z, IN.TexColor.w);
            break;
        ", idx = i));
    }

    let shader_parts = vec![
        format!("#version 330 core

        precision mediump float;

        uniform sampler2D Textures[{max_texs}];
        ", max_texs = max_textures),
        r#"
        in VS_OUTPUT {
            vec2 TexCoord;
            vec4 TexColor;
            float TexIndex;
        } IN;

        out vec4 Color;

        void main() {
            int ttid = int(IN.TexIndex + 0.5);
            vec4 baseColor = vec4(IN.TexColor.x, IN.TexColor.y, IN.TexColor.z, IN.TexColor.w);
            vec4 texColor = baseColor;

            switch (ttid)
            {
        "#.to_string(),
        switches.join(""),
        r#"
                default:
                    texColor = baseColor;
                    break;
            }

            Color = texColor;
        }
        "#.to_string(),
    ];

    shader_parts.join("")
}

pub fn create_vertex_source() -> String {
    let src = r#"
#version 330 core

layout (location = 0) in vec3 Position;
layout (location = 1) in vec2 TexCoord;
layout (location = 2) in vec4 TexColor;
layout (location = 3) in float TexIndex;

// uniform mat4 TexCoordTransform;
uniform mat4 MVP;

out VS_OUTPUT {
    vec2 TexCoord;
    vec4 TexColor;
    float TexIndex;
} OUT;

void main() {
    gl_Position = MVP * vec4(Position, 1.0);

    OUT.TexColor = TexColor;
    OUT.TexIndex = TexIndex; //int(TexIndex -0.5);
    //OUT.TexCoord = vec2(TexCoordTransform * vec4(TexCoord, 1.0, 1.0));
    OUT.TexCoord = TexCoord;
}"#;

    src.to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn generate_frag_shader_by_max_textures() {
        let frag_shader = create_fragment_source(4);
        let expected_shader = r#"
#version 330 core

precision mediump float;

uniform sampler2D Textures[3];

in VS_OUTPUT {
    vec2 TexCoord;
    vec4 TexColor;
    float TexIndex;
} IN;

out vec4 Color;

void main() {
    int ttid = int(IN.TexIndex + 0.5);
    vec4 baseColor = vec4(IN.TexColor.x, IN.TexColor.y, IN.TexColor.z, IN.TexColor.w);
    vec4 texColor = baseColor;

    switch (ttid)
    {
        case  0:
            texColor = texture2D(Textures[0u], IN.TexCoord) * vec4(IN.TexColor.x, IN.TexColor.y, IN.TexColor.z, IN.TexColor.w);
            break;
        case  1:
            texColor = texture2D(Textures[1u], IN.TexCoord) * vec4(IN.TexColor.x, IN.TexColor.y, IN.TexColor.z, IN.TexColor.w);
            break;
        case  2:
            texColor = texture2D(Textures[2u], IN.TexCoord) * vec4(IN.TexColor.x, IN.TexColor.y, IN.TexColor.z, IN.TexColor.w);
            break;
        case  3:
            texColor = texture2D(Textures[3u], IN.TexCoord) * vec4(IN.TexColor.x, IN.TexColor.y, IN.TexColor.z, IN.TexColor.w);
            break;
        default:
            texColor = baseColor;
            break;
    }

    Color = texColor;
}"#;

        assert_eq!(expected_shader, frag_shader);
    }

    #[test]
    fn create_vert_shader_source() {
        let vertex_source = create_vertex_source();
        let expected_source = r#"
#version 330 core

layout (location = 0) in vec3 Position;
layout (location = 1) in vec2 TexCoord;
layout (location = 2) in vec4 TexColor;
layout (location = 3) in float TexIndex;

// uniform mat4 TexCoordTransform;
uniform mat4 MVP;

out VS_OUTPUT {
    vec2 TexCoord;
    vec4 TexColor;
    float TexIndex;
} OUT;

void main() {
    gl_Position = MVP * vec4(Position, 1.0);

    OUT.TexColor = TexColor;
    OUT.TexIndex = TexIndex; //int(TexIndex -0.5);
    //OUT.TexCoord = vec2(TexCoordTransform * vec4(TexCoord, 1.0, 1.0));
    OUT.TexCoord = TexCoord;
}"#;

        assert_eq!(expected_source, vertex_source);
    }
}
