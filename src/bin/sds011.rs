extern crate sds011;
use sds011::sensor::{SDS011};

use clap::{Arg, App};
use std::time::{Duration};
use std::thread::sleep;

fn main() {
    let matches = App::new("SDS011 Driver")
        .version("0.1.0")
        .author("Vadim Manaenko <vadim.razorq@gmail.com>")
        .about("Reads data from Nova SDS011 Sensor")
        .arg(Arg::with_name("port")
            .short("p")
            .long("port")
            .takes_value(true)
            .default_value("/dev/ttyUSB0")
            .help("Specify port a sensor is connected to"))
        .arg(Arg::with_name("work_period")
            .short("w")
            .long("work")
            .takes_value(true)
            .default_value("5")
            .help("Work period in minutes"))
        .get_matches();

    let port = matches.value_of("port").unwrap();
    let work_period_str = matches.value_of("work_period").unwrap();
    let work_period = work_period_str.parse::<u8>().unwrap();

    let mut sensor = SDS011::new(port);
    sensor.set_work_period(work_period);

    loop {
        let message = sensor.query().unwrap();
        println!("{:?}", message);

        sleep(Duration::from_secs(work_period as u64 * 60));
    }

}