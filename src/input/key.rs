#![allow(missing_docs)]
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Key {
    Key1, Key2, Key3, Key4, Key5, Key6, Key7, Key8, Key9, Key0, A, B, C, D, E, F, G, H, I, J, K, L, M, 
    N, O, P, Q, R, S, T, U, V, W, X, Y, Z, Escape, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, 
    F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24, Snapshot, Scroll, Pause, Insert, Home, Delete, End, PageDown, PageUp, Left, Up, Right, 
    Down, Back, Return, Space, Compose, Caret, Numlock, Numpad0, Numpad1, Numpad2, Numpad3, Numpad4, Numpad5, 
    Numpad6, Numpad7, Numpad8, Numpad9, AbntC1, AbntC2, Add, Apostrophe, Apps, At, Ax, Backslash, Calculator, 
    Capital, Colon, Comma, Convert, Decimal, Divide, Equals, Grave, Kana, Kanji, LAlt, LBracket, LControl, 
    LShift, LWin, Mail, MediaSelect, MediaStop, Minus, Multiply, Mute, MyComputer, NavigateForward, 
    NavigateBackward, NextTrack, NoConvert, NumpadComma, NumpadEnter, NumpadEquals, OEM102, Period, PlayPause, 
    Power, PrevTrack, RAlt, RBracket, RControl, RShift, RWin, Semicolon, Slash, Sleep, Stop, Subtract, 
    Sysrq, Tab, Underline, Unlabeled, VolumeDown, VolumeUp, Wake, WebBack, WebFavorites, WebForward, WebHome, 
    WebRefresh, WebSearch, WebStop, Yen,
}

pub(crate) const KEY_LIST: &[Key] = &[Key::Key1, Key::Key2, Key::Key3, Key::Key4, Key::Key5, Key::Key6, Key::Key7, Key::Key8, Key::Key9, Key::Key0, Key::A, Key::B, Key::C, Key::D, 
    Key::E, Key::F, Key::G, Key::H, Key::I, Key::J, Key::K, Key::L, Key::M, Key::N, Key::O, Key::P, Key::Q, Key::R, Key::S, Key::T, Key::U, Key::V, Key::W, Key::X, Key::Y, Key::Z, 
    Key::Escape, Key::F1, Key::F2, Key::F3, Key::F4, Key::F5, Key::F6, Key::F7, Key::F8, Key::F9, Key::F10, Key::F11, Key::F12, Key::F13, Key::F14, Key::F15, Key::F16, Key::F17, Key::F18,
    Key::F19, Key::F20, Key::F21, Key::F22, Key::F23, Key::F24, Key::Snapshot, Key::Scroll, Key::Pause, Key::Insert, Key::Home, Key::Delete, Key::End, Key::PageDown, Key::PageUp, Key::Left, Key::Up, Key::Right, 
    Key::Down, Key::Back, Key::Return, Key::Space, Key::Compose, Key::Caret, Key::Numlock, Key::Numpad0, Key::Numpad1, Key::Numpad2, Key::Numpad3, Key::Numpad4, Key::Numpad5, 
    Key::Numpad6, Key::Numpad7, Key::Numpad8, Key::Numpad9, Key::AbntC1, Key::AbntC2, Key::Add, Key::Apostrophe, Key::Apps, Key::At, Key::Ax, Key::Backslash, Key::Calculator, 
    Key::Capital, Key::Colon, Key::Comma, Key::Convert, Key::Decimal, Key::Divide, Key::Equals, Key::Grave, Key::Kana, Key::Kanji, Key::LAlt, Key::LBracket, Key::LControl, 
    Key::LShift, Key::LWin, Key::Mail, Key::MediaSelect, Key::MediaStop, Key::Minus, Key::Multiply, Key::Mute, Key::MyComputer, Key::NavigateForward, 
    Key::NavigateBackward, Key::NextTrack, Key::NoConvert, Key::NumpadComma, Key::NumpadEnter, Key::NumpadEquals, Key::OEM102, Key::Period, Key::PlayPause, 
    Key::Power, Key::PrevTrack, Key::RAlt, Key::RBracket, Key::RControl, Key::RShift, Key::RWin, Key::Semicolon, Key::Slash, Key::Sleep, Key::Stop, Key::Subtract, 
    Key::Sysrq, Key::Tab, Key::Underline, Key::Unlabeled, Key::VolumeDown, Key::VolumeUp, Key::Wake, Key::WebBack, Key::WebFavorites, Key::WebForward, Key::WebHome, 
    Key::WebRefresh, Key::WebSearch, Key::WebStop, Key::Yen];

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn check_key_list() {
        for i in 0..KEY_LIST.len() {
            assert_eq!(i as u32, KEY_LIST[i] as u32);
        }
    }

    #[test]
    fn key_constants_match() {
        assert_eq!(Key::Key1 as u32, winit::VirtualKeyCode::Key1 as u32);
        assert_eq!(Key::Key2 as u32, winit::VirtualKeyCode::Key2 as u32);
        assert_eq!(Key::Key3 as u32, winit::VirtualKeyCode::Key3 as u32);
        assert_eq!(Key::Key4 as u32, winit::VirtualKeyCode::Key4 as u32);
        assert_eq!(Key::Key5 as u32, winit::VirtualKeyCode::Key5 as u32);
        assert_eq!(Key::Key6 as u32, winit::VirtualKeyCode::Key6 as u32);
        assert_eq!(Key::Key7 as u32, winit::VirtualKeyCode::Key7 as u32);
        assert_eq!(Key::Key8 as u32, winit::VirtualKeyCode::Key8 as u32);
        assert_eq!(Key::Key9 as u32, winit::VirtualKeyCode::Key9 as u32);
        assert_eq!(Key::Key0 as u32, winit::VirtualKeyCode::Key0 as u32);
        assert_eq!(Key::A as u32, winit::VirtualKeyCode::A as u32);
        assert_eq!(Key::B as u32, winit::VirtualKeyCode::B as u32);
        assert_eq!(Key::C as u32, winit::VirtualKeyCode::C as u32);
        assert_eq!(Key::D as u32, winit::VirtualKeyCode::D as u32);
        assert_eq!(Key::E as u32, winit::VirtualKeyCode::E as u32);
        assert_eq!(Key::F as u32, winit::VirtualKeyCode::F as u32);
        assert_eq!(Key::G as u32, winit::VirtualKeyCode::G as u32);
        assert_eq!(Key::H as u32, winit::VirtualKeyCode::H as u32);
        assert_eq!(Key::I as u32, winit::VirtualKeyCode::I as u32);
        assert_eq!(Key::J as u32, winit::VirtualKeyCode::J as u32);
        assert_eq!(Key::K as u32, winit::VirtualKeyCode::K as u32);
        assert_eq!(Key::L as u32, winit::VirtualKeyCode::L as u32);
        assert_eq!(Key::M as u32, winit::VirtualKeyCode::M as u32);
        assert_eq!(Key::N as u32, winit::VirtualKeyCode::N as u32);
        assert_eq!(Key::O as u32, winit::VirtualKeyCode::O as u32);
        assert_eq!(Key::P as u32, winit::VirtualKeyCode::P as u32);
        assert_eq!(Key::Q as u32, winit::VirtualKeyCode::Q as u32);
        assert_eq!(Key::R as u32, winit::VirtualKeyCode::R as u32);
        assert_eq!(Key::S as u32, winit::VirtualKeyCode::S as u32);
        assert_eq!(Key::T as u32, winit::VirtualKeyCode::T as u32);
        assert_eq!(Key::U as u32, winit::VirtualKeyCode::U as u32);
        assert_eq!(Key::V as u32, winit::VirtualKeyCode::V as u32);
        assert_eq!(Key::W as u32, winit::VirtualKeyCode::W as u32);
        assert_eq!(Key::X as u32, winit::VirtualKeyCode::X as u32);
        assert_eq!(Key::Y as u32, winit::VirtualKeyCode::Y as u32);
        assert_eq!(Key::Z as u32, winit::VirtualKeyCode::Z as u32);
        assert_eq!(Key::Escape as u32, winit::VirtualKeyCode::Escape as u32);
        assert_eq!(Key::F1 as u32, winit::VirtualKeyCode::F1 as u32);
        assert_eq!(Key::F2 as u32, winit::VirtualKeyCode::F2 as u32);
        assert_eq!(Key::F3 as u32, winit::VirtualKeyCode::F3 as u32);
        assert_eq!(Key::F4 as u32, winit::VirtualKeyCode::F4 as u32);
        assert_eq!(Key::F5 as u32, winit::VirtualKeyCode::F5 as u32);
        assert_eq!(Key::F6 as u32, winit::VirtualKeyCode::F6 as u32);
        assert_eq!(Key::F7 as u32, winit::VirtualKeyCode::F7 as u32);
        assert_eq!(Key::F8 as u32, winit::VirtualKeyCode::F8 as u32);
        assert_eq!(Key::F9 as u32, winit::VirtualKeyCode::F9 as u32);
        assert_eq!(Key::F10 as u32, winit::VirtualKeyCode::F10 as u32);
        assert_eq!(Key::F11 as u32, winit::VirtualKeyCode::F11 as u32);
        assert_eq!(Key::F12 as u32, winit::VirtualKeyCode::F12 as u32);
        assert_eq!(Key::F13 as u32, winit::VirtualKeyCode::F13 as u32);
        assert_eq!(Key::F14 as u32, winit::VirtualKeyCode::F14 as u32);
        assert_eq!(Key::F15 as u32, winit::VirtualKeyCode::F15 as u32);
        assert_eq!(Key::F16 as u32, winit::VirtualKeyCode::F16 as u32);
        assert_eq!(Key::F17 as u32, winit::VirtualKeyCode::F17 as u32);
        assert_eq!(Key::F18 as u32, winit::VirtualKeyCode::F18 as u32);
        assert_eq!(Key::F19 as u32, winit::VirtualKeyCode::F19 as u32);
        assert_eq!(Key::F20 as u32, winit::VirtualKeyCode::F20 as u32);
        assert_eq!(Key::F21 as u32, winit::VirtualKeyCode::F21 as u32);
        assert_eq!(Key::F22 as u32, winit::VirtualKeyCode::F22 as u32);
        assert_eq!(Key::F23 as u32, winit::VirtualKeyCode::F23 as u32);
        assert_eq!(Key::F24 as u32, winit::VirtualKeyCode::F24 as u32);
        assert_eq!(Key::Snapshot as u32, winit::VirtualKeyCode::Snapshot as u32);
        assert_eq!(Key::Scroll as u32, winit::VirtualKeyCode::Scroll as u32);
        assert_eq!(Key::Pause as u32, winit::VirtualKeyCode::Pause as u32);
        assert_eq!(Key::Insert as u32, winit::VirtualKeyCode::Insert as u32);
        assert_eq!(Key::Home as u32, winit::VirtualKeyCode::Home as u32);
        assert_eq!(Key::Delete as u32, winit::VirtualKeyCode::Delete as u32);
        assert_eq!(Key::End as u32, winit::VirtualKeyCode::End as u32);
        assert_eq!(Key::PageDown as u32, winit::VirtualKeyCode::PageDown as u32);
        assert_eq!(Key::PageUp as u32, winit::VirtualKeyCode::PageUp as u32);
        assert_eq!(Key::Left as u32, winit::VirtualKeyCode::Left as u32);
        assert_eq!(Key::Up as u32, winit::VirtualKeyCode::Up as u32);
        assert_eq!(Key::Right as u32, winit::VirtualKeyCode::Right as u32);
        assert_eq!(Key::Down as u32, winit::VirtualKeyCode::Down as u32);
        assert_eq!(Key::Back as u32, winit::VirtualKeyCode::Back as u32);
        assert_eq!(Key::Return as u32, winit::VirtualKeyCode::Return as u32);
        assert_eq!(Key::Space as u32, winit::VirtualKeyCode::Space as u32);
        assert_eq!(Key::Compose as u32, winit::VirtualKeyCode::Compose as u32);
        assert_eq!(Key::Caret as u32, winit::VirtualKeyCode::Caret as u32);
        assert_eq!(Key::Numlock as u32, winit::VirtualKeyCode::Numlock as u32);
        assert_eq!(Key::Numpad0 as u32, winit::VirtualKeyCode::Numpad0 as u32);
        assert_eq!(Key::Numpad1 as u32, winit::VirtualKeyCode::Numpad1 as u32);
        assert_eq!(Key::Numpad2 as u32, winit::VirtualKeyCode::Numpad2 as u32);
        assert_eq!(Key::Numpad3 as u32, winit::VirtualKeyCode::Numpad3 as u32);
        assert_eq!(Key::Numpad4 as u32, winit::VirtualKeyCode::Numpad4 as u32);
        assert_eq!(Key::Numpad5 as u32, winit::VirtualKeyCode::Numpad5 as u32);
        assert_eq!(Key::Numpad6 as u32, winit::VirtualKeyCode::Numpad6 as u32);
        assert_eq!(Key::Numpad7 as u32, winit::VirtualKeyCode::Numpad7 as u32);
        assert_eq!(Key::Numpad8 as u32, winit::VirtualKeyCode::Numpad8 as u32);
        assert_eq!(Key::Numpad9 as u32, winit::VirtualKeyCode::Numpad9 as u32);
        assert_eq!(Key::AbntC1 as u32, winit::VirtualKeyCode::AbntC1 as u32);
        assert_eq!(Key::AbntC2 as u32, winit::VirtualKeyCode::AbntC2 as u32);
        assert_eq!(Key::Add as u32, winit::VirtualKeyCode::Add as u32);
        assert_eq!(Key::Apostrophe as u32, winit::VirtualKeyCode::Apostrophe as u32);
        assert_eq!(Key::Apps as u32, winit::VirtualKeyCode::Apps as u32);
        assert_eq!(Key::At as u32, winit::VirtualKeyCode::At as u32);
        assert_eq!(Key::Ax as u32, winit::VirtualKeyCode::Ax as u32);
        assert_eq!(Key::Backslash as u32, winit::VirtualKeyCode::Backslash as u32);
        assert_eq!(Key::Calculator as u32, winit::VirtualKeyCode::Calculator as u32);
        assert_eq!(Key::Capital as u32, winit::VirtualKeyCode::Capital as u32);
        assert_eq!(Key::Colon as u32, winit::VirtualKeyCode::Colon as u32);
        assert_eq!(Key::Comma as u32, winit::VirtualKeyCode::Comma as u32);
        assert_eq!(Key::Convert as u32, winit::VirtualKeyCode::Convert as u32);
        assert_eq!(Key::Decimal as u32, winit::VirtualKeyCode::Decimal as u32);
        assert_eq!(Key::Divide as u32, winit::VirtualKeyCode::Divide as u32);
        assert_eq!(Key::Equals as u32, winit::VirtualKeyCode::Equals as u32);
        assert_eq!(Key::Grave as u32, winit::VirtualKeyCode::Grave as u32);
        assert_eq!(Key::Kana as u32, winit::VirtualKeyCode::Kana as u32);
        assert_eq!(Key::Kanji as u32, winit::VirtualKeyCode::Kanji as u32);
        assert_eq!(Key::LAlt as u32, winit::VirtualKeyCode::LAlt as u32);
        assert_eq!(Key::LBracket as u32, winit::VirtualKeyCode::LBracket as u32);
        assert_eq!(Key::LControl as u32, winit::VirtualKeyCode::LControl as u32);
        assert_eq!(Key::LShift as u32, winit::VirtualKeyCode::LShift as u32);
        assert_eq!(Key::LWin as u32, winit::VirtualKeyCode::LWin as u32);
        assert_eq!(Key::Mail as u32, winit::VirtualKeyCode::Mail as u32);
        assert_eq!(Key::MediaSelect as u32, winit::VirtualKeyCode::MediaSelect as u32);
        assert_eq!(Key::MediaStop as u32, winit::VirtualKeyCode::MediaStop as u32);
        assert_eq!(Key::Minus as u32, winit::VirtualKeyCode::Minus as u32);
        assert_eq!(Key::Multiply as u32, winit::VirtualKeyCode::Multiply as u32);
        assert_eq!(Key::Mute as u32, winit::VirtualKeyCode::Mute as u32);
        assert_eq!(Key::MyComputer as u32, winit::VirtualKeyCode::MyComputer as u32);
        assert_eq!(Key::NavigateForward as u32, winit::VirtualKeyCode::NavigateForward as u32);
        assert_eq!(Key::NavigateBackward as u32, winit::VirtualKeyCode::NavigateBackward as u32);
        assert_eq!(Key::NextTrack as u32, winit::VirtualKeyCode::NextTrack as u32);
        assert_eq!(Key::NoConvert as u32, winit::VirtualKeyCode::NoConvert as u32);
        assert_eq!(Key::NumpadComma as u32, winit::VirtualKeyCode::NumpadComma as u32);
        assert_eq!(Key::NumpadEnter as u32, winit::VirtualKeyCode::NumpadEnter as u32);
        assert_eq!(Key::NumpadEquals as u32, winit::VirtualKeyCode::NumpadEquals as u32);
        assert_eq!(Key::OEM102 as u32, winit::VirtualKeyCode::OEM102 as u32);
        assert_eq!(Key::Period as u32, winit::VirtualKeyCode::Period as u32);
        assert_eq!(Key::PlayPause as u32, winit::VirtualKeyCode::PlayPause as u32);
        assert_eq!(Key::Power as u32, winit::VirtualKeyCode::Power as u32);
        assert_eq!(Key::PrevTrack as u32, winit::VirtualKeyCode::PrevTrack as u32);
        assert_eq!(Key::RAlt as u32, winit::VirtualKeyCode::RAlt as u32);
        assert_eq!(Key::RBracket as u32, winit::VirtualKeyCode::RBracket as u32);
        assert_eq!(Key::RControl as u32, winit::VirtualKeyCode::RControl as u32);
        assert_eq!(Key::RShift as u32, winit::VirtualKeyCode::RShift as u32);
        assert_eq!(Key::RWin as u32, winit::VirtualKeyCode::RWin as u32);
        assert_eq!(Key::Semicolon as u32, winit::VirtualKeyCode::Semicolon as u32);
        assert_eq!(Key::Slash as u32, winit::VirtualKeyCode::Slash as u32);
        assert_eq!(Key::Sleep as u32, winit::VirtualKeyCode::Sleep as u32);
        assert_eq!(Key::Stop as u32, winit::VirtualKeyCode::Stop as u32);
        assert_eq!(Key::Subtract as u32, winit::VirtualKeyCode::Subtract as u32);
        assert_eq!(Key::Sysrq as u32, winit::VirtualKeyCode::Sysrq as u32);
        assert_eq!(Key::Tab as u32, winit::VirtualKeyCode::Tab as u32);
        assert_eq!(Key::Underline as u32, winit::VirtualKeyCode::Underline as u32);
        assert_eq!(Key::Unlabeled as u32, winit::VirtualKeyCode::Unlabeled as u32);
        assert_eq!(Key::VolumeDown as u32, winit::VirtualKeyCode::VolumeDown as u32);
        assert_eq!(Key::VolumeUp as u32, winit::VirtualKeyCode::VolumeUp as u32);
        assert_eq!(Key::Wake as u32, winit::VirtualKeyCode::Wake as u32);
        assert_eq!(Key::WebBack as u32, winit::VirtualKeyCode::WebBack as u32);
        assert_eq!(Key::WebFavorites as u32, winit::VirtualKeyCode::WebFavorites as u32);
        assert_eq!(Key::WebForward as u32, winit::VirtualKeyCode::WebForward as u32);
        assert_eq!(Key::WebHome as u32, winit::VirtualKeyCode::WebHome as u32);
        assert_eq!(Key::WebRefresh as u32, winit::VirtualKeyCode::WebRefresh as u32);
        assert_eq!(Key::WebSearch as u32, winit::VirtualKeyCode::WebSearch as u32);
        assert_eq!(Key::WebStop as u32, winit::VirtualKeyCode::WebStop as u32);
        assert_eq!(Key::Yen as u32, winit::VirtualKeyCode::Yen as u32);
    }
}
