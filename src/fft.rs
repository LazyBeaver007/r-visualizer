/* fft.rs
 * takes raw audio samples
 * converts them into frequency bins using rustfft
 * outputs like a vector: [low,mid,high]
 */

use rustfft::{FftPlanner, num_complex::Complex32};
use crossbeam_channel::{Receiver};
use crate::stale::{self, SharedState};

pub fn start_fft_thread(
    sample_rx:Receiver<f32>, state:SharedState, num_bins:usize
)
{
    std::thread::spawn(move||
        {
            let mut planner = FftPlanner::new();
            let fft = planner.plan_fft_forward(1024);

            let mut buffer = Vec::with_capacity(1024);


            loop {
                buffer.clear();
                for _ in 0..1024 {
                    match sample_rx.recv()
                    {
                        Ok(s) => buffer.push(Complex32::new(s,0.0)),
                        Err(_) => return,
                    }
                }

                fft.process(&mut buffer);


                let mut magnitudes = Vec::new();
                for c in &buffer[..buffer.len()/2]
                {
                    magnitudes.push(c.norm());
                }

                let bins = chunk_to_bins(&magnitudes, num_bins);

                if let Ok(mut st) = state.lock()
                {
                    st.freq_bins = bins;
                }
            }
        });
}



fn chunk_to_bins(data: &Vec<f32>, bins: usize)-> Vec<f32>
{
    let chunk = data.len()/bins;
    let mut out = vec![0.0;bins];

    for i in 0..bins
    {
        let start = i*chunk;
        let end = start+chunk;
        let slice = &data[start..end];
        out[i] = slice.iter().copied().sum::<f32>()/slice.len() as f32;
    }

    out


}
