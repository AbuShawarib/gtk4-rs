// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::ShortcutTrigger;
use glib::{prelude::*, translate::*};
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GtkAlternativeTrigger")]
    pub struct AlternativeTrigger(Object<ffi::GtkAlternativeTrigger, ffi::GtkAlternativeTriggerClass>) @extends ShortcutTrigger;

    match fn {
        type_ => || ffi::gtk_alternative_trigger_get_type(),
    }
}

impl AlternativeTrigger {
    #[doc(alias = "gtk_alternative_trigger_new")]
    pub fn new(
        first: impl IsA<ShortcutTrigger>,
        second: impl IsA<ShortcutTrigger>,
    ) -> AlternativeTrigger {
        skip_assert_initialized!();
        unsafe {
            ShortcutTrigger::from_glib_full(ffi::gtk_alternative_trigger_new(
                first.upcast().into_glib_ptr(),
                second.upcast().into_glib_ptr(),
            ))
            .unsafe_cast()
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`AlternativeTrigger`] objects.
    ///
    /// This method returns an instance of [`AlternativeTriggerBuilder`](crate::builders::AlternativeTriggerBuilder) which can be used to create [`AlternativeTrigger`] objects.
    pub fn builder() -> AlternativeTriggerBuilder {
        AlternativeTriggerBuilder::new()
    }

    #[doc(alias = "gtk_alternative_trigger_get_first")]
    #[doc(alias = "get_first")]
    pub fn first(&self) -> ShortcutTrigger {
        unsafe {
            from_glib_none(ffi::gtk_alternative_trigger_get_first(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_alternative_trigger_get_second")]
    #[doc(alias = "get_second")]
    pub fn second(&self) -> ShortcutTrigger {
        unsafe {
            from_glib_none(ffi::gtk_alternative_trigger_get_second(
                self.to_glib_none().0,
            ))
        }
    }
}

impl Default for AlternativeTrigger {
    fn default() -> Self {
        glib::object::Object::new_default::<Self>()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`AlternativeTrigger`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct AlternativeTriggerBuilder {
    builder: glib::object::ObjectBuilder<'static, AlternativeTrigger>,
}

impl AlternativeTriggerBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn first(self, first: &impl IsA<ShortcutTrigger>) -> Self {
        Self {
            builder: self.builder.property("first", first.clone().upcast()),
        }
    }

    pub fn second(self, second: &impl IsA<ShortcutTrigger>) -> Self {
        Self {
            builder: self.builder.property("second", second.clone().upcast()),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`AlternativeTrigger`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> AlternativeTrigger {
        self.builder.build()
    }
}

impl fmt::Display for AlternativeTrigger {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("AlternativeTrigger")
    }
}
