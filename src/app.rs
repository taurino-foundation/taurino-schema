// ============================================================================
// Imports
// ============================================================================
//
// std::fmt::{self, Display, Formatter}
//   - `Display` and `Formatter` are needed to implement the Display trait
//     for `FrontendDist`.
//   - `self` imports the `fmt` module itself so we can refer to `fmt::Result`.
//
// std::path::PathBuf
//   - Owned, heap-allocated filesystem path.
//   - Used for the `Directory` and `Files` variants of `FrontendDist`.
//
// serde::{Deserialize, Serialize}
//   - Derive macros to read from / write to config formats (JSON, TOML, ...).
//
// url::Url
//   - Parsed URL type, used by `dev_url` and `FrontendDist::Url`.
//
// crate::window::WindowConfig
//   - Per-window configuration, defined elsewhere in this crate.
//
use std::{
    fmt::{self, Display, Formatter},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};
use url::Url;

use crate::window::WindowConfig;

// ============================================================================
// ConnecionConfig
// ============================================================================
//
// Small struct holding the IPC endpoint path.
//
// - Only `Deserialize` is derived: this is meant to be read from a config
//   file, not written back out.
// - NOTE: the name is misspelled (`ConnecionConfig` instead of
//   `ConnectionConfig`). This is a real identifier and must either be
//   renamed consistently everywhere or kept as-is.
//
/// Settings for inter-process communication.
#[derive(Debug, Clone, Deserialize)]
pub struct ConnecionConfig {
    /// Path of the IPC endpoint.
    pub path: String,
}

// ============================================================================
// AppConfig
// ============================================================================
//
// Top-level application configuration. All fields are read from the config
// file via serde.
//
// Field-by-field notes:
//
// product_name:
//   - Optional.
//   - `#[serde(alias = "product-name")]` allows either `product_name` or
//     `product-name` as the key in the config file.
//   - Human-readable name used for system display. May also influence
//     package names, install paths, and metadata.
//
// version:
//   - Optional.
//   - Uses a custom deserializer `crate::package::version_deserializer`.
//   - Accepts either a SemVer string OR a path to a `package.json`, from
//     which the `version` field is read.
//   - `default` means the field may be missing entirely.
//
// connection:
//   - Required (not Option, no default).
//   - Holds IPC connection settings (see ConnecionConfig above).
//
// windows:
//   - Required list of window definitions.
//   - Each entry is a WindowConfig describing one application window.
//
// dev_url:
//   - Optional.
//   - `#[serde(alias = "dev-url")]` allows `dev_url` or `dev-url`.
//   - URL of the frontend dev server during development.
//
// frontend_dist:
//   - Optional.
//   - `#[serde(alias = "frontend-dist")]` allows `frontend_dist` or
//     `frontend-dist`.
//   - Describes where the frontend content comes from (URL, directory, or
//     explicit file list). See FrontendDist below.
//   - Relative directory paths are resolved relative to the config file.
//
// identifier:
//   - Required.
//   - Reverse-DNS application identifier, e.g. `com.example.desktop`.
//   - Used to associate app-specific system data and paths.
//   - Intended characters: ASCII letters, digits, hyphens, and dots.
//   - NOTE: the doc says "intended", but the type does NOT enforce this.
//     Validation must happen elsewhere.
//
/// General configuration of the application.
#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    /// Visible name of the application.
    ///
    /// This name is used for display in the system and can
    /// also influence package names, install paths, and metadata.
    #[serde(alias = "product-name")]
    pub product_name: Option<String>,

    /// Version of the application.
    ///
    /// Supports a version number in SemVer format or a path to
    /// a `package.json` whose `version` entry should be read.
    /// Processing is handled by the project-specific version deserializer.
    #[serde(deserialize_with = "crate::package::version_deserializer", default)]
    pub version: Option<String>,

    /// Configuration of the IPC connection.
    pub connection: ConnecionConfig,

    /// Settings of the application windows.
    pub windows: Vec<WindowConfig>,

    /// Address of the user interface during development.
    ///
    /// Usually this URL points to a development server
    /// that serves the frontend files and delivers changes directly.
    #[serde(alias = "dev-url")]
    pub dev_url: Option<Url>,

    /// Source of the frontend content.
    ///
    /// Possible values are a URL, a local directory, or individual file paths.
    /// A relative directory path is resolved starting from the configuration
    /// file.
    ///
    /// For a directory, its contents are embedded recursively;
    /// `index.html` serves as the entry point. A file list allows
    /// targeted selection of the resources to embed.
    ///
    /// If a URL is specified, the application loads its content from that
    /// address instead of embedding local frontend files for it.
    #[serde(alias = "frontend-dist")]
    pub frontend_dist: Option<FrontendDist>,

    /// Unique application identifier in reverse domain notation,
    /// for example `com.example.desktop`.
    ///
    /// It is used to associate application-specific system data and paths.
    /// Intended are ASCII letters, digits, hyphens, and dots.
    pub identifier: String,
}

// ============================================================================
// FrontendDist
// ============================================================================
//
// Enum describing how the frontend is delivered. Three variants:
//
//   Url(Url)          -> load the UI from a URL, no local files embedded
//   Directory(PathBuf)-> a directory of frontend files, embedded recursively,
//                        with `index.html` as the entry point
//   Files(Vec<PathBuf>)-> an explicit list of individual files to embed
//
// Serde attributes:
//
//   `untagged`
//     The config value is NOT tagged with the variant name. Serde tries each
//     variant in order and picks the first that matches. Examples:
//         "https://example.com"       -> Url
//         "./dist"                    -> Directory
//         ["./a.html", "./b.js"]      -> Files
//
//   `deny_unknown_fields`
//     Unknown keys cause an error. Mainly relevant for struct-like variants;
//     for these newtype variants it has limited impact.
//
// Derives:
//
//   Debug, PartialEq, Eq, Clone
//     Standard utility traits. `Eq` is valid here because `Url`, `PathBuf`
//     and `Vec<PathBuf>` all implement `Eq`.
//
//   Deserialize AND Serialize
//     Unlike the config structs above, this type is also serializable,
//     because it may be written back out (e.g. by tooling or for display).
//
//   #[non_exhaustive]
//     Outside this crate, any `match` on this enum must include a wildcard
//     arm. This lets us add new variants later without a breaking change.
//
/// Describes where the user interface is loaded from.
#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
#[serde(untagged, deny_unknown_fields)]
#[non_exhaustive]
pub enum FrontendDist {
    /// URL of the user interface; no local files are embedded for it.
    Url(Url),

    /// Directory containing the frontend files to be served.
    Directory(PathBuf),

    /// Individual files that should be embedded into the application.
    Files(Vec<PathBuf>),
}

// ============================================================================
// impl Display for FrontendDist
// ============================================================================
//
// Provides a human-readable (and for `Files`, JSON) representation.
//
//   Url        -> prints the URL as-is.
//   Directory  -> prints the path via `Path::display()` (lossy, readable).
//   Files      -> serializes the path list to JSON and prints that.
//
// Notes / potential issues to review:
//
//   1. The `Files` arm uses `.unwrap()` on `serde_json::to_string(paths)`.
//      Serializing `Vec<PathBuf>` should not fail in practice, but
//      `unwrap()` inside a `Display` impl can panic during formatting.
//      Safer: fall back to `Debug` formatting or return `fmt::Error`.
//
//   2. `Display` for `Url` and `Directory` is human-oriented, while `Files`
//      produces JSON. That inconsistency may be intentional (for logging)
//      or may be worth reconsidering.
//
//   3. Because `FrontendDist` is both `Serialize` and `Display`, there are
//      two string representations. Make sure callers use the intended one.
//
impl Display for FrontendDist {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Url(address) => write!(formatter, "{address}"),
            Self::Directory(directory) => {
                write!(formatter, "{}", directory.display())
            }
            Self::Files(paths) => {
                write!(formatter, "{}", serde_json::to_string(paths).unwrap())
            }
        }
    }
}

// ============================================================================
// Build time vs. runtime (context)
// ============================================================================
//
// These structs are `Deserialize` targets. They are populated when the
// config file is parsed — either by the build tooling or by the app itself.
//
//   - Fields like `product_name`, `identifier`, `frontend_dist` and
//     `windows` are typically consumed at BUILD/BUNDLE time by the CLI
//     (naming the binary, bundling resources, generating metadata) and
//     also possibly at RUNTIME by the app (setting up windows, loading
//     the frontend).
//
//   - `dev_url` is used at RUNTIME in development mode.
//
//   - The `Display` impl on `FrontendDist` is a runtime utility.
//
// So unlike `main_binary_name` (purely build time), parts of this file are
// used at both build time and runtime, depending on the field and consumer.
//
// ============================================================================
// Summary of potential issues to review
// ============================================================================
//
//   1. `ConnecionConfig` is misspelled (`Connection`).
//
//   2. `serde_json::to_string(paths).unwrap()` inside `Display` can panic —
//      consider handling the error.
//
//   3. `identifier` is documented as restricted to certain characters but
//      is not validated by the type. Validation must exist elsewhere.
//
//   4. `deny_unknown_fields` combined with `untagged` can produce confusing
//      error messages — worth testing how bad config input is reported.
//
//   5. `#[non_exhaustive]` on `FrontendDist` means external crates must
//      handle a wildcard arm; make sure downstream matches do that.
//
