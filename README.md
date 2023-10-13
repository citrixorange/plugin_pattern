# plugin_pattern
The Plugin Pattern coded in Rust Language.

This pattern can be helpful for:

* Encapsulating implementation inside a isolated project compiled as a shared lib.
* No compilation need at Client Apps when new updates are released. Instead only load the new binary and play it!
* This pattern is very useful for libs which source code is private.
* This pattern is very useful for Projects which demmand software certification. On updates re-certification only applied for dynamic lib, exempting the client-app.

In the sample we have a implementation of a simple Counter composed of:

* Encapsuted implementation inside a cdylib crate (DLL).
* Public Interface as a lib crate.
* Client App which uses Public Interface and loads Dll binary.

Instructions:


1) Set DLL_PATH environment variable:
   ```
   $ export DLL_PATH="your_preference_path"
   ```
2) Compile cdylib crate:
   ```
   cd counter_dynamic_lib
   cargo build --release
   ```
3) Copy .so shared lib binary to DLL path:
   ```
   cp target/release/libcounter_dynamic_lib.so $DLL_PATH
   ```
4) Build & Run Client App:
   ```
   cd app
   cargo build --release
   cargo run --release
   ```
   
