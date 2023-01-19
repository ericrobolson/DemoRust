/// Checks if a wrapping sequence is after another.
/// See https://stackoverflow.com/questions/2594423/whats-a-good-way-to-detect-wrap-around-in-a-fixed-width-message-counter
pub fn sequence_a_after_b_u8(a: u8, b: u8) -> bool {
    const MAX: u8 = u8::MAX;
    let a_size = a.wrapping_sub(b) % MAX;
    let b_size = b.wrapping_sub(a) % MAX;
    a_size > b_size
}

/// Checks if a wrapping sequence is after another.
/// See https://stackoverflow.com/questions/2594423/whats-a-good-way-to-detect-wrap-around-in-a-fixed-width-message-counter
pub fn sequence_a_after_b_u16(a: u16, b: u16) -> bool {
    const MAX: u16 = u16::MAX;
    let a_size = a.wrapping_sub(b) % MAX;
    let b_size = b.wrapping_sub(a) % MAX;
    a_size > b_size
}
