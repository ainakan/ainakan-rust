/*
 * Copyright © 2020-2022 Keegan Saunders
 * Copyright © 2022 Jean Marchand
 *
 * Licence: wxWindows Library Licence, Version 3.1
 */

//! Ainakan bindings for Rust.

#![deny(warnings)]
#![deny(missing_docs)]
#![allow(clippy::missing_safety_doc)]

use std::ffi::CStr;

mod device;
pub use device::*;

mod device_manager;
pub use device_manager::*;

mod error;
pub use error::Error;

mod injector;
pub use injector::*;

mod process;
pub use process::*;

mod script;
pub use script::*;

mod session;
pub use session::*;

mod variant;
pub use variant::*;

#[doc(hidden)]
pub type Result<T> = std::result::Result<T, error::Error>;

/// Context required for instantiation of all structures under the Ainakan namespace.
pub struct Ainakan;

impl Ainakan {
    /// Obtain a Ainakan handle, ensuring that the runtime is properly initialized. This may
    /// be called as many times as needed, and results in a no-op if the Ainakan runtime is
    /// already initialized.
    pub unsafe fn obtain() -> Ainakan {
        ainakan_sys::ainakan_init();
        Ainakan {}
    }

    /// Gets the current version of ainakan core
    pub fn version() -> &'static str {
        let version = unsafe { CStr::from_ptr(ainakan_sys::ainakan_version_string() as _) };
        version.to_str().unwrap_or_default()
    }

    /// Schedules the closure to be executed on the main ainakan context.
    pub fn schedule_on_main<F>(&self, func: F)
    where
        F: FnOnce() + Send + 'static,
    {
        unsafe {
            unsafe extern "C" fn trampoline<F: FnOnce() + Send + 'static>(
                func: ainakan_sys::gpointer,
            ) -> ainakan_sys::gboolean {
                let func: &mut Option<F> = &mut *(func as *mut Option<F>);
                let func = func
                    .take()
                    .expect("schedule_on_main closure called multiple times");
                func();
                ainakan_sys::G_SOURCE_REMOVE as ainakan_sys::gboolean
            }
            unsafe extern "C" fn destroy_closure<F: FnOnce() + Send + 'static>(
                ptr: ainakan_sys::gpointer,
            ) {
                let _ = Box::<Option<F>>::from_raw(ptr as *mut _);
            }

            let func = Box::into_raw(Box::new(Some(func)));
            let source = ainakan_sys::g_idle_source_new();
            let ctx = ainakan_sys::ainakan_get_main_context();

            ainakan_sys::g_source_set_callback(
                source,
                Some(trampoline::<F>),
                func as ainakan_sys::gpointer,
                Some(destroy_closure::<F>),
            );
            ainakan_sys::g_source_attach(source, ctx);
            ainakan_sys::g_source_unref(source);
        }
    }
}

impl Drop for Ainakan {
    fn drop(&mut self) {
        unsafe { ainakan_sys::ainakan_deinit() };
    }
}
