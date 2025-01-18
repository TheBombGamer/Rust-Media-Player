use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::{BufReader, Result};
use plotters::prelude::*;
use std::path::Path;

fn main() -> Result<()> {
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&stream_handle)?;

    let file_path = get_audio_file()?;
    visualize_audio(&file_path)?;

    let file = File::open(&file_path)?;
    let source = Decoder::new(BufReader::new(file))?;
    sink.append(source);
    println!("Playing audio...");
    sink.sleep_until_end();

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

fn visualize_audio(file_path: &str) -> Result<()> {
    let mut reader = hound::WavReader::open(file_path)?;
    let samples: Vec<i16> = reader.samples::<i16>().filter_map(Result::ok).collect();

    let root = BitMapBackend::new("waveform.png", (800, 400)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Waveform", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0..samples.len(), i16::MIN..i16::MAX)?;

    chart
        .configure_mesh()
        .x_desc("Sample Index")
        .y_desc("Amplitude")
        .draw()?;

    chart.draw_series(LineSeries::new(
        samples.iter().enumerate().map(|(x, y)| (x, *y)),
        &RED,
    ))?;

    root.present()?;
    println!("Waveform saved to waveform.png");
    Ok(())
}
