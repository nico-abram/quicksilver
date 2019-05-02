use winit::window::MouseCursor as WinitMouseCursor;

/// Mouse cursor styles
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum MouseCursor {
    /// No cursor
    None,

    /// Default cursor
    Default,
    /// Crosshair cursor
    Crosshair,
    /// Hand cursor
    Hand,
    /// Arrow cursor
    Arrow,
    /// Move cursor
    Move,
    /// Text cursor
    Text,
    /// Wait cursor
    Wait,
    /// Help cursor
    Help,
    /// Progress cursor
    Progress,

    /// NotAllowed cursor
    NotAllowed,
    /// ContextMenu cursor
    ContextMenu,
    /// Cell cursor
    Cell,
    /// VerticalText cursor
    VerticalText,
    /// Alias cursor
    Alias,
    /// Copy cursor
    Copy,
    /// NoDrop cursor
    NoDrop,
    /// Grab cursor
    Grab,
    /// Grabbing cursor
    Grabbing,
    /// AllScroll cursor
    AllScroll,
    /// ZoomIn cursor
    ZoomIn,
    /// ZoomOut cursor
    ZoomOut,

    /// EResize cursor
    EResize,
    /// NResize cursor
    NResize,
    /// NeResize cursor
    NeResize,
    /// NwResize cursor
    NwResize,
    /// SResize cursor
    SResize,
    /// SeResize cursor
    SeResize,
    /// SwResize cursor
    SwResize,
    /// WResize cursor
    WResize,
    /// EwResize cursor
    EwResize,
    /// NsResize cursor
    NsResize,
    /// NeswResize cursor
    NeswResize,
    /// NwseResize cursor
    NwseResize,
    /// ColResize cursor
    ColResize,
    /// RowResize cursor
    RowResize,
}

impl MouseCursor {
    #[cfg(target_arch = "wasm32")]
    #[inline]
    pub(crate) fn into_css_style(self) -> &'static str {
        match self {
            MouseCursor::None => "none",
            MouseCursor::Default => "auto",
            MouseCursor::Crosshair => "crosshair",
            MouseCursor::Hand => "pointer",
            MouseCursor::Arrow => "default",
            MouseCursor::Move => "move",
            MouseCursor::Text => "text",
            MouseCursor::Wait => "wait",
            MouseCursor::Help => "help",
            MouseCursor::Progress => "progress",

            MouseCursor::NotAllowed => "not-allowed",
            MouseCursor::ContextMenu => "context-menu",
            MouseCursor::Cell => "cell",
            MouseCursor::VerticalText => "vertical-text",
            MouseCursor::Alias => "alias",
            MouseCursor::Copy => "copy",
            MouseCursor::NoDrop => "no-drop",
            MouseCursor::Grab => "grab",
            MouseCursor::Grabbing => "grabbing",
            MouseCursor::AllScroll => "all-scroll",
            MouseCursor::ZoomIn => "zoom-in",
            MouseCursor::ZoomOut => "zoom-out",

            MouseCursor::EResize => "e-resize",
            MouseCursor::NResize => "n-resize",
            MouseCursor::NeResize => "ne-resize",
            MouseCursor::NwResize => "nw-resize",
            MouseCursor::SResize => "s-resize",
            MouseCursor::SeResize => "se-resize",
            MouseCursor::SwResize => "sw-resize",
            MouseCursor::WResize => "w-resize",
            MouseCursor::EwResize => "ew-resize",
            MouseCursor::NsResize => "ns-resize",
            MouseCursor::NeswResize => "nesw-resize",
            MouseCursor::NwseResize => "nwse-resize",
            MouseCursor::ColResize => "col-resize",
            MouseCursor::RowResize => "row-resize",
        }
    }

    #[inline]
    pub(crate) fn into_gl_cursor(self) -> Option<WinitMouseCursor> {
        match self {
            MouseCursor::None => None,
            MouseCursor::Default => Some(WinitMouseCursor::Default),
            MouseCursor::Crosshair => Some(WinitMouseCursor::Crosshair),
            MouseCursor::Hand => Some(WinitMouseCursor::Hand),
            MouseCursor::Arrow => Some(WinitMouseCursor::Arrow),
            MouseCursor::Move => Some(WinitMouseCursor::Move),
            MouseCursor::Text => Some(WinitMouseCursor::Text),
            MouseCursor::Wait => Some(WinitMouseCursor::Wait),
            MouseCursor::Help => Some(WinitMouseCursor::Help),
            MouseCursor::Progress => Some(WinitMouseCursor::Progress),

            MouseCursor::NotAllowed => Some(WinitMouseCursor::NotAllowed),
            MouseCursor::ContextMenu => Some(WinitMouseCursor::ContextMenu),
            MouseCursor::Cell => Some(WinitMouseCursor::Cell),
            MouseCursor::VerticalText => Some(WinitMouseCursor::VerticalText),
            MouseCursor::Alias => Some(WinitMouseCursor::Alias),
            MouseCursor::Copy => Some(WinitMouseCursor::Copy),
            MouseCursor::NoDrop => Some(WinitMouseCursor::NoDrop),
            MouseCursor::Grab => Some(WinitMouseCursor::Grab),
            MouseCursor::Grabbing => Some(WinitMouseCursor::Grabbing),
            MouseCursor::AllScroll => Some(WinitMouseCursor::AllScroll),
            MouseCursor::ZoomIn => Some(WinitMouseCursor::ZoomIn),
            MouseCursor::ZoomOut => Some(WinitMouseCursor::ZoomOut),

            MouseCursor::EResize => Some(WinitMouseCursor::EResize),
            MouseCursor::NResize => Some(WinitMouseCursor::NResize),
            MouseCursor::NeResize => Some(WinitMouseCursor::NeResize),
            MouseCursor::NwResize => Some(WinitMouseCursor::NwResize),
            MouseCursor::SResize => Some(WinitMouseCursor::SResize),
            MouseCursor::SeResize => Some(WinitMouseCursor::SeResize),
            MouseCursor::SwResize => Some(WinitMouseCursor::SwResize),
            MouseCursor::WResize => Some(WinitMouseCursor::WResize),
            MouseCursor::EwResize => Some(WinitMouseCursor::EwResize),
            MouseCursor::NsResize => Some(WinitMouseCursor::NsResize),
            MouseCursor::NeswResize => Some(WinitMouseCursor::NeswResize),
            MouseCursor::NwseResize => Some(WinitMouseCursor::NwseResize),
            MouseCursor::ColResize => Some(WinitMouseCursor::ColResize),
            MouseCursor::RowResize => Some(WinitMouseCursor::RowResize),
        }
    }
}

impl Default for MouseCursor {
    fn default() -> Self { MouseCursor::Default }
}
