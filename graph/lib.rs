use egui::widgets::Slider;
use nih_plug::prelude::*;
use nih_plug_egui::{
    EguiState, create_egui_editor,
    egui::{self, Vec2},
    resizable_window::ResizableWindow,
    widgets,
};
use std::{f32::consts, sync::Arc};

macro_rules! set_param {
    ($set:expr, $par:expr, $val:expr) => {{
        $set.begin_set_parameter($par);
        $set.set_parameter($par, $val);
        $set.end_set_parameter($par);
    }};
}

pub struct Graph {
    params: Arc<GraphParams>,
    sample_rate: f32,
    phase: f32,
    midi_note_id: u8,
    midi_note_freq: f32,
    midi_note_gain: Smoother<f32>,
}

impl Default for Graph {
    fn default() -> Self {
        Self {
            params: Arc::new(GraphParams::default()),
            sample_rate: 1.,
            phase: 0.,
            midi_note_id: 0,
            midi_note_freq: 1.,
            midi_note_gain: Smoother::new(SmoothingStyle::Linear(5.)),
        }
    }
}

#[derive(Params)]
struct GraphParams {
    #[id = "gain"]
    pub gain: FloatParam,
    #[persist = "editor-state"]
    editor_state: Arc<EguiState>,
}

impl Default for GraphParams {
    fn default() -> Self {
        Self {
            gain: FloatParam::new(
                "gain",
                -10.,
                FloatRange::Linear { min: -30., max: 0. },
            )
            .with_smoother(SmoothingStyle::Linear(3.))
            .with_step_size(0.01)
            .with_unit("dB"),
            editor_state: EguiState::from_size(300, 100),
        }
    }
}

impl Graph {
    /** calculate the sine wave */
    fn calc(&mut self, freq: f32) -> f32 {
        let phs_delta = freq / self.sample_rate;
        let sin = (self.phase * consts::TAU).sin();

        self.phase += phs_delta;
        if self.phase >= 1. {
            self.phase -= 1.;
        }

        sin
    }

    fn set_gain_tar(&mut self, tar: f32) {
        self.midi_note_gain.set_target(self.sample_rate, tar)
    }
}

impl Plugin for Graph {
    const NAME: &str = "BSA Graph";
    const VENDOR: &str = "Skylar Bleed";
    const URL: &str = "";
    const EMAIL: &str = "sbleed@proton.me";
    const VERSION: &str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &[AudioIOLayout] = &[
        AudioIOLayout {
            main_input_channels: None,
            main_output_channels: NonZeroU32::new(2),
            ..AudioIOLayout::const_default()
        },
        AudioIOLayout {
            main_input_channels: None,
            main_output_channels: NonZeroU32::new(1),
            ..AudioIOLayout::const_default()
        },
    ];

    const MIDI_INPUT: MidiConfig = MidiConfig::Basic;
    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn initialize(
        &mut self,
        _: &AudioIOLayout,
        cfg: &BufferConfig,
        _: &mut impl InitContext<Self>,
    ) -> bool {
        self.sample_rate = cfg.sample_rate;
        true
    }

    fn reset(&mut self) {
        self.phase = 0.;
        self.midi_note_id = 0;
        self.midi_note_freq = 1.;
        self.midi_note_gain.reset(0.);
    }

    fn editor(&mut self, _: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        let params = self.params.clone();
        let gui = params.editor_state.clone();

        create_egui_editor(
            self.params.editor_state.clone(),
            (),
            |_, _| {},
            move |ctx, set, _| {
                ResizableWindow::new("graph-win")
                    .min_size(Vec2::new(300., 100.))
                    .show(ctx, gui.as_ref(), |ui| {
                        ui.label("gain");
                        ui.add(
                            Slider::from_get_set(-30.0..=30., |x| {
                                if let Some(gain) = x {
                                    let db = util::gain_to_db(gain as f32);
                                    set_param!(set, &params.gain, db);
                                    gain
                                } else {
                                    util::gain_to_db(params.gain.value())
                                        as f64
                                }
                            })
                            .suffix("db"),
                        );
                    });
            },
        )
    }

    fn process(
        &mut self,
        buf: &mut Buffer,
        _: &mut AuxiliaryBuffers,
        ctx: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        for (id, samples) in buf.iter_samples().enumerate() {
            let gain = self.params.gain.smoothed.next();

            while let Some(ev) = ctx.next_event() {
                if ev.timing() > id as u32 {
                    break;
                }

                match ev {
                    NoteEvent::NoteOn {
                        note, velocity: v, ..
                    } => {
                        self.midi_note_id = note;
                        self.midi_note_freq = util::midi_note_to_freq(note);
                        self.set_gain_tar(v);
                    }

                    NoteEvent::NoteOff { note, .. }
                        if note == self.midi_note_id =>
                    {
                        self.set_gain_tar(0.);
                    }

                    NoteEvent::VoiceTerminated { note, .. }
                        if note == self.midi_note_id =>
                    {
                        self.set_gain_tar(0.);
                    }

                    NoteEvent::PolyPressure {
                        note, pressure: p, ..
                    } if note == self.midi_note_id => {
                        self.set_gain_tar(p);
                    }

                    _ => (),
                }
            }

            let sine =
                self.calc(self.midi_note_freq) * self.midi_note_gain.next();

            for s in samples {
                *s = sine * util::db_to_gain_fast(gain);
            }
        }

        ProcessStatus::KeepAlive
    }
}

impl Vst3Plugin for Graph {
    const VST3_CLASS_ID: [u8; 16] = *b"BSAgraph        ";
    const VST3_SUBCATEGORIES: &[Vst3SubCategory] = &[
        Vst3SubCategory::Instrument,
        Vst3SubCategory::Synth,
        Vst3SubCategory::Tools,
    ];
}

nih_export_vst3!(Graph);
