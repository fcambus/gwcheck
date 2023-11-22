/*
 * gwcheck
 * Copyright (c) 2021-2023, Frederic Cambus
 * https://github.com/fcambus/gwcheck
 *
 * Created: 2011-11-04
 * Last Updated: 2023-11-23
 *
 * gwcheck is released under the BSD 2-Clause license.
 * See LICENSE file for details.
 *
 * SPDX-License-Identifier: BSD-2-Clause
 */

use goblin::elf::Elf;
use getopt::Opt;
use std::fs;
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

    let object = fs::read(&args[0])?;

    let elf = match Elf::parse(&object) {
        Ok(elf) => elf,
        Err(err) => {
            eprintln!("Error: {}", err);
            process::exit(1);
        }
    };

    for section in &elf.section_headers {
        if let Some(Ok(name)) = elf.shdr_strtab.get(section.sh_name) {
            if name.starts_with(".gnu.warning.") {
                let offset = section.sh_offset as usize;
                let size = section.sh_size as usize;

                let data: Vec<u8> = object[offset..(offset + size)].to_vec();

                println!("{}:\n\t{}", name, String::from_utf8_lossy(&data));
            }
        }
    }

    Ok(())
}
