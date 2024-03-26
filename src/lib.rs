use std::net::UdpSocket;
use std::sync::Arc;

use nih_plug::prelude::*;
use rosc::encoder;
use rosc::{OscMessage, OscPacket, OscType};

const OSC_SERVER_ADDRESS: &str = "127.0.0.1:11000";

struct Midi2OSC {
    params: Arc<Midi2OSCParams>,
    soc: UdpSocket,
}

#[derive(Default, Params)]
struct Midi2OSCParams {}

impl Default for Midi2OSC {
    fn default() -> Self {
        Self {
            params: Arc::new(Midi2OSCParams::default()),
            soc: UdpSocket::bind("127.0.0.1:0").unwrap(),
        }
    }
}

impl Plugin for Midi2OSC {
    fn process(
        &mut self,
        _buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        while let Some(event) = context.next_event() {
            match event {
                NoteEvent::NoteOn {
                    timing: _,
                    voice_id: _,
                    channel,
                    note,
                    velocity: _,
                } => {
                    self.soc
                        .send_to(
                            &encoder::encode(&OscPacket::Message(OscMessage {
                                addr: "/live/clip/fire".to_string(),
                                args: vec![OscType::Int(channel as i32), OscType::Int(note as i32)],
                            }))
                            .unwrap(),
                            OSC_SERVER_ADDRESS,
                        )
                        .unwrap();
                }
                NoteEvent::NoteOff {
                    timing: _,
                    voice_id: _,
                    channel,
                    note,
                    velocity: _,
                } => {
                    self.soc
                        .send_to(
                            &encoder::encode(&OscPacket::Message(OscMessage {
                                addr: "/live/clip/stop".to_string(),
                                args: vec![OscType::Int(channel as i32), OscType::Int(note as i32)],
                            }))
                            .unwrap(),
                            OSC_SERVER_ADDRESS,
                        )
                        .unwrap();
                }
                _ => (),
            }
        }

        ProcessStatus::Normal
    }

    const NAME: &'static str = "rust-vst-midi2osc";
    const VENDOR: &'static str = "rust-vst-midi2osc";
    const URL: &'static str = "rust-vst-midi2osc";
    const EMAIL: &'static str = "rust-vst-midi2osc";
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[];
    const MIDI_INPUT: MidiConfig = MidiConfig::MidiCCs;
    const MIDI_OUTPUT: MidiConfig = MidiConfig::MidiCCs;
    const SAMPLE_ACCURATE_AUTOMATION: bool = true;
    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }
}

impl ClapPlugin for Midi2OSC {
    const CLAP_ID: &'static str = "rust-vst-midi2osc";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[ClapFeature::Utility];
}

impl Vst3Plugin for Midi2OSC {
    const VST3_CLASS_ID: [u8; 16] = *b"VSTCLAP_MIDI2OSC";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Instrument, Vst3SubCategory::Tools];
}

nih_export_clap!(Midi2OSC);
nih_export_vst3!(Midi2OSC);
