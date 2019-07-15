use crate::rhi::{
    dx12::{
        dx12_buffer::Dx12Buffer, dx12_descriptor_set::Dx12DescriptorSet, dx12_framebuffer::Dx12Framebuffer,
        dx12_pipeline::Dx12Pipeline, dx12_pipeline_interface::Dx12PipelineInterface, dx12_renderpass::Dx12Renderpass,
    },
    CommandList, ResourceBarrier,
};

pub struct Dx12CommandList {}

impl CommandList for Dx12CommandList {
    type Buffer = Dx12Buffer;
    type CommandList = Dx12CommandList;
    type Renderpass = Dx12Renderpass;
    type Framebuffer = Dx12Framebuffer;
    type Pipeline = Dx12Pipeline;
    type DescriptorSet = Dx12DescriptorSet;
    type PipelineInterface = Dx12PipelineInterface;

    fn resource_barriers(stages_before_barrier: _, stages_after_barrier: _, barriers: Vec<ResourceBarrier>) {
        unimplemented!()
    }

    fn copy_buffer(
        destination_buffer: Dx12Buffer,
        destination_offset: u64,
        source_buffer: Dx12Buffer,
        source_offset: u64,
        num_bytes: u64,
    ) {
        unimplemented!()
    }

    fn execute_command_lists(lists: Vec<Dx12CommandList>) {
        unimplemented!()
    }

    fn begin_renderpass(renderpass: Dx12Renderpass, framebuffer: Dx12Framebuffer) {
        unimplemented!()
    }

    fn end_renderpass() {
        unimplemented!()
    }

    fn bind_pipeline(pipeline: Dx12Pipeline) {
        unimplemented!()
    }

    fn bind_descriptor_sets(descriptor_sets: Vec<Dx12DescriptorSet>, pipeline_interface: Dx12PipelineInterface) {
        unimplemented!()
    }

    fn bind_vertex_buffers(buffers: Vec<Dx12Buffer>) {
        unimplemented!()
    }

    fn bind_index_buffer(buffer: Dx12Buffer) {
        unimplemented!()
    }

    fn draw_indexed_mesh(num_indices: u32, num_instances: u32) {
        unimplemented!()
    }
}
