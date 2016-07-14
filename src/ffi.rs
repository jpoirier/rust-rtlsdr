
#![allow(dead_code,
         non_camel_case_types,
         non_upper_case_globals,
         non_snake_case)]

// extern crate libc;
// use libc::{c_int, c_void, c_uchar, c_char};

use std::os::raw::{c_int, c_void, c_uchar, c_char};
use std::option::Option;

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

// pub fn get_device_count() -> i32 {}
// pub fn get_device_name(index: ) ->  {}
// pub fn get_device_usb_strings( ) ->  i32 {}
// pub fn get_index_by_serial( ) ->  {}

pub fn open(index: i32) -> (Device, Error) {
    unsafe {
        let mut dev: *mut rtlsdr_dev_t = std::ptr::null_mut();
        let err = rtlsdr_open(&mut dev as *mut *mut rtlsdr_dev_t, index as uint32_t);
        return (Device { dev: dev }, get_err_msg(err));
    }
}

impl Device {
    pub fn close(&self) -> Error {
        unsafe {
            return get_err_msg(rtlsdr_close(self.dev));
        }
    }

    pub fn set_xtal_freq(&self, rtlFreqHz: u32, tunerFreqHz: u32) -> Error {
        unsafe {
            return get_err_msg(rtlsdr_set_xtal_freq(self.dev,
                                                    rtlFreqHz,
                                                    tunerFreqHz));
        }
    }

    pub fn get_xtal_freq(&self) -> (u32, u32, Error) {
        let mut rtlFreqHz: u32 = 0;
        let mut tunerFreqHz: u32 = 0;
        unsafe {
            let err = rtlsdr_get_xtal_freq(self.dev,
                                           &mut rtlFreqHz as *mut uint32_t,
                                           &mut tunerFreqHz as *mut uint32_t);
            return (rtlFreqHz, tunerFreqHz, get_err_msg(err));
        }
    }
    //     pub fn get_usb_strings() -> Error {}
    //     pub fn write_eeprom() -> Error {}
    //     pub fn read_eeprom() -> Error {}
    //     pub fn set_center_freq() -> Error {}
    //     pub fn get_center_freq() -> Error {}
    //     pub fn set_freq_correction() -> Error {}
    //     pub fn get_freq_correction() -> Error {}
    //     pub fn get_tuner_type() -> rtlsdr_tuner {}
    //     pub fn get_tuner_gains() -> Error {}
    //     pub fn set_tuner_gain() -> Error {}
    //     pub fn set_tuner_bandwidth() -> Error {}
    //     pub fn get_tuner_gain() -> Error {}
    //     pub fn set_tuner_if_gain() -> Error {}
    //     pub fn set_tuner_gain_mode() -> Error {}
    //     pub fn set_sample_rate() -> Error {}
    //     pub fn get_sample_rate() -> Error {}
    //     pub fn set_testmode() -> Error {}
    //     pub fn set_agc_mode() -> Error {}
    //     pub fn set_direct_sampling() -> Error {}
    //     pub fn get_direct_sampling() -> Error {}
    //     pub fn set_offset_tuning() -> Error {}
    //     pub fn get_offset_tuning() -> Error {}
    //     pub fn reset_buffer() -> Error {}
    //     pub fn read_sync() -> Error {}
    //     pub fn wait_async() -> Error {}
    //     pub fn read_async() -> Error {}
    //     pub fn cancel_async() -> Error {}
}

fn main() {
    println!("opening...");

    let (dev, err) = open(0);
    match err {
        Error::NoError => println!("open successful"),
        _ => return,
    }

    let rtl_freq: u32 = 28800000;
	let tuner_freq: u32 = 28800000;
    let err = dev.set_xtal_freq(rtl_freq, tuner_freq);
    match err {
        Error::NoError => println!("set_xtal_freq successful"),
        _ => return,
    }

    let (rtl_freq, tuner_freq, err) = dev.get_xtal_freq();
     println!("rtl_freq: {}, tuner_freq: {}, err: {:?}", rtl_freq, tuner_freq, err);

    let err = dev.close();
    println!("dev close status: {:?}", err);
}
