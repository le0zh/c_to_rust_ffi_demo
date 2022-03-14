#include "../lib/libc_to_rust_ffi_demo.h"
#include <stdio.h>

void on_state_changed_callback(uint32_t state) {
    printf("[c]OnStateChangedCallback is called: %d\n", state);
}

int main(int argc, char *argv[])
{
    printf("================= c → rust demo===================\n");

    // 单独调用，没有入参，没有返回值
    ffi_say_hello();

    // 返回字符串
    const char *text = ffi_hello("rust");

    printf("[1]got string from rust: %s\n", text);

    // free text
    ffi_free_str(text);

    // 返回数字
    int32_t result = ffi_double_input(2);
    printf("[2]got int from rust: %d\n", result);

    // 返回 bool
    bool is_answer = ffi_is_answer(24);
    printf("[3]got bool from rust: %s\n", is_answer ? "true" : "false");

    // 返回结构体
    RStruct *rs = ffi_data_new();
    printf("[4]got struct from rust:  name as %s\n", rs->name);

    if (rs->value.tag == _Int)
    {
        printf("[4]got struct from rust:  value as int %d\n", rs->value.int_);
    }
    else
    {
        printf("[4]got struct from rust:  value as float %f\n", rs->value.float_);
    }

    // free memory
    ffi_data_free(rs);

    // rs is gone now
    // printf("[4]got struct from rust:  name as %s and value as \sn", rs->name);

    // 使用结构体作为参数请求 rust
    FFIHttpRequest req;
    req.headers = "{'Content-Type': 'application/json;charset=UTF-8'}";
    req.method = "POST";
    req.url = "http://awesome.service.api";
    uint64_t res_status = ffi_http_request(req, 5000);
    printf("[5]got ffi_http_request status: %d\n", res_status);

    // 带有回调函数的请求 
    ffi_call_with_callback(1, on_state_changed_callback);

    return 0;
}
