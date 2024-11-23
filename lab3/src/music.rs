use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, Mutex};

pub struct AudioPlayer {
    sink: Arc<Mutex<Sink>>,
    _stream: OutputStream,
    music_file: String,  // Store the file path for later use
}

impl AudioPlayer {
    pub fn new(music_file: &str) -> Self {
        let (stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        AudioPlayer {
            sink: Arc::new(Mutex::new(sink)),
            _stream: stream,
            music_file: music_file.to_string(), 
        }
    }

    pub fn play(&self) {
        let file = BufReader::new(File::open(&self.music_file).unwrap());
        let source = Decoder::new(file).unwrap();
        

        let mut sink = self.sink.lock().unwrap();
        sink.stop();
        sink.append(source);
        sink.play();
    }

    pub fn stop(&self) {
        self.sink.lock().unwrap().stop();
    }
}