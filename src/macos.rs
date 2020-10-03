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
    fn CFStringCreateWithCharactersNoCopy(
        alloc: CFAllocatorRef,
        chars: *const u16,
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

pub fn get_next_break_utf16(text: *const u16, length: usize) -> Option<usize> {
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
                return None;
            }

            let result = CFStringTokenizerGetCurrentTokenRange(token);
            if result.location != 0 {
                CFRelease(token);
                CFRelease(os_str as *const c_void);
                return Some(result.location as usize);
            }
        }
    }
}

pub fn get_line_break_utf16(text: *const u16, length: usize) -> Option<Vec<usize>> {
    if let Some(b) = get_next_break_utf16(text, length) {
        let breaks = Vec::new();
        breaks.push(b);
        return breaks;
    }
    None
}
