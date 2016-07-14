
#![allow(dead_code,
         non_camel_case_types,
         non_upper_case_globals,
         non_snake_case)]

// extern crate libc;
// use libc::{c_int, c_void, c_uchar, c_char};

use std::os::raw::{c_int, c_void, c_uchar, c_char};
use std::option::Option;
use std::string::String;
use std::ffi::CStr;
use std::ptr;


pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;

pub enum rtlsdr_dev { }
type rtlsdr_dev_t = rtlsdr_dev;


pub struct Device {
    dev: *mut rtlsdr_dev_t,
}

#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum rtlsdr_tuner {
    RTLSDR_TUNER_UNKNOWN = 0,
    RTLSDR_TUNER_E4000 = 1,
    RTLSDR_TUNER_FC0012 = 2,
    RTLSDR_TUNER_FC0013 = 3,
    RTLSDR_TUNER_FC2580 = 4,
    RTLSDR_TUNER_R820T = 5,
    RTLSDR_TUNER_R828D = 6,
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
    Other,
}

pub type rtlsdr_read_async_cb_t = Option<unsafe extern "C" fn(buf: *mut c_uchar,
                                                              len: uint32_t,
                                                              ctx: *mut c_void)>;

#[link(name = "rtlsdr")]
extern "C" {
    fn rtlsdr_get_device_count() -> uint32_t;
    fn rtlsdr_get_device_name(index: uint32_t) -> *const c_char;
    fn rtlsdr_get_device_usb_strings(index: uint32_t,
                                     manufact: *mut c_char,
                                     product: *mut c_char,
                                     serial: *mut c_char)
                                     -> c_int;
    fn rtlsdr_get_index_by_serial(serial: *const c_char) -> c_int;

    fn rtlsdr_open(dev: *mut *mut rtlsdr_dev_t, index: uint32_t) -> c_int;
    fn rtlsdr_close(dev: *mut rtlsdr_dev_t) -> c_int;
    fn rtlsdr_set_xtal_freq(dev: *mut rtlsdr_dev_t,
                            rtl_freq: uint32_t,
                            tuner_freq: uint32_t)
                            -> c_int;
    fn rtlsdr_get_xtal_freq(dev: *mut rtlsdr_dev_t,
                            rtl_freq: *mut uint32_t,
                            tuner_freq: *mut uint32_t)
                            -> c_int;
    fn rtlsdr_get_usb_strings(dev: *mut rtlsdr_dev_t,
                              manufact: *mut c_char,
                              product: *mut c_char,
                              serial: *mut c_char)
                              -> c_int;
    fn rtlsdr_write_eeprom(dev: *mut rtlsdr_dev_t,
                           data: *mut uint8_t,
                           offset: uint8_t,
                           len: uint16_t)
                           -> c_int;
    fn rtlsdr_read_eeprom(dev: *mut rtlsdr_dev_t,
                          data: *mut uint8_t,
                          offset: uint8_t,
                          len: uint16_t)
                          -> c_int;
    fn rtlsdr_set_center_freq(dev: *mut rtlsdr_dev_t, freq: uint32_t) -> c_int;
    fn rtlsdr_get_center_freq(dev: *mut rtlsdr_dev_t) -> c_int;
    fn rtlsdr_set_freq_correction(dev: *mut rtlsdr_dev_t, ppm: c_int) -> c_int;
    fn rtlsdr_get_freq_correction(dev: *mut rtlsdr_dev_t) -> c_int;
    fn rtlsdr_get_tuner_type(dev: *mut rtlsdr_dev_t) -> rtlsdr_tuner;
    fn rtlsdr_get_tuner_gains(dev: *mut rtlsdr_dev_t, gains: *mut c_int) -> c_int;
    fn rtlsdr_set_tuner_gain(dev: *mut rtlsdr_dev_t, gain: c_int) -> c_int;
    fn rtlsdr_set_tuner_bandwidth(dev: *mut rtlsdr_dev_t, bw: uint32_t) -> c_int;
    fn rtlsdr_get_tuner_gain(dev: *mut rtlsdr_dev_t) -> c_int;
    fn rtlsdr_set_tuner_if_gain(dev: *mut rtlsdr_dev_t, stage: c_int, gain: c_int) -> c_int;
    fn rtlsdr_set_tuner_gain_mode(dev: *mut rtlsdr_dev_t, manual: c_int) -> c_int;
    fn rtlsdr_set_sample_rate(dev: *mut rtlsdr_dev_t, rate: uint32_t) -> c_int;
    fn rtlsdr_get_sample_rate(dev: *mut rtlsdr_dev_t) -> c_int;
    fn rtlsdr_set_testmode(dev: *mut rtlsdr_dev_t, on: c_int) -> c_int;
    fn rtlsdr_set_agc_mode(dev: *mut rtlsdr_dev_t, on: c_int) -> c_int;
    fn rtlsdr_set_direct_sampling(dev: *mut rtlsdr_dev_t, on: c_int) -> c_int;
    fn rtlsdr_get_direct_sampling(dev: *mut rtlsdr_dev_t) -> c_int;
    fn rtlsdr_set_offset_tuning(dev: *mut rtlsdr_dev_t, on: c_int) -> c_int;
    fn rtlsdr_get_offset_tuning(dev: *mut rtlsdr_dev_t) -> c_int;
    fn rtlsdr_reset_buffer(dev: *mut rtlsdr_dev_t) -> c_int;
    fn rtlsdr_read_sync(dev: *mut rtlsdr_dev_t,
                        buf: *mut c_void,
                        len: c_int,
                        n_read: *mut c_int)
                        -> c_int;
    fn rtlsdr_wait_async(dev: *mut rtlsdr_dev_t,
                         cb: rtlsdr_read_async_cb_t,
                         ctx: *mut c_void)
                         -> c_int;
    fn rtlsdr_read_async(dev: *mut rtlsdr_dev_t,
                         cb: rtlsdr_read_async_cb_t,
                         ctx: *mut c_void,
                         buf_num: uint32_t,
                         buf_len: uint32_t)
                         -> c_int;
    fn rtlsdr_cancel_async(dev: *mut rtlsdr_dev_t) -> c_int;
}

fn get_err_msg(e: c_int) -> Error {
    use Error::*;
    match e {
        0 => NoError,
        -1 => Io,
        -2 => InvalidParam,
        -3 => Access,
        -4 => NoDevice,
        -5 => NotFound,
        -6 => Busy,
        -7 => Timeout,
        -8 => Overflow,
        -9 => Pipe,
        -10 => Interrupted,
        -11 => NoMem,
        -12 => NotSupported,
        _ => Other,
    }
}

/// Returns the number of devices detected.
pub fn get_device_count() -> i32 {
    unsafe { rtlsdr_get_device_count() as i32 }
}

/// Returns the name of the device by index.
pub fn get_device_name(index: i32) -> String {
    unsafe {
        CStr::from_ptr(rtlsdr_get_device_name(index as uint32_t)).to_string_lossy().into_owned()
    }
}

/// Returns the information of a device by index.
pub fn get_device_usb_strings(index: i32) -> (String, String, String, Error) {
    unsafe {
        let m = String::with_capacity(256);
        let p = String::with_capacity(256);
        let s = String::with_capacity(256);
        let err = rtlsdr_get_device_usb_strings(index as uint32_t,
                                                m.as_ptr() as *mut c_char,
                                                p.as_ptr() as *mut c_char,
                                                s.as_ptr() as *mut c_char);
        (m, p, s, get_err_msg(err))
    }
}

/// Returns a device index by serial id.
pub fn get_index_by_serial(serial: String) -> i32 {
    unsafe { rtlsdr_get_index_by_serial(serial.as_ptr() as *const c_char) as i32 }
}

/// Returns an opened device by index.
pub fn open(index: i32) -> (Device, Error) {
    unsafe {
        let mut dev: *mut rtlsdr_dev_t = std::ptr::null_mut();
        let err = rtlsdr_open(&mut dev as *mut *mut rtlsdr_dev_t, index as uint32_t);
        (Device { dev: dev }, get_err_msg(err))
    }
}

impl Device {
    /// Closes the device.
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
    pub fn set_xtal_freq(&self, rtlFreqHz: i32, tunerFreqHz: i32) -> Error {
        unsafe {
            get_err_msg(rtlsdr_set_xtal_freq(self.dev,
                                             rtlFreqHz as uint32_t,
                                             tunerFreqHz as uint32_t))
        }
    }

    /// Returns the crystal oscillator frequencies.
    /// Typically both ICs (rtlsdr and tuner) use the same clock.
    pub fn get_xtal_freq(&self) -> (i32, i32, Error) {
        let mut rtlFreqHz: u32 = 0;
        let mut tunerFreqHz: u32 = 0;
        unsafe {
            let err = rtlsdr_get_xtal_freq(self.dev,
                                           &mut rtlFreqHz as *mut uint32_t,
                                           &mut tunerFreqHz as *mut uint32_t);
            (rtlFreqHz as i32, tunerFreqHz as i32, get_err_msg(err))
        }
    }

    /// Returns the device information (manufact, product, serial).
    /// Note, strings may be empty.
    pub fn get_usb_strings(&self) -> (String, String, String, Error) {
        unsafe {
            let m = String::with_capacity(256);
            let p = String::with_capacity(256);
            let s = String::with_capacity(256);
            let err = rtlsdr_get_usb_strings(self.dev,
                                             m.as_ptr() as *mut c_char,
                                             p.as_ptr() as *mut c_char,
                                             s.as_ptr() as *mut c_char);
            (m.trim().to_string(), p.trim().to_string(), s.trim().to_string(), get_err_msg(err))
        }
    }

    /// Writes data to the EEPROM.
    // pub fn write_eeprom(&self) -> Error {}

    /// Returns data read from the EEPROM.
    // pub fn read_eeprom(&self) -> Error {}

    /// Sets the center frequency.
    pub fn set_center_freq(&self, freqHz: i32) -> Error {
        unsafe {
            get_err_msg(rtlsdr_set_center_freq(self.dev, freqHz as uint32_t))
        }
    }

    /// Returns the tuned frequency or zero on error.
    pub fn get_center_freq(&self) -> i32 {
        unsafe { rtlsdr_get_center_freq(self.dev) as i32 }
    }

    /// Sets the frequency correction.
    pub fn set_freq_correction(&self, ppm: i32) -> Error {
	    unsafe {
            get_err_msg(rtlsdr_set_freq_correction(self.dev, ppm))
        }
    }

    /// Returns the frequency correction value.
    pub fn get_freq_correction(&self) -> i32 {
        unsafe { rtlsdr_get_freq_correction(self.dev) }
    }

    /// Returns the tuner type.
    pub fn get_tuner_type(&self) -> rtlsdr_tuner {
        unsafe { rtlsdr_get_tuner_type(self.dev) }
    }

    /// Returns a list of supported tuner gains.
    /// Values are in tenths of dB, e.g. 115 means 11.5 dB.
    pub fn get_tuner_gains(&self) -> (Vec<i32>, Error) {
        unsafe {
            let i = rtlsdr_get_tuner_gains(self.dev, ptr::null_mut() as *mut c_int);
            if i <= 0 { return (Vec::new(),  get_err_msg(i)); }
            let v = vec![0; i as usize];
            let err = rtlsdr_get_tuner_gains(self.dev, v.as_ptr() as *mut c_int);
            (v, get_err_msg(err))
        }
    }

    /// Sets the tuner gain. Note, manual gain mode
    /// must be enabled for this to work. Valid gain values may be
    /// queried using GetTunerGains.
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
}

fn main() {
    println!("opening...");

    let (dev, err) = open(0);
    match err {
        Error::NoError => println!("open successful"),
        _ => return,
    }

    let err = dev.set_xtal_freq(28800000, 28800000);
    match err {
        Error::NoError => println!("set_xtal_freq successful"),
        _ => return,
    }

    let (rtl_freq, tuner_freq, err) = dev.get_xtal_freq();
    println!("rtl_freq: {}, tuner_freq: {}, err: {:?}",
             rtl_freq,
             tuner_freq,
             err);

    let err = dev.close();
    println!("dev close status: {:?}", err);
}
