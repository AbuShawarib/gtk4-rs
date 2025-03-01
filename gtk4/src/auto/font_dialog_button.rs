// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{
    Accessible, AccessibleRole, Align, Buildable, ConstraintTarget, FontDialog, FontLevel,
    LayoutManager, Overflow, Widget,
};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute};

glib::wrapper! {
    #[doc(alias = "GtkFontDialogButton")]
    pub struct FontDialogButton(Object<ffi::GtkFontDialogButton, ffi::GtkFontDialogButtonClass>) @extends Widget, @implements Accessible, Buildable, ConstraintTarget;

    match fn {
        type_ => || ffi::gtk_font_dialog_button_get_type(),
    }
}

impl FontDialogButton {
    #[doc(alias = "gtk_font_dialog_button_new")]
    pub fn new(dialog: Option<FontDialog>) -> FontDialogButton {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_font_dialog_button_new(dialog.into_glib_ptr()))
                .unsafe_cast()
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`FontDialogButton`] objects.
    ///
    /// This method returns an instance of [`FontDialogButtonBuilder`](crate::builders::FontDialogButtonBuilder) which can be used to create [`FontDialogButton`] objects.
    pub fn builder() -> FontDialogButtonBuilder {
        FontDialogButtonBuilder::new()
    }

    #[doc(alias = "gtk_font_dialog_button_get_dialog")]
    #[doc(alias = "get_dialog")]
    pub fn dialog(&self) -> Option<FontDialog> {
        unsafe {
            from_glib_none(ffi::gtk_font_dialog_button_get_dialog(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_font_dialog_button_get_font_desc")]
    #[doc(alias = "get_font_desc")]
    pub fn font_desc(&self) -> Option<pango::FontDescription> {
        unsafe {
            from_glib_none(ffi::gtk_font_dialog_button_get_font_desc(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_font_dialog_button_get_font_features")]
    #[doc(alias = "get_font_features")]
    pub fn font_features(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gtk_font_dialog_button_get_font_features(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_font_dialog_button_get_language")]
    #[doc(alias = "get_language")]
    pub fn language(&self) -> Option<pango::Language> {
        unsafe {
            from_glib_full(ffi::gtk_font_dialog_button_get_language(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_font_dialog_button_get_level")]
    #[doc(alias = "get_level")]
    pub fn level(&self) -> FontLevel {
        unsafe { from_glib(ffi::gtk_font_dialog_button_get_level(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_font_dialog_button_get_use_font")]
    #[doc(alias = "get_use_font")]
    pub fn uses_font(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_font_dialog_button_get_use_font(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_font_dialog_button_get_use_size")]
    #[doc(alias = "get_use_size")]
    pub fn uses_size(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_font_dialog_button_get_use_size(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_font_dialog_button_set_dialog")]
    pub fn set_dialog(&self, dialog: &FontDialog) {
        unsafe {
            ffi::gtk_font_dialog_button_set_dialog(self.to_glib_none().0, dialog.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_font_dialog_button_set_font_desc")]
    pub fn set_font_desc(&self, font_desc: &pango::FontDescription) {
        unsafe {
            ffi::gtk_font_dialog_button_set_font_desc(
                self.to_glib_none().0,
                font_desc.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_font_dialog_button_set_font_features")]
    pub fn set_font_features(&self, font_features: Option<&str>) {
        unsafe {
            ffi::gtk_font_dialog_button_set_font_features(
                self.to_glib_none().0,
                font_features.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_font_dialog_button_set_language")]
    pub fn set_language(&self, language: Option<&pango::Language>) {
        unsafe {
            ffi::gtk_font_dialog_button_set_language(
                self.to_glib_none().0,
                mut_override(language.to_glib_none().0),
            );
        }
    }

    #[doc(alias = "gtk_font_dialog_button_set_level")]
    pub fn set_level(&self, level: FontLevel) {
        unsafe {
            ffi::gtk_font_dialog_button_set_level(self.to_glib_none().0, level.into_glib());
        }
    }

    #[doc(alias = "gtk_font_dialog_button_set_use_font")]
    pub fn set_use_font(&self, use_font: bool) {
        unsafe {
            ffi::gtk_font_dialog_button_set_use_font(self.to_glib_none().0, use_font.into_glib());
        }
    }

    #[doc(alias = "gtk_font_dialog_button_set_use_size")]
    pub fn set_use_size(&self, use_size: bool) {
        unsafe {
            ffi::gtk_font_dialog_button_set_use_size(self.to_glib_none().0, use_size.into_glib());
        }
    }

    pub fn get_property_level(&self) -> FontLevel {
        ObjectExt::property(self, "level")
    }

    pub fn set_property_level(&self, level: FontLevel) {
        ObjectExt::set_property(self, "level", level)
    }

    #[doc(alias = "use-font")]
    pub fn get_property_use_font(&self) -> bool {
        ObjectExt::property(self, "use-font")
    }

    #[doc(alias = "use-font")]
    pub fn set_property_use_font(&self, use_font: bool) {
        ObjectExt::set_property(self, "use-font", use_font)
    }

    #[doc(alias = "use-size")]
    pub fn get_property_use_size(&self) -> bool {
        ObjectExt::property(self, "use-size")
    }

    #[doc(alias = "use-size")]
    pub fn set_property_use_size(&self, use_size: bool) {
        ObjectExt::set_property(self, "use-size", use_size)
    }

    #[cfg(feature = "v4_10")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_10")))]
    #[doc(alias = "dialog")]
    pub fn connect_dialog_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_dialog_trampoline<F: Fn(&FontDialogButton) + 'static>(
            this: *mut ffi::GtkFontDialogButton,
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
                b"notify::dialog\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_dialog_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v4_10")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_10")))]
    #[doc(alias = "font-desc")]
    pub fn connect_font_desc_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_font_desc_trampoline<F: Fn(&FontDialogButton) + 'static>(
            this: *mut ffi::GtkFontDialogButton,
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
                b"notify::font-desc\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_font_desc_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v4_10")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_10")))]
    #[doc(alias = "font-features")]
    pub fn connect_font_features_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_font_features_trampoline<F: Fn(&FontDialogButton) + 'static>(
            this: *mut ffi::GtkFontDialogButton,
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
                b"notify::font-features\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_font_features_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v4_10")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_10")))]
    #[doc(alias = "language")]
    pub fn connect_language_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_language_trampoline<F: Fn(&FontDialogButton) + 'static>(
            this: *mut ffi::GtkFontDialogButton,
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
                b"notify::language\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_language_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "level")]
    pub fn connect_level_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_level_trampoline<F: Fn(&FontDialogButton) + 'static>(
            this: *mut ffi::GtkFontDialogButton,
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
                b"notify::level\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_level_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "use-font")]
    pub fn connect_use_font_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_use_font_trampoline<F: Fn(&FontDialogButton) + 'static>(
            this: *mut ffi::GtkFontDialogButton,
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
                b"notify::use-font\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_use_font_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "use-size")]
    pub fn connect_use_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_use_size_trampoline<F: Fn(&FontDialogButton) + 'static>(
            this: *mut ffi::GtkFontDialogButton,
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
                b"notify::use-size\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_use_size_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

#[cfg(feature = "v4_10")]
#[cfg_attr(docsrs, doc(cfg(feature = "v4_10")))]
impl Default for FontDialogButton {
    fn default() -> Self {
        glib::object::Object::new::<Self>()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`FontDialogButton`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct FontDialogButtonBuilder {
    builder: glib::object::ObjectBuilder<'static, FontDialogButton>,
}

impl FontDialogButtonBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    #[cfg(feature = "v4_10")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_10")))]
    pub fn dialog(self, dialog: &FontDialog) -> Self {
        Self {
            builder: self.builder.property("dialog", dialog.clone()),
        }
    }

    #[cfg(feature = "v4_10")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_10")))]
    pub fn font_desc(self, font_desc: &pango::FontDescription) -> Self {
        Self {
            builder: self.builder.property("font-desc", font_desc),
        }
    }

    #[cfg(feature = "v4_10")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_10")))]
    pub fn font_features(self, font_features: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("font-features", font_features.into()),
        }
    }

    #[cfg(feature = "v4_10")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_10")))]
    pub fn language(self, language: &pango::Language) -> Self {
        Self {
            builder: self.builder.property("language", language),
        }
    }

    pub fn level(self, level: FontLevel) -> Self {
        Self {
            builder: self.builder.property("level", level),
        }
    }

    pub fn use_font(self, use_font: bool) -> Self {
        Self {
            builder: self.builder.property("use-font", use_font),
        }
    }

    pub fn use_size(self, use_size: bool) -> Self {
        Self {
            builder: self.builder.property("use-size", use_size),
        }
    }

    pub fn can_focus(self, can_focus: bool) -> Self {
        Self {
            builder: self.builder.property("can-focus", can_focus),
        }
    }

    pub fn can_target(self, can_target: bool) -> Self {
        Self {
            builder: self.builder.property("can-target", can_target),
        }
    }

    pub fn css_classes(self, css_classes: impl Into<glib::StrV>) -> Self {
        Self {
            builder: self.builder.property("css-classes", css_classes.into()),
        }
    }

    pub fn css_name(self, css_name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("css-name", css_name.into()),
        }
    }

    pub fn cursor(self, cursor: &gdk::Cursor) -> Self {
        Self {
            builder: self.builder.property("cursor", cursor.clone()),
        }
    }

    pub fn focus_on_click(self, focus_on_click: bool) -> Self {
        Self {
            builder: self.builder.property("focus-on-click", focus_on_click),
        }
    }

    pub fn focusable(self, focusable: bool) -> Self {
        Self {
            builder: self.builder.property("focusable", focusable),
        }
    }

    pub fn halign(self, halign: Align) -> Self {
        Self {
            builder: self.builder.property("halign", halign),
        }
    }

    pub fn has_tooltip(self, has_tooltip: bool) -> Self {
        Self {
            builder: self.builder.property("has-tooltip", has_tooltip),
        }
    }

    pub fn height_request(self, height_request: i32) -> Self {
        Self {
            builder: self.builder.property("height-request", height_request),
        }
    }

    pub fn hexpand(self, hexpand: bool) -> Self {
        Self {
            builder: self.builder.property("hexpand", hexpand),
        }
    }

    pub fn hexpand_set(self, hexpand_set: bool) -> Self {
        Self {
            builder: self.builder.property("hexpand-set", hexpand_set),
        }
    }

    pub fn layout_manager(self, layout_manager: &impl IsA<LayoutManager>) -> Self {
        Self {
            builder: self
                .builder
                .property("layout-manager", layout_manager.clone().upcast()),
        }
    }

    pub fn margin_bottom(self, margin_bottom: i32) -> Self {
        Self {
            builder: self.builder.property("margin-bottom", margin_bottom),
        }
    }

    pub fn margin_end(self, margin_end: i32) -> Self {
        Self {
            builder: self.builder.property("margin-end", margin_end),
        }
    }

    pub fn margin_start(self, margin_start: i32) -> Self {
        Self {
            builder: self.builder.property("margin-start", margin_start),
        }
    }

    pub fn margin_top(self, margin_top: i32) -> Self {
        Self {
            builder: self.builder.property("margin-top", margin_top),
        }
    }

    pub fn name(self, name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("name", name.into()),
        }
    }

    pub fn opacity(self, opacity: f64) -> Self {
        Self {
            builder: self.builder.property("opacity", opacity),
        }
    }

    pub fn overflow(self, overflow: Overflow) -> Self {
        Self {
            builder: self.builder.property("overflow", overflow),
        }
    }

    pub fn receives_default(self, receives_default: bool) -> Self {
        Self {
            builder: self.builder.property("receives-default", receives_default),
        }
    }

    pub fn sensitive(self, sensitive: bool) -> Self {
        Self {
            builder: self.builder.property("sensitive", sensitive),
        }
    }

    pub fn tooltip_markup(self, tooltip_markup: impl Into<glib::GString>) -> Self {
        Self {
            builder: self
                .builder
                .property("tooltip-markup", tooltip_markup.into()),
        }
    }

    pub fn tooltip_text(self, tooltip_text: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("tooltip-text", tooltip_text.into()),
        }
    }

    pub fn valign(self, valign: Align) -> Self {
        Self {
            builder: self.builder.property("valign", valign),
        }
    }

    pub fn vexpand(self, vexpand: bool) -> Self {
        Self {
            builder: self.builder.property("vexpand", vexpand),
        }
    }

    pub fn vexpand_set(self, vexpand_set: bool) -> Self {
        Self {
            builder: self.builder.property("vexpand-set", vexpand_set),
        }
    }

    pub fn visible(self, visible: bool) -> Self {
        Self {
            builder: self.builder.property("visible", visible),
        }
    }

    pub fn width_request(self, width_request: i32) -> Self {
        Self {
            builder: self.builder.property("width-request", width_request),
        }
    }

    pub fn accessible_role(self, accessible_role: AccessibleRole) -> Self {
        Self {
            builder: self.builder.property("accessible-role", accessible_role),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`FontDialogButton`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> FontDialogButton {
        self.builder.build()
    }
}

impl fmt::Display for FontDialogButton {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("FontDialogButton")
    }
}
