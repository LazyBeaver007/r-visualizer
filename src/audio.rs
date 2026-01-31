/* audio.rs 
 * Handles system audio capture via WASAPI (cpal)
 * sends raw samples to a channel
 */

// stream: stream output
use cpal::traits::{DeviceTrait,HostTrait,StreamTrait};
use crossbeam_channel::Sender;
use anyhow::Result;

pub fn start_audio_capture(sample_tx: Sender<f32>)->Result<()>
{
    let host = cpal::default_host();

    let device = host
        .default_output_device()
        .ok_or_else(|| anyhow::anyhow!("No output device found"))?;

    let config = device.default_output_config()?.config();

    println!("Using device: {:?}",device.name()?);

    let stream = device.build_input_stream(
        &config, move |data: &[f32],_| 
        {
            for &sample in data {
                let _ = sample_tx.send(sample);
            }
        },

        move |err| 
        {
            eprintln!("Audio stream error: {}",err);
        },
        None,

    )?;


    stream.play()?;
    println!("Audio capture started");

    std::mem::forget(stream);

    Ok(())
}
