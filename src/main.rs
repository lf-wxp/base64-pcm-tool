use base64::decode;
use byteorder::{ByteOrder, LittleEndian};
use rodio::{source::Source, OutputStream};
use std::collections::VecDeque;
use std::env;
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::path::Path;
use std::time::Duration;

struct PcmSource {
    data: VecDeque<i16>,
}

impl Iterator for PcmSource {
    type Item = i16;

    fn next(&mut self) -> Option<Self::Item> {
        self.data.pop_front()
    }
}

impl Source for PcmSource {
    fn current_frame_len(&self) -> Option<usize> {
        Some(self.data.len())
    }
    fn channels(&self) -> u16 {
        1
    }
    fn sample_rate(&self) -> u32 {
        16000
    }
    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <input_file> <output_file>", args[0]);
        std::process::exit(1);
    }

    let source = Path::new(&args[1]);
    let dest = Path::new(&args[2]);
    if let Ok(dest_path) = decode_pcm(source, dest) {
        play_pcm(dest_path);
    }
}

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn decode_pcm<'a>(source: &'a Path, dest: &'a Path) -> Result<&'a Path> {
    // 从文本文件中读取经过Base64编码的PCM数据
    let mut input_file = File::open(&source)?;
    let mut encoded_data = String::new();
    input_file.read_to_string(&mut encoded_data)?;

    // encoded_data.retain(|c| c.is_ascii_alphanumeric() || c == '+' || c == '/' || c == '=');

    // 解码Base64编码的PCM数据
    let decoded_data = decode(&encoded_data).expect("Failed to decode base64 data");

    // 将解码后的PCM数据写入到文件中
    let mut output_file = File::create(&dest)?;
    output_file.write_all(&decoded_data)?;
    println!("Decoded PCM data has been written to {:?}", dest);
    Ok(dest)
}

fn play_pcm(path: &Path) {
    
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    // 打开PCM文件
    let file = File::open(path).unwrap();
    let mut buf_reader = BufReader::new(file);

    // 设置音频属性
    let channels = 1; // 双声道
    let sample_rate = 16000; // 采样率
    let bits_per_sample = 16; // 每个样本的位数

    // 从PCM文件中读取数据
    let mut data = Vec::new();
    buf_reader.read_to_end(&mut data).unwrap();

    // 将PCM数据存储在VecDeque中
    let mut pcm_data = VecDeque::new();
    for value in data.chunks(2) {
        let sample = LittleEndian::read_i16(value);
        pcm_data.push_back(sample);
    }

    // 创建音频源
    let source = PcmSource { data: pcm_data };

    // 播放音频
    let _ = stream_handle.play_raw(source.convert_samples()).unwrap();

     // 计算持续时间
     let num_samples = data.len() / (channels as usize * 2);
     let duration = num_samples as u32 * 1000 / sample_rate;

    // 等待音频播放完成
    std::thread::sleep(Duration::from_millis(duration as u64));
}
