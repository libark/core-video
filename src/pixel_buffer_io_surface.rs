use core_foundation::{
    base::{kCFAllocatorDefault, CFAllocatorRef, CFType, TCFType},
    dictionary::{CFDictionary, CFDictionaryRef},
    string::CFStringRef,
};
use io_surface::IOSurface;

use crate::{
    io_surface::IOSurfaceRef,
    pixel_buffer::{CVPixelBuffer, CVPixelBufferRef},
    return_::{kCVReturnSuccess, CVReturn},
};

extern "C" {
    pub static kCVPixelBufferIOSurfaceOpenGLTextureCompatibilityKey: CFStringRef;
    pub static kCVPixelBufferIOSurfaceOpenGLFBOCompatibilityKey: CFStringRef;
    pub static kCVPixelBufferIOSurfaceCoreAnimationCompatibilityKey: CFStringRef;
    pub static kCVPixelBufferIOSurfaceOpenGLESTextureCompatibilityKey: CFStringRef;
    pub static kCVPixelBufferIOSurfaceOpenGLESFBOCompatibilityKey: CFStringRef;

    pub fn CVPixelBufferGetIOSurface(pixelBuffer: CVPixelBufferRef) -> IOSurfaceRef;
    pub fn CVPixelBufferCreateWithIOSurface(
        allocator: CFAllocatorRef,
        surface: IOSurfaceRef,
        pixelBufferAttributes: CFDictionaryRef,
        pixelBufferOut: *mut CVPixelBufferRef,
    ) -> CVReturn;
}

impl CVPixelBuffer {
    pub fn from_io_surface(io_surface: IOSurface, options: &CFDictionary<CFType, CFType>) -> Result<CVPixelBuffer, CVReturn> {
        let mut pixel_buffer: CVPixelBufferRef = std::ptr::null_mut();
        let status = unsafe {
            CVPixelBufferCreateWithIOSurface(kCFAllocatorDefault, io_surface.as_concrete_TypeRef(), options.as_concrete_TypeRef(), &mut pixel_buffer)
        };
        if status == kCVReturnSuccess {
            Ok(unsafe { TCFType::wrap_under_create_rule(pixel_buffer) })
        } else {
            Err(status)
        }
    }

    pub fn get_io_surface(&self) -> Option<IOSurface> {
        unsafe {
            let surface = CVPixelBufferGetIOSurface(self.as_concrete_TypeRef());
            if surface.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(surface))
            }
        }
    }
}
