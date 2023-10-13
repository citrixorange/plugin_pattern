# plugin_pattern
The Plugin Pattern coded in Rust Language.

This pattern can be helpful for:

* Encapsulating implementation inside a isolated project compiled as a shared lib.
* No compilation need at Client Apps when new updates are released. Instead only load the new binary and play it!
* This pattern is very useful for libs which source code is private.
* This pattern is very useful for Projects which demmand software certification. On updates re-certification only applied for dynamic lib, exempting the client-app.
