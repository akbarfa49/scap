// This program is just a testing application
// Refer to `lib.rs` for the library source code

use image::{ImageBuffer, Rgba};
use scap::{
    capturer::{Area, Capturer, Options, Point, Size},
    frame::{self, Frame},
    targets,
};
use std::process;
use windows_capture::encoder::{
    AudioSettingsBuilder, ContainerSettingsBuilder, VideoEncoder, VideoSettingsBuilder,
};

#[tokio::main]
async fn main() {
    // Check if the platform is supported
    if !scap::is_supported() {
        println!("❌ Platform not supported");
        return;
    }

    // Check if we have permission to capture screen
    // If we don't, request it.
    if !scap::has_permission() {
        println!("❌ Permission not granted. Requesting permission...");
        if !scap::request_permission() {
            println!("❌ Permission denied");
            return;
        }
    }
    let search = "BLACK DESERT";
    // // Get recording targets
    let targets = scap::targets::get_all_targets();
    print!("{:?}", targets);
    let target = targets.iter().find_map(|target| match target {
        scap::targets::Target::Window(x) if x.title.contains(search) => Some(target.clone()),
        scap::targets::Target::Display(x) if x.title.contains(search) => Some(target.clone()),
        _ => None,
    });
    // Create Options
    let options = Options {
        fps: 1.0,
        show_cursor: true,
        show_highlight: true,
        excluded_targets: None,
        output_type: scap::frame::FrameType::BGRAFrame,
        output_resolution: scap::capturer::Resolution::_720p,
        crop_area: Some(Area {
            origin: Point { x: 0.0, y: 0.0 },
            size: Size {
                width: 1920.0,
                height: 1080.0,
            },
        }),
        target: target,
        ..Default::default()
    };

    // Create Recorder with options
    let mut recorder = Capturer::build(options).unwrap_or_else(|err| {
        println!("Problem with building Capturer: {err}");
        process::exit(1);
    });

    // Start Capture
    recorder.start_capture();

    // Capture 100 frames
    let mut start_time: u64 = 0;
    for i in 1..5 {
        println!("waiting for data");
        let frame = recorder.get_next_frame().await.expect("Error");
        println!("data arrived");
        match frame {
            Frame::YUVFrame(frame) => {
                println!(
                    "Recieved YUV frame {} of width {} and height {} and pts {}",
                    i, frame.width, frame.height, frame.display_time
                );
            }
            Frame::BGR0(frame) => {
                println!(
                    "Received BGR0 frame of width {} and height {}",
                    frame.width, frame.height
                );
            }
            Frame::RGB(frame) => {
                if start_time == 0 {
                    start_time = frame.display_time;
                }
                println!(
                    "Recieved RGB frame {} of width {} and height {} and time {}",
                    i,
                    frame.width,
                    frame.height,
                    frame.display_time - start_time
                );
            }
            Frame::RGBx(frame) => {
                println!(
                    "Recieved RGBx frame of width {} and height {}",
                    frame.width, frame.height
                );
            }
            Frame::XBGR(frame) => {
                println!(
                    "Recieved xRGB frame of width {} and height {}",
                    frame.width, frame.height
                );
            }
            Frame::BGRx(frame) => {
                println!(
                    "Recieved BGRx frame of width {} and height {}",
                    frame.width, frame.height
                );
            }
            Frame::BGRA(frame) => {
                if start_time == 0 {
                    start_time = frame.display_time;
                }
                println!(
                    "Recieved BGRA frame {} of width {} and height {} and time {}",
                    i,
                    frame.width,
                    frame.height,
                    frame.display_time - start_time
                );
                let bytes: Vec<u8> = frame::convert_bgra_to_rgb(frame.data);
                let width = frame.width;
                let height = frame.height;
                image::save_buffer(
                    format!("{}.png", i + 1),
                    &bytes,
                    width as u32,
                    height as u32,
                    image::ExtendedColorType::Rgb8,
                );
            }
            _ => {
                return;
            }
        }
    }
    // encoder.finish();
    // Stop Capture
    recorder.stop_capture();
    recorder.start_capture();
    for i in 6..10 {
        println!("waiting for data");
        let frame = recorder.get_next_frame().await.expect("Error");
        println!("data arrived");
        match frame {
            Frame::YUVFrame(frame) => {
                println!(
                    "Recieved YUV frame {} of width {} and height {} and pts {}",
                    i, frame.width, frame.height, frame.display_time
                );
            }
            Frame::BGR0(frame) => {
                println!(
                    "Received BGR0 frame of width {} and height {}",
                    frame.width, frame.height
                );
            }
            Frame::RGB(frame) => {
                if start_time == 0 {
                    start_time = frame.display_time;
                }
                println!(
                    "Recieved RGB frame {} of width {} and height {} and time {}",
                    i,
                    frame.width,
                    frame.height,
                    frame.display_time - start_time
                );
            }
            Frame::RGBx(frame) => {
                println!(
                    "Recieved RGBx frame of width {} and height {}",
                    frame.width, frame.height
                );
            }
            Frame::XBGR(frame) => {
                println!(
                    "Recieved xRGB frame of width {} and height {}",
                    frame.width, frame.height
                );
            }
            Frame::BGRx(frame) => {
                println!(
                    "Recieved BGRx frame of width {} and height {}",
                    frame.width, frame.height
                );
            }
            Frame::BGRA(frame) => {
                if start_time == 0 {
                    start_time = frame.display_time;
                }
                println!(
                    "Recieved BGRA frame {} of width {} and height {} and time {}",
                    i,
                    frame.width,
                    frame.height,
                    frame.display_time - start_time
                );
                let bytes: Vec<u8> = frame::convert_bgra_to_rgb(frame.data);
                let width = frame.width;
                let height = frame.height;
                image::save_buffer(
                    format!("{}.png", i + 1),
                    &bytes,
                    width as u32,
                    height as u32,
                    image::ExtendedColorType::Rgb8,
                );
            }
            _ => {
                return;
            }
        }
    }
    recorder.stop_capture();
}
