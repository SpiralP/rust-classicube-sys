use crate::os::*;

pub const Key_Function_Names: &[&str] = &[
  "F1", "F2", "F3", "F4", "F5", "F6", "F7", "F8", "F9", "F10", "F11", "F12", "F13", "F14", "F15",
  "F16", "F17", "F18", "F19", "F20", "F21", "F22", "F23", "F24", "F25", "F26", "F27", "F28", "F29",
  "F30", "F31", "F32", "F33", "F34", "F35",
];
pub const Key_Ascii_Names: &[&str] = &[
  "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S",
  "T", "U", "V", "W", "X", "Y", "Z",
];

pub const Input_Names: [&str; Key__INPUT_COUNT as usize] = [
  "None",
  // Key_Function_Names,
  "F1",
  "F2",
  "F3",
  "F4",
  "F5",
  "F6",
  "F7",
  "F8",
  "F9",
  "F10",
  "F11",
  "F12",
  "F13",
  "F14",
  "F15",
  "F16",
  "F17",
  "F18",
  "F19",
  "F20",
  "F21",
  "F22",
  "F23",
  "F24",
  "F25",
  "F26",
  "F27",
  "F28",
  "F29",
  "F30",
  "F31",
  "F32",
  "F33",
  "F34",
  "F35",
  "ShiftLeft",
  "ShiftRight",
  "ControlLeft",
  "ControlRight",
  "AltLeft",
  "AltRight",
  "WinLeft",
  "WinRight",
  "Up",
  "Down",
  "Left",
  "Right",
  "Number0",
  "Number1",
  "Number2",
  "Number3",
  "Number4",
  "Number5",
  "Number6",
  "Number7",
  "Number8",
  "Number9",
  "Insert",
  "Delete",
  "Home",
  "End",
  "PageUp",
  "PageDown",
  "Menu",
  // Key_Ascii_Names,
  "A",
  "B",
  "C",
  "D",
  "E",
  "F",
  "G",
  "H",
  "I",
  "J",
  "K",
  "L",
  "M",
  "N",
  "O",
  "P",
  "Q",
  "R",
  "S",
  "T",
  "U",
  "V",
  "W",
  "X",
  "Y",
  "Z",
  "Enter",
  "Escape",
  "Space",
  "BackSpace",
  "Tab",
  "CapsLock",
  "ScrollLock",
  "PrintScreen",
  "Pause",
  "NumLock",
  "Keypad0",
  "Keypad1",
  "Keypad2",
  "Keypad3",
  "Keypad4",
  "Keypad5",
  "Keypad6",
  "Keypad7",
  "Keypad8",
  "Keypad9",
  "KeypadDivide",
  "KeypadMultiply",
  "KeypadSubtract",
  "KeypadAdd",
  "KeypadDecimal",
  "KeypadEnter",
  "Tilde",
  "Minus",
  "Plus",
  "BracketLeft",
  "BracketRight",
  "Slash",
  "Semicolon",
  "Quote",
  "Comma",
  "Period",
  "BackSlash",
  "XButton1",
  "XButton2",
  "LeftMouse",
  "RightMouse",
  "MiddleMouse",
];

pub const KeyBind_Defaults: [cc_uint8; KeyBind__KEYBIND_COUNT as usize] = [
  b'W',
  b'S',
  b'A',
  b'D',
  Key__KEY_SPACE as _,
  b'R',
  Key__KEY_ENTER as _,
  b'T',
  b'B',
  b'F',
  Key__KEY_ENTER as _,
  Key__KEY_TAB as _,
  Key__KEY_LSHIFT as _,
  b'X',
  b'Z',
  b'Q',
  b'E',
  Key__KEY_LALT as _,
  Key__KEY_F3 as _,
  Key__KEY_F12 as _,
  Key__KEY_F11 as _,
  Key__KEY_F5 as _,
  Key__KEY_F1 as _,
  Key__KEY_F7 as _,
  b'C',
  Key__KEY_LCTRL as _,
  Key__KEY_LMOUSE as _,
  Key__KEY_MMOUSE as _,
  Key__KEY_RMOUSE as _,
  Key__KEY_F6 as _,
  Key__KEY_LALT as _,
  Key__KEY_F8 as _,
  b'G',
  Key__KEY_F10 as _,
  0,
];

pub const keybindNames: [&str; KeyBind__KEYBIND_COUNT as usize] = [
  "Forward",
  "Back",
  "Left",
  "Right",
  "Jump",
  "Respawn",
  "SetSpawn",
  "Chat",
  "Inventory",
  "ToggleFog",
  "SendChat",
  "PlayerList",
  "Speed",
  "NoClip",
  "Fly",
  "FlyUp",
  "FlyDown",
  "ExtInput",
  "HideFPS",
  "Screenshot",
  "Fullscreen",
  "ThirdPerson",
  "HideGUI",
  "AxisLines",
  "ZoomScrolling",
  "HalfSpeed",
  "DeleteBlock",
  "PickBlock",
  "PlaceBlock",
  "AutoRotate",
  "HotbarSwitching",
  "SmoothCamera",
  "DropBlock",
  "IDOverlay",
  "BreakableLiquids",
];
