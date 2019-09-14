use std::io::{self, Read};

/// Read file to an array of byte.
pub fn read_to_array(arr : &mut [u8], file : &mut dyn Read) -> io::Result<()> {
    let mut read_length = 0;

    while read_length < arr.len() {
        let count = file.read(&mut arr[read_length..])?;
        if count == 0 {
            return Err(
                io::Error::new(io::ErrorKind::UnexpectedEof, 
                "Unexpected EOF")
            )
        }
        read_length += count;
    }

    Ok(())
}