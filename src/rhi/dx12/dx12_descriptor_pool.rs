use crate::rhi::{
    dx12::{dx12_descriptor_set::Dx12DescriptorSet, dx12_pipeline_interface::Dx12PipelineInterface},
    DescriptorPool,
};

pub struct Dx12DescriptorPool {}

impl DescriptorPool for Dx12DescriptorPool {
    type PipelineInterface = Dx12PipelineInterface;
    type DescriptorSet = Dx12DescriptorSet;

    fn create_descriptor_sets(&self, pipeline_interface: Dx12PipelineInterface) -> Vec<Dx12DescriptorSet> {
        unimplemented!()
    }
}
