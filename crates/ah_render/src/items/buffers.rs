use wgpu::Buffer;

struct AHVertexBuffer{
    allocated_size: u32,
    handle:Buffer,
    label:String,
    uploaded:bool,
    attributes:AHTypeVertex,
}

enum AHTypeVertex{
    Position,
    Color,
    TexCoord,
}
struct AHIndexBuffer{
    allocated_size: u32,
    handle:Buffer,
    uploaded:bool,
}
struct AHVertexGroup{
    id:String,
    vertex_buffers:Vec<AHVertexBuffer>,
    index_buffer:AHIndexBuffer,
}

