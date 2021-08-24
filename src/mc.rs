use crate::serialize::buffer::*;
use std::net::TcpStream;

use crate::serialize::packet::Packet;
use crate::serialize::protocol::ProtocolVersion;
use crate::serialize::var::*;
use std::io::{Read, Write};

#[derive(Copy, Clone)]
#[repr(u32)]
pub enum BufferSize {
    Medium = 4096,
}

pub struct Connection {
    stream: TcpStream,
    ver: ProtocolVersion,
    compression: Option<i32>, // compression threshold
    chunk_size: BufferSize,
}

impl Connection {
    pub fn new(addr: String, ver: ProtocolVersion, chunk_size: BufferSize) -> Connection {
        Connection {
            stream: TcpStream::connect(addr).unwrap(),
            ver,
            compression: None,
            chunk_size,
        }
    }

    pub fn send_packet(&mut self, packet: &impl Packet) -> usize {
        // Write and prepend packet buffer with its length
        let mut buf = packet.serialize_with_id(&self.ver);

        let mut final_buf = ByteBuf::new();
        let mut total_len = 0;
        let mut uncompressed_len = 0;

        if self.compression_enabled() {
            // Compress the buffer and move it
            if buf.len() >= self.compression.unwrap() as usize {
                uncompressed_len = buf.len() as i32;
                let mut out = vec![0_u8];
                let mut compressor = flate2::Compress::new(flate2::Compression::fast(), true);
                let _ =
                    compressor.compress_vec(buf.as_slice(), &mut out, flate2::FlushCompress::None);
                *buf = ByteBuf::from(out.as_slice());
            }
            total_len += len_varint(uncompressed_len);
        }

        total_len += buf.len() as i32;

        final_buf.write_var_int(total_len);
        if self.compression_enabled() {
            final_buf.write_var_int(uncompressed_len);
        }
        final_buf.write_all(buf.as_slice()).unwrap();

        self.send_buffer(&final_buf)
    }

    pub fn compression_enabled(&self) -> bool {
        self.compression.is_some()
    }

    pub fn set_compression_threshold(&mut self, threshold: i32) {
        self.compression = Some(threshold);
    }

    pub fn send_buffer(&mut self, buf: &ByteBuf) -> usize {
        self.stream.write(buf.as_slice()).unwrap()
    }

    fn read_packet(&self, len: i32, buf: &mut ByteBuf) -> (i32, ByteBuf) {
        let mut compressed_len = 0;
        let mut compressed_len_len = 0;

        // Optionally read a compression value
        if self.compression_enabled() {
            compressed_len = buf.read_var_int().unwrap();
            compressed_len_len = len_varint(compressed_len);
        }

        // Read off everything except the compressed length VarInt (if it's there)
        let vec = buf.read_bytes((len - compressed_len_len) as usize).unwrap();

        // This buffer contains PacketID + Data
        let mut tmp_buf = ByteBuf::from(vec.as_slice());

        if compressed_len > 0 {
            let mut out = Vec::with_capacity(compressed_len as usize);
            let mut decompressor = flate2::Decompress::new(true);

            // zlib inflate into another slice
            decompressor
                .decompress_vec(vec.as_slice(), &mut out, flate2::FlushDecompress::Finish)
                .unwrap();

            log::debug!(
                "total_in: {} total_out: {}",
                decompressor.total_in(),
                decompressor.total_out()
            );

            // Replace the compressed `tmp_buf` with its uncompressed counterpart
            tmp_buf = ByteBuf::from(out.as_slice());
        }

        let id: i32 = tmp_buf.read_var_int().unwrap();
        if compressed_len > 0 {
            log::debug!(
                "compressed_len: {} Decompressed got id {:x}",
                compressed_len,
                id
            );
        }
        (id, tmp_buf)
    }

    pub fn read_packets(&mut self) -> Vec<(i32, ByteBuf)> {
        let mut slice = vec![0_u8; self.chunk_size as usize];
        let mut packets = Vec::new();
        let n_read = self.stream.read(&mut slice).unwrap();
        let mut buf = ByteBuf::from(&slice[..n_read]);

        while !buf.end() {
            let len = buf.read_var_int().unwrap(); // total packet length

            if !buf.has_readable_bytes(len as usize) {
                let mut rest = vec![0_u8; (len as usize) - buf.remaining()];
                self.stream.read_exact(rest.as_mut_slice()).unwrap();
                buf.write_all(rest.as_mut_slice()).unwrap();
            }
            let (id, pkt) = self.read_packet(len, &mut buf);
            log::debug!("id: {:x}", id);
            packets.push((id, pkt));
        }
        packets
    }
}
