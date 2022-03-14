# FFI demo for C call Rust

> MacOS only for now.

### Build the lib

```shell
sh build.sh
```

### Run the demo

```shell
cd demo
sh run.sh
```

```
================= c → rust demo===================
[0]hello from rust
[1]got string from rust: hello rust!
[2]got int from rust: 4
[3]got bool from rust: false
[4]got struct from rust:  name as my_rstruct
[4]got struct from rust:  value as int 42
[rust] do http request url http://awesome.service.api with timeout 5000
[5]got ffi_http_request status: 200
[rust] do biz logic with 1 ....
[c]OnStateChangedCallback is called: 201
```