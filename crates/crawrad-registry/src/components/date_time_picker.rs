use crate::{ComponentDesc, Events, Props, TextLine, component};
use crawrad_note::note;

#[note("components-date-time-picker-date-time-picker")]
pub fn date_time_picker() -> ComponentDesc {
    let mut d = component!(
        "DateTimePicker",
        display: "DateTimePicker",
        base_name: "date_time_picker",
        palette: "Win32",
        prefix: "dtp",
        size: (140.0, 24.0),
        text: TextLine::Centered,
        props: [
            Props::date_time_picker_kind("dtkDate"),
            Props::date_time_value("2000-01-01"),
            Props::min_date(""),
            Props::max_date(""),
        ],
        events: [
            Events::CHANGE,
            Events::ENTER,
            Events::EXIT,
        ],
    );
    d.props.extend(crate::components::base::appearance_props());
    d
}
