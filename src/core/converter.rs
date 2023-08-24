use std::{io::Write, marker::PhantomData};

pub trait Converter<'a, W: Write>
where
    Self: Sized,
{
    fn wrap(writer: &'a mut W) -> ConverterWriter<W, Self> {
        ConverterWriter {
            converter: Self::new(writer),
            phantom: PhantomData,
        }
    }

    fn new(writer: &'a mut W) -> Self;

    fn convert(&mut self, data: &[u8]) -> std::io::Result<()>;
}

pub struct ConverterWriter<'a, W: Write, C: Converter<'a, W>> {
    converter: C,
    phantom: PhantomData<&'a W>,
}

impl<'a, W: Write, C: Converter<'a, W>> Write for ConverterWriter<'a, W, C> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.converter.convert(buf)?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
