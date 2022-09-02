#[repr(u8)]
#[derive(Copy, Clone)]
pub enum PacketType {
    GFSK = 0x00,
    LoRa = 0x01,
}

pub enum PacketParams {
    GFSK(gfsk::GFSKPacketParams),
    LoRa(lora::LoRaPacketParams),
}

impl Into<[u8; 9]> for PacketParams {
    fn into(self) -> [u8; 9] {
        match self {
            crate::op::PacketParams::GFSK(_) => [0u8; 9],
            crate::op::PacketParams::LoRa(params) => params.into(),
        }
    }
}

#[derive(Debug)]
pub struct PacketStatus {
    inner: [u8; 4],
}

impl From<[u8; 4]> for PacketStatus {
    fn from(buf: [u8; 4]) -> Self {
        Self { inner: buf }
    }
}

pub mod gfsk {
    pub struct GFSKPacketParams {}
}

pub mod lora {
    use clap::{Args, ValueEnum};

    #[allow(dead_code)]
    #[derive(Debug)]
    pub struct LoRaPacketStatus {
        /// Average over last packet received of RSSI in dBm
        rssi_pkt: i16,
        /// Estimation of SNR on last packet received in dBm
        snr_pkt: i8,
        /// Estimation of RSSI of the LoRa® signal (after despreading) on last packet received
        signal_rssi_pkt: i16,
    }

    impl LoRaPacketStatus {
        pub fn get_rssi_pkt(&self) -> i16 {
            self.rssi_pkt
        }

        pub fn get_snr_pkt(&self) -> i8 {
            self.snr_pkt
        }

        pub fn get_signal_rssi_pkt(&self) -> i8 {
            self.snr_pkt
        }
    }

    use crate::op::PacketStatus;
    impl From<PacketStatus> for LoRaPacketStatus {
        fn from(status: PacketStatus) -> Self {
            Self {
                rssi_pkt: (-1 * status.inner[1] as i16) / 2,
                snr_pkt: (status.inner[2] as i8) / 4,
                signal_rssi_pkt: (-1 * status.inner[3] as i16) / 2,
            }
        }
    }

    #[repr(u8)]
    #[derive(Copy, Clone, Debug, PartialEq, ValueEnum)]
    pub enum LoRaHeaderType {
        /// Variable length packet (explicit header)
        VarLen = 0x00,
        /// Fixed length packet (implicit header)
        FixedLen = 0x01,
    }

    #[repr(u8)]
    #[derive(Copy, Clone, Debug, PartialEq, ValueEnum)]
    pub enum LoRaCrcType {
        /// CRC off
        CrcOff = 0x00,
        /// CRC on
        CrcOn = 0x01,
    }

    #[repr(u8)]
    #[derive(Copy, Clone, Debug, PartialEq, ValueEnum)]
    pub enum LoRaInvertIq {
        /// Standard IQ setup
        Standard = 0x00,
        /// Inverted IQ setup
        Inverted = 0x01,
    }

    #[derive(Copy, Clone, Debug, PartialEq, Args)]
    pub struct LoRaPacketParams {
        /// preamble length: number of symbols sent as preamble
        /// The preamble length is a 16-bit value which represents
        /// the number of LoRa® symbols which will be sent by the radio.
        #[clap(value_parser)]
        preamble_len: u16, // 1, 2
        /// Header type. When the byte headerType is at 0x00,
        /// the payload length, coding rate and the header
        /// CRC will be added to the LoRa® header and transported
        /// to the receiver.
        #[clap(arg_enum, value_parser)]
        header_type: LoRaHeaderType, // 3
        /// Size of the payload (in bytes) to transmit or maximum size of the
        /// payload that the receiver can accept.
        #[clap(value_parser)]
        payload_len: u8, // 4
        /// CRC type
        #[clap(arg_enum, value_parser)]
        crc_type: LoRaCrcType, // 5
        /// Invert IW
        #[clap(arg_enum, value_parser)]
        invert_iq: LoRaInvertIq,
    }

    impl Into<[u8; 9]> for LoRaPacketParams {
        fn into(self) -> [u8; 9] {
            let preamble_len = self.preamble_len.to_be_bytes();

            [
                preamble_len[0],
                preamble_len[1],
                self.header_type as u8,
                self.payload_len,
                self.crc_type as u8,
                self.invert_iq as u8,
                0x00,
                0x00,
                0x00,
            ]
        }
    }

    impl Default for LoRaPacketParams {
        fn default() -> Self {
            Self {
                preamble_len: 0x0008,
                header_type: LoRaHeaderType::VarLen,
                payload_len: 0x00,
                crc_type: LoRaCrcType::CrcOff,
                invert_iq: LoRaInvertIq::Standard,
            }
        }
    }

    impl LoRaPacketParams {
        pub fn set_preamble_len(mut self, preamble_len: u16) -> Self {
            self.preamble_len = preamble_len;
            self
        }

        pub fn set_header_type(mut self, header_type: LoRaHeaderType) -> Self {
            self.header_type = header_type;
            self
        }

        pub fn set_payload_len(mut self, payload_len: u8) -> Self {
            self.payload_len = payload_len;
            self
        }

        pub fn set_crc_type(mut self, crc_type: LoRaCrcType) -> Self {
            self.crc_type = crc_type;
            self
        }

        pub fn set_invert_iq(mut self, invert_iq: LoRaInvertIq) -> Self {
            self.invert_iq = invert_iq;
            self
        }

        pub fn get_invert_iq(&self) -> LoRaInvertIq {
            self.invert_iq
        }
    }
}
