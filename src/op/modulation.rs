#[derive(Copy, Clone)]
pub enum ModParams {
    GFSK(gfsk::GFSKModParams),
    LoRa(lora::LoraModParams),
}

impl Into<[u8; 8]> for ModParams {
    fn into(self) -> [u8; 8] {
        match self {
            crate::op::ModParams::GFSK(_) => [0u8; 8],
            crate::op::ModParams::LoRa(params) => params.into(),
        }
    }
}

pub mod gfsk {
    #[derive(Copy, Clone)]
    pub struct GFSKModParams {}
}

pub mod lora {
    use clap::{Args, ValueEnum};

    #[derive(Copy, Clone, Debug, PartialEq, ValueEnum)]
    #[repr(u8)]
    pub enum LoRaSpreadFactor {
        SF5 = 0x05,
        SF6 = 0x06,
        SF7 = 0x07,
        SF8 = 0x08,
        SF9 = 0x09,
        SF10 = 0x0A,
        SF11 = 0x0B,
        SF12 = 0x0C,
    }

    #[derive(Copy, Clone, Debug, PartialEq, ValueEnum)]
    #[repr(u8)]
    pub enum LoRaBandWidth {
        /// 7.81 kHz
        BW7 = 0x00,
        /// 10.42 kHz
        BW10 = 0x08,
        /// 15.63 kHz
        BW15 = 0x01,
        /// 20.83 kHz
        BW20 = 0x09,
        /// 31.25 kHz
        BW31 = 0x02,
        /// 41.67 kHz
        BW41 = 0x0A,
        /// 62.50 kHz
        BW62 = 0x03,
        /// 125 kHz
        BW125 = 0x04,
        /// 250 kHz
        BW250 = 0x05,
        /// 500 kHz
        BW500 = 0x06,
    }

    #[derive(Copy, Clone, Debug, PartialEq, ValueEnum)]
    #[repr(u8)]
    pub enum LoraCodingRate {
        CR4_5 = 0x01,
        CR4_6 = 0x02,
        CR4_7 = 0x03,
        CR4_8 = 0x04,
    }

    #[derive(Copy, Clone, Debug, Args)]
    pub struct LoraModParams {
        #[clap(arg_enum, value_parser)]
        spread_factor: LoRaSpreadFactor,
        #[clap(arg_enum, value_parser)]
        bandwidth: LoRaBandWidth,
        #[clap(arg_enum, value_parser)]
        coding_rate: LoraCodingRate,
        /// LowDataRateOptimize
        #[clap(short, value_parser, default_value_t = false)]
        low_dr_opt: bool,
    }

    impl Default for LoraModParams {
        fn default() -> Self {
            Self {
                spread_factor: LoRaSpreadFactor::SF7,
                bandwidth: LoRaBandWidth::BW125,
                coding_rate: LoraCodingRate::CR4_5,
                low_dr_opt: false,
            }
        }
    }

    impl LoraModParams {
        pub fn set_spread_factor(mut self, spread_factor: LoRaSpreadFactor) -> Self {
            self.spread_factor = spread_factor;
            self
        }
        pub fn set_bandwidth(mut self, bandwidth: LoRaBandWidth) -> Self {
            self.bandwidth = bandwidth;
            self
        }
        pub fn set_coding_rate(mut self, coding_rate: LoraCodingRate) -> Self {
            self.coding_rate = coding_rate;
            self
        }

        pub fn set_low_dr_opt(mut self, low_dr_opt: bool) -> Self {
            self.low_dr_opt = low_dr_opt;
            self
        }

        pub fn get_bandwidth(&self) -> LoRaBandWidth {
            self.bandwidth
        }
    }

    impl Into<[u8; 8]> for LoraModParams {
        fn into(self) -> [u8; 8] {
            [
                self.spread_factor as u8,
                self.bandwidth as u8,
                self.coding_rate as u8,
                self.low_dr_opt as u8,
                0x00,
                0x00,
                0x00,
                0x00,
            ]
        }
    }
}
