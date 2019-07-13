/// Describes what kind of command allocator you want to create
pub struct CommandAllocatorCreateInfo {
    /// The type of command lists which will be allocated by this command allocator
    type : CommandListType;
}