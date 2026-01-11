enable wgpu_mesh_shader;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec4<f32>,
}
struct TriangleOutput {
    @builtin(triangle_indices) indices: vec3<u32>,
}

const positions = array<vec4<f32>, 3>(vec4(0.0, 1.0, 1.0, 1.0), vec4(-1.0, -1.0, 1.0, 1.0), vec4(1.0, -1.0, 1.0, 1.0));
const colors = array<vec4<f32>, 3>(vec4(1.0, 0.0, 0.0, 1.0), vec4(0.0, 1.0, 0.0, 1.0), vec4(0.0, 0.0, 1.0, 1.0));

@vertex
fn vertex(@builtin(vertex_index) vertId: u32) -> VertexOutput {
    var output: VertexOutput;
    output.position = positions[vertId];
    output.color = colors[vertId];
    return output;
}

struct MeshOutput {
    @builtin(vertex_count) vertex_count: u32,
    @builtin(vertices) vertices: array<VertexOutput, 3>,
    @builtin(primitive_count) triangle_count: u32,
    @builtin(primitives) triangles: array<TriangleOutput, 1>,
}
var<workgroup> mesh_output: MeshOutput;

var<task_payload> task_payload: u32;

@task
@payload(task_payload)
@workgroup_size(1)
fn task() -> @builtin(mesh_task_size) vec3<u32> {
    return vec3(1,1,1);
}

@mesh(mesh_output)
@payload(task_payload)
@workgroup_size(1)
fn mesh() {
    mesh_output.vertex_count = 3;
    mesh_output.triangle_count = 1;
    for(var i = 0;i < 3;i++) {
        mesh_output.vertices[i].position = positions[i];
        mesh_output.vertices[i].color = colors[i];
    }
    mesh_output.triangles[0].indices = vec3(0, 1, 2);
}

@fragment
fn fragment(@location(0) color: vec4<f32>) -> @location(0) vec4<f32> {
    return color;
}
