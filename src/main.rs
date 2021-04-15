use clap::{App, AppSettings, Arg, SubCommand};

#[macro_use]
extern crate clap;

fn main() {
    let args = App::new(crate_name!())
        .about(crate_description!())
        .version(crate_version!())
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("extract")
                .about("Extract data as csv files")
                .arg(Arg::from_usage(
                    "-c --config <FILE> 'Configuration file required'",
                ))
                .arg(Arg::from_usage(
                    "-o --output [PATH] 'Set output path (default: current)'",
                )),
        )
        .subcommand(
            SubCommand::with_name("validate")
                .about("Run your validations")
                .arg(Arg::from_usage(
                    "-c --config <FILE> 'Configuration file required'",
                )),
        )
        .get_matches();

    match args.subcommand() {
        ("extract", Some(sub_args)) => {
            let config = sub_args.value_of("config").unwrap();
            println!("config file: {}", config);
            unimplemented!();
        }
        ("validate", Some(sub_args)) => {
            let config = sub_args.value_of("config").unwrap();
            println!("config file: {}", config);
            unimplemented!();
        }
        _ => unreachable!(),
    }
}
