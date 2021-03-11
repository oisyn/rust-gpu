use glam::{Vec2, Vec3A, Vec4};

use crate::{integer::Integer, vector::Vector};

#[spirv(sampler)]
#[derive(Copy, Clone)]
pub struct Sampler {
    _x: u32,
}

#[spirv(image_type(
    // sampled_type is hardcoded to f32 for now
    dim = "Dim2D",
    depth = 0,
    arrayed = 0,
    multisampled = 0,
    sampled = 1,
    image_format = "Unknown"
))]
#[derive(Copy, Clone)]
pub struct Image2d {
    _x: u32,
}

impl Image2d {
    #[spirv_std_macros::gpu_only]
    pub fn sample(&self, sampler: Sampler, coordinate: Vec2) -> Vec4 {
        unsafe {
            let mut result = Default::default();
            asm!(
                "%image = OpLoad _ {this}",
                "%sampler = OpLoad _ {sampler}",
                "%coordinate = OpLoad _ {coordinate}",
                "%sampledImage = OpSampledImage _ %image %sampler",
                "%result = OpImageSampleImplicitLod _ %sampledImage %coordinate",
                "OpStore {result} %result",
                result = in(reg) &mut result,
                this = in(reg) self,
                sampler = in(reg) &sampler,
                coordinate = in(reg) &coordinate,
            );
            result
        }
    }
    /// Fetch a single texel with a sampler set at compile time
    #[spirv_std_macros::gpu_only]
    pub fn fetch<I>(&self, coordinate: impl Vector<I>) -> Vec4
    where
        I: Integer,
    {
        let mut result = Vec4::default();
        unsafe {
            asm! {
                "%image = OpLoad _ {this}",
                "%coordinate = OpLoad _ {coordinate}",
                "%result = OpImageFetch typeof*{result} %image %coordinate",
                "OpStore {result} %result",
                result = in(reg) &mut result,
                this = in(reg) self,
                coordinate = in(reg) &coordinate,
            }
        }

        result
    }
}

#[spirv(image_type(
    // sampled_type is hardcoded to f32 for now
    dim = "Dim2D",
    depth = 0,
    arrayed = 0,
    multisampled = 0,
    sampled = 2,
    image_format = "Unknown"
))]
#[derive(Copy, Clone)]
pub struct StorageImage2d {
    _x: u32,
}

impl StorageImage2d {
    /// Read a texel from an image without a sampler.
    #[spirv_std_macros::gpu_only]
    pub fn read<I, V, V2>(&self, coordinate: V) -> V2
    where
        I: Integer,
        V: Vector<I>,
        V2: Vector<f32>,
    {
        let mut result = V2::default();

        unsafe {
            asm! {
                "%image = OpLoad _ {this}",
                "%coordinate = OpLoad _ {coordinate}",
                "%result = OpImageRead typeof*{result} %image %coordinate",
                "OpStore {result} %result",
                this = in(reg) self,
                coordinate = in(reg) &coordinate,
                result = in(reg) &mut result,
            }
        }

        result
    }

    /// Write a texel to an image without a sampler.
    #[spirv_std_macros::gpu_only]
    pub unsafe fn write<I, V, V2>(&self, coordinate: V, texels: V2)
    where
        I: Integer,
        V: Vector<I>,
        V2: Vector<f32>,
    {
        asm! {
            "%image = OpLoad _ {this}",
            "%coordinate = OpLoad _ {coordinate}",
            "%texels = OpLoad _ {texels}",
            "OpImageWrite %image %coordinate %texels",
            this = in(reg) self,
            coordinate = in(reg) &coordinate,
            texels = in(reg) &texels,
        }
    }
}

#[spirv(image_type(
    // sampled_type is hardcoded to f32 for now
    dim = "Dim2D",
    depth = 0,
    arrayed = 1,
    multisampled = 0,
    sampled = 1,
    image_format = "Unknown"
))]
#[derive(Copy, Clone)]
pub struct Image2dArray {
    _x: u32,
}

impl Image2dArray {
    #[spirv_std_macros::gpu_only]
    pub fn sample(&self, sampler: Sampler, coordinate: Vec3A) -> Vec4 {
        unsafe {
            let mut result = Default::default();
            asm!(
                "%image = OpLoad _ {this}",
                "%sampler = OpLoad _ {sampler}",
                "%coordinate = OpLoad _ {coordinate}",
                "%sampledImage = OpSampledImage _ %image %sampler",
                "%result = OpImageSampleImplicitLod _ %sampledImage %coordinate",
                "OpStore {result} %result",
                result = in(reg) &mut result,
                this = in(reg) self,
                sampler = in(reg) &sampler,
                coordinate = in(reg) &coordinate,
            );
            result
        }
    }
}

#[spirv(sampled_image)]
#[derive(Copy, Clone)]
pub struct SampledImage<I> {
    _image: I,
}

impl SampledImage<Image2d> {
    #[spirv_std_macros::gpu_only]
    pub fn sample(&self, coordinate: Vec2) -> Vec4 {
        unsafe {
            let mut result = Default::default();
            asm!(
                "%sampledImage = OpLoad _ {this}",
                "%coordinate = OpLoad _ {coordinate}",
                "%result = OpImageSampleImplicitLod _ %sampledImage %coordinate",
                "OpStore {result} %result",
                result = in(reg) &mut result,
                this = in(reg) self,
                coordinate = in(reg) &coordinate
            );
            result
        }
    }
}