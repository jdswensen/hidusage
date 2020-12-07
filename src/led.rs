/// A simple light emitting diode (LED) indicator
///
/// An LED or indicator is implemented as an On/Off Control (OOC) using the
/// _Single button toggle_ mode, where a value of 1 will turn on the indicator,
/// and a value of 0 will turn it off.
#[derive(Copy, Clone, PartialEq)]
pub enum Led {
    /// No indicator defined.
    Undefined               = 0x0000,

    /// Indicates that Number Lock is enabled.
    NumLock                 = 0x0001,

    /// Indicates that Capital Lock is enabled.
    CapsLock                = 0x0002,

    /// Indicates that Scroll Lock is enabled.
    ScrollLock              = 0x0003,

    /// Indicates that composition mode is enabled.
    Compose                 = 0x0004,

    /// Indicates that Kana mode is enabled.
    Kana                    = 0x0005,

    /// Indicates that the device is powered.
    Power                   = 0x0006,

    /// Indicates that the Shift function is enabled.
    Shift                   = 0x0007,

    /// (Phone) Indicates that the phone is not accepting incoming calls.
    DoNotDisturb            = 0x0008,

    /// Indicates that amplifier audio output is shut off.
    Mute                    = 0x0009,

    /// Indicates that tone controls are functional.
    ToneEnable              = 0x000A,

    /// Indicates that the high cut filter is enabled.
    HighCutFilter           = 0x000B,

    /// Indicates that the low cut filter is enabled.
    LowCutFilter            = 0x000C,

    /// Indicates that tone shape processing is active.
    EqualizerEnable         = 0x000D,

    /// Indicates that DSP processing is active.
    SoundFieldOn            = 0x000E,

    /// Indicates that surround channel information is being decoded.
    SurroundOn              = 0x000F,

    /// Indicates that the playback device is in repeat mode.
    Repeat                  = 0x0010,

    /// Indicates that the signal currently being received by the tuner is
    /// in stereo
    Stereo                  = 0x0011,

    /// Indicates that a digital audio signal has been detected.
    SamplingRateDetect      = 0x0012,

    /// Indicates that disc media is up to the speed required for playback
    /// or read.
    Spinning                = 0x0013,

    /// Indicates that the video disc media is in Constant Angular Velocity
    /// format.
    Cav                     = 0x0014,

    /// Indicates that the video disc media is in Constant Linear Velocity
    /// format
    Clv                     = 0x0015,

    /// Indicates that a valid recording format has been detected.
    RecordingFormatDetect   = 0x0016,

    /// (Phone) Indicates that the handset is off-hook.
    OffHook                 = 0x0017,

    /// (Phone) Indicates visually that a phone is ringing.
    Ring                    = 0x0018,

    /// (Phone, answering machine) Indicates that a message has been recorded
    /// and has not yet been heard.
    MessageWaiting          = 0x0019,

    /// (Phone) Indicates that the phone is in a mode that transfers data
    /// (rather than voice).
    DataMode                = 0x001A,

    /// Indicates that the device is currently battery powered.
    BatteryOperation        = 0x001B,

    /// Indicates taht the battery is in a nominal charge state.
    BatteryOk               = 0x001C,

    /// Indicates that the battery is in a low charge state.
    BatteryLow              = 0x001D,

    /// (Phone) Indicates that the phone is using the speaker/microphone
    /// instead of a handset or headset.
    Speaker                 = 0x001E,

    /// (Phone) Indicates that the phone is using the headset instead of a
    /// handset or speaker/microphone.
    HeadSet                 = 0x001F,

    /// (Phone) Indicates that the caller is on hold.
    Hold                    = 0x0020,

    /// (Phone) Indicates that the microphone has been muted.
    Microphone              = 0x0021,

    /// (Phone) Indicates that incoming calls are forwarded to a covering
    /// station.
    Coverage                = 0x0022,

    /// (Phone) Indicates that the phone is in after-hours mode.
    NightMode               = 0x0023,

    /// (Phone) Indicates that incoming calls are forwarded to another station.
    SendCalls               = 0x0024,

    /// (Phone) Indicates that a call in the user's pickup group has been
    /// accepted.
    ///
    /// Pickup groups associate phones in an area. They allow a ringing phone
    /// to be picked up any other phone in the group.
    CallPickup              = 0x0025,

    /// (Phone) Indicates that the phone is in conference call mode.
    Conference              = 0x0026,

    /// Indicates that the device is in standby mode.
    Standby                 = 0x0027,

    /// Indicates that the camera is recording images.
    CameraOn                = 0x0028,

    /// Indicates that the camera is powered but not recording images.
    CameraOff               = 0x0029,

    /// Indicates that the device is online.
    Online                  = 0x002A,

    /// Indicates that the device is offline.
    Offline                 = 0x002B,

    /// Indicates that the device is busy executing operations.
    Busy                    = 0x002C,

    /// Indicates that the device is ready to execute operations.
    Ready                   = 0x002D,

    /// Indicates that the device is out of paper.
    PaperOut                = 0x002E,

    /// Indicates that a paper jam has occurred in the device.
    PaperJam                = 0x002F,

    /// Idicates that the device is being controlled remotely.
    Remote                  = 0x0030,

    /// Indicates that a device's media transport mechanism or a device is in
    /// forward mode.
    Forward                 = 0x0031,

    /// Indicates that a device's media transport mechanism or a device is in
    /// reverse mode.
    Reverse                 = 0x0032,

    /// Indicates that a device's media transport mechanism has been
    /// disengaged.
    Stop                    = 0x0033,

    /// Indicates that a device's media transport mechansim is in rewind mode.
    Rewind                  = 0x0034,

    /// Indicates that a device's media transport mechanism is in fast forward
    /// mode.
    FastForward             = 0x0035,

    /// Indicates that a device's media transport mechanism is in playback mode.
    ///
    /// This indicator may also be true when a device is recording.
    Play                    = 0x0036,

    /// Indicates that a device's media transport mechanism has been paused
    /// while playing or recording.
    Pause                   = 0x0037,

    Record                  = 0x0038,

    /// Indicates that an error has occurred on the device.
    Error                   = 0x0039,

    /// This collection allows the usages that it contains to be associated
    /// with a visual output (an LED) that indicates whether a control
    /// identified by the usage is selected. **Usage Selected Indicator** is a
    /// 1-bit field where 1 is selected and 0 is not selected. All usages found
    /// in this collection will be treated On/Off Controls (OOC).
    UsageSelectedIndicator  = 0x003A,

    /// This collection allows the usages that it contains to be associated
    /// with a visual output (an LED) that indicates whether a control
    /// identified by the usage is in use. **Usage In Use Indicator** is a
    /// 1-bit field where 1 is selected and 0 is not selected. All usages
    /// found in this collection will be treated as On/Off Controls (OOC).
    UsageInUseIndicator     = 0x003B,

    /// This usage names a logical collection which must be contained in
    /// another collection. The usage attached to the encompassing collection
    /// is then identified as an indicator that supports multiple illumination
    /// modes. In this collection one or more of the following Indicator
    /// selectors will be found: On, Flash, Slow Blink, Fast Blink, and Off
    UsageMultiModeIndicator = 0x003C,

    /// Light indicator continuously.
    IndicatorOn             = 0x003D,

    /// Single, momentary illumination of indicator.
    IndicatorFlash          = 0x003E,

    /// Continuous flashing of the indicator at a slow rate.
    IndicatorSlowBlink      = 0x003F,

    /// Continuous flashing of the indicator at a fast rate.
    IndicatorFastBlink      = 0x0040,

    /// Turn indicator illumination off.
    IndicatorOff            = 0x0041,

    /// Duration that the indicator is illuminated in flash mode.
    FlashOnTime             = 0x0042,

    /// Duration that the indicator is illuminated in slow blink mode.
    SlowBlinkOnTime         = 0x0043,

    /// Duration that the indicator is off in slow blink mode.
    SlowBlinkOffTime        = 0x0044,

    /// Duration that the indicator is illuminated in fast blink mode.
    FastBlinkOnTime         = 0x0045,

    /// Duration that the indicator is off in fast blink mode.
    FastBlinkOffTime        = 0x0046,

    /// This collection allows the usage that contains it to be an indicator
    /// that supports multiple colors. All usages found in this collection will
    /// be treated as a Selectors (Sel) where one or more of the following
    /// Indicator selectors will be found: Indicator Off, Red, Green, Amber.
    UsageIndicatorColor     = 0x0047,

    /// Indicator color set to Red.
    IndicatorRed            = 0x0048,

    /// Indicator color set to Green.
    IndicatorGreen          = 0x0049,

    /// Indicator color set to Amber.
    IndicatorAmber          = 0x004A,

    /// This usage identifies an indicator that has no permanently assigned
    /// function.
    GenericIndicator        = 0x004B,

    /// Indicates that the system is in a low power state, but is still
    /// powered and retaining some context.
    SystemSuspend           = 0x004C,

    /// Indicates that a battery-operated system is connected to external
    /// power.
    ExternalPowerConnected  = 0x004D,

    /// Indicator color set to Blue.
    IndicatorBlue           = 0x004E,

    /// Indicator color set to Orange.
    IndicatorOrange         = 0x004F,

    /// Indicates that the device is operating within normal parameters.
    GoodStatus              = 0x0050,

    /// Indicates that the device is not operating within normal parameters,
    /// but that the situation has not reached the level of an error
    /// (see Error).
    WarningStatus           = 0x0051,

    /// A collection of controls for a color mixing LED. An RGB LED shall
    /// contain a red, green, and blue channel usage and may include an
    /// intensity usage.
    RgbLed                  = 0x0052,

    /// Control setting the intensity of the red channel of a
    /// color-mixed LED.
    RedLedChannel           = 0x0053,

    /// Control setting the intensity of the blue channel of a
    /// color-mixed LED.
    BlueLedChannel          = 0x0054,

    /// Control setting the intensity of the green channel of a
    /// color-mixed LED.
    GreenLedChannel         = 0x0055,

    /// Control setting the overal intensity of a color-mixed LED. This
    /// control should be represented as a percentage control using a
    /// logical minimum of zero and a logical maximum of 100.
    LedIntensity            = 0x0056,

    Reserved57              = 0x0057,
    Reserved58              = 0x0058,
    Reserved59              = 0x0059,
    Reserved5A              = 0x005A,
    Reserved5B              = 0x005B,
    Reserved5C              = 0x005C,
    Reserved5D              = 0x005D,
    Reserved5E              = 0x005E,
    Reserved5F              = 0x005F,

    /// A collection usage for assigning a player to a game controller. Two or
    /// more Player Selectors shall be included in the Named Array.
    PlayerIndicator         = 0x0060,

    /// Indicates that the game controller is assigned to player 1.
    Player1                 = 0x0061,

    /// Indicates that the game controller is assigned to player 2.
    Player2                 = 0x0062,

    /// Indicates that the game controller is assigned to player 3.
    Player3                 = 0x0063,

    /// Indicates that the game controller is assigned to player 4.
    Player4                 = 0x0064,

    /// Indicates that the game controller is assigned to player 5.
    Player5                 = 0x0065,

    /// Indicates that the game controller is assigned to player 6.
    Player6                 = 0x0066,

    /// Indicates that the game controller is assigned to player 7.
    Player7                 = 0x0067,

    /// Indicates that the game controller is assigned to player 8.
    Player8                 = 0x0068,

    // Reserved: 0x69 -> 0xFFFF
}

impl super::HidUsage for Led {
    fn as_u16(&self) -> u16 {
        *self as u16
    }

    fn as_u32(&self) -> u32 {
        *self as u32
    }

    fn usage_page(&self) -> u16 {
        0x08
    }
}
