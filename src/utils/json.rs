use std::{
    fs::{DirBuilder, File, OpenOptions},
    io::{BufReader, Write},
    path::{Path, PathBuf},
};

use anyhow::{anyhow, Result};

use serde::Serialize;
use serde_json::{from_reader, ser::PrettyFormatter, Serializer, Value};

pub fn read_json<T: AsRef<Path>>(path: T) -> Result<Value> {
    let json_reader = BufReader::new(File::open(path)?);
    Ok(from_reader(json_reader)?)
}

pub fn write_json<T: Serialize, U: AsRef<Path>>(path: U, value: T) -> Result<()> {
    let path = path.as_ref();
    let mut file = if !path.exists() {
        let dir = PathBuf::from(path);
        let dir = match dir.parent() {
            Some(dir) => dir,
            None => return Err(anyhow!("Invalid path.")),
        };
        DirBuilder::new().recursive(true).create(dir).unwrap();
        OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(false)
            .open(path)?
    } else {
        OpenOptions::new().write(true).truncate(true).open(path)?
    };
    let fmt = PrettyFormatter::with_indent(b"    ");
    let mut buf = Vec::new();
    let mut ser = Serializer::with_formatter(&mut buf, fmt);
    value.serialize(&mut ser)?;
    file.write_all(buf.as_slice())?;
    Ok(())
}
