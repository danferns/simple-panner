use nih_plug::prelude::Editor;
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::{assets, widgets::*};
use nih_plug_vizia::{create_vizia_editor, ViziaState, ViziaTheming};
use std::sync::Arc;

mod param_knob;
use self::param_knob::ParamKnob;

use crate::SimplePannerParams;

const STYLE: &str = r#"
    label {
        width: 200px;
        height: 30px;
        child-space: 1s;
        font-size: 20;
        color: #C2C2C2;
    }
    
    knob {
        width: 100px;
        height: 100px;
    }
    
    knob .track {
        background-color: #ffb74d;
    }
    .label_knob {
        border-width: 2px;
        border-color: #28282b;
        background-color: #000000;
        col-between: 10px;
        child-space: 1s;
    }
"#;

#[derive(Lens)]
struct Data {
    params: Arc<SimplePannerParams>,
}

impl Model for Data {}

pub(crate) fn default_state() -> Arc<ViziaState> {
    ViziaState::from_size(200, 150)
}

pub(crate) fn create(
    params: Arc<SimplePannerParams>,
    editor_state: Arc<ViziaState>,
) -> Option<Box<dyn Editor>> {
    create_vizia_editor(editor_state, ViziaTheming::Custom, move |cx, _| {
        assets::register_noto_sans_light(cx);
        assets::register_noto_sans_thin(cx);

        cx.add_theme(STYLE);

        Data {
            params: params.clone(),
        }
        .build(cx);

        ResizeHandle::new(cx);

        VStack::new(cx, |cx| {
            ParamKnob::new(cx, Data::params, |p| &p.gain);
        });
    })
}
