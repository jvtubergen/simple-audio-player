use web_audio_api::context::{AudioContext, BaseAudioContext};
use web_audio_api::node::{AudioNode, AudioScheduledSourceNode};

use clap::Parser;
use std::error::Error;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

#[derive(Parser, Debug)] // requires `derive` feature
#[command(term_width = 0)] // Just to make testing across clap features easier
struct Args {

    /// Audio file path.
    /// (Allow invalid UTF-8 paths)
    #[arg(long, short, required = true, value_name = "FILE-PATH", value_hint = clap::ValueHint::DirPath)]
    file: Option<std::path::PathBuf>,

}


fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let args = Args::parse();
    // println!("{args:?}");

    let audio_file_path = args.file.unwrap();

    // set up the audio context with optimized settings for your hardware
    let context = AudioContext::default();

    // for background music, read from local file
    let file = std::fs::File::open(audio_file_path)?;
    let buffer = context.decode_audio_data_sync(file)?;

    // setup an AudioBufferSourceNode
    let mut src = context.create_buffer_source();
    let duration = buffer.duration();
    src.set_buffer(buffer);
    src.set_loop(false);

    // connect the audio nodes
    src.connect(&context.destination());

    // play the buffer
    src.start();


    // track audio playback is done
    let is_ended =  Arc::new(AtomicBool::new(false));
    let _is_ended = is_ended.clone();
    src.set_onended(move |event| {
        is_ended.store(true, Ordering::Relaxed);
    });

    // playback till done
    loop {
        std::thread::sleep(Duration::from_millis(100));
        if _is_ended.load(Ordering::Relaxed) {
            break;
        }
    }

    Ok(())
}

