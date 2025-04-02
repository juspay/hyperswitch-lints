# Hyperswitch Lints
Custom lints for Hyperswitch using the [dylint](https://github.com/trailofbits/dylint) crate.

## List of Lints
Each individual lint is implemented as its own crate with path `{lint_family_name}/{lint_name}`
where `{lint_name}` is the lint crate and `{lint_family_name}/` is the lint family workspace.
Arranging lints into lint families allows us to be able to either build each lint individually
(to test a single lint) or to build an entire lint family with all constituent lints (to run
all lints, either locally or in CI).

Following are the currently existing lint families and lints
- `use_common_utils/use_common_serde_utils`

### Example: running the `use_common_serde_utils` lint
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
   cargo dylint --path path/to/this/repo --pattern use_common_utils/use_common_serde_utils [-p <specific_crate> [-- --features <specific_features>]]
   ```
5. For example
   ```bash
   cargo dylint --path path/to/this/repo --pattern use_common_utils/use_common_serde_utils -p api_models -- --features v1
   ```
6. The lint will catch all places where the `serde_json` (de)serialization functions are being used raw.

