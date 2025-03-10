// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! A container that allows reflowing its children

use ffi;
use cast::{GTK_LIST_BOX_ROW, GTK_LIST_BOX};
use FFIWidget;
use glib::{to_bool, to_gboolean};

/// GtkFlowBox — A container that allows reflowing its children
struct_Widget!(ListBox);

impl ListBox {
    pub fn new() -> Option<ListBox> {
        assert_initialized_main_thread!();
        let tmp_pointer = unsafe { ffi::gtk_list_box_new() };
        check_pointer!(tmp_pointer, ListBox)
    }

    pub fn prepend<T: ::WidgetTrait>(&self, child: &T) {
        unsafe {
            ffi::gtk_list_box_prepend(GTK_LIST_BOX(self.pointer),
                                      child.unwrap_widget())
        }
    }

    pub fn insert<T: ::WidgetTrait>(&self, child: &T, position: i32) {
        unsafe {
            ffi::gtk_list_box_insert(GTK_LIST_BOX(self.pointer),
                                     child.unwrap_widget(),
                                     position)
        }
    }

    pub fn get_selected_row(&self) -> Option<ListBoxRow> {
        let tmp_pointer = unsafe {
            ffi::gtk_list_box_get_selected_row(GTK_LIST_BOX(self.pointer))
        };
        if tmp_pointer.is_null() {
            None
        } else {
            Some(::FFIWidget::wrap_widget(tmp_pointer as *mut ffi::GtkWidget))
        }
    }

    pub fn get_row_at_index(&self, index: i32) -> Option<ListBoxRow> {
        let tmp_pointer = unsafe {
            ffi::gtk_list_box_get_row_at_index(GTK_LIST_BOX(self.pointer), index)
        };
        if tmp_pointer.is_null() {
            None
        } else {
            Some(::FFIWidget::wrap_widget(tmp_pointer as *mut ffi::GtkWidget))
        }
    }

    pub fn get_row_at_iy(&self, y: i32) -> Option<ListBoxRow> {
        let tmp_pointer = unsafe {
            ffi::gtk_list_box_get_row_at_y(GTK_LIST_BOX(self.pointer), y)
        };
        if tmp_pointer.is_null() {
            None
        } else {
            Some(::FFIWidget::wrap_widget(tmp_pointer as *mut ffi::GtkWidget))
        }
    }

    pub fn select_row(&self, row: &ListBoxRow) {
        unsafe {
            ffi::gtk_list_box_select_row(GTK_LIST_BOX(self.pointer),
                                         GTK_LIST_BOX_ROW(row.unwrap_widget()))
        }
    }

    pub fn set_placeholder<T: ::WidgetTrait>(&self, placeholder: &T) {
        unsafe {
            ffi::gtk_list_box_set_placeholder(GTK_LIST_BOX(self.pointer),
                                              placeholder.unwrap_widget())
        }
    }

    pub fn set_adjustment(&self, adjustment: &::Adjustment) {
        unsafe {
            ffi::gtk_list_box_set_adjustment(GTK_LIST_BOX(self.pointer),
                                             adjustment.unwrap_pointer())
        }
    }

    pub fn get_adjustment(&self) -> Option<::Adjustment> {
        unsafe {
            let ptr = ffi::gtk_list_box_get_adjustment(GTK_LIST_BOX(self.pointer));
            if ptr.is_null() {
                None
            } else {
                Some(::Adjustment::wrap_pointer(ptr))
            }
        }
    }

    pub fn set_selection_mode(&self, mode: ::SelectionMode) {
        unsafe {
            ffi::gtk_list_box_set_selection_mode(GTK_LIST_BOX(self.pointer), mode)
        }
    }

    pub fn get_selection_mode(&self) -> ::SelectionMode {
        unsafe {
            ffi::gtk_list_box_get_selection_mode(GTK_LIST_BOX(self.pointer))
        }
    }

    pub fn invalidate_header(&self) {
        unsafe {
            ffi::gtk_list_box_invalidate_headers(GTK_LIST_BOX(self.pointer))
        }
    }

    pub fn set_activate_on_single_click(&self, single: bool) {
        unsafe {
            ffi::gtk_list_box_set_activate_on_single_click(GTK_LIST_BOX(self.pointer),
                                                           to_gboolean(single))
        }
    }

    pub fn is_activate_on_single_click(&self) -> bool {
        unsafe {
            to_bool(ffi::gtk_list_box_get_activate_on_single_click(GTK_LIST_BOX(self.pointer)))
        }
    }

    pub fn drag_unhighlight_row(&self) {
        unsafe {
            ffi::gtk_list_box_drag_unhighlight_row(GTK_LIST_BOX(self.pointer))
        }
    }

    pub fn drag_highlight_row(&self, row: &ListBoxRow) {
        unsafe {
            ffi::gtk_list_box_drag_highlight_row(GTK_LIST_BOX(self.pointer),
                                                 row.unwrap_widget() as *mut ffi::GtkListBoxRow)
        }
    }
}


// pub fn gtk_list_box_drag_unhighlight_row         (list_box: *GtkListBox);
// pub fn gtk_list_box_drag_highlight_row           (list_box: *GtkListBox, row: *GtkListBoxRow);

impl_drop!(ListBox);
impl_TraitWidget!(ListBox);

impl ::ContainerTrait for ListBox {}

struct_Widget!(ListBoxRow);

impl ListBoxRow {
    pub fn new() -> Option<ListBoxRow> {
        assert_initialized_main_thread!();
        let tmp_pointer = unsafe { ffi::gtk_list_box_row_new() };
        check_pointer!(tmp_pointer, ListBoxRow)
    }

    pub fn changed(&self) {
        unsafe {
            ffi::gtk_list_box_row_changed(GTK_LIST_BOX_ROW(self.pointer))
        }
    }

    pub fn get_header<T: ::WidgetTrait>(&self) -> Option<T> {
        let tmp_pointer = unsafe {
            ffi::gtk_list_box_row_get_header(GTK_LIST_BOX_ROW(self.pointer))
        };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(::FFIWidget::wrap_widget(tmp_pointer))
        }
    }

    pub fn set_header<T: ::WidgetTrait>(&self, header: &T) {
        unsafe {
            ffi::gtk_list_box_row_set_header(GTK_LIST_BOX_ROW(self.pointer),
                                             header.unwrap_widget())
        }
    }

    pub fn get_index(&self) -> i32 {
        unsafe {
            ffi::gtk_list_box_row_get_index(GTK_LIST_BOX_ROW(self.pointer))
        }
    }
}

impl_drop!(ListBoxRow);
impl_TraitWidget!(ListBoxRow);

impl ::ContainerTrait for ListBoxRow {}
impl ::BinTrait for ListBoxRow {}
