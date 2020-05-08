









### Implement



- [x] joystick input

- [x] keyboard input via winit : https://docs.rs/winit/0.22.0/winit/event/enum.WindowEvent.html
- [x] live reload of shaders (implemented in Studio, still need to transfer to the main development environment)

- [x] aircraft model load and render

- [x] joystick input mapped to primitive-axis control of aircraft model

- [ ] integrate live shader reload from Studio into main development space Peregrine.  Studio will probably remain relevant as a testbed for new dev-tools, or something ancillary like that.

- [ ] improve terrain mesh generation algo, offload this to another process, a network linked process rather than a process that will die with the main process, would save iteration time on reloads.
    Use Vulkan external memory extension to allow the terrain-server module process independently started and maintained in a separate process, to load terrain objects into Vulkan device, and then the main development program will compile up and run and access that data.  This is for developmental speed first of all, but also will affect end-user run-time scenarios.
    https://www.reddit.com/r/rust_gamedev/comments/9ijsrs/raw_vulkan_bindings/
    Might need to switch some usage through Ash.

    **Check out**: https://github.com/Ralith/lahar

    _the above is described on the Ash readme as having "tools for asynchronously uploading data to a Vulkan device" ._

- [ ]  Port some usage from Vulkano to Ash and Vk-Sys, as per the research angle alluded in the Reddit thread above.  This will be necessary first of all for the implementation of the terrain server which decouples terrain memory management from the main development program code being iterated over.  Unless you are developing terrain parsing programs, which we are, in which case this terrain-server process becomes with a client/head a terrain-development studio.  instead of live shader reload, it will be based around importing 3dMax files or terrain hgt files.  There is an associated research angle here, which is can we serialize to disk in a format easier to make volatile in buffer than e.g. the original hgt file ?  The way we did the terrain parse in our POC program was partly dependent on a Vulkano implementation detail.  Will need to generalize.

- [ ] raytracing: research and start implementing a raytracing render pipeline.

- [ ] Study this framework documentation https://github.com/MaikKlein/ash
 in order to get another perspective on Vulkan, if not to abandon Vulkano.  Also, because, we'll need to use this and/or Vk-Sys to implement the Vk-external-memory feature and get a multi-process approach to the development structure, and possibly the end-user-program. Some computation could be carried out over a network for example.

- [ ] Kind of unfortunately for time-scale, but probably worth it in the end, is to rewrite the present implementations and capabilities into Ash.  This will give a better perspective into Vulkan, having both wrappers to contrast.  Also getting the sense that Vulkano insulates the developer from Vulkan too much, without a documented portal to a true API -- which I'm guessing Vk-Sys can provide, but not documented.  Ash seems to lack documentation, but seems to be built in a way where one should only need to consult Vulkan docs for the most part.  A Vulkano rep on Gitter alluded that Vulkano can be used in a way with side-by-side raw Vulkan bindings, but this is nowhere documented that I know of.
    - [ ] send in a query on Gitter if not an issue on Github, requesting documentation on accessing Vulkan API directly.  It may turn out to be necessary to implement some of Vulkano's safety mechanisms directly rather than use the framework, ... who knows, will prototype and research this.  
    - [ ] this is built locally and working.  good template for the above.


### Research

- raytracing

- winapi : https://docs.rs/winapi/0.3.8/winapi/
    This is the source for getting peripheral events, multi-input uses this.
