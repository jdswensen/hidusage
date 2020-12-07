#[derive(Copy, Clone, PartialEq)]
pub enum Desktop {
    Undefined00                 = 0x00,
    Pointer                     = 0x01,
    Mouse                       = 0x02,
    Reserved                    = 0x03,
    Joystick                    = 0x04,
    Gamepad                     = 0x05,
    Keyboard                    = 0x06,
    Keypad                      = 0x07,
    MultiAxisController         = 0x08,
    TabletPcSystemControls      = 0x09,
    WaterCoolingDevice          = 0x0A,
    ComputerChassisDevice       = 0x0B,
    WirelessRadioControls       = 0x0C,
    PortableDeviceControl       = 0x0D,
    SystemMultAxisController    = 0x0E,
    SpatialController           - 0x0F,
    AssistiveControl            = 0x10,
    DeviceDock                  = 0x11,
    DockableDevice              = 0x12,

    // Reserved: 0x13 - 0x2F

    X                           = 0x30,
    Y                           = 0x31,
    Z                           = 0x32,
    Rx                          = 0x33,
    Ry                          = 0x34,
    Rz                          = 0x35,
    Slider                      = 0x36,
    Dial                        = 0x37,
    Wheel                       = 0x38,
    HatSwitch                   = 0x39,
    CountedBuffer               = 0x3A,
    ByteCount                   = 0x3B,
    MotionWakeup                = 0x3C,
    Start                       = 0x3D,
    Select                      = 0x3E,
    Reserved3F                  = 0x3F,
    Vx                          = 0x40,
    Vy                          = 0x41,
    Vz                          = 0x42,
    Vbrx                        = 0x43,
    Vbry                        = 0x44,
    Vrbz                        = 0x45,
    Vno                         = 0x46,
}
