//! Common requirements for crate consumers

pub use crate::{Error, Sx127x};

pub use crate::device::fsk::{FskChannel, FskConfig};
pub use crate::device::lora::{LoRaChannel, LoRaConfig};
pub use crate::device::{Channel, Config, Modem, PacketInfo, State};

pub use radio::{Receive as ReceiveTrait, State as StateTrait, Transmit as TransmitTrait};
pub use embedded_spi::wrapper::Wrapper as SpiWrapper;
