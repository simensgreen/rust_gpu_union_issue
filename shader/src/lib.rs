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
    pub fn arr_ref(&self) -> &[f32; 5] { unsafe { &self.arr } }
    pub fn str(self) -> Data { unsafe { self.str } }
    pub fn new(arr: [f32; 5]) -> Self { Self { arr } }
}

#[derive(Clone, Copy)]
pub union DataMatrix {
    array1d: [f32; 25],
    array2d: [[f32; 5]; 5],
    data_array: [Data; 5],
}

impl DataMatrix {
    pub fn array1d(self) -> [f32; 25] { unsafe { self.array1d } }
    pub fn array2d(self) -> [[f32; 5]; 5] { unsafe { self.array2d } }
    pub fn array2d_ref(&self) -> &[[f32; 5]; 5] { unsafe { &self.array2d } }
    pub fn data_array(self) -> [Data; 5] { unsafe { self.data_array } } 
    pub fn new(array1d: [f32; 25]) -> Self { Self { array1d } }
}


#[spirv(compute(threads(64)))]
pub fn main(
    #[spirv(global_invocation_id)] UVec3 { x: id, .. }: UVec3,
    #[spirv(storage_buffer, binding=0)] input: &[DataOrArray],
    #[spirv(storage_buffer, binding=0)] input_matrices: &[DataMatrix],
    #[spirv(storage_buffer, binding=1)] output: &mut [[f32; 5]],
) {
    let id = id as usize;
    // cannot cast between pointer types
    output[id] = input_matrices[id].array2d_ref()[0];
    // cannot memcpy dynamically sized data
    // output[id] = input_matrices[id].array2d()[0];
    output[id] = *input[id].arr_ref();
}
