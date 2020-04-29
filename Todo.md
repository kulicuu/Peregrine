









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

- [ ]  Port some usage from Vulkano to Ash and Vk-Sys, as per the research angle alluded in the Reddit thread above.  This will be necessary first of all for the implementation of the terrain server which decouples terrain memory management from the main development program code being iterated over.  Unless you are developing terrain parsing programs, which we are, in which case this terrain-server process becomes with a client/head a terrain-development studio.  instead of live shader reload, it will be based around importing 3dMax files or terrain hgt files.  There is an associated research angle here, which is can we serialize to disk in a format easier to make volatile in buffer than e.g. the original hgt file ?  The way we did the terrain parse in our POC program was partly dependent on a Vulkano implementation detail.  Will need to generalize.

- [ ] raytracing: research and start implementing a raytracing render pipeline.

- [ ] Study this framework documentation https://github.com/MaikKlein/ash
 in order to get another perspective on Vulkan, if not to abandon Vulkano.  Also, because, we'll need to use this and/or Vk-Sys to implement the Vk-external-memory feature and get a multi-process approach to the development structure, and possibly the end-user-program. Some computation could be carried out over a network for example.



### Research

- raytracing

- winapi : https://docs.rs/winapi/0.3.8/winapi/
    This is the source for getting peripheral events, multi-input uses this.
