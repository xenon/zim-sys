# zim-sys
Work-in-progress experiment getting libzim bindings in Rust.

## Compilation
Currently only supports unix-based machines which also support libzim.
- Windows support could be achieved by using ``vcpkg`` to find libzim on windows
- Only Ubuntu Linux has been tested
### Installing dependencies
The build process requires ``clang`` and the original library ``libzim`` will be needed.
#### Ubuntu
```bash
sudo apt-get install libzim-dev clang
```

### Building zim-sys
```
cargo build
```
Will generate a file in /target/debug/build/zim-sys-<STRING>/out/binding.rs

## Issues
### Exceptions
So-far the library brings in all of the bindings from libzim that we need to get started.
However Rust FFI **does not** support C++ exceptions
#### Solution
We would need to make a wrapper file that wrap all of the classes with the following properties
- If the method can not throw, wrap the method directly
- If the method can throw, catch and wrap the return in a Result type

## Testing / Hacking / Trying it out
Head to ``src/lib.rs`` and make a test. The translated bindings are found in the namespace **root::zim**.
### Warnings
Beware that if the function you call throws an exception it will not work out well in the rust code if it fails!