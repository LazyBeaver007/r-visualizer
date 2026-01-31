mod audio;
mod fft;
mod stale;
mod ui;


use stale::{AppState,SharedState};
use std::sync::{Arc,Mutex};
use crossbeam_channel::unbounded;

fn main() {
    println!("Hello, world!");


    let num_bins = 64;

    let state:SharedState=Arc::new(Mutex::new(AppState::new(num_bins)));

    println!("App state initialized");

    let (sample_tx, sample_rx) = unbounded::<f32>();

    {
        let sample_tx = sample_tx.clone();
        std::thread::spawn(move || {
            if let Err(err) = audio::start_audio_capture(sample_tx)
            {
                eprintln!("Audio thread failed: {:?}",err);
            }
        });
    }

    println!("audio threads working");

    for _ in 0..1000 {
        if let Ok(s) = sample_rx.recv() 
        {
            println!("Sample: {}",s);
        }
    }

}


