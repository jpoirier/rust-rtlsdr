#![allow(non_camel_case_types, non_snake_case)]

extern crate rtlsdr;

use std::os::raw::{c_void, c_uchar};
use std::ptr;
use rtlsdr::Error;
use std::time::Duration;
use std::thread;


pub type uint32_t = u32;

fn sdr_config(dev: &rtlsdr::Device) -> Error {
    let (m, p, s, mut err) = dev.get_usb_strings();
    match err {
        Error::NoError => println!("set_xtal_freq successful"),
        _ => return err,
    };
    println!("m: {}, p: {}, s: {}, err: {:?}", m, p, s, err);


    //---------- Get Tuner Gain ----------
    println!("get_tuner_type: {}", dev.get_tuner_type());
    err = dev.set_xtal_freq(28800000, 28800000);
    match err {
        Error::NoError => println!("get_tuner_type successful - 28800000"),
        _ => return err,
    };

    //---------- Set Tuner Gain ----------
    err = dev.set_tuner_gain_mode(true);
    match err {
        Error::NoError => println!("set_tuner_gain_mode successful..."),
        _ => return err,
    };

    let (gains, mut err) = dev.get_tuner_gains();
    match err {
        Error::NoError => println!("get_tuner_gains successful..."),
        _ => println!("get_tuner_gains failed - {:?}", err), // return err,
    };

    println!("tuner gains:  {:?}", gains);

    err = dev.set_tuner_gain(gains[2]);
    match err {
        Error::NoError => println!("set_tuner_gain successful..."),
        _ => return err,
    };

    //---------- Get/Set Sample Rate ----------
    let samplerate: i32 = 2083334;
    err = dev.set_sample_rate(samplerate);
    match err {
        Error::NoError => println!("set_sample_rate {} successful...", samplerate),
        _ => return err,
    };

    println!("get_sample_rate {} successful...", dev.get_sample_rate());

	//---------- Get/Set Xtal Freq ----------
    let (mut rtl_freq, mut tuner_freq, mut err) = dev.get_xtal_freq();
    match err {
        Error::NoError => println!("get_xtal_freq successful - rtl_freq: {}, tuner_freq: {}", rtl_freq, tuner_freq),
        _ => return err,
    };

    rtl_freq = 28800000;
    tuner_freq = 28800000;

    err = dev.set_xtal_freq(rtl_freq, tuner_freq);
    match err {
        Error::NoError => println!("set_xtal_freq successful - rtl_freq: {}, tuner_freq: {}", rtl_freq, tuner_freq),
        _ => return err,
    };

	//---------- Get/Set Center Freq ----------
    err = dev.set_center_freq(978000000);
    match err {
        Error::NoError => println!("set_center_freq successful - 978000000"),
        _ => return err,
    };

    println!("get_center_freq: {}", dev.get_center_freq());

	//---------- Set Tuner Bandwidth ----------
    let bw: i32 = 1000000;
    println!("Setting bandwidth: {}", bw);

    err = dev.set_tuner_bandwidth(bw);
    match err {
        Error::NoError => println!("set_tuner_bandwidth {} Successful", bw),
        _ => return err,
    };

	//---------- Buffer Reset ----------
    err = dev.reset_buffer();
    match err {
        Error::NoError => println!("reset_buffer successful..."),
        _ => return err,
    };

	//---------- Get/Set Freq Correction ----------
    let mut freq_corr = dev.get_freq_correction();
    println!("get_freq_correction - {}", freq_corr);

    freq_corr += 1;
    let err = dev.set_freq_correction(freq_corr);
    match err {
        Error::NoError => println!("set_freq_correction successful - {}", freq_corr),
        _ => return err,
    };

    //----------  ----------
    Error::NoError
}

unsafe extern "C" fn read_async_callback(buf: *mut c_uchar, len: uint32_t, ctx: *mut c_void) {
    let _ = ctx;
    let v =Vec::<u8>::from_raw_parts(buf, len as usize, len as usize);
    println!("----- read_async_callback buffer size - {}", len);
    println!("----- {} {} {} {} {} {}", v[0], v[1], v[2], v[3], v[4], v[5]);
}


fn main() {
    //---------- Device Check ----------
    let count = rtlsdr::get_device_count();
    if count == 0 {
        println!("No devices found, exiting.");
        return;
    }

    for i in 0..count {
        let (m, p, s, err) = rtlsdr::get_device_usb_strings(i);
        println!("get_device_usb_strings: {:?} - {} {} {}", err, m, p, s);
    }

    let index = 0;
    println!("===== Device name, index {}: {} =====", index, rtlsdr::get_device_name(0));
    println!("===== Running tests using device indx: 0 =====\n");

    let (dev, mut err) = rtlsdr::open(index);
    match err {
        Error::NoError => println!("open successful"),
        _ => return,
    }

    err = sdr_config(&dev);
    match err {
        Error::NoError => println!("sdr_config successful..."),
        _ => return,
    }

    println!("calling read_sync...");
    for i in 0..10 {
        let (_, read_count, err) = dev.read_sync(rtlsdr::DefaultBufLength);
        println!("----- read_sync requested iteration {} -----", i);
        println!("\tread_sync requested - {}", rtlsdr::DefaultBufLength);
        println!("\tread_sync received  - {}", read_count);
        println!("\tread_sync err msg   - {:?}", err);
    }

    dev.reset_buffer();

    let d = dev.clone();
    thread::spawn(move || {
        println!("async_stop thread sleeping for 5 seconds...");
        thread::sleep(Duration::from_millis(5000));
        println!("async_stop thread awake, canceling read async...");
        d.cancel_async();
    });

    println!("calling read_async...");
    err = dev.read_async(Some(read_async_callback),
                        ptr::null_mut(),
                        rtlsdr::DefaultAsyncBufNumber,
                        rtlsdr::DefaultBufLength);
    match err {
        Error::NoError => println!("device close successful..."),
        _ => println!("dev close error - {:?}", err),
    }

    err = dev.close();
    match err {
        Error::NoError => println!("device close successful..."),
        _ => println!("dev close error - {:?}", err),
    }
}
