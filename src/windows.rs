use std::ptr;

use winapi::um::usp10::*;

pub fn get_next_break_utf16(text: *const u16, length: usize) -> Option<usize> {
    unsafe {
        let mut item_buffer = Vec::with_capacity(length + 1);
        let mut out_len = length as i32;

        let hresult = ScriptItemize(
            text,
            length as i32,
            length as i32,
            ptr::null(),
            ptr::null(),
            item_buffer.as_mut_ptr(),
            &mut out_len,
        );
        if hresult != 0 {
            return None;
        }
        if out_len == 0 {
            return None;
        }

        let slice = std::slice::from_raw_parts(text, length);
        let items = std::slice::from_raw_parts(item_buffer.as_ptr(), out_len as usize);
        let mut item_index = 0;
        loop {
            if item_index >= out_len as usize {
                return None;
            }
            let start_offset = items[item_index].iCharPos;
            let mut new_len = length - start_offset as usize;
            if item_index + 1 < out_len as usize {
                new_len = (items[item_index + 1].iCharPos - items[item_index].iCharPos) as usize;
            }
            let mut attr_buffer = Vec::with_capacity(new_len);
            let hresult = ScriptBreak(
                &slice[start_offset as usize],
                new_len as i32,
                &items[item_index].a,
                attr_buffer.as_mut_ptr(),
            );
            if hresult != 0 {
                return None;
            }

            let attrs = std::slice::from_raw_parts(attr_buffer.as_ptr(), new_len);
            let mut i: usize = 0;
            loop {
                if i >= new_len {
                    break;
                }
                if attrs[i].fSoftBreak() != 0 {
                    if i + (start_offset as usize) > 0 {
                        return Some(i + (start_offset as usize));
                    }
                }

                i += 1;
            }

            item_index += 1;
        }
    }
}
