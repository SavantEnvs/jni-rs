#![no_main]
use libfuzzer_sys::fuzz_target;

// Fuzz the jni crate's pure-Rust JNI type-signature parser (no JVM needed): the mayhemheroes
// harness at this vintage of jni-rs (pre crates/jni restructure, pre RuntimeMethodSignature API).
fuzz_target!(|data: &str| {
    let _ = jni::signature::TypeSignature::from_str(data);
});
