use serde_json::Value;

use super::filters::channel_mix::ChannelMix;
use super::filters::chorus::Chorus;
use super::filters::compressor::Compressor;
use super::filters::distortion::Distortion;
use super::filters::echo::Echo;
use super::filters::equalizer::Equalizer;
use crate::types::filters::BandSetting;
use super::filters::highpass::Highpass;
use super::filters::karaoke::Karaoke;
use super::filters::lowpass::Lowpass;
use super::filters::phaser::Phaser;
use super::filters::rotation::Rotation;
use super::filters::timescale::Timescale;
use super::filters::tremolo::Tremolo;
use super::filters::vibrato::Vibrato;

pub struct FiltersManager {
    timescale: Timescale,
    tremolo: Tremolo,
    vibrato: Vibrato,
    lowpass: Lowpass,
    highpass: Highpass,
    rotation: Rotation,
    karaoke: Karaoke,
    distortion: Distortion,
    channel_mix: ChannelMix,
    equalizer: Equalizer,
    chorus: Chorus,
    compressor: Compressor,
    echo: Echo,
    phaser: Phaser,
}

impl FiltersManager {
    pub fn new() -> Self {
        Self {
            timescale: Timescale::new(),
            tremolo: Tremolo::new(),
            vibrato: Vibrato::new(),
            lowpass: Lowpass::new(),
            highpass: Highpass::new(),
            rotation: Rotation::new(),
            karaoke: Karaoke::new(),
            distortion: Distortion::new(),
            channel_mix: ChannelMix::new(),
            equalizer: Equalizer::new(),
            chorus: Chorus::new(),
            compressor: Compressor::new(),
            echo: Echo::new(),
            phaser: Phaser::new(),
        }
    }

    pub fn update(&mut self, options: &Value) {
        let filters = options.get("filters").unwrap_or(options);

        // Helper to get float
        let get_f32 =
            |obj: &Value, key: &str| obj.get(key).and_then(|v| v.as_f64()).map(|v| v as f32);
        let get_usize =
            |obj: &Value, key: &str| obj.get(key).and_then(|v| v.as_u64()).map(|v| v as usize);

        // Timescale
        if let Some(cfg) = filters.get("timescale") {
            self.timescale.update(
                get_f32(cfg, "speed"),
                get_f32(cfg, "pitch"),
                get_f32(cfg, "rate"),
            );
        }

        // Tremolo
        if let Some(cfg) = filters.get("tremolo") {
            self.tremolo
                .update(get_f32(cfg, "frequency"), get_f32(cfg, "depth"));
        }

        // Vibrato
        if let Some(cfg) = filters.get("vibrato") {
            self.vibrato
                .update(get_f32(cfg, "frequency"), get_f32(cfg, "depth"));
        }

        // Lowpass
        if let Some(cfg) = filters.get("lowpass") {
            self.lowpass.update(get_f32(cfg, "smoothing"));
        }

        // Highpass
        if let Some(cfg) = filters.get("highpass") {
            self.highpass.update(get_f32(cfg, "smoothing"));
        }

        // Rotation
        if let Some(cfg) = filters.get("rotation") {
            self.rotation.update(get_f32(cfg, "rotationHz"));
        }

        // Karaoke
        if let Some(cfg) = filters.get("karaoke") {
            self.karaoke.update(
                get_f32(cfg, "level"),
                get_f32(cfg, "monoLevel"),
                get_f32(cfg, "filterBand"),
                get_f32(cfg, "filterWidth"),
            );
        }

        // Distortion
        if let Some(cfg) = filters.get("distortion") {
            self.distortion.update(
                get_f32(cfg, "sinOffset"),
                get_f32(cfg, "sinScale"),
                get_f32(cfg, "cosOffset"),
                get_f32(cfg, "cosScale"),
                get_f32(cfg, "tanOffset"),
                get_f32(cfg, "tanScale"),
                get_f32(cfg, "offset"),
                get_f32(cfg, "scale"),
            );
        }

        // ChannelMix
        if let Some(cfg) = filters.get("channelMix") {
            self.channel_mix.update(
                get_f32(cfg, "leftToLeft"),
                get_f32(cfg, "leftToRight"),
                get_f32(cfg, "rightToLeft"),
                get_f32(cfg, "rightToRight"),
            );
        }

        // Equalizer
        if let Some(cfg) = filters.get("equalizer") {
            if let Some(bands_arr) = cfg.as_array() {
                let bands: Vec<BandSetting> = bands_arr
                    .iter()
                    .map(|b| BandSetting {
                        band: get_usize(b, "band").unwrap_or(0),
                        gain: get_f32(b, "gain").unwrap_or(0.0),
                    })
                    .collect();
                self.equalizer.update(&bands);
            }
        }

        // Chorus
        if let Some(cfg) = filters.get("chorus") {
            self.chorus.update(
                get_f32(cfg, "rate"),
                get_f32(cfg, "depth"),
                get_f32(cfg, "delay"),
                get_f32(cfg, "mix"),
                get_f32(cfg, "feedback"),
            );
        }

        // Compressor
        if let Some(cfg) = filters.get("compressor") {
            self.compressor.update(
                get_f32(cfg, "threshold"),
                get_f32(cfg, "ratio"),
                get_f32(cfg, "attack"),
                get_f32(cfg, "release"),
                get_f32(cfg, "gain"),
            );
        }

        // Echo
        if let Some(cfg) = filters.get("echo") {
            self.echo.update(
                get_f32(cfg, "delay"),
                get_f32(cfg, "feedback"),
                get_f32(cfg, "mix"),
            );
        }

        // Phaser
        if let Some(cfg) = filters.get("phaser") {
            self.phaser.update(
                get_usize(cfg, "stages"),
                get_f32(cfg, "rate"),
                get_f32(cfg, "depth"),
                get_f32(cfg, "feedback"),
                get_f32(cfg, "mix"),
                get_f32(cfg, "minFrequency"),
                get_f32(cfg, "maxFrequency"),
            );
        }
    }

    pub fn process(&mut self, chunk: &[i16]) -> Vec<i16> {
        // Priority 1: Timescale
        // It consumes input and produces output (possibly different size).
        let mut buffer = self.timescale.process(chunk);

        // Priority 10: The rest
        // Applied in-place on the buffer.

        // Tremolo
        self.tremolo.process(&mut buffer);

        // Vibrato
        self.vibrato.process(&mut buffer);

        // Lowpass
        self.lowpass.process(&mut buffer);

        // Highpass
        self.highpass.process(&mut buffer);

        // Rotation
        self.rotation.process(&mut buffer);

        // Karaoke
        self.karaoke.process(&mut buffer);

        // Distortion
        self.distortion.process(&mut buffer);

        // ChannelMix
        self.channel_mix.process(&mut buffer);

        // Equalizer
        self.equalizer.process(&mut buffer);

        // Chorus
        self.chorus.process(&mut buffer);

        // Compressor
        self.compressor.process(&mut buffer);

        // Echo
        self.echo.process(&mut buffer);

        // Phaser
        self.phaser.process(&mut buffer);

        buffer
    }
}
