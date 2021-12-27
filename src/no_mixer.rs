use librespot_playback::mixer::{AudioFilter, Mixer, MixerConfig};
use std::sync::Mutex;

pub struct NoMixer {
    volume: Mutex<u16>,
}

impl Mixer for NoMixer {
    fn open(_: Option<MixerConfig>) -> NoMixer {
        NoMixer {
            // 50%, +1 because it rounds down.
            volume: Mutex::new(0xFFFF / 2 + 1),
        }
    }

    fn start(&self) {}

    fn stop(&self) {}

    fn volume(&self) -> u16 {
        *self.volume.lock().unwrap()
    }

    fn set_volume(&self, volume: u16) {
        *self.volume.lock().unwrap() = volume;
    }

    fn get_audio_filter(&self) -> Option<Box<dyn AudioFilter + Send>> {
        None
    }
}
