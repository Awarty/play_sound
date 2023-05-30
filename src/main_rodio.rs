
use std::{env};

use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, source::Source};
use rodio::*;
use rodio::cpal::traits::{HostTrait,DeviceTrait};

fn listHostDevices(){
   let host = cpal::default_host();
   let devices = host.output_devices().unwrap();
   for device in devices{ 
      let dev:rodio::Device = device.into();
      let devName:String=dev.name().unwrap();
      println!(" # Device : '{}'", devName);
   }
}

fn getOutputStream(device_name:&str) -> (OutputStream,OutputStreamHandle) {
   let host = cpal::default_host();
   let devices = host.output_devices().unwrap();
   let ( mut _stream, mut stream_handle) = OutputStream::try_default().unwrap();
   for device in devices{ 
      let dev:rodio::Device = device.into();
      let devName:String=dev.name().unwrap();
      if devName==device_name {
         println!("Device found: {}", devName);
         ( _stream, stream_handle) = OutputStream::try_from_device(&dev).unwrap();
      }
   }
   return (_stream,stream_handle);
}

fn play_sound(output_device: &str, sound_file: &str){
    //let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    //let (_stream, handle) = getOutputStream("hw:CARD=Headset,DEV=0");
    //let (_stream, handle) = getOutputStream("hw:CARD=PCH,DEV=0");
    //let (_stream, handle) = getOutputStream("hw:CARD=Set,DEV=0");
    //let (_stream, handle) = getOutputStream("hw:CARD=Device,DEV=0");
    let (_stream, handle) = getOutputStream(format!("hw:CARD={},DEV=0", output_device).as_str());
    let sink = rodio::Sink::try_new(&handle).unwrap();

    let file = std::fs::File::open(sound_file).unwrap();
    sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());

    sink.sleep_until_end();
}


fn get_arg<T: std::str::FromStr>(args: &Vec<String>, keys: Vec<&str>) -> Option<T> 
{
    for (i, arg) in args.iter().enumerate() 
        {
            for (_j, key) in keys.iter().enumerate()
                {
                    if arg == key && i < args.len() - 1 
                    {
                        if let Ok(ret) = args[i + 1].parse::<T>() 
                        {
                            return Some(ret);
                        }
                    }
                }
        }
    return None;
}

const ARG_KEY_DEVICE: &str = "--device";
const ARG_KEY_DEVICE_SHORT: &str = "-d";
const ARG_KEY_FILE: &str = "--file";
const ARG_KEY_FILE_SHORT: &str = "-f";

fn main() {
    
    let args: Vec<String> = env::args().collect();
    let output_device = get_arg::<String>(&args, vec!(ARG_KEY_DEVICE, ARG_KEY_DEVICE_SHORT));
    let sound_file = get_arg::<String>(&args, vec!(ARG_KEY_FILE, ARG_KEY_FILE_SHORT));

    match (output_device, sound_file) {
        (Some(device), Some(file)) => {
            play_sound(device.as_str(), file.as_str());
            println!("Playing sound...");
        }
        _ => {
            println!("Forgot some arguments.");
        }
    }
}
