//  @author Merve Gulmez (merve.gulmez@ericsson.com)
//  @version 0.1
//  @date 2023-08-31
//  @copyright © Ericsson AB 2023
//  SPDX-License-Identifier: BSD 3-Clause

use bitflags::bitflags;

pub const SDRAD_SUCCESSFUL_RETURNED: i32 = -1;

pub const SDRAD_WARNING_SAVE_EC: i32 = -5;

bitflags! {
    #[repr(C)]
    pub struct Flags: u32 {
        const SDRAD_EXECUTION_DOMAIN     = 0b00000001;
        const SDRAD_DATA_DOMAIN          = 0b00000010;
        const SDRAD_ISOLATED_DOMAIN      = 0b00000100;
        const SDRAD_NONISOLATED_DOMAIN   = 0b00001000;
        const SDRAD_RETURN_TO_PARENT     = 0b00010000;
        const SDRAD_RETURN_TO_CURRENT    = 0b00100000;
        const SDRAD_NO_HEAP_MERGE        = 0b01000000;
    }
}

#[link(name = "sdrad")]
extern "C" {
    pub fn sdrad_init(udi: u64, domain_feature_flag: Flags) -> i32;
    pub fn sdrad_deinit(udi: u64, a: i32) -> i32;
    pub fn sdrad_enter(udi: u64);
    pub fn sdrad_destroy(udi: u64, domain_feature_flag: Flags);
    pub fn sdrad_exit();
    pub fn sdrad_malloc(udi: u64, size_t: libc::size_t) -> *mut u8;
    pub fn sdrad_realloc(udi: u64, ptr: *mut libc::c_void, size_t: isize) -> *mut u8;
    pub fn sdrad_free(udi: u64, ptr: *mut libc::c_void);
    pub fn sdrad_get_stack_base_address(udi: u64) -> i64;
    pub fn sdrad_set_stack_offset(udi: u64, rsp: u64);
    pub fn sdrad_dprotect(
        target_domain_index: u64,
        source_domain_index: u64,
        access_status: i32,
    ) -> i32;
}
