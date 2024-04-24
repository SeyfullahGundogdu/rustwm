use smithay::input::keyboard::ModifiersState;
use super::{KeyModifier, KeyModifiers};
pub struct KeyModifiersDef(pub Vec<KeyModifier>);

impl From<KeyModifiersDef> for KeyModifiers {
    fn from(src: KeyModifiersDef) -> Self {
        src.0.into_iter().fold(
            KeyModifiers {
                ctrl: false,
                alt: false,
                shift: false,
                logo: false,
            },
            |mut modis, modi: KeyModifier| {
                modis += modi;
                modis
            },
        )
    }
}

impl std::ops::AddAssign<KeyModifier> for KeyModifiers {
    fn add_assign(&mut self, rhs: KeyModifier) {
        match rhs {
            KeyModifier::Ctrl => self.ctrl = true,
            KeyModifier::Alt => self.alt = true,
            KeyModifier::Shift => self.shift = true,
            KeyModifier::Super => self.logo = true,
        };
    }
}

impl PartialEq<ModifiersState> for KeyModifiers {
    fn eq(&self, other: &ModifiersState) -> bool {
        self.ctrl == other.ctrl
            && self.alt == other.alt
            && self.shift == other.shift
            && self.logo == other.logo
    }
}
