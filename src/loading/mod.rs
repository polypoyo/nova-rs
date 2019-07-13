//! Nova's file loading code
//! 
//! Implements a resource pack loader, which may be used for loading Optifine shaderpacks, Minecraft: Java Edition
//! resourcepacks, Bedrock engine resourcepacks, and Nova shaderpacks. It will accomplish this by not knowing about any
//! of those and will instead only take in file paths and will return either streams of bytes or strings. The resource
//! pack loader will also be able to read resource packs in either filesystem folders or a zip folder. It should be 
//! constructed in a way that will allow support for other zip formats