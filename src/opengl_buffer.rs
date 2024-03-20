use core_foundation::{
    base::{kCFAllocatorDefault, CFAllocatorRef, CFType, CFTypeID, TCFType},
    dictionary::{CFDictionary, CFDictionaryRef},
    string::{CFString, CFStringRef},
};
use libc::size_t;

use crate::{
    buffer::TCVBuffer,
    image_buffer::{CVImageBufferRef, TCVImageBuffer},
    return_::{kCVReturnSuccess, CVReturn},
    CGLContextObj, GLenum, GLint,
};

pub type CVOpenGLBufferRef = CVImageBufferRef;

extern "C" {
    pub static kCVOpenGLBufferWidth: CFStringRef;
    pub static kCVOpenGLBufferHeight: CFStringRef;
    pub static kCVOpenGLBufferTarget: CFStringRef;
    pub static kCVOpenGLBufferInternalFormat: CFStringRef;
    pub static kCVOpenGLBufferMaximumMipmapLevel: CFStringRef;

    pub fn CVOpenGLBufferGetTypeID() -> CFTypeID;
    pub fn CVOpenGLBufferRetain(buffer: CVOpenGLBufferRef) -> CVOpenGLBufferRef;
    pub fn CVOpenGLBufferRelease(buffer: CVOpenGLBufferRef);
    pub fn CVOpenGLBufferCreate(
        allocator: CFAllocatorRef,
        width: size_t,
        height: size_t,
        attributes: CFDictionaryRef,
        bufferOut: *mut CVOpenGLBufferRef,
    ) -> CVReturn;
    pub fn CVOpenGLBufferGetAttributes(openGLBuffer: CVOpenGLBufferRef) -> CFDictionaryRef;
    pub fn CVOpenGLBufferAttach(openGLBuffer: CVOpenGLBufferRef, cglContext: CGLContextObj, face: GLenum, level: GLint, screen: GLint) -> CVReturn;
}

impl TCVBuffer for CVOpenGLBuffer {}
impl TCVImageBuffer for CVOpenGLBuffer {}
pub struct CVOpenGLBuffer(CVOpenGLBufferRef);

impl Drop for CVOpenGLBuffer {
    fn drop(&mut self) {
        unsafe { CVOpenGLBufferRelease(self.0) }
    }
}

impl_TCFType!(CVOpenGLBuffer, CVOpenGLBufferRef, CVOpenGLBufferGetTypeID);
impl_CFTypeDescription!(CVOpenGLBuffer);

impl CVOpenGLBuffer {
    pub fn create(width: size_t, height: size_t, attributes: &CFDictionary<CFString, CFType>) -> Result<CVOpenGLBuffer, CVReturn> {
        let mut buffer: CVOpenGLBufferRef = std::ptr::null_mut();
        let status = unsafe { CVOpenGLBufferCreate(kCFAllocatorDefault, width, height, attributes.as_concrete_TypeRef(), &mut buffer) };
        if status == kCVReturnSuccess {
            Ok(unsafe { TCFType::wrap_under_create_rule(buffer) })
        } else {
            Err(status)
        }
    }

    pub fn get_attributes(&self) -> Option<CFDictionary<CFString, CFType>> {
        unsafe {
            let attributes = CVOpenGLBufferGetAttributes(self.as_concrete_TypeRef());
            if attributes.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(attributes))
            }
        }
    }

    pub fn attach(&self, cgl_context: CGLContextObj, face: GLenum, level: GLint, screen: GLint) -> Result<(), CVReturn> {
        let status = unsafe { CVOpenGLBufferAttach(self.as_concrete_TypeRef(), cgl_context, face, level, screen) };
        if status == kCVReturnSuccess {
            Ok(())
        } else {
            Err(status)
        }
    }
}
