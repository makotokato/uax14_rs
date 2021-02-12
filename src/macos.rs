#![allow(non_upper_case_globals)]

use std::os::raw::c_void;
use std::ptr;

use core_foundation::base::*;
use core_foundation::string::*;

type CFLocaleRef = *const c_void;
// From CFStringTokenizer.h
type CFStringTokenizerRef = *const c_void;
type CFStringTokenizerTokenType = usize;
const kCFStringTokenizerUnitLineBreak: CFOptionFlags = 3;
const kCFStringTokenizerTokenNone: CFStringTokenizerTokenType = 0;

extern "C" {
    // This is added at the latest of core-foundation-rs
    fn CFStringCreateWithCharactersNoCopy(
        alloc: CFAllocatorRef,
        chars: *const UniChar,
        numChars: CFIndex,
        contentsDeallocator: CFAllocatorRef,
    ) -> CFStringRef;
    fn CFStringTokenizerCreate(
        alloc: CFAllocatorRef,
        string: CFStringRef,
        range: CFRange,
        options: CFOptionFlags,
        locale: CFLocaleRef,
    ) -> CFStringTokenizerRef;
    fn CFStringTokenizerAdvanceToNextToken(
        tokenizer: CFStringTokenizerRef,
    ) -> CFStringTokenizerTokenType;
    fn CFStringTokenizerGetCurrentTokenRange(tokenizer: CFStringTokenizerRef) -> CFRange;
}

pub fn get_line_break_utf16(text: *const u16, length: usize) -> Option<Vec<usize>> {
    let mut breaks = Vec::new();
    let os_str = unsafe {
        CFStringCreateWithCharactersNoCopy(
            kCFAllocatorDefault,
            text,
            length as CFIndex,
            kCFAllocatorNull,
        )
    };
    let range: CFRange = CFRange {
        location: 0,
        length: length as isize,
    };
    let token = unsafe {
        CFStringTokenizerCreate(
            kCFAllocatorDefault,
            os_str,
            range,
            kCFStringTokenizerUnitLineBreak,
            ptr::null(),
        )
    };
    loop {
        let token_type = unsafe { CFStringTokenizerAdvanceToNextToken(token) };
        if token_type == kCFStringTokenizerTokenNone {
            unsafe {
                CFRelease(token);
                CFRelease(os_str as *const c_void);
            }
            break;
        }

        let result = unsafe { CFStringTokenizerGetCurrentTokenRange(token) };
        if result.location != 0 {
            breaks.push(result.location as usize);
        }
    }

    if breaks.is_empty() {
        return None;
    }
    Some(breaks)
}

#[cfg(test)]
mod tests {
    use crate::macos::get_line_break_utf16;

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

        let text: [u16; 7] = [0x0e01, 0x0e23, 0x0e380, 0x0e07, 0x0e40, 0x0e17, 0x0e1e];
        let breaks = get_line_break_utf16(text.as_ptr(), text.len());
        assert_eq!(breaks, None, "Thai test");

        let text: [u16; 27] = [
            0x1797, 0x17b6, 0x179f, 0x17b6, 0x1781, 0x17d2, 0x1798, 0x17c2, 0x179a, 0x1797, 0x17b6,
            0x179f, 0x17b6, 0x1781, 0x17d2, 0x1798, 0x17c2, 0x179a, 0x1797, 0x17b6, 0x179f, 0x17b6,
            0x1781, 0x17d2, 0x1798, 0x17c2, 0x179a,
        ];
        let breaks = get_line_break_utf16(text.as_ptr(), text.len());
        assert_eq!(breaks.unwrap(), [9, 18], "Khmer test");
    }
}
