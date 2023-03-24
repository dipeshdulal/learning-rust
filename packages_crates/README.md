### Packages and Crates
- Crates are smallest amount of code that Rust compiler considers at a time. Binary crates and Library Crates are available.
- All crates created previoulsy are binary crates.
- Library crates doesn't have main functions. 
- Package is a bundle of more than one crates that provides set of functionality.

### Cargo

- Cargo follows convention;
  - `src/main.rs` is the root of the binary crate.
  - `src/lib.rs` is the root of the library crate.
- Cargo passes these root files to `rustc` to build library or binary.
- If package contains both `src/main.rs` and `src/lib.rs` it has two crates one library and another binary.
- Package can have multiple binary crates, we can place the files in `src/bin` directory: each file will be a separate binary crate.

```rs
// absolute path
crate::front_of_house::hosting::add_to_waitlist();

// relative path
front_of_house::hosting::add_to_waitlist();
```

### Struct and Enums
```rs
mod my_module {

  pub struct MyStruct {
    pub data: i32; // explicitly set the values as public otherwise not accesible
    secret_value: str; 
  }

  pub enum Variations { // all the variations are public by default 
    Red,
    Blue,
    Green
  }
}
```