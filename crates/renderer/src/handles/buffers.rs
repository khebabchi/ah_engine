use wgpu::util::DeviceExt;
/// Vertex metadata
 
struct VertexBufer {
    buffer: wgpu::Buffer,
    layout: wgpu::VertexBufferLayout<'static>,
}

impl VertexBufer {
    pub(crate) fn new<Vx: bytemuck::Pod + VertexMetadata>(
        device: &wgpu::Device,
        data: Vec<Vx>,
    ) -> Self {
        let layout = Vx::layout();
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&data),
            usage: wgpu::BufferUsages::VERTEX,
        });

        Self { buffer, layout }
    }
}
 
struct IndexBufer {
    buffer: wgpu::Buffer,
    indices_count:u32
}

impl IndexBufer {
    pub(crate) fn new(
        device: &wgpu::Device,
        data: Vec<u32>,indices_count:u32
    ) -> Self {
        let layout = Vx::layout();
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&data[..indices_count]),
            usage: wgpu::BufferUsages::INDEX,
        });

        Self { buffer, indices_count }
    }
}


 let index_buffer = device.create_buffer_init(
    &wgpu::util::BufferInitDescriptor {
        label: Some("Index Buffer"),
        contents: bytemuck::cast_slice(INDICES),
        usage: wgpu::BufferUsages::INDEX,
    }
);
let num_indices = INDICES.len() as u32;