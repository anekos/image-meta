#![allow(unused_imports)]

use std::io::{BufRead, BufReader, Cursor, Read, Seek, SeekFrom, Take};

use byteorder::{LittleEndian, ReadBytesExt};

use crate::errors::{ImageError, ImageResult};

pub struct RiffReader<T: Read + Seek> {
    buffer: BufReader<T>,
    form_type: [u8; 4],
    remain: usize,
    skip_for: usize,
}

pub struct Chunk<'a> {
    skip_for: &'a mut usize,
    identifier: [u8; 4],
    buffer: Take<&'a mut dyn BufRead>,
}

impl<T: BufRead + Seek> RiffReader<T> {
    pub fn form_type(&self) -> &[u8; 4] {
        &self.form_type
    }

    pub fn open(buffer: T) -> ImageResult<Self> {
        let mut buffer = BufReader::new(buffer);

        let mut signature = [0u8; 4];
        buffer.read_exact(&mut signature)?;
        if &signature != b"RIFF" {
            return Err(ImageError::InvalidSignature);
        }

        let remain = buffer.read_u32::<LittleEndian>()? as usize;

        let mut form_type = [0u8; 4];
        buffer.read_exact(&mut form_type)?;
        let remain = remain.saturating_sub(4);

        Ok(RiffReader {
            buffer,
            form_type,
            remain,
            skip_for: 0,
        })
    }

    pub fn read_chunk<'a>(&'a mut self) -> ImageResult<Option<Chunk<'a>>> {
        if 0 < self.skip_for {
            self.buffer.seek(SeekFrom::Current(self.skip_for as i64))?;
        }

        if self.remain == 0 {
            return Ok(None);
        }

        let mut identifier = [0u8; 4];
        self.buffer.read_exact(&mut identifier)?;

        let size = self.buffer.read_u32::<LittleEndian>()? as usize;
        self.remain = self.remain.saturating_sub(size + 8);
        let buffer = (&mut self.buffer as &mut dyn BufRead).take(size as u64);
        self.skip_for = size;

        Ok(Some(Chunk {
            buffer,
            identifier,
            skip_for: &mut self.skip_for,
        }))
    }
}

impl<'a> Chunk<'a> {
    pub fn identifier(&self) -> &[u8; 4] {
        &self.identifier
    }
}

impl<'a> Read for Chunk<'a> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.buffer.read(buf).map(|it| {
            *self.skip_for -= it;
            it
        })
    }
}
