// Copyright (c) 2016 Joseph D Poirier <jdpoirier@gmail.com>
// Licensed under the MIT License <LICENSE-MIT.md>

// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(dead_code)]
use std::sync::Arc;
use std::os::raw::{c_int, c_void, c_uchar, c_char};
use std::option::Option;
use std::string::String;
use std::ffi::CStr;
use std::ptr;
use std::str;


// TODO:
// - better function/method documnentation
// - better way to handle errors
// - String vs str
// - tests
// - read more Rust code, learn more Rust, make this lib better

/// Sampling modes.
pub enum SamplingMode {
    None = 0,
    IADC = 1,
    QADC = 2,
    Error = 3,
}

// Convenience constants
// pub const DefaultGain: String = "auto";
pub const DEFAULT_FC: i32 = 80_000_000;
pub const DEFAULT_RS: i32 = 1_024_000;
pub const DEFAULT_READ_SIZE: i32 = 1_024;
pub const CRYSTAL_FREQ: i32 = 28_800_000;
pub const DEFAULT_SAMPLE_RATE: i32 = 2_048_000;
pub const DEFAULT_ASYNC_BUF_NUMBER: i32 = 32;
pub const DEFAULT_BUF_LENGTH: i32 = (16 * 16_384);
pub const MIN_BUF_LENGTH: i32 = 512;
pub const MAX_BUF_LENGTH: i32 = (256 * 16_384);
/// Hardware info strings (product, manufacturer, serial) maximum size.
/// MAX_STR_SIZE = (max string length - 2 (header bytes)) \ 2. Where each
/// info character is followed by a null character.
pub const MAX_STR_SIZE: usize = 35;

// EEPROM stored device information starts after the header bytes.
const STR_OFFSET_START: usize = 0x09;
const EEPROM_SIZE: i32 = 256;
/// Get/set hardware info errors.
const NO_VALID_EEPROM_HEADER: i32 = -13;
const STRING_VALUE_TOO_LONG: i32 = -14;
const STRING_DESCRIPTOR_INVALID: i32 = -15;
const STRING__DESCRIPTOR_TOO_LONG: i32 = -16;

//
const ERROR_ONKNOWN: i32 = -98;
const LIBUSB_ERROR_ONKNOWN: i32 = -99;

// C lib opaque device struct
enum RTLSDRDev { }
type RTLSDRDevT = RTLSDRDev;

#[derive(Copy, Clone)]
pub struct Device {
    dev: *mut RTLSDRDevT,
}

unsafe impl Send for Device {}
unsafe impl Sync for Device {}

// HwInfo holds dongle specific information.
#[derive(Debug)]
pub struct HwInfo {
    pub vendor_id: u16,
    pub product_id: u16,
    pub manufact: String,
    pub product: String,
    pub serial: String,
    pub have_serial: bool,
    pub enable_ir: bool,
    pub remote_wakeup: bool,
}

#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
enum RTLSDRTuner {
    Unknown = 0,
    E4000 = 1,
    FC0012 = 2,
    FC0013 = 3,
    FC2580 = 4,
    R820T = 5,
    R828D = 6,
}

#[derive(Copy, Clone)]
#[derive(Debug)]
pub enum Error {
    NoError,
    Io,
    InvalidParam,
    Access,
    NoDevice,
    NotFound,
    Busy,
    Timeout,
    Overflow,
    Pipe,
    Interrupted,
    NoMem,
    NotSupported,
    NoValidEEPROMHeader,
    StringValueTooLong,
    StringDescriptorInvalid,
    StringDescriptorTooLong,
    Unknown,
}



/// read async callback function
pub type ReadAsyncCbT = Option<unsafe extern "C" fn(buf: *mut c_uchar, len: u32, ctx: *mut c_void)>;

#[link(name = "rtlsdr")]
extern "C" {
    fn rtlsdr_get_device_count() -> u32;
    fn rtlsdr_get_device_name(index: u32) -> *const c_char;
    fn rtlsdr_get_device_usb_strings(index: u32,
                                     manufact: *mut c_char,
                                     product: *mut c_char,
                                     serial: *mut c_char)
                                     -> c_int;
    fn rtlsdr_get_index_by_serial(serial: *const c_char) -> c_int;

    fn rtlsdr_open(dev: *mut *mut RTLSDRDevT, index: u32) -> c_int;
    fn rtlsdr_close(dev: *mut RTLSDRDevT) -> c_int;
    fn rtlsdr_set_xtal_freq(dev: *mut RTLSDRDevT, rtl_freq: u32, tuner_freq: u32) -> c_int;
    fn rtlsdr_get_xtal_freq(dev: *mut RTLSDRDevT,
                            rtl_freq: *mut u32,
                            tuner_freq: *mut u32)
                            -> c_int;
    fn rtlsdr_get_usb_strings(dev: *mut RTLSDRDevT,
                              manufact: *mut c_char,
                              product: *mut c_char,
                              serial: *mut c_char)
                              -> c_int;
    fn rtlsdr_write_eeprom(dev: *mut RTLSDRDevT, data: *mut u8, offset: u8, len: u16) -> c_int;
    fn rtlsdr_read_eeprom(dev: *mut RTLSDRDevT, data: *mut u8, offset: u8, len: u16) -> c_int;
    fn rtlsdr_set_center_freq(dev: *mut RTLSDRDevT, freq: u32) -> c_int;
    fn rtlsdr_get_center_freq(dev: *mut RTLSDRDevT) -> c_int;
    fn rtlsdr_set_freq_correction(dev: *mut RTLSDRDevT, ppm: c_int) -> c_int;
    fn rtlsdr_get_freq_correction(dev: *mut RTLSDRDevT) -> c_int;
    fn rtlsdr_get_tuner_type(dev: *mut RTLSDRDevT) -> RTLSDRTuner;
    fn rtlsdr_get_tuner_gains(dev: *mut RTLSDRDevT, gains: *mut c_int) -> c_int;
    fn rtlsdr_set_tuner_gain(dev: *mut RTLSDRDevT, gain: c_int) -> c_int;
    fn rtlsdr_set_tuner_bandwidth(dev: *mut RTLSDRDevT, bw: u32) -> c_int;
    fn rtlsdr_get_tuner_gain(dev: *mut RTLSDRDevT) -> c_int;
    fn rtlsdr_set_tuner_if_gain(dev: *mut RTLSDRDevT, stage: c_int, gain: c_int) -> c_int;
    fn rtlsdr_set_tuner_gain_mode(dev: *mut RTLSDRDevT, manual: c_int) -> c_int;
    fn rtlsdr_set_sample_rate(dev: *mut RTLSDRDevT, rate: u32) -> c_int;
    fn rtlsdr_get_sample_rate(dev: *mut RTLSDRDevT) -> c_int;
    fn rtlsdr_set_testmode(dev: *mut RTLSDRDevT, on: c_int) -> c_int;
    fn rtlsdr_set_agc_mode(dev: *mut RTLSDRDevT, on: c_int) -> c_int;
    fn rtlsdr_set_direct_sampling(dev: *mut RTLSDRDevT, on: c_int) -> c_int;
    fn rtlsdr_get_direct_sampling(dev: *mut RTLSDRDevT) -> c_int;
    fn rtlsdr_set_offset_tuning(dev: *mut RTLSDRDevT, on: c_int) -> c_int;
    fn rtlsdr_get_offset_tuning(dev: *mut RTLSDRDevT) -> c_int;
    fn rtlsdr_reset_buffer(dev: *mut RTLSDRDevT) -> c_int;
    fn rtlsdr_read_sync(dev: *mut RTLSDRDevT,
                        buf: *mut c_void,
                        len: c_int,
                        n_read: *mut c_int)
                        -> c_int;
    fn rtlsdr_wait_async(dev: *mut RTLSDRDevT, cb: ReadAsyncCbT, ctx: *mut c_void) -> c_int;
    fn rtlsdr_read_async(dev: *mut RTLSDRDevT,
                         cb: ReadAsyncCbT,
                         ctx: *mut c_void,
                         buf_num: u32,
                         buf_len: u32)
                         -> c_int;
    fn rtlsdr_cancel_async(dev: *mut RTLSDRDevT) -> c_int;
}

// FIXME: there has to be a better way...
fn get_err_msg(e: c_int) -> Error {
    match e {
        0 => Error::NoError,
        -1 => Error::Io,
        -2 => Error::InvalidParam,
        -3 => Error::Access,
        -4 => Error::NoDevice,
        -5 => Error::NotFound,
        -6 => Error::Busy,
        -7 => Error::Timeout,
        -8 => Error::Overflow,
        -9 => Error::Pipe,
        -10 => Error::Interrupted,
        -11 => Error::NoMem,
        -12 => Error::NotSupported,
        NO_VALID_EEPROM_HEADER => Error::NoValidEEPROMHeader,
        STRING_VALUE_TOO_LONG => Error::StringValueTooLong,
        STRING_DESCRIPTOR_INVALID => Error::StringDescriptorInvalid,
        STRING__DESCRIPTOR_TOO_LONG => Error::StringDescriptorTooLong,
        _ => Error::Unknown,
    }
}

fn from_tuner_type(t: RTLSDRTuner) -> String {
    match t {
        RTLSDRTuner::Unknown => String::from("Unknown"),
        RTLSDRTuner::E4000 => String::from("E4000"),
        RTLSDRTuner::FC0012 => String::from("FC0012"),
        RTLSDRTuner::FC0013 => String::from("FC0013"),
        RTLSDRTuner::FC2580 => String::from("FC2580"),
        RTLSDRTuner::R820T => String::from("R820T"),
        RTLSDRTuner::R828D => String::from("R828D"),
    }
}

fn from_pchar(p: *const c_char) -> String {
    let c_str = unsafe { CStr::from_ptr(p) };
    String::from(str::from_utf8(c_str.to_bytes()).unwrap())
}

/// Returns the number of devices detected.
pub fn get_device_count() -> i32 {
    unsafe { rtlsdr_get_device_count() as i32 }
}

/// Returns the name of the device by index.
pub fn get_device_name(index: i32) -> String {
    unsafe { CStr::from_ptr(rtlsdr_get_device_name(index as u32)).to_string_lossy().into_owned() }
}

/// Returns the information of a device by index.
pub fn get_device_usb_strings(index: i32) -> (String, String, String, Error) {
    unsafe {
        let m: [c_char; 256] = [0; 256];
        let p: [c_char; 256] = [0; 256];
        let s: [c_char; 256] = [0; 256];
        let err = rtlsdr_get_device_usb_strings(index as u32,
                                                m.as_ptr() as *mut c_char,
                                                p.as_ptr() as *mut c_char,
                                                s.as_ptr() as *mut c_char);
        (from_pchar(m.as_ptr()), from_pchar(p.as_ptr()), from_pchar(s.as_ptr()), get_err_msg(err))
    }
}

/// Returns a device index by serial id.
pub fn get_index_by_serial(serial: String) -> i32 {
    unsafe { rtlsdr_get_index_by_serial(serial.as_ptr() as *const c_char) as i32 }
}

/// Returns an opened device by index.
pub fn open(index: i32) -> (Arc<Device>, Error) {
    unsafe {
        let mut dev: *mut RTLSDRDevT = std::ptr::null_mut();
        let err = rtlsdr_open(&mut dev as *mut *mut RTLSDRDevT, index as u32);
        (Arc::new(Device { dev: dev }), get_err_msg(err))
    }
}

/// Gets the manufacturer, product, and serial strings from data.
fn get_string_descriptors(data: &Vec<u8>) -> (String, String, String, Error) {
    let mut pos = STR_OFFSET_START;
    let mut strings: Vec<String> = Vec::new();

    for _ in 0..3 {
        let l = data[pos] as usize;
        if l > (MAX_STR_SIZE * 2) as usize + 2 {
            return ("".to_string(),
                    "".to_string(),
                    "".to_string(),
                    get_err_msg(STRING_VALUE_TOO_LONG));
        }
        if data[pos + 1] != 0x03 {
            return ("".to_string(),
                    "".to_string(),
                    "".to_string(),
                    get_err_msg(STRING_DESCRIPTOR_INVALID));
        }

        let mut j: usize = 2;
        let mut s = String::new();
        while j < l {
            s.push(data[pos + j] as char);
            j += 2;
        }
        strings.push(s);
        pos += j;
    }
    (strings[0].clone(), strings[1].clone(), strings[2].clone(), get_err_msg(0))
}

/// Sets the manufacturer, product, and serial strings in vec format.
fn set_string_descriptors(info: &HwInfo, data: &mut Vec<u8>) -> Error {
    let mlen = info.manufact.len();
    let plen = info.product.len();
    let slen = info.serial.len();

    if mlen > MAX_STR_SIZE || plen > MAX_STR_SIZE || slen > MAX_STR_SIZE {
        return get_err_msg(STRING__DESCRIPTOR_TOO_LONG);
    }

    let mut pos = STR_OFFSET_START;
    let strings = [&info.manufact, &info.product, &info.serial];
    for s in strings.iter() {
        data[pos] = ((s.len() * 2) + 2) as u8;
        data[pos + 1] = 0x03u8;
        pos += 2;

        for b in s.as_bytes().iter() {
            data[pos] = *b;
            data[pos + 1] = 0x00u8;
            pos += 2;
        }
    }

    get_err_msg(0)
}

impl Device {
    /// Close the device.
    pub fn close(&self) -> Error {
        unsafe { get_err_msg(rtlsdr_close(self.dev)) }
    }

    /// Sets the crystal oscillator frequencies.
    ///
    /// Typically both ICs (rtlsdr and tuner) use the same clock. Changing the
    /// clock may make sense if you are applying an external clock to the tuner
    /// or to compensate the frequency (and sample rate) error caused by the
    /// original (cheap) crystal.
    ///
    /// Note, call this function only if you fully understand the implications.
    pub fn set_xtal_freq(&self, rtl_freq_hz: i32, tuner_freq_hz: i32) -> Error {
        unsafe {
            get_err_msg(rtlsdr_set_xtal_freq(self.dev, rtl_freq_hz as u32, tuner_freq_hz as u32))
        }
    }

    /// Returns the crystal oscillator frequencies.
    /// Typically both ICs (rtlsdr and tuner) use the same clock.
    pub fn get_xtal_freq(&self) -> (i32, i32, Error) {
        let mut rtl_freq_hz: u32 = 0;
        let mut tuner_freq_hz: u32 = 0;
        unsafe {
            let err = rtlsdr_get_xtal_freq(self.dev,
                                           &mut rtl_freq_hz as *mut u32,
                                           &mut tuner_freq_hz as *mut u32);
            (rtl_freq_hz as i32, tuner_freq_hz as i32, get_err_msg(err))
        }
    }

    /// Returns the device information (manufact, product, serial).
    /// Note, strings may be empty.
    pub fn get_usb_strings(&self) -> (String, String, String, Error) {
        let m: [c_char; 256] = [0; 256];
        let p: [c_char; 256] = [0; 256];
        let s: [c_char; 256] = [0; 256];
        unsafe {
            let err = rtlsdr_get_usb_strings(self.dev,
                                             m.as_ptr() as *mut c_char,
                                             p.as_ptr() as *mut c_char,
                                             s.as_ptr() as *mut c_char);
            (from_pchar(m.as_ptr()),
             from_pchar(p.as_ptr()),
             from_pchar(s.as_ptr()),
             get_err_msg(err))
        }
    }

    /// Writes information data to the EEPROM.
    pub fn write_eeprom(&self, data: Vec<u8>, offset: u8) -> Error {
        unsafe {
            let mut err = rtlsdr_write_eeprom(self.dev,
                                              data.as_ptr() as *mut u8,
                                              offset,
                                              data.len() as u16);
            if err >= 0 {
                err = 0;
            }
            get_err_msg(err)
        }
    }

    /// Returns information data read from the EEPROM.
    pub fn read_eeprom(&self, offset: u8, len: u16) -> (Vec<u8>, Error) {
        let mut v = vec![0u8; len as usize];
        unsafe {
            let mut err = rtlsdr_read_eeprom(self.dev, v.as_mut_ptr() as *mut u8, offset, len);
            if err >= 0 {
                err = 0;
            }
            (v, get_err_msg(err))
        }

    }

    /// Sets the center frequency.
    pub fn set_center_freq(&self, freq_hz: i32) -> Error {
        unsafe { get_err_msg(rtlsdr_set_center_freq(self.dev, freq_hz as u32)) }
    }

    /// Returns the tuned frequency or zero on error.
    pub fn get_center_freq(&self) -> i32 {
        unsafe { rtlsdr_get_center_freq(self.dev) as i32 }
    }

    /// Sets the frequency correction.
    pub fn set_freq_correction(&self, ppm: i32) -> Error {
        unsafe { get_err_msg(rtlsdr_set_freq_correction(self.dev, ppm)) }
    }

    /// Returns the frequency correction value.
    pub fn get_freq_correction(&self) -> i32 {
        unsafe { rtlsdr_get_freq_correction(self.dev) }
    }

    /// Returns the tuner type.
    pub fn get_tuner_type(&self) -> String {
        unsafe { from_tuner_type(rtlsdr_get_tuner_type(self.dev)) }
    }

    /// Returns a list of supported tuner gains.
    /// Values are in tenths of dB, e.g. 115 means 11.5 dB.
    pub fn get_tuner_gains(&self) -> (Vec<i32>, Error) {
        unsafe {
            let mut i = rtlsdr_get_tuner_gains(self.dev, ptr::null_mut());
            if i <= 0 {
                // println!("error rtlsdr_get_tuner_gains <= 0: {}", i);
                return (Vec::new(), get_err_msg(i));
            }
            println!("rtlsdr_get_tuner_gains count: {}", i);
            let mut v = vec![0; i as usize];
            i = rtlsdr_get_tuner_gains(self.dev, v.as_mut_ptr());
            let err = if i <= 0 {
                Error::Unknown
            } else {
                Error::NoError
            };

            (v, err)
        }
    }

    /// Sets the tuner gain. Note, manual gain mode
    /// must be enabled for this to work. Valid gain values may be
    /// queried using get_tuner_gains.
    ///
    /// Valid values (in tenths of a dB) are:
    /// -10, 15, 40, 65, 90, 115, 140, 165, 190, 215, 240, 290,
    /// 340, 420, 430, 450, 470, 490
    ///
    /// Gain values are in tenths of dB, e.g. 115 means 11.5 dB.
    pub fn set_tuner_gain(&self, gain: i32) -> Error {
        unsafe { get_err_msg(rtlsdr_set_tuner_gain(self.dev, gain)) }
    }

    /// Returns the tuner gain.
    ///
    /// Gain values are in tenths of dB, e.g. 115 means 11.5 dB.
    pub fn get_tuner_gain(&self) -> i32 {
        unsafe { rtlsdr_get_tuner_gain(self.dev) }
    }

    /// Sets the device bandwidth.
    pub fn set_tuner_bandwidth(&self, bw_hz: i32) -> Error {
        unsafe { get_err_msg(rtlsdr_set_tuner_bandwidth(self.dev, bw_hz as u32)) }
    }

    /// Sets the intermediate frequency gain.
    ///
    /// Intermediate frequency gain stage number 1 to 6.
    /// Gain values are in tenths of dB, e.g. -30 means -3.0 dB.
    pub fn set_tuner_if_gain(&self, stage: i32, gains_tenths_db: i32) -> Error {
        unsafe { get_err_msg(rtlsdr_set_tuner_if_gain(self.dev, stage, gains_tenths_db)) }

    }

    /// Sets the gain mode, automatic or manual.
    /// Manual gain mode must be enabled for the gain setter function to work.
    pub fn set_tuner_gain_mode(&self, manual_mode: bool) -> Error {
        unsafe { get_err_msg(rtlsdr_set_tuner_gain_mode(self.dev, manual_mode as i32)) }

    }

    /// Sets the sample rate.
    ///
    /// When applicable, the baseband filters are also selected based
    /// on the requested sample rate.
    pub fn set_sample_rate(&self, rate_hz: i32) -> Error {
        unsafe { get_err_msg(rtlsdr_set_sample_rate(self.dev, rate_hz as u32)) }
    }

    /// Returns the sample rate.
    pub fn get_sample_rate(&self) -> i32 {
        unsafe { rtlsdr_get_sample_rate(self.dev) }
    }

    /// Sets device to test mode.
    ///
    /// Test mode returns 8 bit counters instead of samples. Note,
    /// the counter is generated inside the device.
    pub fn set_testmode(&self, test_mode: bool) -> Error {
        unsafe { get_err_msg(rtlsdr_set_testmode(self.dev, test_mode as i32)) }
    }

    /// Sets the AGC mode.
    pub fn set_agc_mode(&self, agc_mode: bool) -> Error {
        unsafe { get_err_msg(rtlsdr_set_agc_mode(self.dev, agc_mode as i32)) }
    }

    /// Sets the direct sampling mode.
    ///
    /// When enabled, the IF mode of the device is activated, and
    /// set_center_freq() will control the IF-frequency of the DDC, which
    /// can be used to tune from 0 to 28.8 MHz (xtal frequency of the device).
    pub fn set_direct_sampling(&self, mode: SamplingMode) -> Error {
        unsafe { get_err_msg(rtlsdr_set_direct_sampling(self.dev, mode as i32)) }
    }

    /// Returns the state of direct sampling mode.
    pub fn get_direct_sampling(&self) -> SamplingMode {
        unsafe {
            match rtlsdr_get_direct_sampling(self.dev) {
                0 => SamplingMode::None,
                1 => SamplingMode::IADC,
                2 => SamplingMode::QADC,
                _ => SamplingMode::Error,
            }
        }
    }

    /// Sets the offset tuning mode for zero-IF tuners, which
    /// avoids problems caused by the DC offset of the ADCs and 1/f noise.
    pub fn set_offset_tuning(&self, enable: bool) -> Error {
        unsafe { get_err_msg(rtlsdr_set_offset_tuning(self.dev, enable as i32)) }
    }

    /// Returns the offset tuning mode.
    pub fn get_offset_tuning(&self) -> Error {
        unsafe { get_err_msg(rtlsdr_get_offset_tuning(self.dev)) }
    }

    /// Resets the streaming buffer.
    pub fn reset_buffer(&self) -> Error {
        unsafe { get_err_msg(rtlsdr_reset_buffer(self.dev)) }
    }

    /// Performs a synchronous read of samples and returns
    /// the number of samples read.
    pub fn read_sync(&self, len: i32) -> (Vec<u8>, i32, Error) {
        let mut buf = vec![0u8; len as usize];
        let mut n_read: i32 = 0;
        unsafe {
            let err = rtlsdr_read_sync(self.dev,
                                       buf.as_mut_ptr() as *mut c_void,
                                       len,
                                       &mut n_read as *mut c_int);

            (buf, n_read, get_err_msg(err))
        }

    }

    /// Reads samples asynchronously. Note, this function will block until
    /// canceled using cancel_async. ReadAsyncCbT is a package global variable.
    ///
    /// Optional buf_num buffer count, buf_num * buf_len = overall buffer size,
    /// set to 0 for default buffer count of 32.
    ///
    /// Optional buf_len buffer length, must be multiple of 512, set to 0 for
    /// default buffer length of 262,144 (16 * 32 * 512).
    pub fn read_async(&self,
                      f: ReadAsyncCbT,
                      ctx: *mut c_void,
                      buf_num: i32,
                      buf_len: i32)
                      -> Error {
        unsafe { get_err_msg(rtlsdr_read_async(self.dev, f, ctx, buf_num as u32, buf_len as u32)) }
    }

    /// Cancels all pending asynchronous operations.
    pub fn cancel_async(&self) -> Error {
        unsafe { get_err_msg(rtlsdr_cancel_async(self.dev)) }

    }

    /// Reads the dongle's information from the EEPROM.
    pub fn get_hw_info(&self) -> (HwInfo, Error) {
        let mut have_serial = false;
        let mut remote_wakeup = false;
        let mut enable_ir = false;
        let mut vendor_id = 0u16;
        let mut product_id = 0u16;
        let mut m: String = "".to_string();
        let mut p: String = "".to_string();
        let mut s: String = "".to_string();

        let (data, mut err) = self.read_eeprom(0, EEPROM_SIZE as u16);
        // println!("eeprom data: {:?}, error: {:?}", data, err);

        if let Some(Error::NoError) = Some(err) {
            if (data[0] != 0x28) || (data[1] != 0x32) {
                err = get_err_msg(NO_VALID_EEPROM_HEADER);
            } else {
                vendor_id = (data[3] as u16) << 8 | data[2] as u16;
                product_id = (data[5] as u16) << 8 | data[4] as u16;
                // println!("vendor_id {}, product_id {}", vendor_id, product_id);

                if data[6] == 0xA5 {
                    have_serial = true;
                }
                if (data[7] & 0x01) == 0x01 {
                    remote_wakeup = true;
                }
                if (data[7] & 0x02) == 0x02 {
                    enable_ir = true;
                }

                let (mm, pp, ss, e) = get_string_descriptors(&data);
                m = mm;
                p = pp;
                s = ss;
                err = e;
            }
        }

        let info = HwInfo {
            have_serial: have_serial,
            vendor_id: vendor_id,
            product_id: product_id,
            remote_wakeup: remote_wakeup,
            enable_ir: enable_ir,
            manufact: m,
            product: p,
            serial: s,
        };

        (info, err)
    }

    /// Write the dongle's information to the EEPROM.
    pub fn set_hw_info(&self, info: &HwInfo) -> Error {
        let mlen = info.manufact.len();
        let plen = info.product.len();
        let slen = info.serial.len();
        let stored_len = STR_OFFSET_START + ((2 * mlen) + 2) + ((2 * plen) + 2) + ((2 * slen) + 2);
        let mut data = vec![0u8; stored_len];

        data[0] = 0x28u8;
        data[1] = 0x32u8;
        data[2] = info.vendor_id as u8;
        data[3] = (info.vendor_id >> 8) as u8;
        data[4] = info.product_id as u8;
        data[5] = (info.product_id >> 8) as u8;
        data[6] = 0x00u8;
        data[7] = 0x00u8;
        data[8] = 0x00u8;

        if info.have_serial == true {
            data[6] = 0xA5u8;
        }
        if info.remote_wakeup == true {
            data[7] = data[7] | 0x01;
        }
        if info.enable_ir == true {
            data[7] = data[7] | 0x02;
        }

        let mut err = set_string_descriptors(&info, &mut data);
        if let Some(Error::NoError) = Some(err) {
            err = self.write_eeprom(data, 0);
        }

        err
    }
}
