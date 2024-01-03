// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{
    Accessible, AccessibleRole, Adjustment, Align, Buildable, ConstraintTarget, FlowBoxChild,
    LayoutManager, MovementStep, Orientable, Orientation, Overflow, SelectionMode, Widget,
};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GtkFlowBox")]
    pub struct FlowBox(Object<ffi::GtkFlowBox>) @extends Widget, @implements Accessible, Buildable, ConstraintTarget, Orientable;

    match fn {
        type_ => || ffi::gtk_flow_box_get_type(),
    }
}

impl FlowBox {
    #[doc(alias = "gtk_flow_box_new")]
    pub fn new() -> FlowBox {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(ffi::gtk_flow_box_new()).unsafe_cast() }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`FlowBox`] objects.
    ///
    /// This method returns an instance of [`FlowBoxBuilder`](crate::builders::FlowBoxBuilder) which can be used to create [`FlowBox`] objects.
    pub fn builder() -> FlowBoxBuilder {
        FlowBoxBuilder::new()
    }

    #[cfg(feature = "v4_6")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_6")))]
    #[doc(alias = "gtk_flow_box_append")]
    pub fn append(&self, child: &impl IsA<Widget>) {
        unsafe {
            ffi::gtk_flow_box_append(self.to_glib_none().0, child.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_flow_box_bind_model")]
    pub fn bind_model<P: Fn(&glib::Object) -> Widget + 'static>(
        &self,
        model: Option<&impl IsA<gio::ListModel>>,
        create_widget_func: P,
    ) {
        let create_widget_func_data: Box_<P> = Box_::new(create_widget_func);
        unsafe extern "C" fn create_widget_func_func<P: Fn(&glib::Object) -> Widget + 'static>(
            item: *mut glib::gobject_ffi::GObject,
            user_data: glib::ffi::gpointer,
        ) -> *mut ffi::GtkWidget {
            let item = from_glib_borrow(item);
            let callback = &*(user_data as *mut P);
            (*callback)(&item).to_glib_full()
        }
        let create_widget_func = Some(create_widget_func_func::<P> as _);
        unsafe extern "C" fn user_data_free_func_func<P: Fn(&glib::Object) -> Widget + 'static>(
            data: glib::ffi::gpointer,
        ) {
            let _callback = Box_::from_raw(data as *mut P);
        }
        let destroy_call4 = Some(user_data_free_func_func::<P> as _);
        let super_callback0: Box_<P> = create_widget_func_data;
        unsafe {
            ffi::gtk_flow_box_bind_model(
                self.to_glib_none().0,
                model.map(|p| p.as_ref()).to_glib_none().0,
                create_widget_func,
                Box_::into_raw(super_callback0) as *mut _,
                destroy_call4,
            );
        }
    }

    #[doc(alias = "gtk_flow_box_get_activate_on_single_click")]
    #[doc(alias = "get_activate_on_single_click")]
    pub fn activates_on_single_click(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_flow_box_get_activate_on_single_click(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_flow_box_get_child_at_index")]
    #[doc(alias = "get_child_at_index")]
    pub fn child_at_index(&self, idx: i32) -> Option<FlowBoxChild> {
        unsafe {
            from_glib_none(ffi::gtk_flow_box_get_child_at_index(
                self.to_glib_none().0,
                idx,
            ))
        }
    }

    #[doc(alias = "gtk_flow_box_get_child_at_pos")]
    #[doc(alias = "get_child_at_pos")]
    pub fn child_at_pos(&self, x: i32, y: i32) -> Option<FlowBoxChild> {
        unsafe {
            from_glib_none(ffi::gtk_flow_box_get_child_at_pos(
                self.to_glib_none().0,
                x,
                y,
            ))
        }
    }

    #[doc(alias = "gtk_flow_box_get_column_spacing")]
    #[doc(alias = "get_column_spacing")]
    pub fn column_spacing(&self) -> u32 {
        unsafe { ffi::gtk_flow_box_get_column_spacing(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_flow_box_get_homogeneous")]
    #[doc(alias = "get_homogeneous")]
    pub fn is_homogeneous(&self) -> bool {
        unsafe { from_glib(ffi::gtk_flow_box_get_homogeneous(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_flow_box_get_max_children_per_line")]
    #[doc(alias = "get_max_children_per_line")]
    pub fn max_children_per_line(&self) -> u32 {
        unsafe { ffi::gtk_flow_box_get_max_children_per_line(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_flow_box_get_min_children_per_line")]
    #[doc(alias = "get_min_children_per_line")]
    pub fn min_children_per_line(&self) -> u32 {
        unsafe { ffi::gtk_flow_box_get_min_children_per_line(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_flow_box_get_row_spacing")]
    #[doc(alias = "get_row_spacing")]
    pub fn row_spacing(&self) -> u32 {
        unsafe { ffi::gtk_flow_box_get_row_spacing(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_flow_box_get_selected_children")]
    #[doc(alias = "get_selected_children")]
    pub fn selected_children(&self) -> Vec<FlowBoxChild> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::gtk_flow_box_get_selected_children(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_flow_box_get_selection_mode")]
    #[doc(alias = "get_selection_mode")]
    pub fn selection_mode(&self) -> SelectionMode {
        unsafe { from_glib(ffi::gtk_flow_box_get_selection_mode(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_flow_box_insert")]
    pub fn insert(&self, widget: &impl IsA<Widget>, position: i32) {
        unsafe {
            ffi::gtk_flow_box_insert(
                self.to_glib_none().0,
                widget.as_ref().to_glib_none().0,
                position,
            );
        }
    }

    #[doc(alias = "gtk_flow_box_invalidate_filter")]
    pub fn invalidate_filter(&self) {
        unsafe {
            ffi::gtk_flow_box_invalidate_filter(self.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_flow_box_invalidate_sort")]
    pub fn invalidate_sort(&self) {
        unsafe {
            ffi::gtk_flow_box_invalidate_sort(self.to_glib_none().0);
        }
    }

    #[cfg(feature = "v4_6")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_6")))]
    #[doc(alias = "gtk_flow_box_prepend")]
    pub fn prepend(&self, child: &impl IsA<Widget>) {
        unsafe {
            ffi::gtk_flow_box_prepend(self.to_glib_none().0, child.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_flow_box_remove")]
    pub fn remove(&self, widget: &impl IsA<Widget>) {
        unsafe {
            ffi::gtk_flow_box_remove(self.to_glib_none().0, widget.as_ref().to_glib_none().0);
        }
    }

    #[cfg(feature = "v4_12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_12")))]
    #[doc(alias = "gtk_flow_box_remove_all")]
    pub fn remove_all(&self) {
        unsafe {
            ffi::gtk_flow_box_remove_all(self.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_flow_box_select_all")]
    pub fn select_all(&self) {
        unsafe {
            ffi::gtk_flow_box_select_all(self.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_flow_box_select_child")]
    pub fn select_child(&self, child: &impl IsA<FlowBoxChild>) {
        unsafe {
            ffi::gtk_flow_box_select_child(self.to_glib_none().0, child.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_flow_box_selected_foreach")]
    pub fn selected_foreach<P: FnMut(&FlowBox, &FlowBoxChild)>(&self, func: P) {
        let func_data: P = func;
        unsafe extern "C" fn func_func<P: FnMut(&FlowBox, &FlowBoxChild)>(
            box_: *mut ffi::GtkFlowBox,
            child: *mut ffi::GtkFlowBoxChild,
            user_data: glib::ffi::gpointer,
        ) {
            let box_ = from_glib_borrow(box_);
            let child = from_glib_borrow(child);
            let callback = user_data as *mut P;
            (*callback)(&box_, &child)
        }
        let func = Some(func_func::<P> as _);
        let super_callback0: &P = &func_data;
        unsafe {
            ffi::gtk_flow_box_selected_foreach(
                self.to_glib_none().0,
                func,
                super_callback0 as *const _ as *mut _,
            );
        }
    }

    #[doc(alias = "gtk_flow_box_set_activate_on_single_click")]
    pub fn set_activate_on_single_click(&self, single: bool) {
        unsafe {
            ffi::gtk_flow_box_set_activate_on_single_click(
                self.to_glib_none().0,
                single.into_glib(),
            );
        }
    }

    #[doc(alias = "gtk_flow_box_set_column_spacing")]
    pub fn set_column_spacing(&self, spacing: u32) {
        unsafe {
            ffi::gtk_flow_box_set_column_spacing(self.to_glib_none().0, spacing);
        }
    }

    #[doc(alias = "gtk_flow_box_set_filter_func")]
    pub fn set_filter_func<P: Fn(&FlowBoxChild) -> bool + 'static>(&self, filter_func: P) {
        let filter_func_data: Box_<P> = Box_::new(filter_func);
        unsafe extern "C" fn filter_func_func<P: Fn(&FlowBoxChild) -> bool + 'static>(
            child: *mut ffi::GtkFlowBoxChild,
            user_data: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let child = from_glib_borrow(child);
            let callback = &*(user_data as *mut P);
            (*callback)(&child).into_glib()
        }
        let filter_func = Some(filter_func_func::<P> as _);
        unsafe extern "C" fn destroy_func<P: Fn(&FlowBoxChild) -> bool + 'static>(
            data: glib::ffi::gpointer,
        ) {
            let _callback = Box_::from_raw(data as *mut P);
        }
        let destroy_call3 = Some(destroy_func::<P> as _);
        let super_callback0: Box_<P> = filter_func_data;
        unsafe {
            ffi::gtk_flow_box_set_filter_func(
                self.to_glib_none().0,
                filter_func,
                Box_::into_raw(super_callback0) as *mut _,
                destroy_call3,
            );
        }
    }

    #[doc(alias = "gtk_flow_box_set_hadjustment")]
    pub fn set_hadjustment(&self, adjustment: &impl IsA<Adjustment>) {
        unsafe {
            ffi::gtk_flow_box_set_hadjustment(
                self.to_glib_none().0,
                adjustment.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_flow_box_set_homogeneous")]
    pub fn set_homogeneous(&self, homogeneous: bool) {
        unsafe {
            ffi::gtk_flow_box_set_homogeneous(self.to_glib_none().0, homogeneous.into_glib());
        }
    }

    #[doc(alias = "gtk_flow_box_set_max_children_per_line")]
    pub fn set_max_children_per_line(&self, n_children: u32) {
        unsafe {
            ffi::gtk_flow_box_set_max_children_per_line(self.to_glib_none().0, n_children);
        }
    }

    #[doc(alias = "gtk_flow_box_set_min_children_per_line")]
    pub fn set_min_children_per_line(&self, n_children: u32) {
        unsafe {
            ffi::gtk_flow_box_set_min_children_per_line(self.to_glib_none().0, n_children);
        }
    }

    #[doc(alias = "gtk_flow_box_set_row_spacing")]
    pub fn set_row_spacing(&self, spacing: u32) {
        unsafe {
            ffi::gtk_flow_box_set_row_spacing(self.to_glib_none().0, spacing);
        }
    }

    #[doc(alias = "gtk_flow_box_set_selection_mode")]
    pub fn set_selection_mode(&self, mode: SelectionMode) {
        unsafe {
            ffi::gtk_flow_box_set_selection_mode(self.to_glib_none().0, mode.into_glib());
        }
    }

    #[doc(alias = "gtk_flow_box_set_vadjustment")]
    pub fn set_vadjustment(&self, adjustment: &impl IsA<Adjustment>) {
        unsafe {
            ffi::gtk_flow_box_set_vadjustment(
                self.to_glib_none().0,
                adjustment.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_flow_box_unselect_all")]
    pub fn unselect_all(&self) {
        unsafe {
            ffi::gtk_flow_box_unselect_all(self.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_flow_box_unselect_child")]
    pub fn unselect_child(&self, child: &impl IsA<FlowBoxChild>) {
        unsafe {
            ffi::gtk_flow_box_unselect_child(
                self.to_glib_none().0,
                child.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "accept-unpaired-release")]
    pub fn accepts_unpaired_release(&self) -> bool {
        ObjectExt::property(self, "accept-unpaired-release")
    }

    #[doc(alias = "accept-unpaired-release")]
    pub fn set_accept_unpaired_release(&self, accept_unpaired_release: bool) {
        ObjectExt::set_property(self, "accept-unpaired-release", accept_unpaired_release)
    }

    #[doc(alias = "activate-cursor-child")]
    pub fn connect_activate_cursor_child<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn activate_cursor_child_trampoline<F: Fn(&FlowBox) + 'static>(
            this: *mut ffi::GtkFlowBox,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"activate-cursor-child\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    activate_cursor_child_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_activate_cursor_child(&self) {
        self.emit_by_name::<()>("activate-cursor-child", &[]);
    }

    #[doc(alias = "child-activated")]
    pub fn connect_child_activated<F: Fn(&Self, &FlowBoxChild) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn child_activated_trampoline<
            F: Fn(&FlowBox, &FlowBoxChild) + 'static,
        >(
            this: *mut ffi::GtkFlowBox,
            child: *mut ffi::GtkFlowBoxChild,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(child))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"child-activated\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    child_activated_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "move-cursor")]
    pub fn connect_move_cursor<F: Fn(&Self, MovementStep, i32, bool, bool) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn move_cursor_trampoline<
            F: Fn(&FlowBox, MovementStep, i32, bool, bool) -> bool + 'static,
        >(
            this: *mut ffi::GtkFlowBox,
            step: ffi::GtkMovementStep,
            count: libc::c_int,
            extend: glib::ffi::gboolean,
            modify: glib::ffi::gboolean,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(
                &from_glib_borrow(this),
                from_glib(step),
                count,
                from_glib(extend),
                from_glib(modify),
            )
            .into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"move-cursor\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    move_cursor_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_move_cursor(
        &self,
        step: MovementStep,
        count: i32,
        extend: bool,
        modify: bool,
    ) -> bool {
        self.emit_by_name("move-cursor", &[&step, &count, &extend, &modify])
    }

    #[doc(alias = "select-all")]
    pub fn connect_select_all<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn select_all_trampoline<F: Fn(&FlowBox) + 'static>(
            this: *mut ffi::GtkFlowBox,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"select-all\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    select_all_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_select_all(&self) {
        self.emit_by_name::<()>("select-all", &[]);
    }

    #[doc(alias = "selected-children-changed")]
    pub fn connect_selected_children_changed<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn selected_children_changed_trampoline<F: Fn(&FlowBox) + 'static>(
            this: *mut ffi::GtkFlowBox,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"selected-children-changed\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    selected_children_changed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "toggle-cursor-child")]
    pub fn connect_toggle_cursor_child<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn toggle_cursor_child_trampoline<F: Fn(&FlowBox) + 'static>(
            this: *mut ffi::GtkFlowBox,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"toggle-cursor-child\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    toggle_cursor_child_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_toggle_cursor_child(&self) {
        self.emit_by_name::<()>("toggle-cursor-child", &[]);
    }

    #[doc(alias = "unselect-all")]
    pub fn connect_unselect_all<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn unselect_all_trampoline<F: Fn(&FlowBox) + 'static>(
            this: *mut ffi::GtkFlowBox,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"unselect-all\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    unselect_all_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_unselect_all(&self) {
        self.emit_by_name::<()>("unselect-all", &[]);
    }

    #[doc(alias = "accept-unpaired-release")]
    pub fn connect_accept_unpaired_release_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_accept_unpaired_release_trampoline<
            F: Fn(&FlowBox) + 'static,
        >(
            this: *mut ffi::GtkFlowBox,
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
                b"notify::accept-unpaired-release\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_accept_unpaired_release_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "activate-on-single-click")]
    pub fn connect_activate_on_single_click_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_activate_on_single_click_trampoline<
            F: Fn(&FlowBox) + 'static,
        >(
            this: *mut ffi::GtkFlowBox,
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
                b"notify::activate-on-single-click\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_activate_on_single_click_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "column-spacing")]
    pub fn connect_column_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_column_spacing_trampoline<F: Fn(&FlowBox) + 'static>(
            this: *mut ffi::GtkFlowBox,
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
                b"notify::column-spacing\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_column_spacing_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "homogeneous")]
    pub fn connect_homogeneous_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_homogeneous_trampoline<F: Fn(&FlowBox) + 'static>(
            this: *mut ffi::GtkFlowBox,
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
                b"notify::homogeneous\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_homogeneous_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "max-children-per-line")]
    pub fn connect_max_children_per_line_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_max_children_per_line_trampoline<F: Fn(&FlowBox) + 'static>(
            this: *mut ffi::GtkFlowBox,
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
                b"notify::max-children-per-line\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_max_children_per_line_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "min-children-per-line")]
    pub fn connect_min_children_per_line_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_min_children_per_line_trampoline<F: Fn(&FlowBox) + 'static>(
            this: *mut ffi::GtkFlowBox,
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
                b"notify::min-children-per-line\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_min_children_per_line_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "row-spacing")]
    pub fn connect_row_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_row_spacing_trampoline<F: Fn(&FlowBox) + 'static>(
            this: *mut ffi::GtkFlowBox,
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
                b"notify::row-spacing\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_row_spacing_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "selection-mode")]
    pub fn connect_selection_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_selection_mode_trampoline<F: Fn(&FlowBox) + 'static>(
            this: *mut ffi::GtkFlowBox,
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
                b"notify::selection-mode\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_selection_mode_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for FlowBox {
    fn default() -> Self {
        Self::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`FlowBox`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct FlowBoxBuilder {
    builder: glib::object::ObjectBuilder<'static, FlowBox>,
}

impl FlowBoxBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn accept_unpaired_release(self, accept_unpaired_release: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("accept-unpaired-release", accept_unpaired_release),
        }
    }

    pub fn activate_on_single_click(self, activate_on_single_click: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("activate-on-single-click", activate_on_single_click),
        }
    }

    pub fn column_spacing(self, column_spacing: u32) -> Self {
        Self {
            builder: self.builder.property("column-spacing", column_spacing),
        }
    }

    pub fn homogeneous(self, homogeneous: bool) -> Self {
        Self {
            builder: self.builder.property("homogeneous", homogeneous),
        }
    }

    pub fn max_children_per_line(self, max_children_per_line: u32) -> Self {
        Self {
            builder: self
                .builder
                .property("max-children-per-line", max_children_per_line),
        }
    }

    pub fn min_children_per_line(self, min_children_per_line: u32) -> Self {
        Self {
            builder: self
                .builder
                .property("min-children-per-line", min_children_per_line),
        }
    }

    pub fn row_spacing(self, row_spacing: u32) -> Self {
        Self {
            builder: self.builder.property("row-spacing", row_spacing),
        }
    }

    pub fn selection_mode(self, selection_mode: SelectionMode) -> Self {
        Self {
            builder: self.builder.property("selection-mode", selection_mode),
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

    pub fn orientation(self, orientation: Orientation) -> Self {
        Self {
            builder: self.builder.property("orientation", orientation),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`FlowBox`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> FlowBox {
        self.builder.build()
    }
}
