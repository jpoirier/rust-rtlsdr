#![allow(dead_code)]
use std::sync::Arc;
use std::os::raw::{c_int, c_void, c_uchar, c_char};
use std::option::Option;
use std::string::String;
use std::ffi::CStr;
use std::ptr;
use std::str;



// Sampling modes.
pub enum SamplingMode {
    None = 0,
    IADC = 1,
    QADC = 2,
    Unknown = 3,
}

// pub const DefaultGain: String = "auto";
pub const DEFAULT_FC: i32 = 80_000_000;
pub const DEFAULT_RS: i32 = 1_024_000;
pub const DEFAULT_READ_SIZE: i32 = 1024;
pub const CRYSTAL_FREQ: i32 = 28800000;
pub const DEFAULT_SAMPLE_RATE: i32 = 2048000;
pub const DEFAULT_ASYNC_BUF_NUMBER: i32 = 32;
pub const DEFAULT_BUF_LENGTH: i32 = (16 * 16384);
pub const MIN_BUF_LENGTH: i32 = 512;
pub const MAX_BUF_LENGTH: i32 = (256 * 16384);
pub const LIBUSB_ERROR_OTHER: i32 = -99;
pub const EEPROM_SIZE: i32 = 256;
// MAX_STR_SIZE = (max string length - 2 (header bytes)) \ 2. Where each
// info character is followed by a null char.
pub const MAX_STR_SIZE: i32 = 35;
pub const STR_OFFSET_START: i32 = 0x09;

enum RTLSDRDev { }
type RTLSDRDevT = RTLSDRDev;

#[derive(Copy, Clone)]
pub struct Device {
    dev: *mut RTLSDRDevT,
}

unsafe impl Send for Device {}
unsafe impl Sync for Device {}


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
    Other,
}

/// read async callabck function
pub type ReadAsyncCbT = Option<unsafe extern "C" fn(buf: *mut c_uchar,
                                                              len: u32,
                                                              ctx: *mut c_void)>;

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
    fn rtlsdr_set_xtal_freq(dev: *mut RTLSDRDevT,
                            rtl_freq: u32,
                            tuner_freq: u32)
                            -> c_int;
    fn rtlsdr_get_xtal_freq(dev: *mut RTLSDRDevT,
                            rtl_freq: *mut u32,
                            tuner_freq: *mut u32)
                            -> c_int;
    fn rtlsdr_get_usb_strings(dev: *mut RTLSDRDevT,
                              manufact: *mut c_char,
                              product: *mut c_char,
                              serial: *mut c_char)
                              -> c_int;
    fn rtlsdr_write_eeprom(dev: *mut RTLSDRDevT,
                           data: *mut u8,
                           offset: u8,
                           len: u16)
                           -> c_int;
    fn rtlsdr_read_eeprom(dev: *mut RTLSDRDevT,
                          data: *mut u8,
                          offset: u8,
                          len: u16)
                          -> c_int;
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
    fn rtlsdr_wait_async(dev: *mut RTLSDRDevT,
                         cb: ReadAsyncCbT,
                         ctx: *mut c_void)
                         -> c_int;
    fn rtlsdr_read_async(dev: *mut RTLSDRDevT,
                         cb: ReadAsyncCbT,
                         ctx: *mut c_void,
                         buf_num: u32,
                         buf_len: u32)
                         -> c_int;
    fn rtlsdr_cancel_async(dev: *mut RTLSDRDevT) -> c_int;
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
    unsafe {
        CStr::from_ptr(rtlsdr_get_device_name(index as u32)).to_string_lossy().into_owned()
    }
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
    pub fn set_xtal_freq(&self, rtl_freq_hz: i32, tuner_freq_hz: i32) -> Error {
        unsafe {
            get_err_msg(rtlsdr_set_xtal_freq(self.dev,
                                             rtl_freq_hz as u32,
                                             tuner_freq_hz as u32))
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

    /// Writes data to the EEPROM.
    pub fn write_eeprom(&self, data: Vec<u8>, offset: u8) -> Error {
        unsafe {
            get_err_msg(rtlsdr_write_eeprom(self.dev,
                                            data.as_ptr() as *mut u8,
                                            offset,
                                            data.len() as u16))
        }
    }

    /// Returns data read from the EEPROM.
    pub fn read_eeprom(&self, offset: u8, len: u16) -> (Vec<u8>, Error) {
        let mut v = vec![0u8; len as usize];
        unsafe {
            let err = rtlsdr_read_eeprom(self.dev, v.as_mut_ptr() as *mut u8, offset, len);
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
            let err = if i <= 0 { Error::Other } else { Error::NoError };
            (v, err)
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
    /// SetCenterFreq() will control the IF-frequency of the DDC, which
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
                _ => SamplingMode::Unknown,
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

    /// Reads samples asynchronously. Note, this function
    /// will block until canceled using CancelAsync. ReadAsyncCbT is
    /// a package global variable.
    ///
    /// Optional bufNum buffer count, bufNum * bufLen = overall buffer size,
    /// set to 0 for default buffer count (32).
    /// Optional bufLen buffer length, must be multiple of 512, set to 0 for
    /// default buffer length (16 * 32 * 512).
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
}
