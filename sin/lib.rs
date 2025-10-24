use nih_plug::prelude::*;
use std::{f32::consts, sync::Arc};

pub struct Sin {
    params: Arc<SinParams>,
    sample_rate: f32,
    phase: f32,
    midi_note_id: u8,
    midi_note_freq: f32,
    midi_note_gain: Smoother<f32>,
}

impl Default for Sin {
    fn default() -> Self {
        Self {
            params: Arc::new(SinParams::default()),
            sample_rate: 1.,
            phase: 0.,
            midi_note_id: 0,
            midi_note_freq: 1.,
            midi_note_gain: Smoother::new(SmoothingStyle::Linear(5.)),
        }
    }
}

#[derive(Params)]
struct SinParams {
    #[id = "gain"]
    pub gain: FloatParam,
    #[id = "freq"]
    pub freq: FloatParam,
}

impl Default for SinParams {
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
            freq: FloatParam::new(
                "Freq",
                420.,
                FloatRange::Skewed {
                    min: 1.,
                    max: 20_000.,
                    factor: FloatRange::skew_factor(-2.),
                },
            )
            .with_smoother(SmoothingStyle::Linear(10.))
            .with_value_to_string(formatters::v2s_f32_hz_then_khz(0))
            .with_string_to_value(formatters::s2v_f32_hz_then_khz()),
        }
    }
}

impl Sin {
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

impl Plugin for Sin {
    const NAME: &str = "Bleedsa Sine";
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

    fn process(
        &mut self,
        buf: &mut Buffer,
        _: &mut AuxiliaryBuffers,
        ctx: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {

        for (id, samples) in buf.iter_samples().enumerate() {
            let gain = self.params.gain.smoothed.next();
            let sine = {
                while let Some(e) = ctx.next_event() {
                    if e.timing() > id as u32 {
                        break;
                    }

                    match e {
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
                        NoteEvent::PolyPressure {
                            note, pressure: p, ..
                        } if note == self.midi_note_id => {
                            self.set_gain_tar(p);
                        }
                        _ => (),
                    }
                }

                self.calc(self.midi_note_freq) * self.midi_note_gain.next()
            };

            for s in samples {
                *s = sine * util::db_to_gain_fast(gain);
            }
        }

        ProcessStatus::KeepAlive
    }
}

impl Vst3Plugin for Sin {
    const VST3_CLASS_ID: [u8; 16] = *b"BleedsaSin      ";
    const VST3_SUBCATEGORIES: &[Vst3SubCategory] = &[
        Vst3SubCategory::Instrument,
        Vst3SubCategory::Synth,
        Vst3SubCategory::Tools,
    ];
}

nih_export_vst3!(Sin);
