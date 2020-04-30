



use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer, ImmutableBuffer};
use vulkano::device::{Device, DeviceExtensions};

use std::sync::Arc;
use std::str::FromStr;
use std::io::prelude::*;
use std::path::Path;

pub fn process_str_ints(input : &str) -> Vec<u32> {
    let start = String::from(input);
    let mut lines = start.lines();
    let mut condition = true;
    let mut ret_vec : Vec<u32> = Vec::new();
    while condition == true {
        let cursor = lines.next();
        if cursor == None {
            condition = false;
        } else {
            let x300 = u32::from_str(cursor.unwrap());
            if x300.is_ok() == true {
                ret_vec.push(x300.unwrap());
            } else {
                println!("error on index parse with");
            }
        }
    }
    ret_vec
}

pub fn find_three_floats(input : &str) -> Option<Vec<f64>> {
    let x300 = String::from(input);
    let x301 : Vec<&str> = x300.split(" ").collect();
    if x301.len() == 3 {
        let x302 = f64::from_str(x301[0]);
        let x303 = f64::from_str(x301[1]);
        let x304 = f64::from_str(x301[2]);
        if (x302.is_ok() == true) && (x304.is_ok() == true) && (x304.is_ok() == true)  {
            return Some(vec!(x302.unwrap(), x303.unwrap(), x304.unwrap()));
        }
        else {
            return None
        }
    } else {
        return None
    }
}

pub fn process_str_floats(input: &str) -> Vec<Vec<f64>> {
    let start = String::from(input);
    let mut lines = start.lines();
    let mut condition = true;
    let mut ret_vec : Vec<Vec<f64>> = Vec::new();
    while condition == true {
        let cursor = lines.next();
        if cursor == None {
            condition = false;
        } else {
            // println!("The line: {:?}", cursor.unwrap());
            let x200 = find_three_floats(&cursor.unwrap());
            if x200 != None {
                ret_vec.push(x200.unwrap());
            }
        }
    }
    ret_vec
}

#[derive(Default, Copy, Clone)]
pub struct Vertex {
    pub position: (f32, f32, f32)
}

vulkano::impl_vertex!(Vertex, position);

#[derive(Default, Copy, Clone)]
pub struct Normal {
    pub normal: (f32, f32, f32)
}

vulkano::impl_vertex!(Normal, normal);

pub struct Package {
    pub vertex_buffer: std::sync::Arc<CpuAccessibleBuffer::<[f32]>>,
    pub normals_buffer: std::sync::Arc<CpuAccessibleBuffer::<[f32]>>,
    pub index_buffer: std::sync::Arc<CpuAccessibleBuffer::<[u32]>>
}



// April-28: Can we start one process that loads terrain, keep it
// alive while we restart another compiled process separate, then share
// data on the heap via an <Arc> ?
// That would be a great way to have terrain already loaded and waiting.
// Not such a bad development line, as although it doesn't directly address
// the simulation requirements, it does support development velocity, so offers
// perhaps the best long-term utility on immediate work.
// Anyways there is something slighty amiss in our terrain generation algo, because
// there are degenerate triangles.  

// I don't want the terrain build process to bottleneck the main development process, as it takes quite a bit of time.
// Maybe this could be a separate process, which sends the data over networked.
// Begs the question of how to transmit the data over network.  Loading the data
// into a CpuAccessibleBuffer is a non-starter as those are not available to transfer between processes, or they may be?  Can two applications share a Vulkan instance in some way I wonder.  This could be a research angle.
// Also, Todo, there were some degenerate triangles in this generated mesh, will need to fix the algo.
pub fn build_terrain(device: Arc<Device>) -> (Arc<CpuAccessibleBuffer::<[Vertex]>>, Arc<CpuAccessibleBuffer::<[Normal]>>, Arc<CpuAccessibleBuffer::<[u32]>>) {
    let mut terrain_f = std::fs::File::open("models_materials/terrain_mesh_003.txt").unwrap();
    let mut terrain_buffer = String::new();
    terrain_f.read_to_string(&mut terrain_buffer).unwrap();

    // The poor naming was a stub to move quickly, can refactor
    // with integration of some of these lines, and improve naming.
    let x99 : Vec<&str> = terrain_buffer.split("Vertices:").collect();
    let x100 = String::from(x99[1]);
    let x101 : Vec<&str> = x100.split("Indices:").collect();
    let x102 = String::from(x101[0]);  // This should just mostly be vertices with maybe a blank line and the title line "Vertices:"
    let x103 = String::from(x101[1]); // This should have indices and normals
    let x104 : Vec<&str> = x103.split("Normals:").collect();
    let x160 = String::from(x104[0]); // This should be indices
    let x105 = String::from(x104[1]); // This should be normals
    let x106 = process_str_floats(&x102); // This should be a vector that we can turn into a positions buffer vertex_buffer
    let mut x200 : Vec<Vertex> = Vec::new();

    for (idx, item) in x106.iter().enumerate() {
        x200.push( Vertex { position: (item[0] as f32, item[1] as f32, item[2] as f32)} );
    }

    let vertex_buffer_terrain = CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(), false, x200.iter().cloned()).unwrap();
    let x107 = process_str_floats(&x105);
    let mut x300 : Vec<Normal> = Vec::new();
    for (idx, item) in x107.iter().enumerate() {
        x300.push( Normal { normal: (item[0] as f32, item[1] as f32, item[2] as f32)} );
    }

    let normals_buffer_terrain = CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(), false, x300.iter().cloned()).unwrap();

    let x161 = process_str_ints(&x160);

    let index_buffer_terrain = CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(), false, x161.iter().cloned()).unwrap();

    (vertex_buffer_terrain, normals_buffer_terrain, index_buffer_terrain)
}

pub fn build_lear(device: Arc<Device>) -> Vec<Package> {
    let lear = tobj::load_obj(&Path::new("models_materials/lear_300.obj"));
    let (models, materials) = lear.unwrap();
    let mut mashes : Vec<Package> = Vec::new();

    for (index, model) in models.iter().enumerate() {
        let mesh_500 = &model.mesh;
        mashes.push(Package {
            vertex_buffer: CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(), false, mesh_500.positions.iter().cloned()).unwrap(),
            normals_buffer: CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(), false, mesh_500.normals.iter().cloned()).unwrap(),
            index_buffer: CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(), false, mesh_500.indices.iter().cloned()).unwrap()
        });
    }
    mashes
}
