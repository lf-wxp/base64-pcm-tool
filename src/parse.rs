use base64::{
  engine::general_purpose,
  Engine as _,
};
use rodio::{source::Source, OutputStream};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::time::Duration;

pub type Error = Box<dyn std::error::Error>;
pub type UResult<T> = std::result::Result<T, Error>;

pub fn decode_pcm<'a>(source: &'a Path) -> UResult<Vec<u8>> {
  let mut input_file = File::open(&source)?;
  let mut encoded_data = String::new();
  input_file.read_to_string(&mut encoded_data)?;
  // encoded_data.retain(|c| c.is_ascii_alphanumeric() || c == '+' || c == '/' || c == '=');
  // 解码Base64编码的PCM数据
  Ok(general_purpose::STANDARD.decode(&encoded_data)?)

}

pub fn decode_pcm_save<'a>(source: &'a Path, dest: &'a Path) -> UResult<&'a Path> {
  let decoded_data = decode_pcm(source)?;
  let mut output_file = File::create(&dest)?;
  output_file.write_all(&decoded_data)?;
  println!("Decoded PCM data has been written to {:?}", dest);
  Ok(dest)
}

fn calculate_duration(pcm_data: &[u8], sample_rate: u32, channels: u32) -> Duration {
  let bytes_per_sample = 2; // 16位数据，即2字节
  let total_samples = pcm_data.len() / bytes_per_sample;
  let duration_secs = total_samples as f64 / (sample_rate * channels) as f64;
  Duration::from_secs_f64(duration_secs)
}

fn convert_pcm_to_i16(pcm_data: &[u8], bits_per_sample: u16) -> Vec<i16> {
  match bits_per_sample {
    8 => pcm_data
      .iter()
      .map(|sample| ((*sample as i16) - 128) * 256)
      .collect(),

    16 => pcm_data
      .chunks_exact(2)
      .map(|chunk| i16::from_le_bytes([chunk[0], chunk[1]]))
      .collect(),

    24 => pcm_data
      .chunks_exact(3)
      .map(|chunk| {
        let sample = i32::from_le_bytes([chunk[0], chunk[1], chunk[2], 0]);
        (sample >> 8) as i16
      })
      .collect(),

    _ => panic!("Unsupported bits per sample: {}", bits_per_sample),
  }
}

pub fn play_pcm(path: &Path, channels: u16, sample_rate: u32, bits: u16) -> UResult<()> {
  let (_stream, stream_handle) = OutputStream::try_default().unwrap();
  let data = decode_pcm(&path)?;

  // 将PCM数据转换为i16样本
  let samples = convert_pcm_to_i16(&data, bits);

  // 创建一个SamplesBuffer，以根据声道数、采样率和样本数据播放PCM数据
  let samples_buffer = rodio::buffer::SamplesBuffer::new(channels, sample_rate, samples);

  // 将SamplesBuffer附加到输出流并播放
  stream_handle
    .play_raw(samples_buffer.convert_samples())
    .unwrap();

  let duration = calculate_duration(&data, sample_rate, channels as u32);

  // 等待音频播放完成
  std::thread::sleep(duration);
  Ok(())
}
