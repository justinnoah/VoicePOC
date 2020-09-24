use std::path::Path;
use std::sync::mpsc::{Receiver, Sender, channel};
use std::thread;
use std::time::Duration;
use std::vec::Vec;

use clap::{Arg, App};
use deepspeech::Model;
use sdl2 as sdl;
use sdl::audio::{AudioQueue, AudioSpecDesired};
use sdl::event::Event;
use sdl::keyboard::Keycode;
use sdl::pixels::Color;
use sdl::rect::Rect;
use sdl::ttf;

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

fn init_deepspeech(to_main: Sender<String>, from_main: Receiver<Vec<i16>>) {
    let mut model = Model::load_from_files(&Path::new("./deepspeech.pbmm"))
                 .unwrap();
    model.enable_external_scorer(&Path::new("./deepspeech.scorer"));
    loop {
        match from_main.try_recv() {
            Ok(audio) => {
                let words = model.speech_to_text(&audio).unwrap();
                to_main.send(words).unwrap();
            },
            _ => {
                std::thread::sleep(Duration::from_millis(1));
            }
        }
    }
}

fn main() {
    let options = App::new("voice_poc")
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

    match options.is_present("list audio devices") {
        true => {
            println!("List of Recording Devices Available\n");
            let n = 0; // asys.get_num_devices(true).unwrap();
            for i in 0..n {
                // println!("{}: {}", i, asys.get_device_name(i, true).unwrap());
                println!("{}", i);
            }
        }
        _ => (),
    }


    let (ds_send, main_recv) = channel();
    let (main_send, ds_recv) = channel();
    thread::spawn(move|| {
        init_deepspeech(ds_send, ds_recv);
    });

    // The following command line options will need sdl to be init'd
   let ctx = sdl::init().unwrap();

    // Init audio subsystem
    let asys = ctx.audio().unwrap();

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
    let mic: AudioQueue<i16> = asys.open_queue(
        None,
        true, // new parameter, iscapture
        &ASPEC_DESIRED).unwrap();

    // Create a TTF Context and use the open window
    let f_ctx = ttf::init().unwrap();

    let font = f_ctx.load_font(
        &Path::new("fonts/Digitalt-04no.ttf"), 72).unwrap();
    let font_color = Color::RGB(255, 255, 255);
    let tc = canvas.texture_creator();

    // are we recording?
    let mut recording: bool = false;
    let mut event_pump = ctx.event_pump().unwrap();
    'running: loop {
        match main_recv.try_recv() {
            Ok(words) => {
                let rect_size = font.size_of(&words).unwrap();
                let (x, _) = canvas.window().size();
                let words_surface = font.render(&words).blended_wrapped(font_color, x);
                match words_surface {
                    Ok(surface) => {
                        let texture = tc.create_texture_from_surface(surface).unwrap();
                        canvas.copy(
                            &texture,
                            None,
                            Some(Rect::new(0, 0, rect_size.0, rect_size.1))).unwrap();
                    },
                    _ => (),
                }
            },
            _ => (),
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), ..} => {
                    // Clear the microphone buffer/queue
                    mic.clear();

                    // Peace out
                    drop(mic);
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::Space), ..} => {
                    // If currently recording, stop the recording and send
                    // the data to deepspeech
                    match recording {
                        true => {
                            recording = false;
                            mic.pause();

                            let raw_audio = mic.dequeue(mic.size()).1;
                            main_send.send(raw_audio).unwrap();
                            canvas.clear();
                        },
                        false => {
                            recording = true;
                            mic.resume();
                            println!("Yes? ");
                        },
                    }
                }
                _ => (),
            }
        }
        canvas.present();

        // 60Hz
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
