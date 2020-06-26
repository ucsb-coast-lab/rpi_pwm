// Interrupting the process by pressing Ctrl-C causes the application to exit
// immediately without disabling the PWM channel. Check out the
// gpio_blinkled_signals.rs example to learn how to properly handle incoming
// signals to prevent an abnormal termination.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::error::Error;
use std::thread;
use std::time::Duration;

// The simple-signal crate is used to handle incoming signals.
use simple_signal::{self, Signal};

use rppal::pwm::{Channel, Polarity, Pwm};
//
// Period: 20 ms (50 Hz). Pulse width: min. 1200 µs, neutral 1500 µs, max. 1800 µs.
const PERIOD_MS: u64 = 20;
const PULSE_MIN_US: u64 = 1200;
const PULSE_NEUTRAL_US: u64 = 1500;
const PULSE_MAX_US: u64 = 1800;

fn main() -> Result<(), Box<dyn Error>> {
    // Enable PWM channel 0 (BCM GPIO 18, physical pin 12) with the specified period,
    // and rotate the servo by setting the pulse width to its maximum value.
    let pwm = Pwm::with_period(
        Channel::Pwm0,
        Duration::from_millis(PERIOD_MS),
        Duration::from_micros(PULSE_MAX_US),
        Polarity::Normal,
        true,
    )?;

    // PWM startup sequence
    thread::sleep(Duration::from_millis(500));
    pwm.set_pulse_width(Duration::from_micros(PULSE_MIN_US))?;
    thread::sleep(Duration::from_millis(500));
    pwm.set_pulse_width(Duration::from_micros(PULSE_NEUTRAL_US))?;
    thread::sleep(Duration::from_millis(300));
    pwm.set_pulse_width(Duration::from_micros(PULSE_MAX_US))?;
    thread::sleep(Duration::from_millis(20));

    // We'll be checking to see if Ctrl^C is called, and if so, will kill the PWM signal
    let running = Arc::new(AtomicBool::new(true));
    // When a SIGINT (Ctrl-C) or SIGTERM signal is caught, atomically set running to false.
    simple_signal::set_handler(&[Signal::Int, Signal::Term], {
        let running = running.clone();
        move |_| {
            running.store(false, Ordering::SeqCst);
        }
    });

    // Run the PWM signal at ~1/3 (1500-1800 positive range) thrust
    // Wil run for either 10 seconds or until command is manually canceled
    let mut i = 0;
    while running.load(Ordering::SeqCst) && (i < 10) {
        pwm.set_pulse_width(Duration::from_micros(1600))?;
        thread::sleep(Duration::from_millis(1000));
        i +=1;
    }
    
    // Makes sure PWM gets disabled before shutdown, although this should happen 
    // when the pwm struct goes out of scope at the end anyway
    pwm.disable()?;

    Ok(())

}