# Hyperswitch Lints
Example repository to demonstrate how to implement custom lints for Hyperswitch using the [dylint](https://github.com/trailofbits/dylint) crate.

## Running the `use_common_serde_utils`
To run the example lint on Hyperswitch
1. Ensure you have the nightly toolchain installed
   ```bash
   rustup toolchain install nightly
   ```
2. Install `cargo-dylint` and `dylint-link`
   ```bash
   cargo install cargo-dylint dylint-link
   ```
3. Clone this repository
4. Go to the Hyperswitch root and run the following command
   ```bash
   cargo dylint --path path/to/this/repo --pattern lints/use_common_serde_utils [-p <specific_crate> [-- --features <specific_features>]]
   ```
5. For example
   ```bash
   cargo dylint --path path/to/this/repo --pattern lints/use_common_serde_utils -p api_models -- --features v1
   ```
6. The lint will catch all places where the `serde_json` (de)serialization functions are being used raw.

