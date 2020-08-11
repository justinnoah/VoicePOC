use std::path::Path;
use std::time::Duration;

use clap::{Arg, App};
use deepspeech::Model;
use sdl2 as sdl;
use sdl::audio::{AudioQueue, AudioSpecDesired};
use sdl::event::Event;
use sdl::keyboard::Keycode;
use sdl::pixels::Color;

const VERSION: &'static str = "0.0.0";

const ASPEC_DESIRED: AudioSpecDesired = AudioSpecDesired {
    channels: Some(1),
    freq: Some(16_000),
    samples: None,
};

fn send_version_info_to_stdout() {
    println!("Version: {}", VERSION);
    println!("SDL Version: {}", VERSION); // TODO
    println!("DeepSpeech Version: {}", VERSION); // TODO
}

fn main() {
    let options = App::new("realEdit")
                      .version(VERSION)
                      .author("Justin Noah <justinnoah+viagithub@gmail.com>")
                      .arg(Arg::with_name("list audio devices")
                           .long("lad")
                           .help("List the audio devices SDL can use")
                           .required(false)
                           .takes_value(false))
                      .arg(Arg::with_name("list video devices")
                           .long("lvd")
                           .help("List the video devices SDL can use")
                           .required(false)
                           .takes_value(false))
                      .arg(Arg::with_name("version")
                           .short("v")
                           .long("version")
                           .help("Send the version number to stdout")
                           .required(false)
                           .takes_value(false))
                      .get_matches();

    match options.is_present("version") {
        true => send_version_info_to_stdout(),
        _ => (),
    }

    // The following command line options will need sdl to be init'd
   let ctx = sdl::init().unwrap();

    // Init audio subsystem
    let asys = ctx.audio().unwrap();
    match options.is_present("list audio devices") {
        true => {
            println!("List of Recording Devices Available\n");
            let n = asys.get_num_devices(true).unwrap();
            for i in 0..n {
                println!("{}: {}", i, asys.get_device_name(i, true).unwrap());
            }
        }
        _ => (),
    }

    // Init video subsystem, gotta show some words
    let vsys = ctx.video().unwrap();
    let window = vsys.window("Demo", 800, 600)
        .opengl()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas()
        .build()
        .unwrap();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    // Thread unsafe way of doing things
    let mut m = Model::load_from_files(&Path::new("./deepspeech.pbmm")).unwrap();
    let _mic: AudioQueue<_> = asys.open_queue(
        None,
        true, // new parameter, iscapture
        &ASPEC_DESIRED).unwrap();
    let _spkr: sdl::audio::AudioQueue<i16> = asys.open_queue(
        None,
        false,
        &ASPEC_DESIRED).unwrap();

    // are we recording?
    let mut recording: bool = false;
    let mut event_pump = ctx.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), ..} => {
                    // Clear the microphone buffer/queue
                    _mic.clear();
                    drop(_mic);

                    // Clear the microphone buffer/queue
                    _spkr.clear();
                    drop(_spkr);

                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::Space), ..} => {
                    // If currently recording, stop the recording and send
                    // the data to deepspeech
                    match recording {
                        true => {
                            recording = false;
                            _mic.pause();

                            let detected_words = m.speech_to_text(
                                _mic.dequeue(_mic.size()).1.as_slice()
                            ).unwrap();
                            println!("Did you say?: {}", detected_words);
                        },
                        _ => {
                            recording = true;
                            _mic.resume();
                            println!("Yes? ");
                        },
                    }
                }
                _ => (),
            }
        }
        // 60Hz
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
