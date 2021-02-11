use std::ptr;

use winapi::um::usp10::*;

pub fn get_line_break_utf16(text: *const u16, length: usize) -> Option<Vec<usize>> {
    let mut breaks = Vec::new();

    let mut item_buffer = Vec::with_capacity(length + 1);
    let mut out_len = length as i32;

    let hresult = unsafe {
        ScriptItemize(
            text,
            length as i32,
            length as i32,
            ptr::null(),
            ptr::null(),
            item_buffer.as_mut_ptr(),
            &mut out_len,
        )
    };
    if hresult != 0 {
        return None;
    }
    if out_len == 0 {
        return None;
    }

    let slice = unsafe { std::slice::from_raw_parts(text, length) };
    let items = unsafe { std::slice::from_raw_parts(item_buffer.as_ptr(), out_len as usize) };
    let mut item_index = 0;
    loop {
        if item_index >= out_len as usize {
            break;
        }
        let start_offset = items[item_index].iCharPos as usize;
        let mut new_len = length - start_offset;
        if item_index + 1 < out_len as usize {
            new_len = (items[item_index + 1].iCharPos - items[item_index].iCharPos) as usize;
        }
        let mut attr_buffer = Vec::with_capacity(new_len);
        let hresult = unsafe {
            ScriptBreak(
                &slice[start_offset],
                new_len as i32,
                &items[item_index].a,
                attr_buffer.as_mut_ptr(),
            )
        };
        if hresult != 0 {
            return None;
        }

        let attrs = unsafe { std::slice::from_raw_parts(attr_buffer.as_ptr(), new_len) };
        let mut i: usize = 0;
        loop {
            if i >= new_len {
                break;
            }
            if attrs[i].fSoftBreak() != 0 {
                if i + start_offset > 0 {
                    breaks.push(i + start_offset);
                }
            }

            i += 1;
        }

        item_index += 1;
    }

    if breaks.is_empty() {
        return None;
    }
    Some(breaks)
}

#[cfg(test)]
mod tests {
    use crate::windows::get_line_break_utf16;

    #[test]
    fn macos_line_break() {
        let text: [u16; 5] = [0x42, 0x42, 0x42, 0x20, 0x42];
        let breaks = get_line_break_utf16(text.as_ptr(), text.len());
        assert_eq!(breaks.unwrap(), [4], "ASCII and SP");

        let text: [u16; 14] = [
            0x0e20, 0x0e32, 0x0e29, 0x0e32, 0x0e44, 0x0e17, 0x0e22, 0x0e20, 0x0e32, 0x0e29, 0x0e32,
            0x0e44, 0x0e17, 0x0e22,
        ];
        let breaks = get_line_break_utf16(text.as_ptr(), text.len());
        assert_eq!(breaks.unwrap(), [4, 7, 11], "Thai test");

        let text: [u16; 4] = [0x0e20, 0x0e32, 0x0e29, 0x0e32];
        let breaks = get_line_break_utf16(text.as_ptr(), text.len());
        assert_eq!(breaks, None, "Thai test");
    }
}
