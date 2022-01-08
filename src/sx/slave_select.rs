use embedded_hal::blocking::spi::{Transfer, Write};
use embedded_hal::digital::v2::OutputPin;

use crate::err::{PinError, SpiError};

pub struct SlaveSelect<TNSS: OutputPin> {
    nss: TNSS,
}

impl<TNSS: OutputPin> SlaveSelect<TNSS> {
    pub fn new(nss: TNSS) -> Self {
        Self { nss }
    }
}

pub struct SlaveSelectGuard<'nss, 'spi, TNSS: OutputPin, TSPI: Write<u8> + Transfer<u8>> {
    nss: &'nss mut TNSS,
    spi: &'spi mut TSPI,
}

impl<TNSS: OutputPin> SlaveSelect<TNSS> {
    pub fn select<'spi, TSPI: Write<u8> + Transfer<u8>>(
        &'spi mut self,
        spi: &'spi mut TSPI,
    ) -> Result<SlaveSelectGuard<TNSS, TSPI>, PinError<<TNSS as OutputPin>::Error>> {
        //  Check that buffer is empty
        unsafe {
            debug_assert!(BUFLEN == 0);
            debug_assert!(!TRANSFERRED);
        }

        //  Set Chip Select to Low
        self.nss.set_low().map_err(PinError::Output)?;

        //  Return the guard
        Ok(SlaveSelectGuard {
            nss: &mut self.nss,
            spi,
        })
    }
}

impl<'nss, 'spi, TNSS: OutputPin, TSPI: Write<u8> + Transfer<u8>> Drop
    for SlaveSelectGuard<'nss, 'spi, TNSS, TSPI>
{
    fn drop(&mut self) {
        unsafe {
            if BUFLEN > 0 {
                //  Write the data over SPI
                debug_assert!(!TRANSFERRED);
                self.spi.write(&BUF[0..BUFLEN])
                    .unwrap_or_default();
                
                //  Empty the buffer
                BUFLEN = 0;
            }    
            TRANSFERRED = false;
        }

        //  Set Chip Select to High
        let _ = self.nss.set_high();
    }
}

impl<'nss, 'spi, TNSS, TSPI, TSPIERR> Write<u8> for SlaveSelectGuard<'nss, 'spi, TNSS, TSPI>
where
    TNSS: OutputPin,
    TSPI: Write<u8, Error = TSPIERR> + Transfer<u8, Error = TSPIERR>,
{
    type Error = SpiError<TSPIERR>;

    fn write(&mut self, words: &[u8]) -> Result<(), Self::Error> {
        unsafe {
            //  Prevent a second write
            debug_assert!(!TRANSFERRED);

            //  Copy the transmit data to the buffer, write later
            BUF[BUFLEN..(BUFLEN + words.len())]
                .clone_from_slice(words);
            BUFLEN += words.len();
        }
        Ok(())
    }
}

impl<'nss, 'spi, TNSS, TSPI, TSPIERR> Transfer<u8> for SlaveSelectGuard<'nss, 'spi, TNSS, TSPI>
where
    TNSS: OutputPin,
    TSPI: Write<u8, Error = TSPIERR> + Transfer<u8, Error = TSPIERR>,
{
    type Error = SpiError<TSPIERR>;
    fn transfer<'w>(&mut self, words: &'w mut [u8]) -> Result<&'w [u8], Self::Error> {
        unsafe {
            //  Prevent a second transfer
            debug_assert!(!TRANSFERRED);

            //  Copy the transmit data to the buffer
            BUF[BUFLEN..(BUFLEN + words.len())]
                .clone_from_slice(words);
            BUFLEN += words.len();

            //  Transfer the data over SPI
            let res = self.spi.transfer(&mut BUF[0..BUFLEN])
                .map_err(SpiError::Transfer);

            //  Copy the result from SPI
            words.clone_from_slice(&BUF[BUFLEN - words.len()..BUFLEN]);

            //  Empty the buffer
            BUFLEN = 0;

            //  Prevent a second write or transfer
            TRANSFERRED = true;
            res
        }
    }
}

/// Buffer for SPI Transfer
static mut BUF: [ u8; 64 ] = [ 0; 64 ];

/// Length of buffer for SPI Transfer
static mut BUFLEN: usize = 0;

/// True if we have just executed an SPI Transfer
static mut TRANSFERRED: bool = false;