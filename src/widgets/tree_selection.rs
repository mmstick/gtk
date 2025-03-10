// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! GtkTreeSelection — The selection object for GtkTreeView

use std::ptr;
use glib;
use glib::translate::{ToGlibPtr, ToGlibPtrMut, Uninitialized, from_glib, some_if};
use ffi;
use {TreeView, TreePath, TreeIter, TreeModel};

pub struct TreeSelection {
    pointer: *mut ffi::GtkTreeSelection
}

impl TreeSelection {
    pub fn set_mode(&self, mode: ::SelectionMode) {
        unsafe { ffi::gtk_tree_selection_set_mode(self.pointer, mode) }
    }

    pub fn get_mode(&self) -> ::SelectionMode{
        unsafe { ffi::gtk_tree_selection_get_mode(self.pointer) }
    }

    pub fn get_user_data<'r, T>(&self) -> &'r mut T {
        unsafe { ::std::mem::transmute(ffi::gtk_tree_selection_get_user_data(self.pointer)) }
    }

    pub fn get_tree_view(&self) -> Option<TreeView> {
        let tmp_pointer = unsafe { ffi::gtk_tree_selection_get_tree_view(self.pointer) } as *mut ffi::GtkWidget;

        if tmp_pointer.is_null() {
            None
        } else {
            Some(::FFIWidget::wrap_widget(tmp_pointer))
        }
    }

    pub fn get_selected(&self) -> Option<(TreeModel, TreeIter)> {
        unsafe {
            let mut model = ptr::null_mut();
            let mut iter = TreeIter::uninitialized();
            let ok = ffi::gtk_tree_selection_get_selected(self.pointer, &mut model,
                iter.to_glib_none_mut().0);
            some_if(ok, (TreeModel::wrap_pointer(model), iter))
        }
    }

    pub fn count_selected_rows(&self) -> i32 {
        unsafe { ffi::gtk_tree_selection_count_selected_rows(self.pointer) }
    }

    pub fn select_path(&self, path: &TreePath) {
        unsafe { ffi::gtk_tree_selection_select_path(self.pointer, path.unwrap_pointer()) }
    }

    pub fn unselect_path(&self, path: &TreePath) {
        unsafe { ffi::gtk_tree_selection_unselect_path(self.pointer, path.unwrap_pointer()) }
    }

    pub fn path_is_selected(&self, path: &TreePath) -> bool {
        match unsafe { ffi::gtk_tree_selection_path_is_selected(self.pointer, path.unwrap_pointer()) } {
            0 => false,
            _ => true
        }
    }

    pub fn select_iter(&self, iter: &TreeIter) {
        unsafe {
            ffi::gtk_tree_selection_select_iter(self.pointer, iter.to_glib_none().0 as *mut _)
        }
    }

    pub fn unselect_iter(&self, iter: &TreeIter) {
        unsafe {
            ffi::gtk_tree_selection_unselect_iter(self.pointer, iter.to_glib_none().0 as *mut _)
        }
    }

    pub fn iter_is_selected(&self, iter: &TreeIter) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_selection_iter_is_selected(self.pointer,
                                                               iter.to_glib_none().0 as *mut _))
        }
    }

    pub fn select_all(&self) {
        unsafe { ffi::gtk_tree_selection_select_all(self.pointer) }
    }

    pub fn unselect_all(&self) {
        unsafe { ffi::gtk_tree_selection_unselect_all(self.pointer) }
    }

    pub fn select_range(&self, start_path: &TreePath, end_path: &TreePath) {
        unsafe { ffi::gtk_tree_selection_select_range(self.pointer, start_path.unwrap_pointer(),
            end_path.unwrap_pointer()) }
    }

    pub fn unselect_range(&self, start_path: &TreePath, end_path: &TreePath) {
        unsafe { ffi::gtk_tree_selection_unselect_range(self.pointer, start_path.unwrap_pointer(),
            end_path.unwrap_pointer()) }
    }

    pub unsafe fn wrap(pointer: *mut ffi::GtkTreeSelection) -> Option<TreeSelection> {
        if pointer.is_null() {
            None
        } else {
            ::gobject_ffi::g_object_ref(pointer as *mut _);
            Some(TreeSelection { pointer: pointer })
        }
    }
}

impl glib::traits::FFIGObject for TreeSelection {
    fn unwrap_gobject(&self) -> *mut ::gobject_ffi::GObject {
        ::cast::G_OBJECT_FROM_TREE_SELECTION(self.pointer)
    }

    unsafe fn wrap_object(object: *mut ::gobject_ffi::GObject) -> TreeSelection {
        ::gobject_ffi::g_object_ref(object as *mut _);
        TreeSelection { pointer: object as *mut ffi::GtkTreeSelection }
    }
}

impl_drop!(TreeSelection, GTK_TREE_SELECTION);
