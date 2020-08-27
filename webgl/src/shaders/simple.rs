pub const VERT: &str = r#"
attribute vec4 position;

uniform vec4 u_Scale;
uniform vec4 u_Translation;

void main() {
    gl_Position = position * u_Scale + u_Translation;
}
"#;

pub const FRAG: &str = r#"
precision mediump float;

uniform vec4 u_Color;

void main() {
    gl_FragColor = u_Color;
}
"#;
