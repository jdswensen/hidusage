#[derive(Copy, Clone, PartialEq)]
pub enum Keyboard {
    // Keyboard errors
    Reserved00          = 0x00,
    ErrorRollOver       = 0x01,
    PostFail            = 0x02,
    ErrorUndefined      = 0x03,

    // Keyboard character keys
    CharA               = 0x04,
    CharB               = 0x05,
    CharC               = 0x06,
    CharD               = 0x07,
    CharE               = 0x08,
    CharF               = 0x09,
    CharG               = 0x0A,
    CharH               = 0x0B,
    CharI               = 0x0C,
    CharJ               = 0x0D,
    CharK               = 0x0E,
    CharL               = 0x0F,
    CharM               = 0x10,
    CharN               = 0x11,
    CharO               = 0x12,
    CharP               = 0x13,
    CharQ               = 0x14,
    CharR               = 0x15,
    CharS               = 0x16,
    CharT               = 0x17,
    CharU               = 0x18,
    CharV               = 0x19,
    CharW               = 0x1A,
    CharX               = 0x1B,
    CharY               = 0x1C,
    CharZ               = 0x1D,

    // Keyboard number keys
    Num1                = 0x1E,
    Num2                = 0x1F,
    Num3                = 0x20,
    Num4                = 0x21,
    Num5                = 0x22,
    Num6                = 0x23,
    Num7                = 0x24,
    Num8                = 0x25,
    Num9                = 0x26,
    Num0                = 0x27,

    // Keyboard control keys
    ReturnEnter         = 0x28,
    Escape              = 0x29,
    DeleteBackspace     = 0x2A,
    Tab                 = 0x2B,
    Spacebar            = 0x2C,
    Hyphen              = 0x2D,
    EqualsSign          = 0x2E,
    OpenBracket         = 0x2F,
    CloseBracket        = 0x30,
    Backslash           = 0x31,
    NonUsPound          = 0x32,
    Semicolon           = 0x33,
    Quote               = 0x34,
    GraveAccentTilde    = 0x35,
    Comma               = 0x36,
    Period              = 0x37,
    Forwardslash        = 0x38,
    CapsLock            = 0x39,

    // Keyboard function keys (Set 1)
    F1                  = 0x3A,
    F2                  = 0x3B,
    F3                  = 0x3C,
    F4                  = 0x3D,
    F5                  = 0x3E,
    F6                  = 0x3F,
    F7                  = 0x40,
    F8                  = 0x41,
    F9                  = 0x42,
    F10                 = 0x43,
    F11                 = 0x44,
    F12                 = 0x45,

    PrintScreen         = 0x46,
    ScrollLock          = 0x47,
    Pause               = 0x48,
    Insert              = 0x49,
    Home                = 0x4A,
    PageUp              = 0x4B,
    DeleteForward       = 0x4C,
    End                 = 0x4D,
    PageDown            = 0x4E,
    RightArrow          = 0x4F,
    LeftArrow           = 0x50,
    DownArrow           = 0x51,
    UpArrow             = 0x52,

    // Keypad
    NumLock             = 0x53,
    KeypadForwardslash  = 0x54,
    KeypadAsterisk      = 0x55,
    KeypadHyphen        = 0x56,
    KeypadPlus          = 0x57,
    KeypadEnter         = 0x58,
    Keypad1             = 0x59,
    Keypad2             = 0x5A,
    Keypad3             = 0x5B,
    Keypad4             = 0x5C,
    Keypad5             = 0x5D,
    Keypad6             = 0x5E,
    Keypad7             = 0x5F,
    Keypad8             = 0x60,
    Keypad9             = 0x61,
    Keypad0             = 0x62,
    KeypadPeriod        = 0x63,

    NonUsBackslash      = 0x64,
    Application         = 0x65,
    Power               = 0x66,

    KeypadEqualSign     = 0x67,

    // Keyboard function keys (Set 2)
    F13                 = 0x68,
    F14                 = 0x69,
    F15                 = 0x6A,
    F16                 = 0x6B,
    F17                 = 0x6C,
    F18                 = 0x6D,
    F19                 = 0x6E,
    F20                 = 0x6F,
    F21                 = 0x70,
    F22                 = 0x71,
    F23                 = 0x72,
    F24                 = 0x73,

    Execute             = 0x74,
    Help                = 0x75,
    Menu                = 0x76,
    Select              = 0x77,
    Stop                = 0x78,
    Again               = 0x79,
    Undo                = 0x7A,
    Cut                 = 0x7B,
    Copy                = 0x7C,
    Paste               = 0x7D,
    Find                = 0x7E,
    Mute                = 0x7F,
    VolumeUp            = 0x80,
    VolumeDown          = 0x81,
    LockingCapsLock     = 0x82,
    LockingNumLock      = 0x83,
    LockingScrollLock   = 0x84,
    KeypadComma         = 0x85,
    KeypadEqualSignAS400= 0x86,

    International1      = 0x87,
    International2      = 0x88,
    International3      = 0x89,
    International4      = 0x8A,
    International5      = 0x8B,
    International6      = 0x8C,
    International7      = 0x8D,
    International8      = 0x8E,
    International9      = 0x8F,

    Language1           = 0x90,
    Language2           = 0x91,
    Language3           = 0x92,
    Language4           = 0x93,
    Language5           = 0x94,
    Language6           = 0x95,
    Language7           = 0x96,
    Language8           = 0x97,
    Language9           = 0x98,

    AlternateErase      = 0x99,
    SysReqOrAttention   = 0x9A,
    Cancel              = 0x9B,
    Clear               = 0x9C,
    Prior               = 0x9D,
    Return              = 0x9E,
    Separator           = 0x9F,
    Out                 = 0xA0,
    Oper                = 0xA1,
    ClearOrAgain        = 0xA2,
    CrSelOrProps        = 0xA3,
    ExSel               = 0xA4,

    // Reserved: 0xA5 - 0xDF

    // Modifier keys
    LeftControl         = 0xE0,
    LeftShift           = 0xE1,
    LeftAlt             = 0xE2,
    LeftGui             = 0xE3,
    RightControl        = 0xE4,
    RightShift          = 0xE5,
    RightAlt            = 0xE6,
    RightGui            = 0xE7,

    // Reserved: 0xE8 - 0xFFFF
}

impl super::HidUsage for Keyboard {
    fn as_u16(&self) -> u16 {
        *self as u16
    }

    fn as_u32(&self) -> u32 {
        *self as u32
    }

    fn usage_page(&self) -> u16 {
        0x07
    }

    // fn into_report_desc_part(self) -> super::ReportDesc {
    //     unimplemented!()
    // }
}
