use core_foundation::{
    base::{kCFAllocatorDefault, CFAllocatorRef, CFType, CFTypeID, TCFType},
    dictionary::{CFDictionary, CFDictionaryRef},
    string::CFString,
};
use libc::{c_void, size_t};
#[cfg(feature = "objc")]
use objc::runtime::Object;

use crate::{
    base::CVOptionFlags,
    image_buffer::{CVImageBuffer, CVImageBufferRef},
    opengl_es_texture::{CVOpenGLESTexture, CVOpenGLESTextureRef},
    return_::{kCVReturnSuccess, CVReturn},
    GLenum, GLint, GLsizei,
};

#[repr(C)]
pub struct __CVOpenGLESTextureCache(c_void);

pub type CVOpenGLESTextureCacheRef = *mut __CVOpenGLESTextureCache;

#[cfg(feature = "objc")]
pub type CVEAGLContext = *mut Object;
#[cfg(not(feature = "objc"))]
pub type CVEAGLContext = *mut c_void;

extern "C" {
    pub fn CVOpenGLESTextureCacheCreate(
        allocator: CFAllocatorRef,
        cacheAttributes: CFDictionaryRef,
        eaglContext: CVEAGLContext,
        textureAttributes: CFDictionaryRef,
        cacheOut: *mut CVOpenGLESTextureCacheRef,
    ) -> CVReturn;
    pub fn CVOpenGLESTextureCacheCreateTextureFromImage(
        allocator: CFAllocatorRef,
        textureCache: CVOpenGLESTextureCacheRef,
        sourceImage: CVImageBufferRef,
        textureAttributes: CFDictionaryRef,
        target: GLenum,
        internalFormat: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        type_: GLenum,
        planeIndex: size_t,
        textureOut: *mut CVOpenGLESTextureRef,
    ) -> CVReturn;
    pub fn CVOpenGLESTextureCacheFlush(textureCache: CVOpenGLESTextureCacheRef, options: CVOptionFlags);
    pub fn CVOpenGLESTextureCacheGetTypeID() -> CFTypeID;
}

declare_TCFType! {
    CVOpenGLESTextureCache, CVOpenGLESTextureCacheRef
}
impl_TCFType!(CVOpenGLESTextureCache, CVOpenGLESTextureCacheRef, CVOpenGLESTextureCacheGetTypeID);
impl_CFTypeDescription!(CVOpenGLESTextureCache);

impl CVOpenGLESTextureCache {
    pub fn new(
        cache_attributes: &CFDictionary<CFString, CFType>,
        eagl_context: CVEAGLContext,
        texture_attributes: &CFDictionary<CFString, CFType>,
    ) -> Result<CVOpenGLESTextureCache, CVReturn> {
        let mut cache: CVOpenGLESTextureCacheRef = std::ptr::null_mut();
        let status = unsafe {
            CVOpenGLESTextureCacheCreate(
                kCFAllocatorDefault,
                cache_attributes.as_concrete_TypeRef(),
                eagl_context,
                texture_attributes.as_concrete_TypeRef(),
                &mut cache,
            )
        };
        if status == kCVReturnSuccess {
            Ok(unsafe { TCFType::wrap_under_create_rule(cache) })
        } else {
            Err(status)
        }
    }

    pub fn create_texture_from_image(
        &self,
        source_image: CVImageBuffer,
        texture_attributes: &CFDictionary<CFString, CFType>,
        target: GLenum,
        internal_format: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        type_: GLenum,
        plane_index: size_t,
    ) -> Result<CVOpenGLESTexture, CVReturn> {
        let mut texture: CVOpenGLESTextureRef = std::ptr::null_mut();
        let status = unsafe {
            CVOpenGLESTextureCacheCreateTextureFromImage(
                kCFAllocatorDefault,
                self.as_concrete_TypeRef(),
                source_image.as_concrete_TypeRef(),
                texture_attributes.as_concrete_TypeRef(),
                target,
                internal_format,
                width,
                height,
                format,
                type_,
                plane_index,
                &mut texture,
            )
        };
        if status == kCVReturnSuccess {
            Ok(unsafe { TCFType::wrap_under_create_rule(texture) })
        } else {
            Err(status)
        }
    }

    pub fn flush(&self, options: CVOptionFlags) {
        unsafe { CVOpenGLESTextureCacheFlush(self.as_concrete_TypeRef(), options) }
    }
}
