pub const SHADER: &str = r#"
    attribute vec4 aaPosition;
    uniform mat4 uTransform;
    uniform mat4 uRotation;

    void main() {
        gl_Position = uTransform * aPosition;
    }
"#;
