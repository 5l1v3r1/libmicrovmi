use clap::{App, Arg, ArgMatches};
use env_logger;

use microvmi::api::{DriverInitParam, Introspectable, Registers};

fn parse_args() -> ArgMatches<'static> {
    App::new(file!())
        .version("0.1")
        .author("Mathieu Tarral")
        .about("Dumps the state of registers on VCPU 0")
        .arg(Arg::with_name("vm_name").index(1).required(true))
        .arg(
            Arg::with_name("kvmi_socket")
                .short("k")
                .takes_value(true)
                .help(
                "pass additional KVMi socket initialization parameter required for the KVM driver",
            ),
        )
        .get_matches()
}

fn main() {
    env_logger::init();

    let matches = parse_args();
    let domain_name = matches.value_of("vm_name").unwrap();

    let init_option = match matches.value_of("kvmi_socket") {
        None => None,
        Some(socket) => Some(DriverInitParam::KVMiSocket(String::from(socket))),
    };
    let mut drv: Box<dyn Introspectable> = microvmi::init(domain_name, None, init_option);

    println!("pausing the VM");
    drv.pause().expect("Failed to pause VM");

    println!("dumping registers on VCPU 0");
    let regs: Registers = drv.read_registers(0).expect("Failed to read registers");
    println!("{:#x?}", regs);

    println!("resuming the VM");
    drv.resume().expect("Failed to resume VM");
}
