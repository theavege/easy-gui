# easytk

A small cross-platform component and model layer for FLTK on Windows and GTK 3 on Linux.

The Cargo package and Rust crate are both named `easytk`.

## Requirements

- Rust 1.85 or newer (edition 2024).
- Windows: MSVC Rust target and a Visual Studio C++ toolchain.
- Linux: GTK 3 development files and `pkg-config`.

Ubuntu/Debian: `sudo apt install libgtk-3-dev pkg-config`

Fedora: `sudo dnf install gtk3-devel pkgconf-pkg-config`

## Build the library

```sh
cargo check --all-targets
```

On Windows, select the MSVC target when needed:

```powershell
rustup target add x86_64-pc-windows-msvc
cargo check --target x86_64-pc-windows-msvc --all-targets
```

## Run the demo

The `simple` example includes network-backed request and translation demos. Its optional `reqwest` feature enables those dependencies; the example is not built unless that feature is selected.

```sh
cargo run --example simple --features reqwest
```

On Windows:

```powershell
cargo run --example simple --features reqwest --target x86_64-pc-windows-msvc
```

## Component API

Import the platform API with `use easytk::prelude::*;`.

On Windows, implement `Component` for a type that is `Default + Clone + 'static`; its associated `State` must be `Default + 'static`. The view is polled on FLTK's event loop. `Settings` supports fullscreen, initial and minimum sizes, font settings, window class, and an icon. An unavailable `font_name` falls back to Cascadia Mono when installed, then Courier Bold. File chooser dialogs start in the user profile directory when available.

On Linux, implement `Component` for a `Default + 'static` type. Its `Event` must also be `Send + 'static` to cross worker-thread boundaries. Events are dispatched on GTK's main context, `handle` returns `true` when the view needs refreshing, and the event source is removed when the root widget is destroyed. `Settings` supports initial size and application ID; the window title uses that ID.

The platform APIs intentionally expose native widgets and have some differences: Windows views use mutable component instances and return any FLTK widget, while Linux views use shared references and return a `gtk::Box`. Windows `Settings` has more appearance controls, and the Linux `Update` implementation currently covers `gtk::Entry`; the Windows prelude provides update/config helpers for several FLTK widgets.

See [examples/simple.rs](examples/simple.rs) for the cross-platform demo. CI checks the library and feature-enabled example on Windows and Linux.
