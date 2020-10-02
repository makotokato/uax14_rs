use pango_sys::*;
use std::char::decode_utf16;
use std::os::raw::c_char;

pub fn get_next_break_utf16(text: *const u16, length: usize) -> Option<usize> {
    unsafe {
        let slice = std::slice::from_raw_parts(text, length);
        let s: String = decode_utf16(slice.iter().cloned())
            .map(|r| r.unwrap())
            .collect();
        let language = pango_language_from_string("en".as_ptr() as *const c_char);
        let mut attr_buffer = Vec::with_capacity(length + 1);

        pango_get_log_attrs(
            s.as_ptr() as *const i8,
            s.len() as i32,
            -1,
            language,
            attr_buffer.as_mut_ptr(),
            (length + 1) as i32,
        );

        // TODO: PangoLogAttr define in pango-sys is incorrect.
        let attrs = std::slice::from_raw_parts(attr_buffer.as_ptr() as *const u32, length);
        let mut i = 0;
        loop {
            if i >= length {
                return None;
            }
            if (attrs[i] & 1) == 1 {
                return Some(i);
            }
            i += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::pango::get_next_break_utf16;

    #[test]
    fn pango_line_break() {
        let text: [u16; 5] = [0x42, 0x42, 0x42, 0x20, 0x42];
        let first_break = get_next_break_utf16(text.as_ptr(), text.len());
        assert_eq!(first_break.unwrap(), 4, "space");
    }
}
