#![allow(unused_assignments, unused_variables)]

use getopt::Opt;
use std::process;

fn usage() {
    println!(
        "gwcheck [-hv] object\n \
            The options are as follows:\n \
            	-h	Display usage.\n \
            	-v	Display version."
    );

    process::exit(0);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args: Vec<String> = std::env::args().collect();
    let mut opts = getopt::Parser::new(&args, "hv");

    loop {
        match opts.next().transpose()? {
            None => break,
            Some(opt) => match opt {
                Opt('h', None) => usage(),
                Opt('v', None) => {
                    println!("gwcheck 1.0.0");
                    process::exit(0);
                }
                _ => unreachable!(),
            },
        }
    }

    let args = args.split_off(opts.index());

    if args.len() < 1 {
        usage();
    }

    Ok(())
}
