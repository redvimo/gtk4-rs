// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Renderer;
use glib::object::Cast;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    pub struct NglRenderer(Object<ffi::GskNglRenderer, ffi::GskNglRendererClass>) @extends Renderer;

    match fn {
        get_type => || ffi::gsk_ngl_renderer_get_type(),
    }
}

impl NglRenderer {
    #[doc(alias = "gsk_ngl_renderer_new")]
    pub fn new() -> NglRenderer {
        assert_initialized_main_thread!();
        unsafe { Renderer::from_glib_full(ffi::gsk_ngl_renderer_new()).unsafe_cast() }
    }
}

#[cfg(any(feature = "v4_2", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v4_2")))]
impl Default for NglRenderer {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for NglRenderer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("NglRenderer")
    }
}
