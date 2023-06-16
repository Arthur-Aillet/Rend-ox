[[block]]
struct Data {
    world: mat4x4<f32>;
    view: mat4x4<f32>;
    proj: mat4x4<f32>;
};

struct InstanceInput {
    [[location(5)]] model_matrix_0: vec4<f32>;
    [[location(6)]] model_matrix_1: vec4<f32>;
    [[location(7)]] model_matrix_2: vec4<f32>;
    [[location(8)]] model_matrix_3: vec4<f32>;
    [[location(9)]] color: vec3<f32>;
};

struct Vertex {
    [[builtin(position)]] vpos: vec4<f32>;
    [[location(0)]] pos: vec4<f32>;
    [[location(1)]] uv: vec3<f32>;
    [[location(2)]] normal: vec3<f32>;
    [[location(3)]] color: vec3<f32>;
};

[[group(0), binding(0)]]
var<uniform> uniforms: Data;

fn custom_inverse(m: mat3x3<f32>) -> mat3x3<f32> {
    let determinant: f32 = determinant(m);
    let invdet: f32 = 1.0 / determinant;
    var minv: mat3x3<f32>;
    minv[0][0] = (m[1][1] * m[2][2] - m[2][1] * m[1][2]) * invdet;
    minv[0][1] = (m[0][2] * m[2][1] - m[0][1] * m[2][2]) * invdet;
    minv[0][2] = (m[0][1] * m[1][2] - m[0][2] * m[1][1]) * invdet;
    minv[1][0] = (m[1][2] * m[2][0] - m[1][0] * m[2][2]) * invdet;
    minv[1][1] = (m[0][0] * m[2][2] - m[0][2] * m[2][0]) * invdet;
    minv[1][2] = (m[1][0] * m[0][2] - m[0][0] * m[1][2]) * invdet;
    minv[2][0] = (m[1][0] * m[2][1] - m[2][0] * m[1][1]) * invdet;
    minv[2][1] = (m[2][0] * m[0][1] - m[0][0] * m[2][1]) * invdet;
    minv[2][2] = (m[0][0] * m[1][1] - m[1][0] * m[0][1]) * invdet;
    return minv;
}

[[stage(vertex)]]
fn main(
    [[location(0)]] pos: vec3<f32>,
    [[location(1)]] uv: vec3<f32>,
    [[location(2)]] normal: vec3<f32>,
    instance: InstanceInput,
) -> Vertex {
    let model_matrix = mat4x4<f32>(
            instance.model_matrix_0,
            instance.model_matrix_1,
            instance.model_matrix_2,
            instance.model_matrix_3,
        );
    let world: mat4x4<f32> = uniforms.world * model_matrix;
    let worldview: mat4x4<f32> = uniforms.view * world;
    let wv3: mat3x3<f32> = (mat3x3<f32>(worldview[0].xyz, worldview[1].xyz, worldview[2].xyz));
//    let out_normal: vec3<f32> = wv3 * normal;
//    let out_normal: vec3<f32> = transpose(custom_inverse(wv3)) * normal;
    let out_pos: vec4<f32> = world * vec4<f32>(pos, 1.0);
    let v_pos: vec4<f32> = uniforms.proj * worldview * vec4<f32>(pos, 1.0);
    return Vertex(v_pos, out_pos, uv, normalize(wv3 * normal), instance.color);
}
