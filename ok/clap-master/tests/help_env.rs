#![cfg(feature = "env")]

use std::env;

use clap::{App, Arg};

mod utils;

static HIDE_ENV: &str = "ctest 0.1

USAGE:
    ctest [OPTIONS]

OPTIONS:
    -c, --cafe <FILE>    A coffeehouse, coffee shop, or café.
    -h, --help           Print help information
    -V, --version        Print version information
";

static SHOW_ENV: &str = "ctest 0.1

USAGE:
    ctest [OPTIONS]

OPTIONS:
    -c, --cafe <FILE>    A coffeehouse, coffee shop, or café. [env: ENVVAR=MYVAL]
    -h, --help           Print help information
    -V, --version        Print version information
";

static HIDE_ENV_VALS: &str = "ctest 0.1

USAGE:
    ctest [OPTIONS]

OPTIONS:
    -c, --cafe <FILE>    A coffeehouse, coffee shop, or café. [env: ENVVAR]
    -h, --help           Print help information
    -V, --version        Print version information
";

static SHOW_ENV_VALS: &str = "ctest 0.1

USAGE:
    ctest [OPTIONS]

OPTIONS:
    -c, --cafe <FILE>    A coffeehouse, coffee shop, or café. [env: ENVVAR=MYVAL]
    -h, --help           Print help information
    -V, --version        Print version information
";

static HIDE_ENV_FLAG: &str = "ctest 0.1

USAGE:
    ctest [OPTIONS]

OPTIONS:
    -c, --cafe       A coffeehouse, coffee shop, or café.
    -h, --help       Print help information
    -V, --version    Print version information
";

static SHOW_ENV_FLAG: &str = "ctest 0.1

USAGE:
    ctest [OPTIONS]

OPTIONS:
    -c, --cafe       A coffeehouse, coffee shop, or café. [env: ENVVAR=MYVAL]
    -h, --help       Print help information
    -V, --version    Print version information
";

static HIDE_ENV_VALS_FLAG: &str = "ctest 0.1

USAGE:
    ctest [OPTIONS]

OPTIONS:
    -c, --cafe       A coffeehouse, coffee shop, or café. [env: ENVVAR]
    -h, --help       Print help information
    -V, --version    Print version information
";

static SHOW_ENV_VALS_FLAG: &str = "ctest 0.1

USAGE:
    ctest [OPTIONS]

OPTIONS:
    -c, --cafe       A coffeehouse, coffee shop, or café. [env: ENVVAR=MYVAL]
    -h, --help       Print help information
    -V, --version    Print version information
";

#[test]
fn hide_env() {
    env::set_var("ENVVAR", "MYVAL");

    let app = App::new("ctest").version("0.1").arg(
        Arg::new("cafe")
            .short('c')
            .long("cafe")
            .value_name("FILE")
            .hide_env(true)
            .env("ENVVAR")
            .about("A coffeehouse, coffee shop, or café.")
            .takes_value(true),
    );

    assert!(utils::compare_output(app, "ctest --help", HIDE_ENV, false));
}

#[test]
fn show_env() {
    env::set_var("ENVVAR", "MYVAL");

    let app = App::new("ctest").version("0.1").arg(
        Arg::new("cafe")
            .short('c')
            .long("cafe")
            .value_name("FILE")
            .env("ENVVAR")
            .about("A coffeehouse, coffee shop, or café.")
            .takes_value(true),
    );

    assert!(utils::compare_output(app, "ctest --help", SHOW_ENV, false));
}

#[test]
fn hide_env_vals() {
    env::set_var("ENVVAR", "MYVAL");

    let app = App::new("ctest").version("0.1").arg(
        Arg::new("cafe")
            .short('c')
            .long("cafe")
            .value_name("FILE")
            .hide_env_values(true)
            .env("ENVVAR")
            .about("A coffeehouse, coffee shop, or café.")
            .takes_value(true),
    );

    assert!(utils::compare_output(
        app,
        "ctest --help",
        HIDE_ENV_VALS,
        false
    ));
}

#[test]
fn show_env_vals() {
    env::set_var("ENVVAR", "MYVAL");

    let app = App::new("ctest").version("0.1").arg(
        Arg::new("cafe")
            .short('c')
            .long("cafe")
            .value_name("FILE")
            .env("ENVVAR")
            .about("A coffeehouse, coffee shop, or café.")
            .takes_value(true),
    );

    assert!(utils::compare_output(
        app,
        "ctest --help",
        SHOW_ENV_VALS,
        false
    ));
}

#[test]
fn hide_env_flag() {
    env::set_var("ENVVAR", "MYVAL");

    let app = App::new("ctest").version("0.1").arg(
        Arg::new("cafe")
            .short('c')
            .long("cafe")
            .hide_env(true)
            .env("ENVVAR")
            .about("A coffeehouse, coffee shop, or café."),
    );

    assert!(utils::compare_output(
        app,
        "ctest --help",
        HIDE_ENV_FLAG,
        false
    ));
}

#[test]
fn show_env_flag() {
    env::set_var("ENVVAR", "MYVAL");

    let app = App::new("ctest").version("0.1").arg(
        Arg::new("cafe")
            .short('c')
            .long("cafe")
            .env("ENVVAR")
            .about("A coffeehouse, coffee shop, or café."),
    );

    assert!(utils::compare_output(
        app,
        "ctest --help",
        SHOW_ENV_FLAG,
        false
    ));
}

#[test]
fn hide_env_vals_flag() {
    env::set_var("ENVVAR", "MYVAL");

    let app = App::new("ctest").version("0.1").arg(
        Arg::new("cafe")
            .short('c')
            .long("cafe")
            .hide_env_values(true)
            .env("ENVVAR")
            .about("A coffeehouse, coffee shop, or café."),
    );

    assert!(utils::compare_output(
        app,
        "ctest --help",
        HIDE_ENV_VALS_FLAG,
        false
    ));
}

#[test]
fn show_env_vals_flag() {
    env::set_var("ENVVAR", "MYVAL");

    let app = App::new("ctest").version("0.1").arg(
        Arg::new("cafe")
            .short('c')
            .long("cafe")
            .env("ENVVAR")
            .about("A coffeehouse, coffee shop, or café."),
    );

    assert!(utils::compare_output(
        app,
        "ctest --help",
        SHOW_ENV_VALS_FLAG,
        false
    ));
}
