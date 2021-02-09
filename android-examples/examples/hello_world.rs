use ndk::trace;
use uax14_rs::*;

#[cfg_attr(target_os = "android", ndk_glue::main(backtrace = "on"))]
fn main() {
    let _trace;
    if trace::is_trace_enabled() {
        _trace = trace::Section::new("uax14-rs example main").unwrap();
    }

    let native_activity = ndk_glue::native_activity();
    let vm_ptr = native_activity.vm();
    let vm = unsafe { jni::JavaVM::from_raw(vm_ptr) }.unwrap();
    let env = vm.attach_current_thread().unwrap();
    let env = env.get_native_interface();

    const TEST_STR: &str = "Permission is hereby granted";    
    const TEST_STR2: &str = "ภาษาไทยภาษาไทย ภาษาไทยภาษาไทย ภาษาไทยภาษาไทย ภาษาไทยภาษาไทยABC";

    // Without Android API
    println!("Native implementation. Thai language isn't followed.");
    let utf16: Vec<u16> = TEST_STR.encode_utf16().map(|x| x).collect();
    println!("TEST_STR break iter count = {}", LineBreakIteratorUTF16::new(&utf16).count());
    println!("TEST_STR len = {}", utf16.iter().count());
    let mut iter = LineBreakIteratorUTF16::new(&utf16);
    println!("next = {}", iter.next().unwrap());
    println!("next = {}", iter.next().unwrap());
    println!("next = {}", iter.next().unwrap());
    println!("next = {}", iter.next().unwrap());
    let utf16: Vec<u16> = TEST_STR2.encode_utf16().map(|x| x).collect();
    println!("TEST_STR2 break iter count = {}", LineBreakIteratorUTF16::new(&utf16).count());
    println!("TEST_STR2 len = {}", utf16.iter().count());
    let mut iter = LineBreakIteratorUTF16::new(&utf16);
    println!("next = {}", iter.next().unwrap());
    println!("next = {}", iter.next().unwrap());
    println!("next = {}", iter.next().unwrap());
    println!("next = {}", iter.next().unwrap());

    // With Android API using JNI
    println!("Using Android API");
    let utf16: Vec<u16> = TEST_STR2.encode_utf16().map(|x| x).collect();
    println!("TEST_STR2 break iter count = {}", LineBreakIteratorUTF16::new(&utf16).count());
    let mut iter = LineBreakIteratorUTF16::new(&utf16);
    iter.set_jni_env(env as * mut std::ffi::c_void);
    println!("next = {}", iter.next().unwrap());
    println!("next = {}", iter.next().unwrap());
    println!("next = {}", iter.next().unwrap());
    println!("next = {}", iter.next().unwrap());
    println!("next = {}", iter.next().unwrap());
    println!("next = {}", iter.next().unwrap());
    println!("next = {}", iter.next().unwrap());
    println!("next = {}", iter.next().unwrap());
    println!("next = {}", iter.next().unwrap());
    println!("next = {}", iter.next().unwrap());
    println!("next = {}", iter.next().unwrap());
    println!("next = {}", iter.next().unwrap());
    println!("next = {}", iter.next().unwrap());
    println!("next = {}", iter.next().unwrap());
    println!("next = {}", iter.next().unwrap());
    println!("next = {}", iter.next().unwrap());
    println!("next = {}", iter.next().unwrap());
    println!("next = {}", iter.next().unwrap());
    println!("next = {}", iter.next().unwrap());
    println!("next = {}", iter.next().unwrap());
}
