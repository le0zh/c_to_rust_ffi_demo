use libc::{c_char};
use std::{
    ffi::{CStr, CString},
    panic::catch_unwind,
    ptr,
};
use std::boxed::Box;


#[no_mangle] 
pub extern "C" fn ffi_say_hello() { 
    println!("[0]hello from rust");
}

// rust 返回字符串给 C
#[no_mangle]
pub extern "C" fn ffi_hello(name: *const c_char) -> *const c_char {
    if name.is_null() {
        return ptr::null();
    }

    let result = catch_unwind(|| {
        if let Ok(s) = unsafe { CStr::from_ptr(name).to_str() } {
            let result = format!("hello {}!", s);
            // 可以使用 unwrap，因为 result 不包含 \0
            let s = CString::new(result).unwrap();

            s.into_raw()
            // 相当于：
            // let p = s.as_ptr();
            // std::mem::forget(s);
            // p
        } else {
            ptr::null()
        }
    });

    match result {
        Ok(s) => s,
        Err(_) => ptr::null(),
    }
}

// 提供给 C 侧释放字符串指针，调用者需要保证指针来自 rust
#[no_mangle]
pub unsafe extern "C" fn ffi_free_str(s: *mut c_char) {
    if !s.is_null() {
        let _ = CString::from_raw(s);
    }
}

#[no_mangle]
pub unsafe extern "C" fn ffi_double_input(input: i32) -> i32 {
    input * 2
}

#[no_mangle]
pub unsafe extern "C" fn ffi_is_answer(input: i32) -> bool {
    input == 42
}

#[repr(C)]
pub struct RStruct {
    name: *const c_char,
    value: Value,
}

#[repr(C)]
pub enum Value {
    _Int(i32),
    _Float(f64),
}

#[no_mangle]
pub extern "C" fn ffi_data_new() -> *mut RStruct {

    Box::into_raw(Box::new(RStruct {
        name: CString::new("my_rstruct")
            .unwrap()
            .into_raw(),
        value: Value::_Int(42),
    }))
}

#[no_mangle]
pub extern "C" fn ffi_data_free(ptr: *mut RStruct) {
    if ptr.is_null() {
        return;
    }

    unsafe {
        let box_res = Box::from_raw(ptr);
        // 需要逐个释放结构体中的所有字符串数据
        CString::from_raw(box_res.name as *mut c_char);
    }
}

#[repr(C)]
#[allow(clippy::upper_case_acronyms)]
pub struct FFIHttpRequest {
    pub url: *const c_char,
    pub method: *const c_char,
    pub headers: *const c_char,
}

// NOTE: FFIHttpRequest 是在 C 语言维护的，不需要在 rust 侧释放
#[no_mangle]
pub extern "C" fn ffi_http_request(request: FFIHttpRequest, timeout: u64) -> u64 {
    // c_str to String
    if let Ok(s) = unsafe { CStr::from_ptr(request.url).to_str() } {
        // do with request and timeout
        println!("[rust] do http request url {} with timeout {}", s, timeout);
    }

    200
}


pub type OnStateChanged = extern "C" fn(u32);

#[no_mangle]
pub extern  "C" fn ffi_call_with_callback(input: u32, cb: OnStateChanged) -> u32{

    println!("[rust] do biz logic with {} ....", input);

    cb(input + 200);

    0
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
