#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct List {
    pub next: *mut List,
}
impl Default for List {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PersistentRooted {
    pub root_list: List,
}
impl Default for PersistentRooted {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
