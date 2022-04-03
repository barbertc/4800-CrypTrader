const ffi = require('ffi-napi')

rust = ffi.Library('../rust-app/target/release/librust_app', {
    'test_fun': ['int', ['int', 'int']]
})

function test(n1, n2) {
    return rust.test_fun(n1, n2);
}