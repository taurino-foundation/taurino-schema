use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::path::PathBuf;

use crate::Theme;

/// An event from a window.
#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub enum WindowEvent {
    /// The size of the window has changed. Contains the client area's new dimensions.
    Resized(dpi::PhysicalSize<u32>),
    /// The position of the window has changed. Contains the window's new position.
    Moved(dpi::PhysicalPosition<i32>),
    /// The window has been requested to close.
    CloseRequested { window_label: String },
    /// The window has been destroyed.
    Destroyed,
    /// The window gained or lost focus.
    ///
    /// The parameter is true if the window has gained focus, and false if it has lost focus.
    Focused(bool),
    /// The window's scale factor has changed.
    ///
    /// The following user actions can cause DPI changes:
    ///
    /// - Changing the display's resolution.
    /// - Changing the display's scale factor (e.g. in Control Panel on Windows).
    /// - Moving the window to a display with a different scale factor.
    ScaleFactorChanged {
        /// The new scale factor.
        scale_factor: f64,
        /// The window inner size.
        new_inner_size: dpi::PhysicalSize<u32>,
    },
    /// An event associated with the drag and drop action.
    DragDrop(DragDropEvent),
    /// The system window theme has changed.
    ///
    /// Applications might wish to react to this to change the theme of the content of the window when the system changes the window theme.
    ThemeChanged(Theme),
    /*
    /// Emitted when the application has been suspended.
    ///
    /// ## Platform-specific
    ///
    /// - **Android**: This is triggered by `onPause` method of the Activity.
    /// - **iOS**: This is triggered by `applicationWillResignActive` method of the UIApplicationDelegate.
    /// - **Linux / macOS / Windows**: Unsupported.
    #[cfg(mobile)]
    #[cfg_attr(docsrs, doc(cfg(any(target_os = "android", target_os = "ios"))))]
    Suspended,
    */

    /*
    /// Emitted when the application has been resumed.
    ///
    /// ## Platform-specific
    ///
    /// - **Android**: This is triggered by `onResume` method of the Activity. The first onResume() is ignored to match the iOS implementation, since that is called on activity creation.
    /// - **iOS**: This is triggered by `applicationWillEnterForeground` method of the UIApplicationDelegate.
    /// - **Linux / macOS / Windows**: Unsupported.
    #[cfg(mobile)]
    #[cfg_attr(docsrs, doc(cfg(any(target_os = "android", target_os = "ios"))))]
    Resumed,
    */
}

/// An event from a window.
#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub enum WebviewEvent {
    /// An event associated with the drag and drop action.
    DragDrop(DragDropEvent),
}

/// The drag drop event payload.
#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[non_exhaustive]
pub enum DragDropEvent {
    /// A drag operation has entered the webview.
    Enter {
        /// List of paths that are being dragged onto the webview.
        paths: Vec<PathBuf>,
        /// The position of the mouse cursor.
        position: dpi::PhysicalPosition<f64>,
    },
    /// A drag operation is moving over the webview.
    Over {
        /// The position of the mouse cursor.
        position: dpi::PhysicalPosition<f64>,
    },
    /// The file(s) have been dropped onto the webview.
    Drop {
        /// List of paths that are being dropped onto the window.
        paths: Vec<PathBuf>,
        /// The position of the mouse cursor.
        position: dpi::PhysicalPosition<f64>,
    },
    /// The drag operation has been cancelled or left the window.
    Leave,
}
