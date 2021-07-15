// flip on --------------------------------------------------------------------

pub fn flip_on_left_u32(given: &mut u32, bits: u8) -> bool {
    if bits > 32 {
        false
    } else {
        *given |= u32::MAX << (32 - bits);
        true
    }
}

pub fn flip_on_left_u128(given: &mut u128, bits: u8) -> bool {
    if bits > 128 {
        false
    } else {
        *given |= u128::MAX << (128 - bits);
        true
    }
}

pub fn flip_on_right_u32(given: &mut u32, bits: u8) -> bool {
    if bits > 32 {
        false
    } else {
        *given |= u32::MAX >> (32 - bits);
        true
    }
}

pub fn flip_on_right_u128(given: &mut u128, bits: u8) -> bool {
    if bits > 128 {
        false
    } else {
        *given |= u128::MAX >> (128 - bits);
        true
    }
}

// flip off -------------------------------------------------------------------

pub fn flip_off_left_u32(given: &mut u32, bits: u8) -> bool {
    if bits > 32 {
        false
    } else {
        *given &= u32::MAX >> bits;
        true
    }
}

pub fn flip_off_left_u128(given: &mut u128, bits: u8) -> bool {
    if bits > 128 {
        false
    } else {
        *given &= u128::MAX >> bits;
        true
    }
}

pub fn flip_off_right_u32(given: &mut u32, bits: u8) -> bool {
    if bits > 32 {
        false
    } else {
        *given &= u32::MAX << bits;
        true
    }
}

pub fn flip_off_right_u128(given: &mut u128, bits: u8) -> bool {
    if bits > 128 {
        false
    } else {
        *given &= u128::MAX << bits;
        true
    }
}
