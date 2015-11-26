extern crate libc;

use libc::{c_int, c_uint, c_ulong, c_char, c_void, uint64_t, useconds_t, FILE};

#[repr(C)]
pub struct Yauid {
    i_lockfile: c_int,
    c_lockfile: *const c_char,
    h_lockfile: *mut FILE,
    node_id: c_ulong,
    try_count: c_uint,
    sleep_usec: useconds_t,
    pub error: c_int,
    ext_value: *mut c_void,
}

extern {
    pub fn yauid_init(filepath_key: *const c_char, filepath_node_id: *const c_char) -> *mut Yauid;
    pub fn yauid_destroy(yauid: *mut Yauid);
    pub fn yauid_get_key(yauid: *mut Yauid) -> uint64_t;
    pub fn yauid_set_node_id(yauid: *mut Yauid, node_id: c_ulong);
    pub fn yauid_get_error_text_by_code(error: c_int) -> *const c_char;
}
