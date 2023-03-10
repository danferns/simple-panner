use nih_plug::prelude::*;
use nih_plug_vizia::ViziaState;
use std::sync::Arc;

mod editor;
pub struct SimplePanner {
    params: Arc<SimplePannerParams>,
}

#[derive(Params)]
pub struct SimplePannerParams {
    /// The parameter's ID is used to identify the parameter in the wrappred plugin API. As long as
    /// these IDs remain constant, you can rename and reorder these fields as you wish. The
    /// parameters are exposed to the host in the same order they were defined. In this case, this
    /// gain parameter is stored as linear gain while the values are displayed in decibels.
    #[persist = "editor-state"]
    editor_state: Arc<ViziaState>,
    #[id = "pan"]
    pub pan: FloatParam,
    #[id = "mix"]
    pub mix: FloatParam,
}

impl Default for SimplePanner {
    fn default() -> Self {
        Self {
            params: Arc::new(SimplePannerParams::default()),
        }
    }
}

impl Default for SimplePannerParams {
    fn default() -> Self {
        Self {
            editor_state: editor::default_state(),
            pan: FloatParam::new("Pan", 0., FloatRange::Linear { min: -1., max: 1. })
                .with_smoother(SmoothingStyle::Linear(10.0))
                .with_value_to_string(formatters::v2s_f32_panning())
                .with_string_to_value(formatters::s2v_f32_panning()),

            mix: FloatParam::new("Channel Mix", 0., FloatRange::Linear { min: 0., max: 1. })
                .with_smoother(SmoothingStyle::Linear(10.0))
                .with_unit("%")
                .with_value_to_string(formatters::v2s_f32_percentage(0))
                .with_string_to_value(formatters::s2v_f32_percentage()),
        }
    }
}

impl Plugin for SimplePanner {
    const NAME: &'static str = "Simple Panner";
    const VENDOR: &'static str = "Daniel Fernandes";
    const URL: &'static str = env!("CARGO_PKG_HOMEPAGE");
    const EMAIL: &'static str = "dannywritescode@gmail.com";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const DEFAULT_INPUT_CHANNELS: u32 = 2;
    const DEFAULT_OUTPUT_CHANNELS: u32 = 2;

    const DEFAULT_AUX_INPUTS: Option<AuxiliaryIOConfig> = None;
    const DEFAULT_AUX_OUTPUTS: Option<AuxiliaryIOConfig> = None;

    const MIDI_INPUT: MidiConfig = MidiConfig::None;
    const MIDI_OUTPUT: MidiConfig = MidiConfig::None;

    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    // More advanced plugins can use this to run expensive background tasks. See the field's
    // documentation for more information. `()` means that the plugin does not have any background
    // tasks.
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn editor(&self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        editor::create(self.params.clone(), self.params.editor_state.clone())
    }

    fn accepts_bus_config(&self, config: &BusConfig) -> bool {
        // This works with stereo IO
        config.num_input_channels == config.num_output_channels && config.num_input_channels == 2
    }

    fn initialize(
        &mut self,
        _bus_config: &BusConfig,
        _buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        // Resize buffers and perform other potentially expensive initialization operations here.
        // The `reset()` function is always called right after this function. You can remove this
        // function if you do not need it.
        true
    }

    fn reset(&mut self) {
        // Reset buffers and envelopes here. This can be called from the audio thread and may not
        // allocate. You can remove this function if you do not need it.
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        for mut channel_samples in buffer.iter_samples() {
            // Smoothing is optionally built into the parameters themselves
            let pan = self.params.pan.smoothed.next();
            let mix = self.params.mix.smoothed.next() / 2.;

            let x = std::f32::consts::PI * (pan + 1.) / 4.;

            unsafe {
                let left = channel_samples.get_unchecked_mut(0).clone();
                let right = channel_samples.get_unchecked_mut(1).clone();
                *channel_samples.get_unchecked_mut(0) =
                    x.cos() * std::f32::consts::SQRT_2 * ((1. - mix) * left + mix * right);
                *channel_samples.get_unchecked_mut(1) =
                    x.sin() * std::f32::consts::SQRT_2 * ((1. - mix) * right + mix * left);
            }
        }

        ProcessStatus::Normal
    }
}

impl ClapPlugin for SimplePanner {
    const CLAP_ID: &'static str = "com.danferns.simple-panner";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("Simple panning plugin");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;

    // Don't forget to change these features
    const CLAP_FEATURES: &'static [ClapFeature] = &[ClapFeature::AudioEffect, ClapFeature::Stereo];
}

impl Vst3Plugin for SimplePanner {
    const VST3_CLASS_ID: [u8; 16] = *b"DAN_SIMPLEPANNER";

    // And don't forget to change these categories, see the docstring on `VST3_CATEGORIES` for more
    // information
    const VST3_CATEGORIES: &'static str = "Fx|Spatial";
}

nih_export_clap!(SimplePanner);
nih_export_vst3!(SimplePanner);
