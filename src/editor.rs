use nih_plug::prelude::Editor;
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::{assets, widgets::*};
use nih_plug_vizia::{create_vizia_editor, ViziaState, ViziaTheming};
use std::sync::Arc;

mod param_knob;
use self::param_knob::ParamKnob;

use crate::SimplePannerParams;

#[derive(Lens)]
struct Data {
    params: Arc<SimplePannerParams>,
}

impl Model for Data {}

pub(crate) fn default_state() -> Arc<ViziaState> {
    ViziaState::from_size(200, 180)
}

pub(crate) fn create(
    params: Arc<SimplePannerParams>,
    editor_state: Arc<ViziaState>,
) -> Option<Box<dyn Editor>> {
    create_vizia_editor(editor_state, ViziaTheming::Custom, move |cx, _| {
        assets::register_noto_sans_light(cx);
        assets::register_noto_sans_thin(cx);

        cx.add_stylesheet("src/styles/main.css")
            .expect("main.css not found.");

        Data {
            params: params.clone(),
        }
        .build(cx);

        ResizeHandle::new(cx);

        VStack::new(cx, |cx| {
            ParamKnob::new(cx, Data::params, |p| &p.pan);
        })
        .class("main");
    })
}
