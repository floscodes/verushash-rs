unsafe extern "C" {
    pub unsafe fn verus_hash(result: *mut u8, data: *const u8, len: usize);

    pub unsafe fn verus_hash_v2(result: *mut u8, data: *const u8, len: usize);
}
