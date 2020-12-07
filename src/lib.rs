#![no_std]
pub mod keyboard;
pub mod button;
pub mod led;

enum UsageTypeControls {
    LinearControl,
    OnOffControl,
    MomentaryControl,
    OneShotControl,
    RetriggerControl
}

enum UsageTypeData {
    Selector,
    StaticValue,
    StaticFlag,
    DynamicValue,
    DynamicFlag,
}

enum UsageTypeCollection {
    NamedArray,
    Application,
    Logical,
    Physical,
    UsageSwitch,
    UsageModifier,
}

// todo: This can cause boilerplate code for as_u32 and as_u16.
//       Could a macro be used to fix this? Generics? Both?
//
// question: should this be some sort of iterator so that reports can be
// gathered as part of the same collection?
pub (crate) trait HidUsage {
    /// Cast the enum value as u32
    fn as_u32(&self) -> u32;

    /// Cast the enum value as u16
    fn as_u16(&self) -> u16;

    /// HID Usage Page identifier.
    fn usage_page(&self) -> u16;

    /// HID Usage ID for a specific Usage Page.
    fn usage_id(&self) -> u16 {
        self.as_u16()
    }

    /// Return the full 32 bit usage value which includes Usage Page
    /// and Usage ID.
    fn extended_usage(&self) -> u32 {
        ((self.usage_page() as u32) << 16) | self.usage_id() as u32
    }

    // todo: transform into a report desc part
    // fn into_report_desc_part(self) -> ReportDesc;

    // todo: usage type controls and data for the specific enum
}
