use ndk::trace;
use std::time::Instant;
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

    const TEST_STR: &str = "Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the \"Software\"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions: The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software. THE SOFTWARE IS PROVIDED \"AS IS\", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.";
    const TEST_STR2: &str = "ภาษาไทยภาษาไทย ภาษาไทยภาษาไทย ภาษาไทยภาษาไทย ภาษาไทยภาษาไทยABC";

    // Without Android API
    println!("Native implementation. Thai language isn't followed.");
    let utf16: Vec<u16> = TEST_STR.encode_utf16().map(|x| x).collect();
    println!(
        "TEST_STR break iter count = {}",
        LineBreakIteratorUTF16::new(&utf16).count()
    );
    println!("TEST_STR len = {}", utf16.iter().count());

    let utf16: Vec<u16> = TEST_STR2.encode_utf16().map(|x| x).collect();
    println!(
        "TEST_STR2 break iter count = {}",
        LineBreakIteratorUTF16::new(&utf16).count()
    );
    println!("TEST_STR2 len = {}", utf16.iter().count());
    let mut iter = LineBreakIteratorUTF16::new(&utf16);
    println!("next = {}", iter.next().unwrap());
    println!("next = {}", iter.next().unwrap());
    println!("next = {}", iter.next().unwrap());
    println!("next = {}", iter.next().unwrap());

    // With Android API using JNI
    println!("Using Android API");
    let utf16: Vec<u16> = TEST_STR2.encode_utf16().map(|x| x).collect();
    println!(
        "TEST_STR2 break iter count = {}",
        LineBreakIteratorUTF16::new(&utf16).count()
    );
    let mut iter = LineBreakIteratorUTF16::new(&utf16);
    iter.set_jni_env(env as *mut std::ffi::c_void);
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

    let utf16: Vec<u16> = TEST_STR.encode_utf16().map(|x| x).collect();
    let now = Instant::now();
    let mut count = 0;
    for _i in 0..1000 {
        let iter = LineBreakIteratorUTF16::new(&utf16);
        count += iter.count();
    }
    println!("char bench. times = {} ns", now.elapsed().as_nanos() / 1000);

    let utf16: Vec<u16> = TEST_STR2.encode_utf16().map(|x| x).collect();
    let now = Instant::now();
    for _i in 0..1000 {
        let iter = LineBreakIteratorUTF16::new(&utf16);
        count += iter.count();
    }
    println!(
        "complex char bench. times = {} ns",
        now.elapsed().as_nanos() / 1000
    );

    let now = Instant::now();
    for _i in 0..10000 {
        let mut iter = LineBreakIteratorUTF16::new(&utf16);
        iter.set_jni_env(env as *mut std::ffi::c_void);
        count += iter.count();
    }
    println!(
        "complex char bench. times = {} ns",
        now.elapsed().as_nanos() / 10000
    );
}
