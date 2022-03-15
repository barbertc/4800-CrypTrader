const path = require('path');
const express = require('express');
const PORT = process.env.PORT || 3001;
const app = express();
const ffi = require('ffi-napi');

app.use(express.static(path.resolve(__dirname, '../react_client/build')));

app.get("/api", (req, res) => {
    res.json({ message: `Listening on port ${PORT}` });
});

app.get('*', (req, res) => {
    res.sendFile(path.resolve(__dirname, '../react_client/build', 'index.html'));
});

app.listen(PORT, () => {
    console.log(`Server listening on ${PORT}`);
})

const lib = ffi.Library('rust_prog/target/release/librust_prog.so', {
    'add2numbers': ['int', ['int', 'int']],
});

let res = lib.add2numbers(100, 50);
console.log(res);
