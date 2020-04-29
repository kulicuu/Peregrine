#  Terrain-Server:

This project aims at setting up a kind of local terrain server.  I'd like to see if we could setup a process, even with perhaps an interface front-end, that would preload a selection of terrain, put it into heap memory, and wait for clients that will want access to this heap stored terrain data.  The goal will be acheived if the non-serialized, memory reference data can be shared between processes that were not linked statically.  I.e. a process can approach via the network and request memory like Rust's Arc type.


It will need another project for an independent executable client.


Another technical approach here would be possible if it's possible to share a Vulkan instance -- including device memory buffers, between applications.  I'm guessing it's not but I haven't fully researched it yet.

Yeah, after more research, it looks like -- although sharing Vulkan instances would be a non-starter, shared memory may be possible.  In this case the terrain server could load data which the main application could use.  Looking into it more...

For example, looking at the resources on https://www.informit.com/articles/article.aspx?p=2756465&seqNum=3

, we could probably find ways to write to Vulkan device memory in a way that is visible from distinct applications.  This has immediate legitimacy as an experimental project.  Start one process, create some memory artifacts in Vulkan via an instance, then start another application, and try also to access device memory.  Will it keep each application's memory sequestered or allow shared access?

https://stackoverflow.com/questions/49920858/how-to-share-buffer-or-image-between-multiple-vkdevices

It would also be a good idea to understand best way to accomplish this in Rust, though for our use-case, likely the purely Vulkan option would be better.



# !!! https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_KHR_external_memory.html

> An application may wish to reference device memory in multiple Vulkan logical devices or instances, in multiple **processes**, and/or in multiple APIs

## Resources:

https://saprykin.github.io/plibsys-docs/pmain_8h.html

https://github.com/saprykin/plibsys
