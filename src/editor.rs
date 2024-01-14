use nih_plug::prelude::Editor;
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::{assets, widgets::*};
use nih_plug_vizia::{create_vizia_editor, ViziaState, ViziaTheming};
use std::sync::Arc;

mod param_knob;
use self::param_knob::ParamKnob;

use crate::SimplePannerParams;

const STYLE: &str = r#"
.main {
    background-color: #222222;
}

.param_knob {
    width: 150px;
}

label {
    child-space: 1s;
    font-size: 20;
    color: #c2c2c2;
}

knob {
    width: 100px;
    height: 100px;
}

knob .knob-track {
    background-color: #92facf;
}

.track {
    color: #92facf;
    background-color: #363636;
}

.tick {
    background-color: #363636;
}

.tick {
    color: #696969;
}

"#;

#[derive(Lens)]
struct Data {
    params: Arc<SimplePannerParams>,
}

impl Model for Data {}

pub(crate) fn default_state() -> Arc<ViziaState> {
    ViziaState::new(|| (350, 200))
}

pub(crate) fn create(
    params: Arc<SimplePannerParams>,
    editor_state: Arc<ViziaState>,
) -> Option<Box<dyn Editor>> {
    create_vizia_editor(editor_state, ViziaTheming::Custom, move |cx, _| {
        assets::register_noto_sans_light(cx);
        assets::register_noto_sans_thin(cx);

        cx.add_stylesheet(STYLE).expect("Failed to add stylesheet");

        Data {
            params: params.clone(),
        }
        .build(cx);

        HStack::new(cx, |cx| {
            ParamKnob::new(cx, Data::params, |p| &p.pan, true);
            ParamKnob::new(cx, Data::params, |p| &p.focus, false);
        })
        .child_space(Stretch(0.25))
        .class("main");

        ResizeHandle::new(cx);
    })
}
