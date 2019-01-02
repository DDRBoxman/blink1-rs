extern crate hidapi;

use hidapi::HidApi;
use hidapi::HidError;
use hidapi::HidResult;

const BLINK1_VENDOR_ID: u16 = 0x27b8;
const BLINK1_PRODUCT_ID: u16 = 0x01ed;

const BLINK1_REPORT_ID: u8 = 1;
const BLINK1_REPORT_SIZE: usize = 8;
const BLINK1_REPORT2_ID: u8 = 2;
const BLINK1_REPORT2_SIZE: usize = 60;
const BLINK1_BUF_SIZE:usize = BLINK1_REPORT_SIZE + 1;
const BLINK1_BUF2_SIZE: usize = BLINK1_REPORT2_SIZE + 1;

pub struct Blink1Device {
    device: hidapi::HidDevice,
}

impl Blink1Device {
    pub fn find_first() -> Result<Blink1Device, HidError> {
        match HidApi::new() {
            Ok(api) => {
                // Connect to device using its VID and PID
                match api.open(BLINK1_VENDOR_ID, BLINK1_PRODUCT_ID) {
                    Ok(device) => Ok(Blink1Device { device: device }),
                    Err(e) => return Err(e),
                }
            }
            Err(e) => return Err(e),
        }
    }

    pub fn fade_off(&self, index: u8) -> HidResult<()> {
        return self.fade_to_rgb(index, 0, 0, 0);
    }

    pub fn fade_to_rgb(&self, index: u8, r: u8, g: u8, b: u8) -> HidResult<()> {
        let dms = 100; // 1000 ms

        let mut buf: [u8; BLINK1_BUF_SIZE] = [0; BLINK1_BUF_SIZE];

        buf[0] = BLINK1_REPORT_ID;
        buf[1] = 'c' as u8;   // command code for 'fade to rgb'
        buf[2] = r;
        buf[3] = g;
        buf[4] = b;
        buf[5] = 0;
        buf[6] = dms % 0xff;
        buf[7] = index;

        return self.device.send_feature_report(&buf);
    }
}
