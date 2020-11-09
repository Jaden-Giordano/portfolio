pub const VERT: &str = r#"
attribute vec4 position;

uniform vec4 u_Scale;
uniform vec4 u_Translation;
uniform float u_Rotation;

void main() {
    vec4 scaled_position = position * u_Scale;

    float x = scaled_position.x;
    float y = scaled_position.y;

    vec4 rotated_position = vec4(x * cos(u_Rotation) - y * sin(u_Rotation), x * sin(u_Rotation) + y * cos(u_Rotation), 0.0, 0.0);


    gl_Position = rotated_position + u_Translation;
}
"#;

pub const FRAG: &str = r#"
precision mediump float;

uniform vec4 u_Color;

void main() {
    gl_FragColor = u_Color;
}
"#;

//vec4 rotation = vec4(cos(u_Rotation), sin(u_Rotation) * -1.0, sin(u_Rotation), cos(u_Rotation));
