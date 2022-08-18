# Browser url open repro
Opens url with iOS after 2 seconds of running.

To run:
1. Install [`cargo-mobile`](https://github.com/BrainiumLLC/cargo-mobile)
2. Change dev team from `mobile.toml` (get it from the redirect url of https://developer.apple.com/account/#/membership)
3. Run `cargo mobile init`.
4. Attach iOS device to your mac
5. Run `cargo apple run`

Cargo mobile will generate the required xcode project and run it on your device.