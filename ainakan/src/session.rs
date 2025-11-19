/*
 * Copyright © 2022 Jean Marchand
 *
 * Licence: wxWindows Library Licence, Version 3.1
 */

use ainakan_sys::_AinakanSession;
use std::ffi::CString;
use std::marker::PhantomData;
use std::ptr::null_mut;

use crate::script::{Script, ScriptOption};
use crate::{Error, Result};

/// Represents a Ainakan session.
pub struct Session<'a> {
    session_ptr: *mut _AinakanSession,
    phantom: PhantomData<&'a _AinakanSession>,
}

impl<'a> Session<'a> {
    pub(crate) fn from_raw(session_ptr: *mut _AinakanSession) -> Session<'a> {
        Session {
            session_ptr,
            phantom: PhantomData,
        }
    }

    /// Returns if the session is detached or not.
    pub fn is_detached(&self) -> bool {
        unsafe { ainakan_sys::ainakan_session_is_detached(self.session_ptr) == 1 }
    }

    /// Creates a [`Script`] attached to current session.
    pub fn create_script<'b>(
        &'a self,
        source: &str,
        option: &mut ScriptOption,
    ) -> Result<Script<'b>>
    where
        'a: 'b,
    {
        let mut error: *mut ainakan_sys::GError = std::ptr::null_mut();
        match CString::new(source) {
            Ok(source) => {
                let script = unsafe {
                    ainakan_sys::ainakan_session_create_script_sync(
                        self.session_ptr,
                        source.as_ptr(),
                        option.as_mut_ptr(),
                        null_mut(),
                        &mut error,
                    )
                };
                if error.is_null() {
                    Ok(Script::from_raw(script))
                } else {
                    Err(Error::ScriptCreationError)
                }
            }
            Err(_) => Err(Error::CStringFailed),
        }
    }

    /// Detaches the current session.
    pub fn detach(&self) -> Result<()> {
        let mut error: *mut ainakan_sys::GError = std::ptr::null_mut();
        unsafe {
            ainakan_sys::ainakan_session_detach_sync(self.session_ptr, std::ptr::null_mut(), &mut error)
        }

        if error.is_null() {
            Ok(())
        } else {
            Err(Error::SessionDetachError)
        }
    }
}

impl Drop for Session<'_> {
    fn drop(&mut self) {
        unsafe { ainakan_sys::ainakan_unref(self.session_ptr as _) }
    }
}
