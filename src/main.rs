use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::SeekFrom;

fn main() -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open("TmForever.exe")?;
    file.seek(SeekFrom::Start(0x502a68))?;
    let mut buf = [0; 8];
    file.read_exact(&mut buf)?;
    let signature = vec![0x73, 0x0B, 0x5E, 0xB8, 0x01, 0x00, 0x00, 0x00];
    assert_eq!(signature, &buf);

    // patch out the check
    buf[4] = 0;
    file.seek(SeekFrom::Start(0x502a68))?;
    file.write(&buf)?;
    file.flush()?;
    Ok(())
}
