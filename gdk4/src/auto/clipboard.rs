// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::ContentFormats;
use crate::ContentProvider;
use crate::Display;
use crate::Texture;
use glib::object::IsA;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use std::pin::Pin;
use std::ptr;

glib::wrapper! {
    #[doc(alias = "GdkClipboard")]
    pub struct Clipboard(Object<ffi::GdkClipboard>);

    match fn {
        type_ => || ffi::gdk_clipboard_get_type(),
    }
}

impl Clipboard {
    #[doc(alias = "gdk_clipboard_get_content")]
    #[doc(alias = "get_content")]
    pub fn content(&self) -> Option<ContentProvider> {
        unsafe { from_glib_none(ffi::gdk_clipboard_get_content(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_clipboard_get_display")]
    #[doc(alias = "get_display")]
    pub fn display(&self) -> Display {
        unsafe { from_glib_none(ffi::gdk_clipboard_get_display(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_clipboard_get_formats")]
    #[doc(alias = "get_formats")]
    pub fn formats(&self) -> ContentFormats {
        unsafe { from_glib_none(ffi::gdk_clipboard_get_formats(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_clipboard_is_local")]
    pub fn is_local(&self) -> bool {
        unsafe { from_glib(ffi::gdk_clipboard_is_local(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_clipboard_read_text_async")]
    pub fn read_text_async<P: FnOnce(Result<Option<glib::GString>, glib::Error>) + 'static>(
        &self,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
        callback: P,
    ) {
        let main_context = glib::MainContext::ref_thread_default();
        let is_main_context_owner = main_context.is_owner();
        let has_acquired_main_context = (!is_main_context_owner)
            .then(|| main_context.acquire().ok())
            .flatten();
        assert!(
            is_main_context_owner || has_acquired_main_context.is_some(),
            "Async operations only allowed if the thread is owning the MainContext"
        );

        let user_data: Box_<glib::thread_guard::ThreadGuard<P>> =
            Box_::new(glib::thread_guard::ThreadGuard::new(callback));
        unsafe extern "C" fn read_text_async_trampoline<
            P: FnOnce(Result<Option<glib::GString>, glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut gio::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret =
                ffi::gdk_clipboard_read_text_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
                Box_::from_raw(user_data as *mut _);
            let callback: P = callback.into_inner();
            callback(result);
        }
        let callback = read_text_async_trampoline::<P>;
        unsafe {
            ffi::gdk_clipboard_read_text_async(
                self.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    pub fn read_text_future(
        &self,
    ) -> Pin<
        Box_<
            dyn std::future::Future<Output = Result<Option<glib::GString>, glib::Error>> + 'static,
        >,
    > {
        Box_::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
            obj.read_text_async(Some(cancellable), move |res| {
                send.resolve(res);
            });
        }))
    }

    #[doc(alias = "gdk_clipboard_read_texture_async")]
    pub fn read_texture_async<P: FnOnce(Result<Option<Texture>, glib::Error>) + 'static>(
        &self,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
        callback: P,
    ) {
        let main_context = glib::MainContext::ref_thread_default();
        let is_main_context_owner = main_context.is_owner();
        let has_acquired_main_context = (!is_main_context_owner)
            .then(|| main_context.acquire().ok())
            .flatten();
        assert!(
            is_main_context_owner || has_acquired_main_context.is_some(),
            "Async operations only allowed if the thread is owning the MainContext"
        );

        let user_data: Box_<glib::thread_guard::ThreadGuard<P>> =
            Box_::new(glib::thread_guard::ThreadGuard::new(callback));
        unsafe extern "C" fn read_texture_async_trampoline<
            P: FnOnce(Result<Option<Texture>, glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut gio::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret =
                ffi::gdk_clipboard_read_texture_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
                Box_::from_raw(user_data as *mut _);
            let callback: P = callback.into_inner();
            callback(result);
        }
        let callback = read_texture_async_trampoline::<P>;
        unsafe {
            ffi::gdk_clipboard_read_texture_async(
                self.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    pub fn read_texture_future(
        &self,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<Option<Texture>, glib::Error>> + 'static>>
    {
        Box_::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
            obj.read_texture_async(Some(cancellable), move |res| {
                send.resolve(res);
            });
        }))
    }

    #[doc(alias = "gdk_clipboard_read_value_async")]
    pub fn read_value_async<P: FnOnce(Result<glib::Value, glib::Error>) + 'static>(
        &self,
        type_: glib::types::Type,
        io_priority: glib::Priority,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
        callback: P,
    ) {
        let main_context = glib::MainContext::ref_thread_default();
        let is_main_context_owner = main_context.is_owner();
        let has_acquired_main_context = (!is_main_context_owner)
            .then(|| main_context.acquire().ok())
            .flatten();
        assert!(
            is_main_context_owner || has_acquired_main_context.is_some(),
            "Async operations only allowed if the thread is owning the MainContext"
        );

        let user_data: Box_<glib::thread_guard::ThreadGuard<P>> =
            Box_::new(glib::thread_guard::ThreadGuard::new(callback));
        unsafe extern "C" fn read_value_async_trampoline<
            P: FnOnce(Result<glib::Value, glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut gio::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret =
                ffi::gdk_clipboard_read_value_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(from_glib_none(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
                Box_::from_raw(user_data as *mut _);
            let callback: P = callback.into_inner();
            callback(result);
        }
        let callback = read_value_async_trampoline::<P>;
        unsafe {
            ffi::gdk_clipboard_read_value_async(
                self.to_glib_none().0,
                type_.into_glib(),
                io_priority.into_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    pub fn read_value_future(
        &self,
        type_: glib::types::Type,
        io_priority: glib::Priority,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<glib::Value, glib::Error>> + 'static>>
    {
        Box_::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
            obj.read_value_async(type_, io_priority, Some(cancellable), move |res| {
                send.resolve(res);
            });
        }))
    }

    #[doc(alias = "gdk_clipboard_set_content")]
    pub fn set_content(
        &self,
        provider: Option<&impl IsA<ContentProvider>>,
    ) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gdk_clipboard_set_content(
                    self.to_glib_none().0,
                    provider.map(|p| p.as_ref()).to_glib_none().0
                ),
                "Can't set new clipboard content"
            )
        }
    }

    #[doc(alias = "gdk_clipboard_set_text")]
    pub fn set_text(&self, text: &str) {
        unsafe {
            ffi::gdk_clipboard_set_text(self.to_glib_none().0, text.to_glib_none().0);
        }
    }

    #[doc(alias = "gdk_clipboard_set_texture")]
    pub fn set_texture(&self, texture: &impl IsA<Texture>) {
        unsafe {
            ffi::gdk_clipboard_set_texture(
                self.to_glib_none().0,
                texture.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gdk_clipboard_store_async")]
    pub fn store_async<P: FnOnce(Result<(), glib::Error>) + 'static>(
        &self,
        io_priority: glib::Priority,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
        callback: P,
    ) {
        let main_context = glib::MainContext::ref_thread_default();
        let is_main_context_owner = main_context.is_owner();
        let has_acquired_main_context = (!is_main_context_owner)
            .then(|| main_context.acquire().ok())
            .flatten();
        assert!(
            is_main_context_owner || has_acquired_main_context.is_some(),
            "Async operations only allowed if the thread is owning the MainContext"
        );

        let user_data: Box_<glib::thread_guard::ThreadGuard<P>> =
            Box_::new(glib::thread_guard::ThreadGuard::new(callback));
        unsafe extern "C" fn store_async_trampoline<
            P: FnOnce(Result<(), glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut gio::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let _ = ffi::gdk_clipboard_store_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
                Box_::from_raw(user_data as *mut _);
            let callback: P = callback.into_inner();
            callback(result);
        }
        let callback = store_async_trampoline::<P>;
        unsafe {
            ffi::gdk_clipboard_store_async(
                self.to_glib_none().0,
                io_priority.into_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    pub fn store_future(
        &self,
        io_priority: glib::Priority,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {
        Box_::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
            obj.store_async(io_priority, Some(cancellable), move |res| {
                send.resolve(res);
            });
        }))
    }

    #[doc(alias = "changed")]
    pub fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn changed_trampoline<F: Fn(&Clipboard) + 'static>(
            this: *mut ffi::GdkClipboard,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    changed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "content")]
    pub fn connect_content_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_content_trampoline<F: Fn(&Clipboard) + 'static>(
            this: *mut ffi::GdkClipboard,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::content\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_content_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "formats")]
    pub fn connect_formats_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_formats_trampoline<F: Fn(&Clipboard) + 'static>(
            this: *mut ffi::GdkClipboard,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::formats\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_formats_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "local")]
    pub fn connect_local_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_local_trampoline<F: Fn(&Clipboard) + 'static>(
            this: *mut ffi::GdkClipboard,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::local\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_local_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Clipboard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Clipboard")
    }
}
