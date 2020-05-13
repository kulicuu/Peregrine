

// was studying vulkan-tutorial-rust and got hit with breaking changes as this version of Ash 0.31.0 has breaking changes against the tutorial's usage of legacy ash 0.29.0.  So going through the Ash example which is being updated recently to guage their recommended usage -- also check the issue critiquing the patterns in the examples.  Will go through this a few times and maybe take revisions back to a fork of the tutorial code.


use winapi;


use ash::extensions::khr::Win32Surface;
pub use ash::version::{DeviceV1_0, EntryV1_0, InstanceV1_0};
use ash::{vk, Device, Entry, Instance};
use std::borrow::Cow;
use std::cell::RefCell;
use std::default::Default;
use std::ffi::{CStr, CString};
use std::ops::Drop;
use std::os::raw::c_void;
// use winit::window::Window;


#[macro_export]
macro_rules! offset_of {
    ($base:path, $field:ident) => {{
        #[allow(unused_unsafe)]
        unsafe {
            let b: $base = mem::zeroed();
            (&b.$field as *const _ as isize) - (&b as *const _ as isize)
        }
    }};
}


pub fn record_submit_commandbuffer<D: DeviceV1_0, F: FnOnce(&D, vk::CommandBuffer)>(
    device: &D,
    command_buffer: vk::CommandBuffer,
    submit_queue: vk::Queue,
    wait_mask: &[vk::PipelineStageFlags],
    wait_semaphores: &[vk::Semaphore],
    signal_semaphores: &[vk::Semaphore],
    f: F,
) {
    unsafe {
        device
            .reset_command_buffer(
                command_buffer,
                vk::CommandBufferResetFlags::RELEASE_RESOURCES,
            )
            .expect("Reset command buffer failed.");

        let command_buffer_begin_info = vk::CommandBufferBeginInfo::builder()
            .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);

        device
            .begin_command_buffer(command_buffer, &command_buffer_begin_info)
            .expect("Begin commandbuffer");

        f(device, command_buffer);

        device
            .end_command_buffer(command_buffer)
            .expect("End commandbuffer");

        let submit_fence = device
            .create_fence(&vk::FenceCreateInfo::default(), None)
            .expect("Create fence failed.");

        let command_buffers = vec![command_buffer];

        let submit_info = vk::SubmitInfo::builder()
            .wait_semaphores(wait_semaphores)
            .wait_dst_stage_mask(wait_mask)
            .command_buffers(&command_buffers)
            .signal_semaphores(signal_semaphores);

        device
            .queue_submit(submit_queue, &[submit_info.build()], submit_fence)
            .expect("queue submit failed");

        device
            .wait_for_fences(&[submit_fence], true, std::u64::MAX)
            .expect("Wait for fence failed");

        device.destroy_fence(submit_fence, None);
    }
}

#[cfg(target_os = "windows")]
unsafe fn create_surface<E: EntryV1_0, I: InstanceV1_0>(
    entry: &E,
    instance: &I,
    window: &winit::window::Window,
) -> Result<vk::SurfaceKHR, vk::Result> {
    use std::ptr;
    use winapi::shared::windef::HWND;
    use winapi::um::libloaderapi::GetModuleHandleW;
    use winit::platform::windows::WindowExtWindows;
    let hwnd = window.hwnd() as HWND;
    let hinstance = GetModuleHandleW(ptr::null()) as *const c_void;
    let win32_create_info = vk::Win32SurfaceCreateInfoKHR {
        s_type: vk::StructureType::WIN32_SURFACE_CREATE_INFO_KHR,
        p_next: ptr::null(),
        flags: Default::default(),
        hinstance,
        hwnd: hwnd as *const c_void,
    };
    let win32_surface_loader = Win32Surface::new(entry, instance);
    win32_surface_loader.create_win32_surface(&win32_create_info, None)
}




pub struct AshBase {
    pub entry: Entry,

}













fn main () {
    println!("ws-400");



}
