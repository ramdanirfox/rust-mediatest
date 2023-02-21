
#![cfg_attr(test, allow(unused_must_use))]

use libxm::{XMContext};
use std::io::Read;
use std::io::Cursor;
use std::io::Seek;
use std::io::Write;
use std::vec;
use bytemuck::{cast_slice, cast_ref};
use hound::{WavWriter, Error};

// extern crate openmpt;
use openmpt::module::{ctls, Module, Logger};

use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, Sink, source::Source};

use tokio::time;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    // Get a output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    // Load a sound from a file, using a path relative to Cargo.toml
    // let file = BufReader::new(File::open("examples/insidecol1.ogg").unwrap());
    // let wav_xm = renderXM();
    let wav_xm = render_open_mpt();
    // println!("Buffer {:?}", wav_xm);
    // let file_wav_xm = BufReader::new();
    // let cursor = Cursor::new(&wav_xm[..]);
    // let byte_slice: &[u8] = cast_slice(&wav_xm);
    // let byte_slice2 = byte_slice.to_owned();
    // let mut cursor = Cursor::new(byte_slice2);    
    // let mut file = File::create("examples/wave.wav").unwrap();
    // let byte_slice3 = byte_slice.to_owned();
    // file.write_all(&byte_slice3).unwrap();
    // file.flush().unwrap();

    let mut file = BufReader::new(wav_xm);
    file.seek(std::io::SeekFrom::Start(0)).expect("gagal geser");
    // let file = BufReader::new(File::open("examples/insidecol1.ogg").unwrap());
    // println!("Buffer2 {:?}", file);
    // Decode that sound file into a source
    let source = Decoder::new(file).unwrap();

    let sink = Sink::try_new(&stream_handle).unwrap();
    sink.append(source);

    // Play the sound directly on the device
    // stream_handle.play_raw(source.convert_samples());

    // The sound plays in a separate audio thread,
    // so we need to keep the main thread alive while it's playing.
    // std::thread::sleep(std::time::Duration::from_secs(60));

    println!("Does it let go?");
    let mut interval = time::interval(time::Duration::from_secs(1));
    for _i in 0..5 {
        interval.tick().await;
        task_that_takes_a_second(_i).await;
        let reducer = _i as f32/20 as f32;
        let spd = 1 as f32 + reducer;
        println!("set speed to: {}x", spd);
        sink.set_speed(spd);
    }

    
    // let type_id = std::any::TypeId::of::<dyn MyTrait>();
    // if type_id == my_object.type_id() {
    //     println!("type {}", std::any::type_name::<MyStruct>());
    // }
    // else {
    //     println!("Type mismatch!");
    // }
    // println!("type : {}", std::any::type_name_of_val(&wavXM));
    
    sink.sleep_until_end();

    // let test = render_file_to_wav("afairwarning.it");
    // println!("mencoba {}", test);
}

async fn task_that_takes_a_second(i: i32) {
    println!("playing: {}s", i);
    // time::sleep(time::Duration::from_secs(1)).await
}

fn render_open_mpt() -> Cursor<Vec<u8>> {
    // let wavbuffer = openmpt_render_to_buffer("");
    // let cursorbuffer = Cursor::new(wavbuffer);
    // return cursorbuffer;
    // return openmpt_render_to_buffer("examples/01_space_light.it");
    return openmpt_render_to_buffer("examples/slash - a fair warning.it");
}

fn renderXM() -> Cursor<Vec<u8>> {
    // Read the contents of the module into `data`
    let mut data = Vec::new();
    File::open("examples/song.xm").unwrap().read_to_end(&mut data).unwrap();

    let mut xm = XMContext::new(&data, 48000).unwrap();
    println!("name : {:?}", xm.loop_count());
    xm.set_max_loop_count(1);
    let mut buffer = [0.0; 4096];
    let mut vecbuf = Vec::new();
    while xm.loop_count() == 0 {
        // let a = xm.position();
        // println!("loopcnt {}", a.pattern_index);
        xm.generate_samples(&mut buffer);
        vecbuf.extend_from_slice(&buffer);
        // The buffer is filled with stereo PCM data. Use it for whatever you need...
    }
    println!("cobavec : {}", vecbuf.len());
    // write_f32_vector_to_file("examples/vecbuf.wav", &vecbuf).expect("Error gaes");
    // write_wav_file("examples/vechound.wav", &vecbuf).expect("Error gaes");
    let houndbuffer = write_wav_to_buffer(&vecbuf, 48000, 32);
    return houndbuffer;
    // let a = 9 as u8;
    // return Cursor::new(Vec::new());
    // return buffer;
    // The song has looped once.
}

fn openmpt_render_to_buffer(file_path : &str) -> Cursor<Vec<u8>> {
	let mut stream = File::open(file_path).expect("unable to open file");

    let init_ctls = [ctls::Ctl::PlaybackTempoFactor(1.5), ctls::Ctl::PlaybackPitchFactor(0.8)];
    // init_ctls[0].set(ctls::CtlParam::StereoSeparation, 2.0);
    // init_ctls[1].set(ctls::Ctl::PlaybackTempoFactor(1.5), 44100.0);

	let mut module = Module::create(&mut stream, Logger::StdErr, &init_ctls).unwrap();
	// let mut module = Module::create(&mut stream, Logger::StdErr, &[]).unwrap();

	let spec = hound::WavSpec {
		channels: 2,
		sample_rate: 44100,
		bits_per_sample: 32, // c_float is equivalent to f32
		sample_format: hound::SampleFormat::Float,
	};

	let out_file = String::from(file_path) + ".wav";
	
	// let mut writer = hound::WavWriter::create(out_file, spec).unwrap();
	let mut buffer = vec![0f32; 48000]; // 1 second at a time
    let mut buffer2 = Vec::new();
    let mut buffercup = Cursor::new(buffer2); // 1 second at a time
    let mut writer = hound::WavWriter::new(&mut buffercup, spec).unwrap();
    
	loop {
		let avail_samples = module.read_interleaved_float_stereo(
				48000, &mut buffer) << 1; // We're in interleaved stereo
		if avail_samples <= 0 { break; }

		for sample in &buffer[..avail_samples] {
			writer.write_sample(*sample);
		}
	}
    writer.finalize();
    return buffercup;
}

fn write_f32_vector_to_file(filename: &str, data: &Vec<f32>) -> std::io::Result<()> {
    let mut file = File::create(filename)?;
    for val in data.iter() {
        file.write_all(&val.to_le_bytes())?;
    }
    Ok(())
}

fn write_wav_file(filename: &str, data: &[f32]) -> Result<(), Error> {
    let spec = hound::WavSpec {
        channels: 2,
        sample_rate: 48000,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = WavWriter::create(filename, spec)?;

    for sample in data {
        let sample_i16 = (sample * i16::MAX as f32) as i16;
        writer.write_sample(sample_i16)?;
    }

    writer.finalize()?;
    Ok(())
}

fn write_wav_to_buffer(data: &[f32], sample_rate: u32, bits_per_sample: u16) -> Cursor<Vec<u8>> {
// fn write_wav_to_buffer(data: &[f32], sample_rate: u32, bits_per_sample: u16) -> Result<(), Error> {
    let spec = hound::WavSpec {
        channels: 2,
        sample_rate,
        bits_per_sample,
        sample_format: hound::SampleFormat::Float,
    };

    let mut buffer = Cursor::new(Vec::new());
    // let mut writer = WavWriter::create("examples/vechounddbg.wav", spec)?;
    let mut writer = WavWriter::new(&mut buffer, spec).unwrap();
    for sample in data {
        writer.write_sample(*sample).unwrap();
    }

    writer.finalize().unwrap();
    return buffer;
    // Ok(())
}