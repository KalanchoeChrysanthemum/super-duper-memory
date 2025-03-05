// rust-analyzer will complain about this file
// dont worry about it, it compiles fine, despite it looking like its missing the dependecy

// can fix in IDE setting
// VSCode =>  "rust-analyzer.cargo.features": ["gpu"] in settings.json
// Zed => ?
// nvim => ?

use wgpu::*;
use pollster;

// most of this code is ripped from other places, I just cranked up some of the numbers
// it basically freezes my system for a few seconds
// it runs something on my gpu, can measure it with `nvtop`

#[cfg(feature = "gpu")]
fn main() {

    println!("Running GPU benchmark...");

    let instance = Instance::default();
    let adapter = pollster::block_on(instance.request_adapter(&RequestAdapterOptions::default()))
        .expect("Failed to find a GPU adapter");
    let (device, queue) = pollster::block_on(adapter.request_device(&DeviceDescriptor::default(), None))
        .expect("Failed to create GPU device");


    // usually this would be in another glsl file
    // but i really just want this in one binary
    let shader_code = r#"
        @group(0) @binding(0) var<storage, read_write> buffer: array<u32>;

        @compute @workgroup_size(256)
        fn main(@builtin(global_invocation_id) id: vec3<u32>) {
            let index = id.x;
            var value: u32 = buffer[index];

            // Heavy computation loop
            for (var i = 0u; i < 50000000u; i++) { 
                value = (value * 1664525u + 1013904223u) ^ (value >> 5);
                if (i % 1024u == 0u) {
                    buffer[index] = value; // Force global memory writes
                }
            }

            buffer[index] = value;
        }
    "#;

    let shader = device.create_shader_module(ShaderModuleDescriptor {
        label: Some("Compute Shader"),
        source: ShaderSource::Wgsl(shader_code.into()),
    });

    let buffer_size = 1024 * 1024 * 4; // 4 mb
    let storage_buffer = device.create_buffer(&BufferDescriptor {
        label: Some("Storage Buffer"),
        size: buffer_size as u64,
        usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC | BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    let bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
        label: Some("Bind Group Layout"),
        entries: &[BindGroupLayoutEntry {
            binding: 0,
            visibility: ShaderStages::COMPUTE,
            ty: BindingType::Buffer {
                ty: BufferBindingType::Storage { read_only: false },
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }],
    });

    let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
        label: Some("Pipeline Layout"),
        bind_group_layouts: &[&bind_group_layout],
        push_constant_ranges: &[],
    });

    let compute_pipeline = device.create_compute_pipeline(&ComputePipelineDescriptor {
        label: Some("Compute Pipeline"),
        layout: Some(&pipeline_layout),
        module: &shader,
        entry_point: Some("main"),
        compilation_options: PipelineCompilationOptions::default(), 
        cache: None,
    });

    let bind_group = device.create_bind_group(&BindGroupDescriptor {
        label: Some("Bind Group"),
        layout: &bind_group_layout,
        entries: &[BindGroupEntry {
            binding: 0,
            resource: storage_buffer.as_entire_binding(),
        }],
    });

    let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor {
        label: Some("Command Encoder"),
    });

    {
        let mut compute_pass = encoder.begin_compute_pass(&ComputePassDescriptor {
            label: Some("Compute Pass"),
            timestamp_writes: None,
        });
        compute_pass.set_pipeline(&compute_pipeline);
        compute_pass.set_bind_group(0, &bind_group, &[]);
        // huge workload
        compute_pass.dispatch_workgroups(16384, 1, 1); 
    }

    queue.submit(Some(encoder.finish()));

    println!("Finished GPU benchmark");
}
