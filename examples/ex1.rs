

extern crate rtlsdr;

use rtlsdr::Error;

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
    let freq_corr = dev.get_freq_correction();
    println!("get_freq_correction - {}", freq_corr);

    let err = dev.set_freq_correction(freq_corr+1);
    match err {
        Error::NoError => println!("set_freq_correction successful - {}", freq_corr),
        _ => return err,
    };

    //----------  ----------
    Error::NoError
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
    };

    err = sdr_config(&dev);
    match err {
        Error::NoError => println!("sdr_config successful..."),
        _ => return,
    };

    err = dev.close();
    match err {
        Error::NoError => println!("close successful..."),
        _ => println!("dev close error - {:?}", err),
    };
}
