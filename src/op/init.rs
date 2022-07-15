#[repr(u8)]
#[derive(Copy, Clone)]
pub enum StandbyConfig {
    StbyRc = 0x00,
    StbyXOSC = 0x01,
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum RegulatorMode {
    LdoOnly = 0x00,
    DcDc = 0x01,
}
