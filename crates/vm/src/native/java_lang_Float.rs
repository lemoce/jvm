#![allow(non_snake_case)]

use crate::native::{new_fn, JNIEnv, JNINativeMethod, JNIResult};
use crate::oop::Oop;

pub fn get_native_methods() -> Vec<JNINativeMethod> {
    vec![new_fn(
        "floatToRawIntBits",
        "(F)I",
        Box::new(jvm_floatToRawIntBits),
    )]
}

fn jvm_floatToRawIntBits(_env: JNIEnv, args: &Vec<Oop>) -> JNIResult {
    let v = args.get(0).unwrap().extract_float();
    let v = v.to_bits().to_be_bytes();
    let v = i32::from_be_bytes([v[0], v[1], v[2], v[3]]);
    Ok(Some(Oop::new_int(v)))
}
