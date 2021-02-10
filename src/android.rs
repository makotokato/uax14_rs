use jni::objects::JValue;
use jni::signature::{JavaType, Primitive};
use jni::sys;
use jni::JNIEnv;
use std::char::decode_utf16;
use std::ffi::c_void;

pub fn get_line_break_utf16(
    jni_env: *mut c_void,
    text: *const u16,
    length: usize,
) -> Option<Vec<usize>> {
    let env = unsafe { JNIEnv::from_raw(jni_env as *mut sys::JNIEnv).ok()? };

    let mut breaks = Vec::new();
    let class = env.auto_local(env.find_class("android/icu/text/BreakIterator").ok()?);

    let iter_obj = env.auto_local(
        env.call_static_method(
            &class,
            "getLineInstance",
            "()Landroid/icu/text/BreakIterator;",
            &[],
        )
        .ok()?
        .l()
        .ok()?,
    );

    let slice = unsafe { std::slice::from_raw_parts(text, length) };
    let s: String = decode_utf16(slice.iter().cloned())
        .map(|r| r.unwrap())
        .collect();

    let java_string = env.auto_local(env.new_string(s).unwrap());
    env.call_method(
        &iter_obj,
        "setText",
        "(Ljava/lang/String;)V",
        &[JValue::from(&java_string)],
    )
    .ok()?
    .v()
    .ok()?;

    let next_method = env.get_method_id(&class, "next", "()I").ok()?;
    loop {
        let ret = JavaType::Primitive(Primitive::Int);
        let location = env
            .call_method_unchecked(&iter_obj, next_method, ret, &[])
            .ok()?
            .i()
            .ok()?;
        if location < 0 || length <= location as usize {
            break;
        }
        breaks.push(location as usize);
    }

    if breaks.is_empty() {
        return None;
    }
    Some(breaks)
}
