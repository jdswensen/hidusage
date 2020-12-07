/// A simple button declaration.
///
/// The Button page is the first place an application should look for user
/// selection controls. System graphical user interfaces typically employ a
/// pointer and a set of hierarchical selectors to select, move and otherwise
/// manipulate their environment. For these purposes the following assignment
/// of significance can be applied to the Button usages:
///
/// - Button 1, Primary Button: Used for object selecting, dragging, and double
///   click activation. On MacOS, this is the only button. Microsoft operating
///   systems call this a logical left button, because it is not necessarily
///   physically located on the left of the pointing device.
///
/// - Button 2, Secondary Button: Used by newer graphical user interfaces to
///   browse object properties. Exposed by systems to applications that
///   typically assign application-specific functionality.
///
/// - Button 3, Teritiary Button: An optional control that is exposed to
///   applications, but seldom assigned functionality due to prevalence of
///   two and one button devices.
///
/// - Button 4 - 255: As button number increases, its significance as a
///   selector decreases.
///
/// In many ways the assignment of button numbers is similar to the
/// assignment of Effort in Physical descriptors. Button 1 would be used
/// to define the button a finger rests on when the hand is in the at rest
/// position, that is, virtually no effort is required by the user to
/// activate the button. Button values increment as the finger has to stretch
/// to reach a control. See the Physical Descriptors in the HID Specification
/// for methods of further qualifying buttons.
///
/// Buttons can be defined as Selectors, On/Off Controls, Momentary Controls,
/// or One-Shot Controls depending on the context of their declaration.
///
/// When defining buttons as selectors, Usage ID 0 is defined to indicate that
/// no buttons are pressed. When declaring an array of buttons, one can:
///
/// - Declare all buttons of interest, include the usage No Button Pressed
#[derive(Copy, Clone, PartialEq)]
pub enum Button {
    NoButton                    = 0x00,
    Button1                     = 0x01,
    Button2                     = 0x02,
    Button3                     = 0x03,
    Button4                     = 0x04,
    Button5                     = 0x05,
    Button6                     = 0x06,
    Button7                     = 0x07,
    Button8                     = 0x08,
    Button9                     = 0x09,
    Button10                    = 0x0A,
    Button11                    = 0x0B,
    Button12                    = 0x0C,
    Button13                    = 0x0D,
    Button14                    = 0x0E,
    Button15                    = 0x0F,
}

impl super::HidUsage for Button {
    fn as_u16(&self) -> u16 {
        *self as u16
    }

    fn as_u32(&self) -> u32 {
        *self as u32
    }

    fn usage_page(&self) -> u16 {
        0x09
    }

    // fn into_report_desc_part(self) -> super::ReportDesc {
    //     unimplemented!()
    // }
}
