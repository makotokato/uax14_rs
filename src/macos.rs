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
    unsafe {
        let os_str = CFStringCreateWithCharactersNoCopy(
            kCFAllocatorDefault,
            text,
            length as CFIndex,
            kCFAllocatorNull,
        );
        let range: CFRange = CFRange {
            location: 0,
            length: length as isize,
        };
        let token = CFStringTokenizerCreate(
            kCFAllocatorDefault,
            os_str,
            range,
            kCFStringTokenizerUnitLineBreak,
            ptr::null(),
        );
        loop {
            let token_type = CFStringTokenizerAdvanceToNextToken(token);
            if token_type == kCFStringTokenizerTokenNone {
                CFRelease(token);
                CFRelease(os_str as *const c_void);
                break;
            }

            let result = CFStringTokenizerGetCurrentTokenRange(token);
            if result.location != 0 {
                breaks.push(result.location as usize);
            }
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
    }
}
