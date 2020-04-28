#  Terrain-Server:

This project aims at setting up a kind of local terrain server.  I'd like to see if we could setup a process, even with perhaps an interface front-end, that would preload a selection of terrain, put it into heap memory, and wait for clients that will want access to this heap stored terrain data.  The goal will be acheived if the non-serialized, memory reference data can be shared between processes that were not linked statically.  I.e. a process can approach via the network and request memory like Rust's Arc type.


It will need another project for an independent executable client.


Another technical approach here would be possible if it's possible to share a Vulkan instance -- including device memory buffers, between applications.  I'm guessing it's not but I haven't fully researched it yet.



## Resources:

https://saprykin.github.io/plibsys-docs/pmain_8h.html

https://github.com/saprykin/plibsys
