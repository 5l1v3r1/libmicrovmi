use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Instant;

use clap::{App, Arg, ArgMatches};
use colored::*;
use env_logger;

use microvmi::api::{CrType, EventReplyType, EventType, InterceptType, Introspectable};

fn parse_args() -> ArgMatches<'static> {
    App::new(file!())
        .version("0.3")
        .author("Mathieu Tarral")
        .about("Watches control register VMI events")
        .arg(Arg::with_name("vm_name").index(1).required(true))
        .arg(
            Arg::with_name("register")
                .multiple(true)
                .takes_value(true)
                .short("r")
                .default_value("3")
                .help("control register to intercept. Possible values: [0 3 4]"),
        )
        .get_matches()
}

fn toggle_cr_intercepts(drv: &mut Box<dyn Introspectable>, vec_cr: &Vec<CrType>, enabled: bool) {
    drv.pause().expect("Failed to pause VM");

    for cr in vec_cr {
        let intercept = InterceptType::Cr(*cr);
        let status_str = if enabled { "Enabling" } else { "Disabling" };
        println!("{} intercept on {:?}", status_str, cr);
        for vcpu in 0..drv.get_vcpu_count().unwrap() {
            drv.toggle_intercept(vcpu, intercept, enabled)
                .expect(&format!("Failed to enable {:?}", cr));
        }
    }

    drv.resume().expect("Failed to resume VM");
}

fn main() {
    env_logger::init();

    let matches = parse_args();

    let domain_name = matches.value_of("vm_name").unwrap();
    let registers: Vec<_> = matches.values_of("register").unwrap().collect();

    // check parameters
    let mut vec_cr = Vec::new();
    for reg_str in registers {
        let cr = match reg_str {
            "0" => CrType::Cr0,
            "3" => CrType::Cr3,
            "4" => CrType::Cr4,
            x => panic!(
                "Provided register value \"{}\" is not a valid/interceptable control register.",
                x
            ),
        };
        vec_cr.push(cr);
    }

    // set CTRL-C handler
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    println!("Initialize Libmicrovmi");
    let mut drv: Box<dyn Introspectable> = microvmi::init(domain_name, None);

    // enable control register interception
    toggle_cr_intercepts(&mut drv, &vec_cr, true);

    println!("Listen for control register events...");
    // record elapsed time
    let start = Instant::now();
    // listen
    let mut i: u64 = 0;
    while running.load(Ordering::SeqCst) {
        let event = drv.listen(1000).expect("Failed to listen for events");
        match event {
            Some(ev) => {
                let (cr_type, new) = match ev.kind {
                    EventType::Cr {
                        cr_type,
                        new,
                        old: _,
                    } => (cr_type, new),
                };
                let cr_color = match cr_type {
                    CrType::Cr0 => "blue",
                    CrType::Cr3 => "green",
                    CrType::Cr4 => "red",
                };
                let ev_nb_output = format!("{}", i).cyan();
                let vcpu_output = format!("VCPU {}", ev.vcpu).yellow();
                let cr_output = format!("{:?}", cr_type).color(cr_color);
                // let output = format!("[{}] VCPU {} - {:?}: 0x{:x}", i, ev.vcpu, cr_type, new);
                println!(
                    "[{}] {} - {}: 0x{:x}",
                    ev_nb_output, vcpu_output, cr_output, new
                );
                drv.reply_event(ev, EventReplyType::Continue)
                    .expect("Failed to send event reply");
                i = i + 1;
            }
            None => println!("No events yet..."),
        }
    }
    let duration = start.elapsed();

    // disable control register interception
    toggle_cr_intercepts(&mut drv, &vec_cr, false);

    let ev_per_sec = i as f64 / duration.as_secs_f64();
    println!(
        "Caught {} events in {:.2} seconds ({:.2} events/sec)",
        i,
        duration.as_secs_f64(),
        ev_per_sec
    );
}
