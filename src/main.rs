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


    {
        let state_clone = state.clone();
        std::thread::spawn(move || {
            fft::start_fft_thread(sample_rx, state_clone, num_bins);
        });
    }

    println!("audio threads working and  fft init");


   // loop 
    //{
      //  if let Ok(st) = state.lock() 
        //{
         //   println!("{:?}",st.freq_bins);
        //}

        //std::thread::sleep(std::time::Duration::from_millis(100));
    //}

    //for _ in 0..1000 {
      //  if let Ok(s) = sample_rx.recv() 
        //{
          //  println!("Sample: {}",s);
        //}
    //}
    //
    //


    

    if let Err(err) = ui::start_ui_loop(state.clone())
    {
        eprintln!("UI crashed: {:?}",err);
    }
}


