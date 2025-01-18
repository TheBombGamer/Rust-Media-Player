use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::{BufReader, Result};
use plotters::prelude::*;
use std::path::Path;
use gtk::prelude::*;
use gtk::{Button, Scale, Box as GtkBox, Window, WindowType};
use std::sync::{Arc, Mutex};

struct Equalizer {
    gains: Vec<f32>,
}

impl Equalizer {
    fn new(band_count: usize) -> Self {
        Self {
            gains: vec![1.0; band_count],
        }
    }

    fn apply(&self, samples: &mut [i16]) {
        for (i, sample) in samples.iter_mut().enumerate() {
            let band = (i % self.gains.len()) as f32;
            *sample = (*sample as f32 * self.gains[band as usize]) as i16;
        }
    }
}

fn main() -> Result<()> {
    let application = gtk::Application::new(Some("com.example.media_player"), Default::default());
    application.connect_activate(|app| {
        let window = Window::new(WindowType::Toplevel);
        window.set_title("Media Player with Equalizer");
        window.set_default_size(800, 600);
        
        let vbox = GtkBox::new(gtk::Orientation::Vertical, 5);
        window.add(&vbox);

        let equalizer = Arc::new(Mutex::new(Equalizer::new(5)));

        for i in 0..5 {
            let equalizer_clone = Arc::clone(&equalizer);
            let scale = Scale::new(gtk::Orientation::Horizontal, None);
            scale.set_range(0.0, 2.0);
            scale.set_value(1.0);
            scale.set_inverted(true);
            scale.connect_value_changed(move |s| {
                let mut eq = equalizer_clone.lock().unwrap();
                eq.gains[i] = s.get_value() as f32;
            });
            vbox.pack_start(&scale, false, false, 0);
        }

        let play_button = Button::with_label("Play");
        vbox.pack_start(&play_button, false, false, 0);
        
        play_button.connect_clicked(move |_| {
            let file_path = get_audio_file().unwrap();
            let (_stream, stream_handle) = OutputStream::try_default().unwrap();
            let sink = Sink::try_new(&stream_handle).unwrap();

            let file = File::open(&file_path).unwrap();
            let source = Decoder::new(BufReader::new(file)).unwrap();
            let mut samples: Vec<i16> = source.collect::<Result<Vec<i16>, _>>().unwrap();

            {
                let eq = equalizer.lock().unwrap();
                eq.apply(&mut samples);
            }

            sink.append(rodio::buffer::SamplesBuffer::new(1, 44100, samples));
            sink.sleep_until_end();
        });

        window.show_all();
    });

    application.run();
    Ok(())
}

fn get_audio_file() -> Result<String> {
    println!("Enter the path to the audio file (WAV format):");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let path = input.trim();
    if Path::new(path).exists() {
        Ok(path.to_string())
    } else {
        Err(std::io::Error::new(std::io::ErrorKind::NotFound, "File not found"))
    }
}
