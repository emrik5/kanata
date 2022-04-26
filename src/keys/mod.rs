//! Platform specific code for OS key code mappings.

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use linux::*;

#[cfg(target_os = "windows")]
#[allow(dead_code)]
mod windows;
#[cfg(target_os = "windows")]
pub use windows::*;

/// Convert a `&str` to an `OsCode`.
///
/// kmonad's str to key mapping is found here as a reference:
/// https://github.com/kmonad/kmonad/blob/master/src/KMonad/Keyboard/Keycode.hs
///
/// Do your best to keep the str side a maximum character length of 4 so that configuration file
/// can stay clean.
pub fn str_to_oscode(s: &str) -> Option<OsCode> {
    Some(match s {
        "grv" => OsCode::KEY_GRAVE,
        "1" => OsCode::KEY_1,
        "2" => OsCode::KEY_2,
        "3" => OsCode::KEY_3,
        "4" => OsCode::KEY_4,
        "5" => OsCode::KEY_5,
        "6" => OsCode::KEY_6,
        "7" => OsCode::KEY_7,
        "8" => OsCode::KEY_8,
        "9" => OsCode::KEY_9,
        "0" => OsCode::KEY_0,
        "+" => OsCode::KEY_KPPLUS,
        "-" => OsCode::KEY_MINUS,
        "=" => OsCode::KEY_EQUAL,
        "bspc" => OsCode::KEY_BACKSPACE,
        "tab" => OsCode::KEY_TAB,
        "q" => OsCode::KEY_Q,
        "w" => OsCode::KEY_W,
        "e" => OsCode::KEY_E,
        "r" => OsCode::KEY_R,
        "t" => OsCode::KEY_T,
        "y" => OsCode::KEY_Y,
        "u" => OsCode::KEY_U,
        "i" => OsCode::KEY_I,
        "o" => OsCode::KEY_O,
        "p" => OsCode::KEY_P,
        "{" => OsCode::KEY_LEFTBRACE,
        "}" => OsCode::KEY_RIGHTBRACE,
        "[" => OsCode::KEY_LEFTBRACE,
        "]" => OsCode::KEY_RIGHTBRACE,
        "\\" => OsCode::KEY_BACKSLASH,
        "caps" => OsCode::KEY_CAPSLOCK,
        "a" => OsCode::KEY_A,
        "s" => OsCode::KEY_S,
        "d" => OsCode::KEY_D,
        "f" => OsCode::KEY_F,
        "g" => OsCode::KEY_G,
        "h" => OsCode::KEY_H,
        "j" => OsCode::KEY_J,
        "k" => OsCode::KEY_K,
        "l" => OsCode::KEY_L,
        ";" => OsCode::KEY_SEMICOLON,
        "'" => OsCode::KEY_APOSTROPHE,
        "ret" => OsCode::KEY_ENTER,
        "lsft" => OsCode::KEY_LEFTSHIFT,
        "z" => OsCode::KEY_Z,
        "x" => OsCode::KEY_X,
        "c" => OsCode::KEY_C,
        "v" => OsCode::KEY_V,
        "b" => OsCode::KEY_B,
        "n" => OsCode::KEY_N,
        "m" => OsCode::KEY_M,
        "," => OsCode::KEY_COMMA,
        "." => OsCode::KEY_DOT,
        "/" => OsCode::KEY_SLASH,
        "esc" => OsCode::KEY_ESC,
        "rsft" => OsCode::KEY_RIGHTSHIFT,
        "lctl" => OsCode::KEY_LEFTCTRL,
        "lmet" => OsCode::KEY_LEFTMETA,
        "lalt" => OsCode::KEY_LEFTALT,
        "spc" => OsCode::KEY_SPACE,
        "ralt" => OsCode::KEY_RIGHTALT,
        "rmet" => OsCode::KEY_RIGHTMETA,
        "rctl" => OsCode::KEY_RIGHTCTRL,
        "del" => OsCode::KEY_DELETE,
        "ins" => OsCode::KEY_INSERT,
        "bck" => OsCode::KEY_BACK,
        "fwd" => OsCode::KEY_FORWARD,
        "pgup" => OsCode::KEY_PAGEUP,
        "pgdn" => OsCode::KEY_PAGEDOWN,
        "up" => OsCode::KEY_UP,
        "down" => OsCode::KEY_DOWN,
        "left" => OsCode::KEY_LEFT,
        "rght" => OsCode::KEY_RIGHT,
        "home" => OsCode::KEY_HOME,
        "end" => OsCode::KEY_END,
        "nlk" => OsCode::KEY_NUMLOCK,
        "f1" => OsCode::KEY_F1,
        "f2" => OsCode::KEY_F2,
        "f3" => OsCode::KEY_F3,
        "f4" => OsCode::KEY_F4,
        "f5" => OsCode::KEY_F5,
        "f6" => OsCode::KEY_F6,
        "f7" => OsCode::KEY_F7,
        "f8" => OsCode::KEY_F8,
        "f9" => OsCode::KEY_F9,
        "f10" => OsCode::KEY_F10,
        "f11" => OsCode::KEY_F11,
        "f12" => OsCode::KEY_F12,
        "kp0" => OsCode::KEY_KP0,
        "kp1" => OsCode::KEY_KP1,
        "kp2" => OsCode::KEY_KP2,
        "kp3" => OsCode::KEY_KP3,
        "kp4" => OsCode::KEY_KP4,
        "kp5" => OsCode::KEY_KP5,
        "kp6" => OsCode::KEY_KP6,
        "kp7" => OsCode::KEY_KP7,
        "kp8" => OsCode::KEY_KP8,
        "kp9" => OsCode::KEY_KP9,
        _ => return None,
    })
}
