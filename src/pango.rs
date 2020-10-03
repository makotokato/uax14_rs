use std::char::decode_utf16;
use std::os::raw::{c_char, c_int, c_uint, c_void};

// TODO:
// PangoLogAttr define in pango-sys is incorrect, so we don't use pango-sys.

#[repr(C)]
pub struct PangoLanguage(c_void);

#[repr(C)]
pub struct PangoLogAttr {
    // all members are bit field.
    mask: c_uint,
}

impl PangoLogAttr {
    pub fn is_line_break(&self) -> bool {
        (self.mask & 1) != 0
    }
}

#[link(name = "pango-1.0")]
extern "C" {
    pub fn pango_language_from_string(language: *const c_char) -> *mut PangoLanguage;

    pub fn pango_get_log_attrs(
        text: *const c_char,
        length: c_int,
        level: c_int,
        language: *mut PangoLanguage,
        log_attrs: *mut PangoLogAttr,
        attrs_len: c_int,
    );
}

pub fn get_line_break_utf16(text: *const u16, length: usize) -> Option<Vec<usize>> {
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

        let attrs = std::slice::from_raw_parts(attr_buffer.as_ptr(), length);
        let mut i = 0;
        let mut breaks: Vec<usize> = Vec::new();
        loop {
            if i >= length {
                break;
            }
            if attrs[i].is_line_break() {
                breaks.push(i);
            }
            i += 1;
        }
        if breaks.len() > 0 {
            return Some(breaks);
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::pango::get_line_break_utf16;

    #[test]
    fn pango_line_break() {
        let text: [u16; 5] = [0x42, 0x42, 0x42, 0x20, 0x42];
        let breaks = get_line_break_utf16(text.as_ptr(), text.len());
        assert_eq!(breaks.unwrap(), [4], "space");
    }
}
