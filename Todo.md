









### Implement



- [x] joystick input

- [x] keyboard input via winit : https://docs.rs/winit/0.22.0/winit/event/enum.WindowEvent.html
- [x] live reload of shaders (implemented in Studio, still need to transfer to the main development environment)

- [x] aircraft model load and render

- [x] joystick input mapped to primitive-axis control of aircraft model

- [ ] integrate live shader reload from Studio into main development space Peregrine.  Studio will probably remain relevant as a testbed for new dev-tools, or something ancillary like that.

- [ ] improve terrain mesh generation algo, offload this to another process, a network linked process rather than a process that will die with the main process, would save iteration time on reloads.

- raytracing: research and start implementing a raytracing render pipeline.





### Research

- raytracing

- winapi : https://docs.rs/winapi/0.3.8/winapi/
    This is the source for getting peripheral events, multi-input uses this.
