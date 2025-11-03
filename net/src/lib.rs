use crate::error::NetResult;
use bincode::{Decode, Encode};
use omnistat_core::types::digital_information::DigitalInformation;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tracing::debug;

mod error;

pub struct Message {
    length: u32,
    data: Vec<u8>,
}

impl Message {
    pub fn encode<T>(encodable: &T) -> NetResult<Self>
    where
        T: Encode,
    {
        let uncompressed = bincode::encode_to_vec(encodable, bincode::config::standard())?;
        let compressed = zstd::encode_all(uncompressed.as_slice(), 0)?;

        debug!(
            "Message encoded ({}) =zstd=> ({})",
            DigitalInformation::from_bytes(uncompressed.len()).format_pretty(),
            DigitalInformation::from_bytes(compressed.len()).format_pretty()
        );

        Ok(Self {
            length: compressed.len() as u32,
            data: compressed,
        })
    }

    pub fn decode<T>(self) -> NetResult<T>
    where
        T: Decode<()>,
    {
        let decompressed = zstd::decode_all(self.data.as_slice())?;
        let (decoded, _) = bincode::decode_from_slice(&decompressed, bincode::config::standard())?;

        debug!(
            "Message decoded ({}) =zstd=> ({})",
            DigitalInformation::from_bytes(self.data.len()).format_pretty(),
            DigitalInformation::from_bytes(decompressed.len()).format_pretty()
        );

        Ok(decoded)
    }

    pub async fn read<R>(reader: &mut R) -> NetResult<Self>
    where
        R: AsyncReadExt + Unpin,
    {
        let mut len_bytes = [0u8; 4];
        reader.read_exact(&mut len_bytes).await?;
        let length = u32::from_be_bytes(len_bytes);

        debug!(
            "Read message length: {}",
            DigitalInformation::from_bytes(length as usize).format_pretty()
        );

        let mut data = vec![0u8; length as usize];
        reader.read_exact(&mut data).await?;

        debug!(
            "Read message data: {}",
            DigitalInformation::from_bytes(data.len()).format_pretty()
        );

        Ok(Self { length, data })
    }

    pub async fn write<W>(&self, writer: &mut W) -> NetResult<()>
    where
        W: AsyncWriteExt + Unpin,
    {
        let len_bytes: [u8; 4] = self.length.to_be_bytes();
        writer.write_all(&len_bytes).await?;

        debug!(
            "Wrote message length: {}",
            DigitalInformation::from_bytes(self.length as usize).format_pretty()
        );

        writer.write_all(&self.data).await?;
        debug!(
            "Wrote message data: {}",
            DigitalInformation::from_bytes(self.data.len()).format_pretty()
        );

        Ok(())
    }
}

pub trait MessageSend {
    fn send<W>(&self, writer: &mut W) -> impl Future<Output = NetResult<()>>
    where
        W: AsyncWriteExt + Unpin;
}

pub trait MessageRecv {
    fn recv<R>(reader: &mut R) -> impl Future<Output = NetResult<Self>>
    where
        R: AsyncReadExt + Unpin,
        Self: Sized;
}

impl<T> MessageSend for T
where
    T: Encode,
{
    async fn send<W>(&self, writer: &mut W) -> NetResult<()>
    where
        W: AsyncWriteExt + Unpin,
    {
        let message = Message::encode(self)?;
        message.write(writer).await
    }
}

impl<T> MessageRecv for T
where
    T: Decode<()>,
{
    async fn recv<R>(reader: &mut R) -> NetResult<Self>
    where
        R: AsyncReadExt + Unpin,
    {
        let message = Message::read(reader).await?;
        message.decode()
    }
}
