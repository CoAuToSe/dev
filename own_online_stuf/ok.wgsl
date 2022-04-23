// #include<sceneUboDeclaration>
// #include<meshUboDeclaration>

// attribute position : vec3<f32>;
// attribute normal : vec3<f32>;
// attribute uv: vec2<f32>;

// varying vNormal : vec3<f32>;
// varying vUV : vec2<f32>;

[[block]] struct MyUBO {
    scale: f32;
};

var<uniform> myUBO: MyUBO;

[[stage(vertex)]]
fn vs_main(input : VertexInputs) -> FragmentInputs {
    gl_Position = scene.viewProjection * mesh.world * vec4<f32>(position * vec3<f32>(myUBO.scale), 1.0);
    vNormal = normal;
    vUV = uv;
}    
uniform vColor : array<vec4<f32>, 2>;

varying vNormal : vec3<f32>;
varying vUV : vec2<f32>;

var diffuse : texture_2d<f32>;
var mySampler : sampler;

[[stage(fragment)]]
fn fs_main(input : FragmentInputs) -> FragmentOutputs {
    gl_FragColor = textureSample(diffuse, mySampler, vUV) * uniforms.vColor[1];
}