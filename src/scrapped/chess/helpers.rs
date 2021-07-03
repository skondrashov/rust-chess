#[inline(always)]
pub fn boolean_to_u8_mask(boolean: bool) -> u8 {
    !((boolean as u8) - 1)
}

#[inline(always)]
pub fn u32_to_bool(u32value: u32) -> bool {
    u32value > 0
}

#[inline(always)]
pub fn u8_to_bool(u8value: u8) -> bool {
    u8value > 0
}
