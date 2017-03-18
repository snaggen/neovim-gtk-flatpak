// This file was generated by gir (c3b4020) from gir-files (71d73f0)
// DO NOT EDIT

use TreeIter;
use TreeModel;
use TreeSortable;
use ffi;
use glib::translate::*;

glib_wrapper! {
    pub struct ListStore(Object<ffi::GtkListStore>): TreeModel, TreeSortable;

    match fn {
        get_type => || ffi::gtk_list_store_get_type(),
    }
}

impl ListStore {
    //pub fn new(n_columns: i32, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> ListStore {
    //    unsafe { TODO: call ffi::gtk_list_store_new() }
    //}

    //pub fn newv(n_columns: i32, types: /*Unimplemented*/&CArray TypeId { ns_id: 0, id: 30 }) -> ListStore {
    //    unsafe { TODO: call ffi::gtk_list_store_newv() }
    //}

    pub fn append(&self) -> TreeIter {
        unsafe {
            let mut iter = TreeIter::uninitialized();
            ffi::gtk_list_store_append(self.to_glib_none().0, iter.to_glib_none_mut().0);
            iter
        }
    }

    pub fn clear(&self) {
        unsafe {
            ffi::gtk_list_store_clear(self.to_glib_none().0);
        }
    }

    pub fn insert(&self, position: i32) -> TreeIter {
        unsafe {
            let mut iter = TreeIter::uninitialized();
            ffi::gtk_list_store_insert(self.to_glib_none().0, iter.to_glib_none_mut().0, position);
            iter
        }
    }

    pub fn insert_after(&self, sibling: Option<&TreeIter>) -> TreeIter {
        unsafe {
            let mut iter = TreeIter::uninitialized();
            ffi::gtk_list_store_insert_after(self.to_glib_none().0, iter.to_glib_none_mut().0, mut_override(sibling.to_glib_none().0));
            iter
        }
    }

    pub fn insert_before(&self, sibling: Option<&TreeIter>) -> TreeIter {
        unsafe {
            let mut iter = TreeIter::uninitialized();
            ffi::gtk_list_store_insert_before(self.to_glib_none().0, iter.to_glib_none_mut().0, mut_override(sibling.to_glib_none().0));
            iter
        }
    }

    //pub fn insert_with_values(&self, position: i32, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> TreeIter {
    //    unsafe { TODO: call ffi::gtk_list_store_insert_with_values() }
    //}

    //pub fn insert_with_valuesv(&self, position: i32, columns: /*Unimplemented*/&CArray TypeId { ns_id: 0, id: 14 }, values: &[&glib::Value], n_values: i32) -> TreeIter {
    //    unsafe { TODO: call ffi::gtk_list_store_insert_with_valuesv() }
    //}

    pub fn iter_is_valid(&self, iter: &TreeIter) -> bool {
        unsafe {
            from_glib(ffi::gtk_list_store_iter_is_valid(self.to_glib_none().0, mut_override(iter.to_glib_none().0)))
        }
    }

    pub fn move_after(&self, iter: &TreeIter, position: Option<&TreeIter>) {
        unsafe {
            ffi::gtk_list_store_move_after(self.to_glib_none().0, mut_override(iter.to_glib_none().0), mut_override(position.to_glib_none().0));
        }
    }

    pub fn move_before(&self, iter: &TreeIter, position: Option<&TreeIter>) {
        unsafe {
            ffi::gtk_list_store_move_before(self.to_glib_none().0, mut_override(iter.to_glib_none().0), mut_override(position.to_glib_none().0));
        }
    }

    pub fn prepend(&self) -> TreeIter {
        unsafe {
            let mut iter = TreeIter::uninitialized();
            ffi::gtk_list_store_prepend(self.to_glib_none().0, iter.to_glib_none_mut().0);
            iter
        }
    }

    pub fn remove(&self, iter: &TreeIter) -> bool {
        unsafe {
            from_glib(ffi::gtk_list_store_remove(self.to_glib_none().0, mut_override(iter.to_glib_none().0)))
        }
    }

    //pub fn reorder(&self, new_order: /*Unimplemented*/&CArray TypeId { ns_id: 0, id: 14 }) {
    //    unsafe { TODO: call ffi::gtk_list_store_reorder() }
    //}

    //pub fn set(&self, iter: &TreeIter, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi::gtk_list_store_set() }
    //}

    //pub fn set_column_types(&self, n_columns: i32, types: /*Unimplemented*/&CArray TypeId { ns_id: 0, id: 30 }) {
    //    unsafe { TODO: call ffi::gtk_list_store_set_column_types() }
    //}

    //pub fn set_valist(&self, iter: &TreeIter, var_args: /*Unknown conversion*//*Unimplemented*/Unsupported) {
    //    unsafe { TODO: call ffi::gtk_list_store_set_valist() }
    //}

    //pub fn set_valuesv(&self, iter: &TreeIter, columns: /*Unimplemented*/&CArray TypeId { ns_id: 0, id: 14 }, values: &[&glib::Value], n_values: i32) {
    //    unsafe { TODO: call ffi::gtk_list_store_set_valuesv() }
    //}

    pub fn swap(&self, a: &TreeIter, b: &TreeIter) {
        unsafe {
            ffi::gtk_list_store_swap(self.to_glib_none().0, mut_override(a.to_glib_none().0), mut_override(b.to_glib_none().0));
        }
    }
}
