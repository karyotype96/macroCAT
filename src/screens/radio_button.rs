// TODO: Implement radio button functionality
pub struct RadioButton {
    label: Option<String>,
    position: (f32, f32),
    is_checked: bool,
}

pub struct RadioGroup {
    buttons: Vec<RadioButton>,
}

impl RadioButton {}