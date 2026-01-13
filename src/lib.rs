use anyhow::{Context, Result};
use std::fs;
use std::path::Path;
use rodio::Source;
use rodio::buffer::SamplesBuffer;

/// Play an MP3 file using the default audio device
///
/// # Arguments
/// * `file_path` - Path to MP3 file to play
///
/// # Returns
/// * Success if playback completes, error if playback fails
pub fn play_mp3(file_path: &Path, start: Option<f64>, duration: Option<f64>) -> Result<()> {
    // Verify file exists
    fs::metadata(file_path)
        .with_context(|| format!("Failed to access: {}", file_path.display()))?;

    // Get default audio output stream
    let (_stream, stream_handle) = rodio::OutputStream::try_default()
        .context("Failed to initialize audio output device")?;

    let sink = rodio::Sink::try_new(&stream_handle)
        .context("Failed to create audio sink")?;

    // Open file and decode as MP3
    let file = fs::File::open(file_path)
        .with_context(|| format!("Failed to open: {}", file_path.display()))?;

    // If no start/end specified, play the whole file via Decoder
    if start.is_none() && duration.is_none() {
        let source = rodio::Decoder::new(file)
            .context("Failed to decode MP3 - ensure file is valid MP3 format")?;

        // Get duration if available
        let total_dur: Option<std::time::Duration> = source.total_duration();

        // Append source to sink and play
        sink.append(source);

        // Display playback info
        if let Some(dur) = total_dur {
            println!(
                "Playing: {} (Duration: {:.1}s)",
                file_path.display(),
                dur.as_secs_f64()
            );
        } else {
            println!("Playing: {}", file_path.display());
        }

        // Wait until playback completes
        sink.sleep_until_end();
        return Ok(());
    }

    // Range playback: decode samples, skip until start, collect until end
    let decoder = rodio::Decoder::new(file)
        .context("Failed to decode MP3 - ensure file is valid MP3 format")?;

    let sample_rate = decoder.sample_rate();
    let channels = decoder.channels();

    // Compute sample indices (samples are interleaved per channel)
    let start_sec = start.unwrap_or(0.0).max(0.0);
    let start_samples = start_sec * (sample_rate as f64 * channels as f64);
    // If duration is specified, interpret as duration seconds -> compute end sample index
    let end_samples = duration.map(|d| {
        let dur = d.max(0.0);
        ((start_sec + dur) * (sample_rate as f64 * channels as f64)) as usize
    });
    let start_idx = start_samples as usize;

    // Collect samples (i16) from decoder iterator
    // Note: Decoder implements Iterator over samples of type i16 (signed 16-bit)
    let mut collected: Vec<i16> = Vec::new();
    for (i, sample) in decoder.into_iter().enumerate() {
        if i < start_idx {
            continue;
        }
        if let Some(es) = end_samples {
            if i >= es {
                break;
            }
        }
        collected.push(sample);
    }

    if collected.is_empty() {
        // Nothing to play
        println!("No audio in specified range for {}", file_path.display());
        return Ok(());
    }

    // Build a SamplesBuffer and play
    let buffer = SamplesBuffer::new(channels, sample_rate, collected);
    if let Some(dur) = duration {
        println!(
            "Playing: {} (from {:.3}s, duration {:.3}s)",
            file_path.display(),
            start_sec,
            dur
        );
    } else {
        println!("Playing: {} (from {:.3}s to end)", file_path.display(), start_sec);
    }

    sink.append(buffer);
    sink.sleep_until_end();

    Ok(())
}
