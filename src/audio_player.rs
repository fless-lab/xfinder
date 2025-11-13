// src/audio_player.rs
// Lecteur audio simple avec rodio

use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use anyhow::Result;

pub struct AudioPlayer {
    _stream: OutputStream,
    sink: Sink,
    current_file: Option<String>,
    is_playing: bool,
}

impl AudioPlayer {
    pub fn new() -> Result<Self> {
        let (stream, stream_handle) = OutputStream::try_default()?;
        let sink = Sink::try_new(&stream_handle)?;

        Ok(Self {
            _stream: stream,
            sink,
            current_file: None,
            is_playing: false,
        })
    }

    pub fn load_and_play(&mut self, file_path: &str) -> Result<()> {
        // Arrêter la lecture en cours
        self.sink.stop();

        // Charger le nouveau fichier
        let file = File::open(file_path)?;
        let source = Decoder::new(BufReader::new(file))?;

        self.sink.append(source);
        self.sink.play();

        self.current_file = Some(file_path.to_string());
        self.is_playing = true;

        Ok(())
    }

    pub fn pause(&mut self) {
        self.sink.pause();
        self.is_playing = false;
    }

    pub fn resume(&mut self) {
        self.sink.play();
        self.is_playing = true;
    }

    pub fn stop(&mut self) {
        self.sink.stop();
        self.current_file = None;
        self.is_playing = false;
    }

    pub fn is_playing(&self) -> bool {
        self.is_playing && !self.sink.empty()
    }

    pub fn current_file(&self) -> Option<&str> {
        self.current_file.as_deref()
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.sink.set_volume(volume.clamp(0.0, 1.0));
    }

    pub fn get_volume(&self) -> f32 {
        self.sink.volume()
    }
}

impl Default for AudioPlayer {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| {
            // Fallback si l'audio n'est pas disponible
            let (stream, stream_handle) = OutputStream::try_default().unwrap();
            let sink = Sink::try_new(&stream_handle).unwrap();
            Self {
                _stream: stream,
                sink,
                current_file: None,
                is_playing: false,
            }
        })
    }
}
