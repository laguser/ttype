use std::f32::consts::PI;
use std::io::Cursor;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};

// синтезатор звуков печатной машинки на лету :d
// чтобы не таскать с собой тяжелые wav-файлы в бинарнике
pub struct SoundEngine {
    _stream: Option<OutputStream>,
    handle: Option<OutputStreamHandle>,
    click_wav: Vec<u8>,
    space_wav: Vec<u8>,
    bell_wav: Vec<u8>,
    error_wav: Vec<u8>,
}

// собираем валидный pcm wav заголовок вручную O_0
fn generate_wav(samples: &[i16], sample_rate: u32) -> Vec<u8> {
    let mut header = Vec::with_capacity(44 + samples.len() * 2);
    let data_len = (samples.len() * 2) as u32;
    let file_len = 36 + data_len;

    header.extend_from_slice(b"RIFF");
    header.extend_from_slice(&file_len.to_le_bytes());
    header.extend_from_slice(b"WAVE");

    header.extend_from_slice(b"fmt ");
    header.extend_from_slice(&16u32.to_le_bytes());
    header.extend_from_slice(&1u16.to_le_bytes());  // pcm
    header.extend_from_slice(&1u16.to_le_bytes());  // mono
    header.extend_from_slice(&sample_rate.to_le_bytes());
    header.extend_from_slice(&(sample_rate * 2).to_le_bytes());
    header.extend_from_slice(&2u16.to_le_bytes());
    header.extend_from_slice(&16u16.to_le_bytes());

    header.extend_from_slice(b"data");
    header.extend_from_slice(&data_len.to_le_bytes());

    for &s in samples {
        header.extend_from_slice(&s.to_le_bytes());
    }

    header
}

impl SoundEngine {
    pub fn new() -> Self {
        // если аудиоустройства нет или alsa заблокирована, просто тихо живем дальше :3
        let (stream, handle) = match OutputStream::try_default() {
            Ok((s, h)) => (Some(s), Some(h)),
            Err(_) => (None, None),
        };

        let sample_rate = 44100;

        // резкий щелчок механической клавиши (~25ms)
        let click_samples = {
            let duration = 0.025;
            let n = (sample_rate as f32 * duration) as usize;
            let mut buf = Vec::with_capacity(n);
            for i in 0..n {
                let t = i as f32 / sample_rate as f32;
                let decay = (1.0 - (i as f32 / n as f32)).powf(3.0);
                let freq = 1200.0 - (t / duration) * 900.0;
                let phase = 2.0 * PI * freq * t;
                buf.push((phase.sin() * decay * 18000.0) as i16);
            }
            buf
        };

        // глухой стук пробела / каретки (~40ms)
        let space_samples = {
            let duration = 0.040;
            let n = (sample_rate as f32 * duration) as usize;
            let mut buf = Vec::with_capacity(n);
            for i in 0..n {
                let t = i as f32 / sample_rate as f32;
                let decay = (1.0 - (i as f32 / n as f32)).powf(2.5);
                let freq = 380.0 - (t / duration) * 260.0;
                let phase = 2.0 * PI * freq * t;
                buf.push((phase.sin() * decay * 22000.0) as i16);
            }
            buf
        };

        // латунный звоночек машинки при конце раунда (~350ms) d:
        let bell_samples = {
            let duration = 0.35;
            let n = (sample_rate as f32 * duration) as usize;
            let mut buf = Vec::with_capacity(n);
            for i in 0..n {
                let t = i as f32 / sample_rate as f32;
                let decay = (-6.0 * t).exp();
                let o1 = (2.0 * PI * 1760.0 * t).sin();
                let o2 = (2.0 * PI * 3520.0 * t).sin() * 0.4;
                let o3 = (2.0 * PI * 5280.0 * t).sin() * 0.15;
                buf.push(((o1 + o2 + o3) * decay * 14000.0) as i16);
            }
            buf
        };

        // противный зуммер на ошибку ):
        let error_samples = {
            let duration = 0.06;
            let n = (sample_rate as f32 * duration) as usize;
            let mut buf = Vec::with_capacity(n);
            for i in 0..n {
                let t = i as f32 / sample_rate as f32;
                let decay = (1.0 - (i as f32 / n as f32)).powf(1.8);
                let phase = 2.0 * PI * 130.0 * t;
                let s = if phase.sin() > 0.0 { 1.0 } else { -1.0 };
                buf.push((s * decay * 13000.0) as i16);
            }
            buf
        };

        Self {
            _stream: stream,
            handle,
            click_wav: generate_wav(&click_samples, sample_rate),
            space_wav: generate_wav(&space_samples, sample_rate),
            bell_wav: generate_wav(&bell_samples, sample_rate),
            error_wav: generate_wav(&error_samples, sample_rate),
        }
    }

    fn play_raw(&self, wav_bytes: &[u8], volume: f32) {
        if let Some(ref handle) = self.handle {
            let cursor = Cursor::new(wav_bytes.to_vec());
            if let Ok(source) = Decoder::new(cursor) {
                if let Ok(sink) = Sink::try_new(handle) {
                    sink.set_volume(volume);
                    sink.append(source);
                    sink.detach();
                }
            }
        }
    }

    pub fn play_click(&self) {
        self.play_raw(&self.click_wav, 0.45);
    }

    pub fn play_space(&self) {
        self.play_raw(&self.space_wav, 0.55);
    }

    pub fn play_bell(&self) {
        self.play_raw(&self.bell_wav, 0.50);
    }

    pub fn play_error(&self) {
        self.play_raw(&self.error_wav, 0.35);
    }
}
