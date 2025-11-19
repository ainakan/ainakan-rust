/*
 * Copyright © 2020-2022 Keegan Saunders
 *
 * Licence: wxWindows Library Licence, Version 3.1
 */

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]

#[allow(clippy::all)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub use bindings::*;

#[cfg(not(any(target_vendor = "apple", target_os = "windows")))]
pub use crate::{
    _ainakan_g_bytes_get_data as g_bytes_get_data, _ainakan_g_bytes_new as g_bytes_new,
    _ainakan_g_bytes_unref as g_bytes_unref, _ainakan_g_clear_object as g_clear_object,
    _ainakan_g_hash_table_iter_init as g_hash_table_iter_init,
    _ainakan_g_hash_table_iter_next as g_hash_table_iter_next,
    _ainakan_g_hash_table_size as g_hash_table_size, _ainakan_g_idle_source_new as g_idle_source_new,
    _ainakan_g_signal_connect_data as g_signal_connect_data,
    _ainakan_g_source_attach as g_source_attach,
    _ainakan_g_source_set_callback as g_source_set_callback, _ainakan_g_source_unref as g_source_unref,
    _ainakan_g_variant_get_boolean as g_variant_get_boolean,
    _ainakan_g_variant_get_int64 as g_variant_get_int64,
    _ainakan_g_variant_get_string as g_variant_get_string,
    _ainakan_g_variant_get_type_string as g_variant_get_type_string,
    _ainakan_g_variant_iter_init as g_variant_iter_init,
    _ainakan_g_variant_iter_loop as g_variant_iter_loop,
};
