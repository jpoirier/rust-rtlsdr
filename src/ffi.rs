
#![allow(dead_code,
         non_camel_case_types,
         non_upper_case_globals,
         non_snake_case)]

use libc::{c_int, c_void, c_uchar, c_char};
use std::{};

pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;

pub enum rtlsdr_dev { }
pub type rtlsdr_dev_t = rtlsdr_dev;

#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum rtlsdr_tuner {
    RTLSDR_TUNER_UNKNOWN = 0,
    RTLSDR_TUNER_E4000   = 1,
    RTLSDR_TUNER_FC0012  = 2,
    RTLSDR_TUNER_FC0013  = 3,
    RTLSDR_TUNER_FC2580  = 4,
    RTLSDR_TUNER_R820T   = 5,
    RTLSDR_TUNER_R828D   = 6,
}
pub type rtlsdr_read_async_cb_t = ::std::option::Option<unsafe extern "C" fn(buf: *mut c_uchar, len: uint32_t, ctx: *mut c_void)>;

#[link(name = "rtlsdr", kind = "dylib")]
extern "C" {
    pub fn rtlsdr_get_device_count() -> uint32_t;
    pub fn rtlsdr_get_device_name(index: uint32_t) -> *const c_char;
    pub fn rtlsdr_get_device_usb_strings(index: uint32_t, manufact: *mut c_char, product: *mut c_char, serial: *mut c_char) -> c_int;
    pub fn rtlsdr_get_index_by_serial(serial: *const c_char) -> c_int;
    pub fn rtlsdr_open(dev: *mut *mut rtlsdr_dev_t, index: uint32_t) -> c_int;
    pub fn rtlsdr_close(dev: *mut rtlsdr_dev_t) -> c_int;
    pub fn rtlsdr_set_xtal_freq(dev: *mut rtlsdr_dev_t, rtl_freq: uint32_t, tuner_freq: uint32_t) -> c_int;
    pub fn rtlsdr_get_xtal_freq(dev: *mut rtlsdr_dev_t, rtl_freq: *mut uint32_t, tuner_freq: *mut uint32_t) -> c_int;
    pub fn rtlsdr_get_usb_strings(dev: *mut rtlsdr_dev_t, manufact: *mut c_char, product: *mut c_char, serial: *mut c_char) -> c_int;
    pub fn rtlsdr_write_eeprom(dev: *mut rtlsdr_dev_t, data: *mut uint8_t, offset: uint8_t, len: uint16_t) -> c_int;
    pub fn rtlsdr_read_eeprom(dev: *mut rtlsdr_dev_t, data: *mut uint8_t, offset: uint8_t, len: uint16_t) -> c_int;
    pub fn rtlsdr_set_center_freq(dev: *mut rtlsdr_dev_t, freq: uint32_t) -> c_int;
    pub fn rtlsdr_get_center_freq(dev: *mut rtlsdr_dev_t) -> c_int;
    pub fn rtlsdr_set_freq_correction(dev: *mut rtlsdr_dev_t, ppm: c_int) -> c_int;
    pub fn rtlsdr_get_freq_correction(dev: *mut rtlsdr_dev_t) -> c_int;
    pub fn rtlsdr_get_tuner_type(dev: *mut rtlsdr_dev_t) -> rtlsdr_tuner;
    pub fn rtlsdr_get_tuner_gains(dev: *mut rtlsdr_dev_t, gains: *mut c_int) -> c_int;
    pub fn rtlsdr_set_tuner_gain(dev: *mut rtlsdr_dev_t, gain: c_int) -> c_int;
    pub fn rtlsdr_set_tuner_bandwidth(dev: *mut rtlsdr_dev_t, bw: uint32_t) -> c_int;
    pub fn rtlsdr_get_tuner_gain(dev: *mut rtlsdr_dev_t) -> c_int;
    pub fn rtlsdr_set_tuner_if_gain(dev: *mut rtlsdr_dev_t, stage: c_int, gain: c_int) -> c_int;
    pub fn rtlsdr_set_tuner_gain_mode(dev: *mut rtlsdr_dev_t, manual: c_int) -> c_int;
    pub fn rtlsdr_set_sample_rate(dev: *mut rtlsdr_dev_t, rate: uint32_t) -> c_int;
    pub fn rtlsdr_get_sample_rate(dev: *mut rtlsdr_dev_t) -> c_int;
    pub fn rtlsdr_set_testmode(dev: *mut rtlsdr_dev_t, on: c_int) -> c_int;
    pub fn rtlsdr_set_agc_mode(dev: *mut rtlsdr_dev_t, on: c_int) -> c_int;
    pub fn rtlsdr_set_direct_sampling(dev: *mut rtlsdr_dev_t, on: c_int) -> c_int;
    pub fn rtlsdr_get_direct_sampling(dev: *mut rtlsdr_dev_t) -> c_int;
    pub fn rtlsdr_set_offset_tuning(dev: *mut rtlsdr_dev_t, on: c_int) -> c_int;
    pub fn rtlsdr_get_offset_tuning(dev: *mut rtlsdr_dev_t) -> c_int;
    pub fn rtlsdr_reset_buffer(dev: *mut rtlsdr_dev_t) -> c_int;
    pub fn rtlsdr_read_sync(dev: *mut rtlsdr_dev_t, buf: *mut c_void, len: c_int, n_read: *mut c_int) -> c_int;
    pub fn rtlsdr_wait_async(dev: *mut rtlsdr_dev_t, cb: rtlsdr_read_async_cb_t, ctx: *mut c_void) -> c_int;
    pub fn rtlsdr_read_async(dev: *mut rtlsdr_dev_t, cb: rtlsdr_read_async_cb_t, ctx: *mut c_void, buf_num: uint32_t, buf_len: uint32_t) -> c_int;
    pub fn rtlsdr_cancel_async(dev: *mut rtlsdr_dev_t) -> c_int;
}
