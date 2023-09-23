use detour::static_detour;
use std::io::Write;

static mut ORIGINAL: detour::StaticDetour<fn(&[u8]) -> std::io::Result<usize>> =
    unsafe { static_detour!(@static fn(&[u8]) -> std::io::Result<usize>) };

fn my_invoke(
    obj: &Object,
    a_result_token: ExprTokenType,
    a_flags: i64,
    a_this_token: ExprTokenType,
    a_arg_count: i64,
    a_arg: ExprTokenType,
    a_extra: ExprTokenType,
) -> ResultType {
    // Check if the object has a `base.IsType` key.
    if obj.has_key("base.IsType") {
        // If it does, call the `__getValue()` method.
        return obj.__getValue();
    } else {
        // If it doesn't, call the original `Object::Invoke` function.
        unsafe {
            ORIGINAL.call(
                obj,
                a_result_token,
                a_flags,
                a_this_token,
                a_arg_count,
                a_arg,
                a_extra,
            )
        }
    }
}

fn main() {
    unsafe {
        ORIGINAL
            .initialize(std::io::stdout().write, my_invoke)
            .expect("Failed to initialize detour");
        ORIGINAL.enable().expect("Failed to enable detour");
    }

    // The hooked function will be called whenever std::io::stdout().write is called.
    println!("Hello, world!");

    unsafe {
        ORIGINAL.disable().expect("Failed to disable detour");
    }
}
