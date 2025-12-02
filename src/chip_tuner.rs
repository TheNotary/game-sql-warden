use openmpt::module::Module;
use rodio::{OutputStreamBuilder, Sink};
use std::fs;
use std::path::Path;
use std::thread;

struct ModulePlayer {
    module: Module,
    sample_rate: i32,
}

impl ModulePlayer {
    fn new(file_path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        // Read the .it file
        let mut file_data = fs::read(file_path)?;

        // Create OpenMPT module from file data
        let Ok(module) =
            Module::create_from_memory(&mut file_data, openmpt::module::Logger::None, &[])
        else {
            return Err("khihi".into());
        };

        let sample_rate = 48000;

        Ok(ModulePlayer {
            module,
            sample_rate,
        })
    }

    fn play_continuous(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Open the default output stream using the new API
        let stream = OutputStreamBuilder::open_default_stream()
            .map_err(|e| format!("Failed to open audio stream: {}", e))?;
        let mixer = stream.mixer();
        let sink = Sink::connect_new(mixer);

        // Enable repeat
        self.module.set_repeat_count(-1); // -1 means infinite repeat

        loop {
            // Generate interleaved audio samples (L, R, L, R, ...)
            let mut interleaved = vec![0.0f32; 4096 * 2];

            // Read stereo samples from module
            let count = self
                .module
                .read_interleaved_float_stereo(self.sample_rate, &mut interleaved);

            if count == 0 {
                // If no samples were read, restart the module
                self.module.set_position_seconds(0.0);
                continue;
            }

            // Trim to actual number of samples read
            interleaved.truncate(count * 2);

            // Create a source from the samples
            let source = rodio::buffer::SamplesBuffer::new(
                2, // channels
                self.sample_rate as u32,
                interleaved,
            );

            sink.append(source);

            // Wait a bit to avoid overwhelming the sink
            while sink.len() > 5 {
                std::thread::sleep(std::time::Duration::from_millis(10));
            }
        }
    }
}

pub fn play_chip() -> Result<(), Box<dyn std::error::Error>> {
    let file_path_str = "music/glitch13-8bit.it";
    let file_path = Path::new(file_path_str);

    if !file_path.exists() {
        eprintln!("Error: File '{}' not found", file_path_str);
    }

    let _handle = thread::spawn(|| {
        if let Ok(mut player) = ModulePlayer::new(file_path) {
            let _ = player.play_continuous();
        }
    });

    Ok(())
}
