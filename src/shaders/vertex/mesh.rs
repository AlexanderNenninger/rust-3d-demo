pub const SHADER: &str = r#"
    attribute vec4 aPosition;
    attribute vec3 aVertexNormal;
    attribute vec3 aBaseColor;

    uniform mat4 uNormalsRotation;
    uniform mat4 uProjection;
    varying lowp vec4 vColor;

    void main() {
        gl_Position = uProjection * vec4(aPosition.x, aPosition.y, aPosition.z, 1.0);

        vec3 ambientLight = vec3(0.2, 0.2, 0.2);
        vec3 directionalLightColor = vec3(1, 1, 1);
        vec3 directionalVector = normalize(vec3(-0.85, 0.8, 0.75));

        vec4 transformedNormal = uNormalsRotation * vec4(aVertexNormal, 1.0);
        float directional = max(dot(transformedNormal.xyz, directionalVector), 0.0);
        vec3 vLighting = ambientLight + (directionalLightColor * directional);

        vColor = vec4(aBaseColor * vLighting, 1.0);
    }
"#;