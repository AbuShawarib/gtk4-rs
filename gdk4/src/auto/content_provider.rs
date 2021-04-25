// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::ContentFormats;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use std::pin::Pin;
use std::ptr;

glib::wrapper! {
    pub struct ContentProvider(Object<ffi::GdkContentProvider, ffi::GdkContentProviderClass>);

    match fn {
        type_ => || ffi::gdk_content_provider_get_type(),
    }
}

impl ContentProvider {
    #[doc(alias = "gdk_content_provider_new_for_bytes")]
    pub fn for_bytes(mime_type: &str, bytes: &glib::Bytes) -> ContentProvider {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gdk_content_provider_new_for_bytes(
                mime_type.to_glib_none().0,
                bytes.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_content_provider_new_for_value")]
    pub fn for_value(value: &glib::Value) -> ContentProvider {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gdk_content_provider_new_for_value(
                value.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_content_provider_new_union")]
    pub fn new_union(providers: &[ContentProvider]) -> ContentProvider {
        assert_initialized_main_thread!();
        let n_providers = providers.len() as usize;
        unsafe {
            from_glib_full(ffi::gdk_content_provider_new_union(
                providers.to_glib_full(),
                n_providers,
            ))
        }
    }
}

pub const NONE_CONTENT_PROVIDER: Option<&ContentProvider> = None;

pub trait ContentProviderExt: 'static {
    #[doc(alias = "gdk_content_provider_content_changed")]
    fn content_changed(&self);

    #[doc(alias = "gdk_content_provider_get_value")]
    fn value(&self, value: &mut glib::Value) -> Result<(), glib::Error>;

    #[doc(alias = "gdk_content_provider_ref_formats")]
    fn ref_formats(&self) -> Option<ContentFormats>;

    #[doc(alias = "gdk_content_provider_ref_storable_formats")]
    fn ref_storable_formats(&self) -> Option<ContentFormats>;

    #[doc(alias = "gdk_content_provider_write_mime_type_async")]
    fn write_mime_type_async<
        P: IsA<gio::OutputStream>,
        Q: IsA<gio::Cancellable>,
        R: FnOnce(Result<(), glib::Error>) + Send + 'static,
    >(
        &self,
        mime_type: &str,
        stream: &P,
        io_priority: glib::Priority,
        cancellable: Option<&Q>,
        callback: R,
    );

    fn write_mime_type_async_future<P: IsA<gio::OutputStream> + Clone + 'static>(
        &self,
        mime_type: &str,
        stream: &P,
        io_priority: glib::Priority,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>>;

    #[doc(alias = "get_property_formats")]
    fn formats(&self) -> Option<ContentFormats>;

    #[doc(alias = "get_property_storable_formats")]
    fn storable_formats(&self) -> Option<ContentFormats>;

    fn connect_content_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_formats_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_storable_formats_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<ContentProvider>> ContentProviderExt for O {
    fn content_changed(&self) {
        unsafe {
            ffi::gdk_content_provider_content_changed(self.as_ref().to_glib_none().0);
        }
    }

    fn value(&self, value: &mut glib::Value) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::gdk_content_provider_get_value(
                self.as_ref().to_glib_none().0,
                value.to_glib_none_mut().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn ref_formats(&self) -> Option<ContentFormats> {
        unsafe {
            from_glib_full(ffi::gdk_content_provider_ref_formats(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn ref_storable_formats(&self) -> Option<ContentFormats> {
        unsafe {
            from_glib_full(ffi::gdk_content_provider_ref_storable_formats(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn write_mime_type_async<
        P: IsA<gio::OutputStream>,
        Q: IsA<gio::Cancellable>,
        R: FnOnce(Result<(), glib::Error>) + Send + 'static,
    >(
        &self,
        mime_type: &str,
        stream: &P,
        io_priority: glib::Priority,
        cancellable: Option<&Q>,
        callback: R,
    ) {
        let user_data: Box_<R> = Box_::new(callback);
        unsafe extern "C" fn write_mime_type_async_trampoline<
            R: FnOnce(Result<(), glib::Error>) + Send + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut gio::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let _ = ffi::gdk_content_provider_write_mime_type_finish(
                _source_object as *mut _,
                res,
                &mut error,
            );
            let result = if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<R> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = write_mime_type_async_trampoline::<R>;
        unsafe {
            ffi::gdk_content_provider_write_mime_type_async(
                self.as_ref().to_glib_none().0,
                mime_type.to_glib_none().0,
                stream.as_ref().to_glib_none().0,
                io_priority.to_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn write_mime_type_async_future<P: IsA<gio::OutputStream> + Clone + 'static>(
        &self,
        mime_type: &str,
        stream: &P,
        io_priority: glib::Priority,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {
        let mime_type = String::from(mime_type);
        let stream = stream.clone();
        Box_::pin(gio::GioFuture::new(self, move |obj, send| {
            let cancellable = gio::Cancellable::new();
            obj.write_mime_type_async(
                &mime_type,
                &stream,
                io_priority,
                Some(&cancellable),
                move |res| {
                    send.resolve(res);
                },
            );

            cancellable
        }))
    }

    fn formats(&self) -> Option<ContentFormats> {
        unsafe {
            let mut value = glib::Value::from_type(<ContentFormats as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"formats\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `formats` getter")
        }
    }

    fn storable_formats(&self) -> Option<ContentFormats> {
        unsafe {
            let mut value = glib::Value::from_type(<ContentFormats as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"storable-formats\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `storable-formats` getter")
        }
    }

    fn connect_content_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn content_changed_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GdkContentProvider,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<ContentProvider>,
        {
            let f: &F = &*(f as *const F);
            f(&ContentProvider::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"content-changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    content_changed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_formats_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_formats_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GdkContentProvider,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<ContentProvider>,
        {
            let f: &F = &*(f as *const F);
            f(&ContentProvider::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::formats\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_formats_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_storable_formats_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_storable_formats_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GdkContentProvider,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<ContentProvider>,
        {
            let f: &F = &*(f as *const F);
            f(&ContentProvider::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::storable-formats\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_storable_formats_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for ContentProvider {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ContentProvider")
    }
}
