extern crate libc;
extern crate yauid_sys;

use std::{ptr, ffi};
use yauid_sys::{yauid_init, yauid_destroy, yauid_get_key, yauid_set_node_id, yauid_get_error_text_by_code};

pub struct Yauid {
    _ffi_filepath_key: ffi::CString,
    _ffi_filepath_node_id: Option<ffi::CString>,
    ya: *mut yauid_sys::Yauid,
}

impl Drop for Yauid { fn drop(&mut self) { self.close(); } }

#[derive(Clone, Debug)]
pub struct Error(pub String);

impl Yauid {
    pub fn new(filepath_key: &str, node_id: u64) -> Result<Yauid, Error> {
        let ffi_filepath_key = try!(ffi::CString::new(filepath_key.as_bytes()).map_err(|_| Error("invalid filepath_key".to_owned())));
        let ya = unsafe { yauid_init(ffi_filepath_key.as_ptr(), ptr::null()) };
        if ya.is_null() {
            Err(Error("yauid_init failure".to_owned()))
        } else {
            let obj = Yauid {
                _ffi_filepath_key: ffi_filepath_key,
                _ffi_filepath_node_id: None,
                ya: ya,
            };
            match unsafe { (*obj.ya).error } {
                0 => {
                    unsafe { yauid_set_node_id(obj.ya, node_id) };
                    match unsafe { (*obj.ya).error } {
                        0 => Ok(obj),
                        status => Err(Error(obj.status_to_string(status))),
                    }
                },
                status => Err(Error(obj.status_to_string(status))),
            }
        }
    }

    pub fn with_node_id(filepath_key: &str, filepath_node_id: &str) -> Result<Yauid, Error> {
        let ffi_filepath_key = try!(ffi::CString::new(filepath_key.as_bytes()).map_err(|_| Error("invalid filepath_key".to_owned())));
        let ffi_filepath_node_id = try!(ffi::CString::new(filepath_node_id.as_bytes()).map_err(|_| Error("invalid node_id".to_owned())));
        let ya = unsafe { yauid_init(ffi_filepath_key.as_ptr(), ffi_filepath_node_id.as_ptr()) };
        if ya.is_null() {
            Err(Error("yauid_init failure".to_owned()))
        } else {
            let obj = Yauid {
                _ffi_filepath_key: ffi_filepath_key,
                _ffi_filepath_node_id: Some(ffi_filepath_node_id),
                ya: ya,
            };
            match unsafe { (*obj.ya).error } {
                0 => Ok(obj),
                status => Err(Error(obj.status_to_string(status))),
            }
        }
    }

    pub fn close(&mut self) {
        if !self.ya.is_null() {
            unsafe { yauid_destroy(self.ya) };
        }
    }

    pub fn get_key(&self) -> Result<u64, Error> {
        let key = unsafe { yauid_get_key(self.ya) };
        match unsafe { (*self.ya).error } {
            0 => Ok(key as u64),
            status => Err(Error(self.status_to_string(status))),
        }
    }

    fn status_to_string(&self, status: libc::c_int) -> String {
        String::from_utf8_lossy(unsafe { ffi::CStr::from_ptr(yauid_get_error_text_by_code(status)).to_bytes() }).into_owned()
    }
}

#[cfg(test)]
mod test {
    use super::{Yauid};

    #[test]
    fn create_fail() {
        match Yauid::new("/zz/qq/ww/yauid.tmp", 1) {
            Ok(_) => panic!("Expected create error, but operation succeeded"),
            Err(_) => ()
        }
    }

    #[test]
    fn create_get_drop() {
        let y = Yauid::new("/tmp/yauid.tmp", 1).unwrap();
        let id_a = y.get_key().unwrap();
        let id_b = y.get_key().unwrap();
        assert!(id_a < id_b);
    }
}
