// scratch project sandbox workspace.  exploring Ash via the tutorial vulkan-tutorial-rust

use peregrine::ash_utility;
use peregrine::ash_utility::constants::*;
use peregrine::ash_utility::share;

use ash::version::EntryV1_0;
use ash::version::InstanceV1_0;
// use ash::{vk_version_major, vk_version_minor, vk_version_patch};
use ash::vk::{version_major, version_minor, version_patch};


use ash::vk;


use std::ffi::CString;
use std::ptr;

use winit::event::{Event, VirtualKeyCode, ElementState, KeyboardInput, WindowEvent};
use winit::event_loop::{EventLoop, ControlFlow};
use winit::window::Window;

const WINDOW_TITLE: &'static str = "ws 300";
// const WINDOW_WIDTH: u32 = 2000;
// const WINDOW_HEIGHT: u32 = 1400;


// const VALIDATION: ValidationInfo = ValidationInfo {
//     is_enabled: true,
//     required_validation_layers: ["VK_LAYER_KHRONOS_validation"],
// };


struct QueueFamilyIndices {
    graphics_family: Option<u32>,
}

impl QueueFamilyIndices {
    pub fn is_complete(&self) -> bool {
        self.graphics_family.is_some()
    }
}


struct VulkanApp {
    _entry: ash::Entry,
    instance: ash::Instance,
    debug_utils_loader: ash::extensions::ext::DebugUtils,
    debug_messager: vk::DebugUtilsMessengerEXT,
    _physical_device: vk::PhysicalDevice,
}


impl VulkanApp {

    pub fn new() -> VulkanApp {

        let entry = ash::Entry::new().unwrap();
        let instance = share::create_instance(
            &entry,
            WINDOW_TITLE,
            VALIDATION.is_enable,
            &VALIDATION.required_validation_layers.to_vec(),
        );
        // let instance = VulkanApp::create_instance(&entry);

        let (debug_utils_loader, debug_messager) = ash_utility::debug::setup_debug_utils(VALIDATION.is_enable, &entry, &instance);
        let physical_device = VulkanApp::pick_physical_device(&instance);

        VulkanApp {
            _entry: entry,
            instance,
            debug_utils_loader,
            debug_messager,
            _physical_device: physical_device,
        }
    }

    fn pick_physical_device(instance: &ash::Instance) -> vk::PhysicalDevice {
        let physical_devices = unsafe {
            instance
                .enumerate_physical_devices()
                .expect("Failed to enumerate Physical Devices.")
        };

        println!("{} devices (GPU) found with Vulkan support.", physical_devices.len());

        let mut result = None;

        for &physical_device in physical_devices.iter() {
            if VulkanApp::is_physical_device_suitable(instance, physical_device) {
                if result.is_none() {
                    result = Some(physical_device)
                }
            }
        }

        match result {
            None => panic!("Failed to find a suitable GPU."),
            Some(physical_device) => physical_device,
        }
    }

    fn is_physical_device_suitable(
        instance: &ash::Instance,
        physical_device: vk::PhysicalDevice,
    ) -> bool {
        let device_properties = unsafe { instance.get_physical_device_properties(physical_device)  };
        let device_features = unsafe { instance.get_physical_device_features(physical_device) };
        let device_queue_families = unsafe { instance.get_physical_device_queue_family_properties(physical_device) };

        let device_type = match device_properties.device_type {
            vk::PhysicalDeviceType::CPU => "Cpu",
            vk::PhysicalDeviceType::INTEGRATED_GPU => "Integrated GPU",
            vk::PhysicalDeviceType::DISCRETE_GPU => "Discrete GPU",
            vk::PhysicalDeviceType::OTHER => "Unknown",
            _ => panic!(),
        };

        let device_name = ash_utility::tools::vk_to_string(&device_properties.device_name);
        println!(
            "\tDevice Name: {}, id: {}, type: {}",
            device_name, device_properties.device_id, device_type
        );

        let major_version = version_major(device_properties.api_version);
        let minor_version = version_minor(device_properties.api_version);
        let patch_version = version_patch(device_properties.api_version);

        println!(
            "\tAPI Version: {}.{}.{}",
            major_version, minor_version, patch_version
        );

        println!("\tSupport Queue Family: {}", device_queue_families.len());
        println!("\t\tQueue Count | Graphics, Compute, Transfer, Sparse Binding");

        for queue_family in device_queue_families.iter() {
            let is_graphics_support = if queue_family.queue_flags.contains(vk::QueueFlags::GRAPHICS) {
                "support"
            } else {
                "unsupport"
            };
            let is_compute_support = if queue_family.queue_flags.contains(vk::QueueFlags::COMPUTE) {
                "support"
            } else {
                "unsupported"
            };
            let is_transfer_support = if queue_family.queue_flags.contains(vk::QueueFlags::TRANSFER) {
                "supported"
            } else {
                "unsupported"
            };
            let is_sparse_support = if queue_family.queue_flags.contains(vk::QueueFlags::SPARSE_BINDING) {
                "supported"
            } else {
                "unsupported"
            };

            println!(
                "\t\t{}\t     | {}, {}, {}, {}",
                queue_family.queue_count,
                is_graphics_support,
                is_compute_support,
                is_transfer_support,
                is_sparse_support
            );
        }

        println!(
            "\tGeometry Shader support: {}",
            if device_features.geometry_shader == 1 {
                "Supported"
            } else {
                "Unsupported"
            }
        );

        let indices = VulkanApp::find_queue_family(instance, physical_device);

        return indices.is_complete();
    }

    fn find_queue_family(
        instance: &ash::Instance,
        physical_device: vk::PhysicalDevice,
    ) -> QueueFamilyIndices {
        let queue_families = unsafe { instance.get_physical_device_queue_family_properties(physical_device) };

        let mut queue_family_indices = QueueFamilyIndices {
            graphics_family: None,
        };

        let mut index = 0;
        for queue_family in queue_families.iter() {
            if queue_family.queue_count > 0
                && queue_family.queue_flags.contains(vk::QueueFlags::GRAPHICS) {
                    queue_family_indices.graphics_family = Some(index);
                }
            if queue_family_indices.is_complete() {
                break;
            }

            index += 1;
        }

        queue_family_indices
    }



    // fn init_window(event_loop: &EventLoop<()>) -> winit::window::Window {
    //     winit::window::WindowBuilder::new()
    //         .with_title(WINDOW_TITLE)
    //         .with_inner_size(winit::dpi::LogicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT))
    //         .build(event_loop)
    //         .expect("Failed to create window.")
    // }

    // fn create_instance(entry: &ash::Entry) -> ash::Instance {
    //
    //     let app_name = CString::new(WINDOW_TITLE).unwrap();
    //     let engine_name = CString::new("Vulkan Engine").unwrap();
    //
    //     let app_info = vk::ApplicationInfo {
    //         s_type: vk::StructureType::APPLICATION_INFO,
    //         p_next: ptr::null(),
    //         p_application_name: app_name.as_ptr(),
    //         application_version: APPLICATION_VERSION,
    //         p_engine_name: engine_name.as_ptr(),
    //         engine_version: ENGINE_VERSION,
    //         api_version: API_VERSION,
    //     };
    //
    //     let extension_names = ash_utility::platforms::required_extension_names();
    //
    //     let create_info = vk::InstanceCreateInfo {
    //         s_type: vk::StructureType::INSTANCE_CREATE_INFO,
    //         p_next: ptr::null(),
    //         flags: vk::InstanceCreateFlags::empty(),
    //         p_application_info: &app_info,
    //         pp_enabled_layer_names: ptr::null(),
    //         enabled_layer_count: 0,
    //         pp_enabled_extension_names: extension_names.as_ptr(),
    //         enabled_extension_count: extension_names.len() as u32,
    //     };
    //
    //     let instance : ash::Instance = unsafe {
    //         entry
    //             .create_instance(&create_info, None)
    //             .expect("Failed to create instance")
    //     };
    //
    //     instance
    // }


    fn draw_frame(&mut self) {

    }

    pub fn main_loop(mut self, event_loop: EventLoop<()>, window: Window) {

        event_loop.run(move |event, _, control_flow| {

            match event {
                | Event::WindowEvent { event, .. } => {
                    match event {
                        | WindowEvent::CloseRequested => {
                            *control_flow = ControlFlow::Exit
                        },
                        | WindowEvent::KeyboardInput { input, .. } => {
                            match input {
                                | KeyboardInput { virtual_keycode, state, .. } => {
                                    match (virtual_keycode, state) {
                                        | (Some(VirtualKeyCode::Escape), ElementState::Pressed) => {
                                            dbg!();
                                            *control_flow = ControlFlow::Exit
                                        },
                                        | _ => {},
                                    }
                                },
                            }
                        },
                        | _ => {},
                    }
                },
                | Event::MainEventsCleared => {
                    window.request_redraw();
                },
                | Event::RedrawRequested(_window_id) => {
                    self.draw_frame();
                },
                _ => (),
            }
        })
    }
}

impl Drop for VulkanApp {
    fn drop(&mut self) {
        unsafe {
            if VALIDATION.is_enable {
                self.debug_utils_loader
                    .destroy_debug_utils_messenger(self.debug_messager, None);
            }

            self.instance.destroy_instance(None);
        }
    }
}

fn main() {
    println!("workspace 300: ");

    let event_loop = EventLoop::new();
    // let window = VulkanApp::init_window(&event_loop);

    let window = ash_utility::window::init_window(&event_loop, WINDOW_TITLE, WINDOW_WIDTH, WINDOW_HEIGHT);




    let vulkan_app = VulkanApp::new();
    vulkan_app.main_loop(event_loop, window);

}
