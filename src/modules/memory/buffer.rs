struct circular_buffer {
    buffer: *mut u8,
    size_head: usize,
    size_tail: usize,
    size_max: usize,
    full: bool,
}

// pub