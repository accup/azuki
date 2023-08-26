use std::{
    ffi::OsStr,
    fs::File,
    io::{stdin, stdout, BufReader, BufWriter, Read, Stdin, Stdout, Write},
    path::Path,
};

pub fn with_extension(path: Option<&str>, extension: &str) -> Option<String> {
    let Some(path) = path else { return None };

    let path = Path::new(&path);

    let path = if let Some(path_extension) = path.extension() {
        path.with_extension(format!(
            "{}.{}",
            path_extension.to_string_lossy(),
            extension
        ))
    } else {
        path.with_extension(extension)
    };

    Some(path.to_string_lossy().to_string())
}

pub fn without_extension(path: Option<&str>, extension: &str) -> Option<String> {
    let Some(path) = path else { return None };

    let path = Path::new(&path);
    let extension = OsStr::new(extension);

    let path = if let Some(path_extension) = path.extension() {
        if path_extension == extension {
            path.with_extension(OsStr::new(""))
        } else {
            path.to_path_buf()
        }
    } else {
        path.to_path_buf()
    };

    Some(path.to_string_lossy().to_string())
}

pub enum Reading {
    BufReader(BufReader<File>),
    Stdin(Stdin),
}

impl Reading {
    pub fn open(path: Option<&str>) -> std::io::Result<Self> {
        if let Some(path) = path {
            let file = File::open(path)?;

            let reader = BufReader::new(file);

            Ok(Self::BufReader(reader))
        } else {
            Ok(Self::Stdin(stdin()))
        }
    }

    pub fn read_data(&mut self) -> std::io::Result<Vec<u8>> {
        let mut buffer = vec![];
        self.read_to_end(&mut buffer)?;
        Ok(buffer)
    }
}

impl Read for Reading {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self {
            Self::BufReader(reader) => reader.read(buf),
            Self::Stdin(reader) => reader.read(buf),
        }
    }
}

pub enum Writing {
    BufWriter(BufWriter<File>),
    Stdout(Stdout),
}

impl Writing {
    pub fn create(path: Option<&str>) -> std::io::Result<Self> {
        if let Some(path) = path {
            let file = File::create(path)?;

            let writer = BufWriter::new(file);

            Ok(Self::BufWriter(writer))
        } else {
            Ok(Self::Stdout(stdout()))
        }
    }
}

impl Write for Writing {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self {
            Self::BufWriter(writer) => writer.write(buf),
            Self::Stdout(writer) => writer.write(buf),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        match self {
            Self::BufWriter(writer) => writer.flush(),
            Self::Stdout(writer) => writer.flush(),
        }
    }
}
