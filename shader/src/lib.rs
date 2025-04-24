#![cfg_attr(target_arch = "spirv", no_std)]
use spirv_std::{spirv, glam::UVec3};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Data {
    pub a: f32,
    pub b: [f32; 3],
    pub c: f32,
}

#[derive(Clone, Copy)]
pub union DataOrArray {
    arr: [f32; 5],
    str: Data,
}

impl DataOrArray {
    pub fn arr(self) -> [f32; 5] { unsafe { self.arr } }
    pub fn str(self) -> Data { unsafe { self.str } }
    pub fn new(arr: [f32; 5]) -> Self { Self { arr } }
}


#[spirv(compute(threads(64)))]
pub fn main(
    #[spirv(global_invocation_id)] UVec3 { x: id, .. }: UVec3,
    #[spirv(storage_buffer, binding=0)] input: &[DataOrArray],
    #[spirv(storage_buffer, binding=1)] output: &mut [[f32; 5]],
) {
    let id = id as usize;
    output[id] = input[id].arr();
}
