/*!
    Minimalist audio lib based on [rodio](https://github.com/tomaka/rodio).
*/

#![deny(missing_docs)]

extern crate rodio;

use rodio::Source;
use rodio::{Device, Sink};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Cursor, Seek, SeekFrom};

/// Enum to denote how the audio is stored
enum AudioType {
    /// The Audio is packed into the binary
    Packed(Vec<u8>),
    /// The Audio is loose on the filesystem
    Loose(File),
}

/// Struct representing the Audio system
pub struct Audio {
    /// The endpoint to play the audio from
    endpoint: Device,
    /// HashMap of available audio clips
    audio: HashMap<&'static str, AudioType>,
    /// HashMap of the playback channels that are associated with audio clips
    channels: HashMap<&'static str, Sink>,
}

/// Implementation of Default trait for Audio
impl Default for Audio {
    /// Creates a default instance of Audio
    fn default() -> Self {
        Self::new()
    }
}

impl Audio {
    /// Creates a new instance of the Audio system
    pub fn new() -> Audio {
        Audio {
            endpoint: rodio::default_output_device().unwrap(),
            audio: HashMap::new(),
            channels: HashMap::new(),
        }
    }

    /// Adds a piece of audio to the system
    pub fn add_audio(&mut self, name: &'static str, path: &'static str) {
        let file = File::open(path).unwrap();
        self.audio.insert(name, AudioType::Loose(file));
        self.channels.insert(name, Sink::new(&self.endpoint));
    }

    /// Add audio that is packed into the binary
    pub fn add_packed_audio(&mut self, name: &'static str, bytes: Vec<u8>) {
        self.audio.insert(name, AudioType::Packed(bytes));
        self.channels.insert(name, Sink::new(&self.endpoint));
    }

    /// Plays an piece of Audio that has been loaded into the system
    pub fn play(&mut self, name: &'static str) {
        let audio = &self.audio[name];

        match audio {
            &AudioType::Packed(ref audio) => {
                let audio = audio.clone();
                let cursor = Cursor::new(audio);
                self.channels[name].append(rodio::Decoder::new(BufReader::new(cursor)).unwrap());
            }
            &AudioType::Loose(ref audio) => {
                let mut audio = audio.try_clone().unwrap();
                audio.seek(SeekFrom::Start(0u64)).unwrap();
                self.channels[name].append(rodio::Decoder::new(BufReader::new(audio)).unwrap());
            }
        }
    }

    /// Loops a piece of Audio that has been loaded into the system
    pub fn repeat(&mut self, name: &'static str) {
        let audio = &self.audio[name];

        match audio {
            &AudioType::Packed(ref audio) => {
                let audio = audio.clone();
                let cursor = Cursor::new(audio);
                let decoder = rodio::Decoder::new(BufReader::new(cursor)).unwrap();
                self.channels[name].append(decoder.repeat_infinite());
            }
            &AudioType::Loose(ref audio) => {
                let mut audio = audio.try_clone().unwrap();
                audio.seek(SeekFrom::Start(0u64)).unwrap();
                let decoder = rodio::Decoder::new(BufReader::new(audio)).unwrap();
                self.channels[name].append(decoder.repeat_infinite());
            }
        }
    }
}
