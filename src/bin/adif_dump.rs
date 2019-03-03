//
// src/bin/adif_dump.rs: dumps contents of an ADIF file
//

use std::env;
use std::fs;
use std::process;

extern crate adif;

fn main()
{
    let argv : Vec<String> = env::args().collect();
    let progname = if argv.len() > 0 { &argv[0] } else { "adif_dump" };
    let mut i = 1;
    let mut colspec : Option<Vec<&String>> = None;
    let mut colnames : Vec<&String> = Vec::new();

    /*
     * This is very primitive option parsing for now.
     */
    while i < argv.len() && argv[i].starts_with("-") {
        if argv[i] == "--" {
            i += 1;
            break;
        }

        if argv[i] == "-o" {
            if i + 1 >= argv.len() {
                usage(progname,
                    &format!("option requires an argument: {}", argv[i]));
            }

            colnames.push(&argv[i + 1]);
            i += 2;
            continue;
        }

        usage(progname, &format!("unrecognized option: {}", argv[i]));
    }

    if i != argv.len() - 1 {
        usage(progname, "expected one argument");
    }

    if colnames.len() > 0 {
        colspec = Some(colnames);
    }

    let filename = &argv[i];
    let which = adif::AdifDumpWhichRecords::ADR_ALL;

    match adif_dump_file(filename, which, &colspec) {
        Ok(()) => (),
        Err(errmsg) => fatal(progname, &errmsg)
    }
}

fn usage(progname: &str, message: &str)
{
    eprintln!("{}", message);
    eprintln!("usage: {} FILENAME", progname);
    process::exit(2);
}

fn fatal(progname: &str, message: &str)
{
    eprintln!("{}: {}", progname, message);
    process::exit(1);
}

pub fn adif_dump_file(filename: &str, which: adif::AdifDumpWhichRecords,
    colspec : &Option<Vec<&String>>) ->
    Result<(), String>
{
    let mut file = match fs::File::open(filename) {
        Ok(file) => file,
        Err(error) => {
            return Err(format!("open \"{}\": {}", filename, error))
        }
    };

    match adif::adif_parse(filename, &mut file) {
        Ok(adif) => Ok(adif::adif_dump(adif, which, colspec)),
        Err(err) => Err(format!("{}", err))
    }
}
