/* stale.rs
 * shared state of the app
 * holds the latest frequency data
 * animation params
 * thread-safe wrapper
 */

use std::sync::{Arc,Mutex};

pub struct AppState 
{
    pub freq_bins: Vec<f32>,
}

impl AppState 
    {
        pub fn new(size: usize)->Self 
        {
            Self {
                freq_bins: vec![0.0;size],
            }
        }
    }

pub type SharedState = Arc<Mutex<AppState>>;
