# FAQ

## Why switch to Rust?

C++ is a horrible language

C++ is just a specification, and that specification is only for the language and its standard library. There's no 
gold-standard compiler, standard library implementation, build system, or package manager, meaning that these things
must be provided by the community

While the community has done a decent job at providing compilers and standard libraries, or at least as good a job as
the C++ specification allows, they've failed to provide and use a useful build system or package manager. 

CMake is a complete embarrassment. It's stringly typed beyond all reason, uninitialized variables are automatically 
initialized to the empty string, most commands have little to no validation on file paths, low-level details constantly 
leak into your file - it's a wonderful example of how to make unusable software. Despite its many failures, CMake is the
de-facto standard C++ build system. I'm quite frankly aghast that so many C++ developers looked at CMake and though
"yes this is a good thing to use"

There are other build systems available, such as Meson or build2, but CMake has become so prolific that every third
party library worth using has a CMake file. This means that, no matter what build system Nova itself uses, it'll have
to handle dependencies that use CMake. CMake is not worth using in the slightest

The situation with package managers isn't much better than build systems. `VCPKG`, which Nova tried to use, is built on
CMake and thus has the same problems that CMake has. `Conan`, the other C++ package manager of any appreciable 
popularity, doesn't have package definitions for everything that Nova uses, so we'd have to write package definitions
for a lot of dependencies - and now we have to deal with CMake again

CMake is not worth using

### If you hate CMake, why not make your own build system/package manager?

I shouldn't have to

Almost every other language I've used - Java, C#, Python, Ruby, Haskell, Rust, Javascript - either ships with a build 
system and package manager, or has high-quality community-made build systems and package managers. The zeitgeist in 
every single other language I've used is so much better than C++ that it's not even funny. C++ is older than these other 
languages, so in theory the C++ community and C++ standards committee has had longer to solve this problem than these 
other languages, but they just haven't. Everyone simply accepts CMake

If I did make my own build system/package manager, I'd have to either convince everyone else to use it, write build 
files for every dependency I want to use, or have my build system interface with CMake. I'm not interested in filling
in a gap left by 30 years of C++ programmers who should be ashamed that they have yet to solve the problem of actually
using C++, I'm interested in making a renderer

## Why are you using Ash? Vulkano is so much nicer!

Vulkano hides a lot of the Vulkan API in an attempt to make Vulkan easier to work with. This has no benefit for Nova.
Nova doesn't use Vulkan directly, instead it has a Render Hardware Interface (RHI) which wraps all supported graphics
API. The RHI is tailored to exactly serve Nova's needs. In order to be implemented in the most efficient possible way,
the RHI needs to work with Vulkan as directly as possible, instead of working with a higher-level wrapper which tries
to hide parts of Vulkan