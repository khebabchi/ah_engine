use rodio::{Decoder, OutputStream, Sink};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Error};
use std::sync::{Arc, Mutex};

#[derive(Debug)]
struct AudioPlayer {
    sink: Arc<Mutex<Sink>>, // Allows pausing/resuming
    audio_files: HashMap<String, String>, // Store file paths with names
    current_audio: Option<String>, // Track current audio
}

impl AudioPlayer {
    /// Create a new AudioPlayer
    fn new() -> Result<Self, Error> {
        let (_stream, stream_handle) = OutputStream::try_default()?;
        let sink = Sink::try_new(&stream_handle)?;

        Ok(Self {
            sink: Arc::new(Mutex::new(sink)),
            audio_files: HashMap::new(),
            current_audio: None,
        })
    }

    /// Add an audio file with a name
    fn add_audio(&mut self, name: &str, file_path: &str) {
        self.audio_files.insert(name.to_string(), file_path.to_string());
        println!("Added: {}", name);
    }

    /// Remove an audio file by name
    fn remove_audio(&mut self, name: &str) {
        if self.audio_files.remove(name).is_some() {
            println!("Removed: {}", name);
        } else {
            println!("Audio not found: {}", name);
        }
    }

    /// Set an audio track and prepare it for playback
    fn set_audio(&mut self, name: &str) -> Result<(), Error> {
        if let Some(file_path) = self.audio_files.get(name) {
            let file = File::open(file_path)?;
            let source = Decoder::new(BufReader::new(file))?;
            let sink = self.sink.lock().unwrap();

            sink.stop(); // Stop any currently playing audio
            sink.append(source);
            self.current_audio = Some(name.to_string());

            println!("Set audio: {}", name);
        } else {
            println!("Audio not found: {}", name);
        }
        Ok(())
    }

    /// Play the currently set audio
    fn play(&self) {
        let sink = self.sink.lock().unwrap();
        if let Some(audio) = &self.current_audio {
            println!("Playing: {}", audio);
            sink.play();
        } else {
            println!("No audio set!");
        }
    }

    /// Resume playing after pausing
    fn resume(&self) {
        let sink = self.sink.lock().unwrap();
        sink.play();
        println!("Resumed playback");
    }

    /// Pause playback
    fn pause(&self) {
        let sink = self.sink.lock().unwrap();
        sink.pause();
        println!("Paused playback");
    }

    /// Stop playback completely
    fn stop(&self) {
        let sink = self.sink.lock().unwrap();
        sink.stop();
        println!("Stopped playback");
    }
}