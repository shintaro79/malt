use clap::{App, AppSettings, SubCommand};

#[macro_use]
extern crate clap;

fn main() {
    let options = App::new(crate_name!())
        .about(crate_description!())
        .version(crate_version!())
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("extract")
                .about("Extract data as csv files")
        )
        .subcommand(
            SubCommand::with_name("validate")
                .about("Run your validations")
        );

    let matchers = options.get_matches();
    match matchers.subcommand() {
        ("extract", sub_match) => {
            unimplemented!();
        }
        ("validate", sub_match) => {
            unimplemented!();
        }
        _ => unreachable!(),
    }
}
