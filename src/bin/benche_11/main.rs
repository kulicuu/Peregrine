


// So, what we want to do in this 'lab bench' kind of thing, is start a process,
// this process will load terrain into memory according to the procedure
// we've implemented in Peregrine.  Then once that's started, we'll start another process,
// via a distinct cargo command, and this process needs to load the terrain somehow from the
// first process, and then render it.
// If we're successful then we can save tons of time in development, by separating costly terrain
// generating procedures from the main developmet editing loop.  This will also come in handy
// in a real program by speeding up for example certain scenario reload situations.


// The fastest way to prove this would be to get an Arc or something similar setup with some kind of buffer
// then just immediately try to connect another process and share the reference.
// The rest is already demonstrated.





// https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_KHR_external_memory.html
















use vulkano::instance::{Instance, PhysicalDevice};
use vulkano::instance::InstanceExtensions;

use vulkano::device::{Device, DeviceExtensions, Features};



fn main() {
    println!("benche 11");


    let instance = Instance::new(None, &InstanceExtensions::none(), None)
    .expect("failed to create instance");


    let physical = PhysicalDevice::enumerate(&instance).next()
    .expect("no device available");


    let queue_family = physical.queue_families()
    .find(|&q| {
        println!("q: {:?}", q);
        q.supports_graphics()
    })
    .expect("couldn't find a grahpical queue family");


    let (device, mut queues) = {
        Device::new(physical, &Features::none(), &DeviceExtensions::none(),
        [(queue_family, 0.5)].iter().cloned()).expect("failed to create device")
    };

    let queue = queues.next().unwrap();

}
