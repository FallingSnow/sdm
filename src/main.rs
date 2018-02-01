#[macro_use]
extern crate clap;
#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_term;

extern crate serde;
#[macro_use]
extern crate serde_derive;

extern crate dbus;
extern crate pam_auth;

extern crate config;

use clap::{App, Arg, ArgMatches};
use slog::{Drain, Logger};
use slog_async::Async;
use slog_term::{FullFormat, TermDecorator};
use dbus::{BusType, Connection, Message};

use std::{rc, cell};

#[derive(Debug, Deserialize)]
struct System {

}
#[derive(Debug, Deserialize)]
struct Settings {
    system: System,
}

fn setup_logger() -> Logger {
    let decor = TermDecorator::new().build();
    let drain = FullFormat::new(decor).build().fuse();
    let drain = Async::new(drain).build().fuse();
    let log = Logger::root(drain, o!());
    debug!(log, "Initialized logging");
    log
}

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .multiple(true)
                .help("Enable verbose output"),
        )
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .default_value("/etc/sdm/sdm.yaml")
                .help("Configuration file"),
        )
        .get_matches();

    let log = setup_logger();

    match run(&log, matches) {
        Ok(()) => {
            info!(log, "Done!");
        }
        Err(e) => {
            error!(log, "{}", e);
        }
    }
}

fn get_settings(path: &str) -> Result<Settings, config::ConfigError> {
    let mut settings = config::Config::new();
    settings.merge(config::File::with_name(path))?;
    settings.try_into()
}

fn run(log: &Logger, args: ArgMatches) -> Result<(), String> {
    let settings = get_settings(args.value_of("config").unwrap()).expect("Could not get config file.");
    // let con = Connection::get_private(BusType::System).unwrap();
    // let m = Message::new_method_call(
    //     "org.freedesktop.login1",
    //     "/org/freedesktop/login1",
    //     "org.freedesktop.login1.Manager",
    //     "GetUser",
    // ).unwrap().append(1001u32);

    // let done: rc::Rc<cell::Cell<bool>> = Default::default();
    // let done2 = done.clone();
    // con.add_handler(con.send_with_reply(m, move |reply| {
    //     let v: Vec<&str> = reply.unwrap().read1().unwrap();
    //     println!("The names on the D-Bus are: {:?}", v);
    //     done2.set(true);
    // }).unwrap());
    // while !done.get() {
    //     con.incoming(100).next();
    // }
    // let res = con.send_with_reply_and_block(m, 1000i32).unwrap().get_items();
    // trace!(log, "{:?}", res);

    let user = "test";
    let password = "testpassword";
    let mut authenticator = pam_auth::Authenticator::new("sdm").expect("Unable to create authenticator.");
    authenticator.set_credentials(user, password);
    authenticator.authenticate().expect("Unable to authenticate user.");
    trace!(log, "Authenticated user: {}", user);
    authenticator.open_session().expect("Unable to open session.");
    trace!(log, "Opened session");

    for (key, value) in std::env::vars() {
        trace!(log, "{}: {}", key, value);
    }

    trace!(log, "{:?}", args);

    Ok(())
}
