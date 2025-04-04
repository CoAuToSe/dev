// Std
use std::{
    convert::From,
    error,
    fmt::{self, Debug, Display, Formatter},
    io,
    result::Result as StdResult,
};

// Internal
use crate::{
    build::Arg,
    output::fmt::Colorizer,
    parse::features::suggestions,
    util::{color::ColorChoice, safe_exit, SUCCESS_CODE, USAGE_CODE},
    App, AppSettings,
};

/// Short hand for [`Result`] type
///
/// [`Result`]: std::result::Result
pub type Result<T> = StdResult<T, Error>;

/// Command line argument parser kind of error
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ErrorKind {
    /// Occurs when an [`Arg`] has a set of possible values,
    /// and the user provides a value which isn't in that set.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind};
    /// let result = App::new("prog")
    ///     .arg(Arg::new("speed")
    ///         .possible_value("fast")
    ///         .possible_value("slow"))
    ///     .try_get_matches_from(vec!["prog", "other"]);
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().kind, ErrorKind::InvalidValue);
    /// ```
    InvalidValue,

    /// Occurs when a user provides a flag, option, argument or subcommand which isn't defined.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind};
    /// let result = App::new("prog")
    ///     .arg(Arg::from("--flag 'some flag'"))
    ///     .try_get_matches_from(vec!["prog", "--other"]);
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().kind, ErrorKind::UnknownArgument);
    /// ```
    UnknownArgument,

    /// Occurs when the user provides an unrecognized [`Subcommand`] which meets the threshold for
    /// being similar enough to an existing subcommand.
    /// If it doesn't meet the threshold, or the 'suggestions' feature is disabled,
    /// the more general [`UnknownArgument`] error is returned.
    ///
    /// # Examples
    ///
    #[cfg_attr(not(feature = "suggestions"), doc = " ```no_run")]
    #[cfg_attr(feature = "suggestions", doc = " ```")]
    /// # use clap::{App, Arg, ErrorKind, };
    /// let result = App::new("prog")
    ///     .subcommand(App::new("config")
    ///         .about("Used for configuration")
    ///         .arg(Arg::new("config_file")
    ///             .about("The configuration file to use")
    ///             .index(1)))
    ///     .try_get_matches_from(vec!["prog", "confi"]);
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().kind, ErrorKind::InvalidSubcommand);
    /// ```
    ///
    /// [`Subcommand`]: crate::Subcommand
    /// [`UnknownArgument`]: ErrorKind::UnknownArgument
    InvalidSubcommand,

    /// Occurs when the user provides an unrecognized [`Subcommand`] which either
    /// doesn't meet the threshold for being similar enough to an existing subcommand,
    /// or the 'suggestions' feature is disabled.
    /// Otherwise the more detailed [`InvalidSubcommand`] error is returned.
    ///
    /// This error typically happens when passing additional subcommand names to the `help`
    /// subcommand. Otherwise, the more general [`UnknownArgument`] error is used.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind, };
    /// let result = App::new("prog")
    ///     .subcommand(App::new("config")
    ///         .about("Used for configuration")
    ///         .arg(Arg::new("config_file")
    ///             .about("The configuration file to use")
    ///             .index(1)))
    ///     .try_get_matches_from(vec!["prog", "help", "nothing"]);
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().kind, ErrorKind::UnrecognizedSubcommand);
    /// ```
    ///
    /// [`Subcommand`]: crate::Subcommand
    /// [`InvalidSubcommand`]: ErrorKind::InvalidSubcommand
    /// [`UnknownArgument`]: ErrorKind::UnknownArgument
    UnrecognizedSubcommand,

    /// Occurs when the user provides an empty value for an option that does not allow empty
    /// values.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind, ArgSettings};
    /// let res = App::new("prog")
    ///     .arg(Arg::new("color")
    ///          .setting(ArgSettings::TakesValue)
    ///          .setting(ArgSettings::ForbidEmptyValues)
    ///          .long("color"))
    ///     .try_get_matches_from(vec!["prog", "--color="]);
    /// assert!(res.is_err());
    /// assert_eq!(res.unwrap_err().kind, ErrorKind::EmptyValue);
    /// ```
    EmptyValue,

    /// Occurs when the user doesn't use equals for an option that requires equal
    /// sign to provide values.
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind, ArgSettings};
    /// let res = App::new("prog")
    ///     .arg(Arg::new("color")
    ///          .setting(ArgSettings::TakesValue)
    ///          .setting(ArgSettings::RequireEquals)
    ///          .long("color"))
    ///     .try_get_matches_from(vec!["prog", "--color", "red"]);
    /// assert!(res.is_err());
    /// assert_eq!(res.unwrap_err().kind, ErrorKind::NoEquals);
    /// ```
    NoEquals,

    /// Occurs when the user provides a value for an argument with a custom validation and the
    /// value fails that validation.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind};
    /// fn is_numeric(val: &str) -> Result<(), String> {
    ///     match val.parse::<i64>() {
    ///         Ok(..) => Ok(()),
    ///         Err(..) => Err(String::from("Value wasn't a number!")),
    ///     }
    /// }
    ///
    /// let result = App::new("prog")
    ///     .arg(Arg::new("num")
    ///          .validator(is_numeric))
    ///     .try_get_matches_from(vec!["prog", "NotANumber"]);
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().kind, ErrorKind::ValueValidation);
    /// ```
    ValueValidation,

    /// Occurs when a user provides more values for an argument than were defined by setting
    /// [`Arg::max_values`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind};
    /// let result = App::new("prog")
    ///     .arg(Arg::new("arg")
    ///         .max_values(2))
    ///     .try_get_matches_from(vec!["prog", "too", "many", "values"]);
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().kind, ErrorKind::TooManyValues);
    /// ```
    /// [`Arg::max_values`]: Arg::max_values()
    TooManyValues,

    /// Occurs when the user provides fewer values for an argument than were defined by setting
    /// [`Arg::min_values`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind};
    /// let result = App::new("prog")
    ///     .arg(Arg::new("some_opt")
    ///         .long("opt")
    ///         .min_values(3))
    ///     .try_get_matches_from(vec!["prog", "--opt", "too", "few"]);
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().kind, ErrorKind::TooFewValues);
    /// ```
    /// [`Arg::min_values`]: Arg::min_values()
    TooFewValues,

    /// Occurs when a user provides more occurrences for an argument than were defined by setting
    /// [`Arg::max_occurrences`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind};
    /// let result = App::new("prog")
    ///     .arg(Arg::new("verbosity")
    ///         .short('v')
    ///         .max_occurrences(2))
    ///     .try_get_matches_from(vec!["prog", "-vvv"]);
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().kind, ErrorKind::TooManyOccurrences);
    /// ```
    /// [`Arg::max_occurrences`]: Arg::max_occurrences()
    TooManyOccurrences,

    /// Occurs when the user provides a different number of values for an argument than what's
    /// been defined by setting [`Arg::number_of_values`] or than was implicitly set by
    /// [`Arg::value_names`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind};
    /// let result = App::new("prog")
    ///     .arg(Arg::new("some_opt")
    ///         .long("opt")
    ///         .takes_value(true)
    ///         .number_of_values(2))
    ///     .try_get_matches_from(vec!["prog", "--opt", "wrong"]);
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().kind, ErrorKind::WrongNumberOfValues);
    /// ```
    ///
    /// [`Arg::number_of_values`]: Arg::number_of_values()
    /// [`Arg::value_names`]: Arg::value_names()
    WrongNumberOfValues,

    /// Occurs when the user provides two values which conflict with each other and can't be used
    /// together.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind};
    /// let result = App::new("prog")
    ///     .arg(Arg::new("debug")
    ///         .long("debug")
    ///         .conflicts_with("color"))
    ///     .arg(Arg::new("color")
    ///         .long("color"))
    ///     .try_get_matches_from(vec!["prog", "--debug", "--color"]);
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().kind, ErrorKind::ArgumentConflict);
    /// ```
    ArgumentConflict,

    /// Occurs when the user does not provide one or more required arguments.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind};
    /// let result = App::new("prog")
    ///     .arg(Arg::new("debug")
    ///         .required(true))
    ///     .try_get_matches_from(vec!["prog"]);
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().kind, ErrorKind::MissingRequiredArgument);
    /// ```
    MissingRequiredArgument,

    /// Occurs when a subcommand is required (as defined by [`AppSettings::SubcommandRequired`]),
    /// but the user does not provide one.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, AppSettings, ErrorKind};
    /// let err = App::new("prog")
    ///     .setting(AppSettings::SubcommandRequired)
    ///     .subcommand(App::new("test"))
    ///     .try_get_matches_from(vec![
    ///         "myprog",
    ///     ]);
    /// assert!(err.is_err());
    /// assert_eq!(err.unwrap_err().kind, ErrorKind::MissingSubcommand);
    /// # ;
    /// ```
    ///
    /// [`AppSettings::SubcommandRequired`]: crate::AppSettings::SubcommandRequired
    MissingSubcommand,

    /// Occurs when the user provides multiple values to an argument which doesn't allow that.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind};
    /// let result = App::new("prog")
    ///     .arg(Arg::new("debug")
    ///         .long("debug")
    ///         .multiple_occurrences(false))
    ///     .try_get_matches_from(vec!["prog", "--debug", "--debug"]);
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().kind, ErrorKind::UnexpectedMultipleUsage);
    /// ```
    UnexpectedMultipleUsage,

    /// Occurs when the user provides a value containing invalid UTF-8.
    ///
    /// To allow arbitrary data
    /// - Set [`ArgSettings::AllowInvalidUtf8`] for argument values
    /// - Set [`AppSettings::AllowInvalidUtf8ForExternalSubcommands`] for external-subcommand
    ///   values
    ///
    /// # Platform Specific
    ///
    /// Non-Windows platforms only (such as Linux, Unix, OSX, etc.)
    ///
    /// # Examples
    ///
    #[cfg_attr(not(unix), doc = " ```ignore")]
    #[cfg_attr(unix, doc = " ```")]
    /// # use clap::{App, Arg, ErrorKind, AppSettings};
    /// # use std::os::unix::ffi::OsStringExt;
    /// # use std::ffi::OsString;
    /// let result = App::new("prog")
    ///     .arg(Arg::new("utf8")
    ///         .short('u')
    ///         .takes_value(true))
    ///     .try_get_matches_from(vec![OsString::from("myprog"),
    ///                                 OsString::from("-u"),
    ///                                 OsString::from_vec(vec![0xE9])]);
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().kind, ErrorKind::InvalidUtf8);
    /// ```
    ///
    /// [`ArgSettings::AllowInvalidUtf8`]: crate::ArgSettings::AllowInvalidUtf8
    /// [`AppSettings::AllowInvalidUtf8ForExternalSubcommands`]: crate::AppSettings::AllowInvalidUtf8ForExternalSubcommands
    InvalidUtf8,

    /// Not a true "error" as it means `--help` or similar was used.
    /// The help message will be sent to `stdout`.
    ///
    /// **Note**: If the help is displayed due to an error (such as missing subcommands) it will
    /// be sent to `stderr` instead of `stdout`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind};
    /// let result = App::new("prog")
    ///     .try_get_matches_from(vec!["prog", "--help"]);
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().kind, ErrorKind::DisplayHelp);
    /// ```
    DisplayHelp,

    /// Occurs when either an argument or a [`Subcommand`] is required, as defined by
    /// [`AppSettings::ArgRequiredElseHelp`] and
    /// [`AppSettings::SubcommandRequiredElseHelp`], but the user did not provide
    /// one.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, AppSettings, ErrorKind, };
    /// let result = App::new("prog")
    ///     .setting(AppSettings::ArgRequiredElseHelp)
    ///     .subcommand(App::new("config")
    ///         .about("Used for configuration")
    ///         .arg(Arg::new("config_file")
    ///             .about("The configuration file to use")))
    ///     .try_get_matches_from(vec!["prog"]);
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().kind, ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand);
    /// ```
    ///
    /// [`Subcommand`]: crate::Subcommand
    /// [`AppSettings::ArgRequiredElseHelp`]: crate::AppSettings::ArgRequiredElseHelp
    /// [`AppSettings::SubcommandRequiredElseHelp`]: crate::AppSettings::SubcommandRequiredElseHelp
    DisplayHelpOnMissingArgumentOrSubcommand,

    /// Not a true "error" as it means `--version` or similar was used.
    /// The message will be sent to `stdout`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind};
    /// let result = App::new("prog")
    ///     .version("3.0")
    ///     .try_get_matches_from(vec!["prog", "--version"]);
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().kind, ErrorKind::DisplayVersion);
    /// ```
    DisplayVersion,

    /// Occurs when using the [`ArgMatches::value_of_t`] and friends to convert an argument value
    /// into type `T`, but the argument you requested wasn't used. I.e. you asked for an argument
    /// with name `config` to be converted, but `config` wasn't used by the user.
    ///
    /// [`ArgMatches::value_of_t`]: crate::ArgMatches::value_of_t()
    ArgumentNotFound,

    /// Represents an [I/O error].
    /// Can occur when writing to `stderr` or `stdout` or reading a configuration file.
    ///
    /// [I/O error]: std::io::Error
    Io,

    /// Represents a [Format error] (which is a part of [`Display`]).
    /// Typically caused by writing to `stderr` or `stdout`.
    ///
    /// [`Display`]: std::fmt::Display
    /// [Format error]: std::fmt::Error
    Format,
}

/// Command Line Argument Parser Error
///
/// See [`App::error`] to create an error.
///
/// [`App::error`]: crate::App::error
#[derive(Debug)]
pub struct Error {
    /// Formatted error message, enhancing the cause message with extra information
    pub(crate) message: Colorizer,
    /// The type of error
    pub kind: ErrorKind,
    /// Additional information depending on the error kind, like values and argument names.
    /// Useful when you want to render an error of your own.
    pub info: Vec<String>,
    pub(crate) source: Option<Box<dyn error::Error + Send + Sync>>,
    backtrace: Backtrace,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        #[cfg(feature = "debug")]
        {
            writeln!(f, "{}", self.message)?;
            writeln!(f)?;
            writeln!(f, "Backtrace:")?;
            writeln!(f, "{}", self.backtrace)?;
        }
        #[cfg(not(feature = "debug"))]
        {
            Display::fmt(&self.message, f)?;
        }
        Ok(())
    }
}

fn start_error(c: &mut Colorizer, msg: impl Into<String>) {
    c.error("error:");
    c.none(" ");
    c.none(msg);
}

fn put_usage(c: &mut Colorizer, usage: impl Into<String>) {
    c.none("\n\n");
    c.none(usage);
}

fn try_help(app: &App, c: &mut Colorizer) {
    if !app.settings.is_set(AppSettings::DisableHelpFlag) {
        c.none("\n\nFor more information try ");
        c.good("--help");
        c.none("\n");
    } else if app.has_subcommands() && !app.settings.is_set(AppSettings::DisableHelpSubcommand) {
        c.none("\n\nFor more information try ");
        c.good("help");
        c.none("\n");
    }
}

impl Error {
    /// Returns the singular or plural form on the verb to be based on the argument's value.
    fn singular_or_plural(n: usize) -> String {
        if n > 1 {
            String::from("were")
        } else {
            String::from("was")
        }
    }

    /// Should the message be written to `stdout` or not
    #[inline]
    pub fn use_stderr(&self) -> bool {
        !matches!(
            self.kind,
            ErrorKind::DisplayHelp | ErrorKind::DisplayVersion
        )
    }

    /// Prints the error and exits. Depending on the error kind, this
    /// either prints to `stderr` and exits with a status of `1`
    /// or prints to `stdout` and exits with a status of `0`.
    pub fn exit(&self) -> ! {
        if self.use_stderr() {
            // Swallow broken pipe errors
            let _ = self.message.print();
            safe_exit(USAGE_CODE);
        }

        // Swallow broken pipe errors
        let _ = self.message.print();
        safe_exit(SUCCESS_CODE)
    }

    /// Prints formatted and colored error to `stdout` or `stderr` according to its error kind
    ///
    /// # Example
    /// ```no_run
    /// use clap::App;
    ///
    /// match App::new("App").try_get_matches() {
    ///     Ok(matches) => {
    ///         // do_something
    ///     },
    ///     Err(err) => {
    ///         err.print().expect("Error writing Error");
    ///         // do_something
    ///     },
    /// };
    /// ```
    pub fn print(&self) -> io::Result<()> {
        self.message.print()
    }

    pub(crate) fn new(message: Colorizer, kind: ErrorKind) -> Self {
        Self {
            message,
            kind,
            info: vec![],
            source: None,
            backtrace: Backtrace::new(),
        }
    }

    pub(crate) fn user_error(
        app: &App,
        usage: String,
        kind: ErrorKind,
        message: impl std::fmt::Display,
    ) -> Self {
        let mut c = Colorizer::new(true, app.get_color());

        start_error(&mut c, message.to_string());
        put_usage(&mut c, usage);
        try_help(app, &mut c);

        Self {
            message: c,
            kind,
            info: vec![],
            source: None,
            backtrace: Backtrace::new(),
        }
    }

    pub(crate) fn argument_conflict(
        app: &App,
        arg: &Arg,
        other: Option<String>,
        usage: String,
    ) -> Self {
        let mut c = Colorizer::new(true, app.get_color());
        let arg = arg.to_string();

        start_error(&mut c, "The argument '");
        c.warning(arg.clone());
        c.none("' cannot be used with ");

        match other {
            Some(ref name) => {
                c.none("'");
                c.warning(name);
                c.none("'");
            }
            None => {
                c.none("one or more of the other specified arguments");
            }
        };

        put_usage(&mut c, usage);
        try_help(app, &mut c);

        let mut info = vec![arg];
        if let Some(other) = other {
            info.push(other);
        }

        Error {
            message: c,
            kind: ErrorKind::ArgumentConflict,
            info,
            source: None,
            backtrace: Backtrace::new(),
        }
    }

    pub(crate) fn empty_value(app: &App, arg: &Arg, usage: String) -> Self {
        let mut c = Colorizer::new(true, app.get_color());
        let arg = arg.to_string();

        start_error(&mut c, "The argument '");
        c.warning(arg.clone());
        c.none("' requires a value but none was supplied");
        put_usage(&mut c, usage);
        try_help(app, &mut c);

        Error {
            message: c,
            kind: ErrorKind::EmptyValue,
            info: vec![arg],
            source: None,
            backtrace: Backtrace::new(),
        }
    }

    pub(crate) fn no_equals(app: &App, arg: String, usage: String) -> Self {
        let mut c = Colorizer::new(true, app.get_color());

        start_error(&mut c, "Equal sign is needed when assigning values to '");
        c.warning(&arg);
        c.none("'.");

        put_usage(&mut c, usage);
        try_help(app, &mut c);

        Error {
            message: c,
            kind: ErrorKind::NoEquals,
            info: vec![arg],
            source: None,
            backtrace: Backtrace::new(),
        }
    }

    pub(crate) fn invalid_value<G>(
        app: &App,
        bad_val: String,
        good_vals: &[G],
        arg: &Arg,
        usage: String,
    ) -> Self
    where
        G: AsRef<str> + Display,
    {
        let mut c = Colorizer::new(true, app.get_color());
        let suffix = suggestions::did_you_mean(&bad_val, good_vals.iter()).pop();

        let mut sorted: Vec<String> = good_vals
            .iter()
            .map(|v| v.to_string())
            .map(|v| {
                if v.contains(char::is_whitespace) {
                    format!("{:?}", v)
                } else {
                    v
                }
            })
            .collect();
        sorted.sort();

        start_error(&mut c, "");
        c.warning(format!("{:?}", bad_val));
        c.none(" isn't a valid value for '");
        c.warning(arg.to_string());
        c.none("'\n\t[possible values: ");

        if let Some((last, elements)) = sorted.split_last() {
            for v in elements {
                c.good(v);
                c.none(", ");
            }

            c.good(last);
        }

        c.none("]");

        if let Some(val) = suffix {
            c.none("\n\n\tDid you mean ");
            c.good(format!("{:?}", val));
            c.none("?");
        }

        put_usage(&mut c, usage);
        try_help(app, &mut c);

        let mut info = vec![arg.to_string(), bad_val];
        info.extend(sorted);

        Error {
            message: c,
            kind: ErrorKind::InvalidValue,
            info,
            source: None,
            backtrace: Backtrace::new(),
        }
    }

    pub(crate) fn invalid_subcommand(
        app: &App,
        subcmd: String,
        did_you_mean: String,
        name: String,
        usage: String,
    ) -> Self {
        let mut c = Colorizer::new(true, app.get_color());

        start_error(&mut c, "The subcommand '");
        c.warning(subcmd.clone());
        c.none("' wasn't recognized\n\n\tDid you mean ");
        c.good(did_you_mean);
        c.none("");
        c.none(format!(
            "?\n\nIf you believe you received this message in error, try re-running with '{} ",
            name
        ));
        c.good("--");
        c.none(format!(" {}'", subcmd));
        put_usage(&mut c, usage);
        try_help(app, &mut c);

        Error {
            message: c,
            kind: ErrorKind::InvalidSubcommand,
            info: vec![subcmd],
            source: None,
            backtrace: Backtrace::new(),
        }
    }

    pub(crate) fn unrecognized_subcommand(app: &App, subcmd: String, name: String) -> Self {
        let mut c = Colorizer::new(true, app.get_color());

        start_error(&mut c, " The subcommand '");
        c.warning(subcmd.clone());
        c.none("' wasn't recognized\n\n");
        c.warning("USAGE:");
        c.none(format!("\n    {} <subcommands>", name));
        try_help(app, &mut c);

        Error {
            message: c,
            kind: ErrorKind::UnrecognizedSubcommand,
            info: vec![subcmd],
            source: None,
            backtrace: Backtrace::new(),
        }
    }

    pub(crate) fn missing_required_argument(
        app: &App,
        required: Vec<String>,
        usage: String,
    ) -> Self {
        let mut c = Colorizer::new(true, app.get_color());

        start_error(
            &mut c,
            "The following required arguments were not provided:",
        );

        let mut info = vec![];
        for v in required {
            c.none("\n    ");
            c.good(v.to_string());
            info.push(v.to_string());
        }

        put_usage(&mut c, usage);
        try_help(app, &mut c);

        Error {
            message: c,
            kind: ErrorKind::MissingRequiredArgument,
            info,
            source: None,
            backtrace: Backtrace::new(),
        }
    }

    pub(crate) fn missing_subcommand(app: &App, name: String, usage: String) -> Self {
        let mut c = Colorizer::new(true, app.get_color());

        start_error(&mut c, "'");
        c.warning(name);
        c.none("' requires a subcommand, but one was not provided");
        put_usage(&mut c, usage);
        try_help(app, &mut c);

        Error {
            message: c,
            kind: ErrorKind::MissingSubcommand,
            info: vec![],
            source: None,
            backtrace: Backtrace::new(),
        }
    }

    pub(crate) fn invalid_utf8(app: &App, usage: String) -> Self {
        let mut c = Colorizer::new(true, app.get_color());

        start_error(
            &mut c,
            "Invalid UTF-8 was detected in one or more arguments",
        );
        put_usage(&mut c, usage);
        try_help(app, &mut c);

        Error {
            message: c,
            kind: ErrorKind::InvalidUtf8,
            info: vec![],
            source: None,
            backtrace: Backtrace::new(),
        }
    }

    pub(crate) fn too_many_occurrences(
        app: &App,
        arg: &Arg,
        max_occurs: usize,
        curr_occurs: usize,
        usage: String,
    ) -> Self {
        let mut c = Colorizer::new(true, app.get_color());
        let verb = Error::singular_or_plural(curr_occurs);

        start_error(&mut c, "The argument '");
        c.warning(arg.to_string());
        c.none("' allows at most ");
        c.warning(max_occurs.to_string());
        c.none(" occurrences, but ");
        c.warning(curr_occurs.to_string());
        c.none(format!(" {} provided", verb));
        put_usage(&mut c, usage);
        try_help(app, &mut c);

        Error {
            message: c,
            kind: ErrorKind::TooManyOccurrences,
            info: vec![
                arg.to_string(),
                curr_occurs.to_string(),
                max_occurs.to_string(),
            ],
            source: None,
            backtrace: Backtrace::new(),
        }
    }

    pub(crate) fn too_many_values(app: &App, val: String, arg: String, usage: String) -> Self {
        let mut c = Colorizer::new(true, app.get_color());

        start_error(&mut c, "The value '");
        c.warning(val.clone());
        c.none("' was provided to '");
        c.warning(&arg);
        c.none("' but it wasn't expecting any more values");
        put_usage(&mut c, usage);
        try_help(app, &mut c);

        Error {
            message: c,
            kind: ErrorKind::TooManyValues,
            info: vec![arg, val],
            source: None,
            backtrace: Backtrace::new(),
        }
    }

    pub(crate) fn too_few_values(
        app: &App,
        arg: &Arg,
        min_vals: usize,
        curr_vals: usize,
        usage: String,
    ) -> Self {
        let mut c = Colorizer::new(true, app.get_color());
        let verb = Error::singular_or_plural(curr_vals);

        start_error(&mut c, "The argument '");
        c.warning(arg.to_string());
        c.none("' requires at least ");
        c.warning(min_vals.to_string());
        c.none(" values, but only ");
        c.warning(curr_vals.to_string());
        c.none(format!(" {} provided", verb));
        put_usage(&mut c, usage);
        try_help(app, &mut c);

        Error {
            message: c,
            kind: ErrorKind::TooFewValues,
            info: vec![arg.to_string(), curr_vals.to_string(), min_vals.to_string()],
            source: None,
            backtrace: Backtrace::new(),
        }
    }

    pub(crate) fn value_validation(
        app: &App,
        arg: String,
        val: String,
        err: Box<dyn error::Error + Send + Sync>,
    ) -> Self {
        let Self {
            mut message,
            kind,
            info,
            source,
            backtrace,
        } = Self::value_validation_with_color(arg, val, err, app.get_color());
        try_help(app, &mut message);
        Self {
            message,
            kind,
            info,
            source,
            backtrace,
        }
    }

    pub(crate) fn value_validation_without_app(
        arg: String,
        val: String,
        err: Box<dyn error::Error + Send + Sync>,
    ) -> Self {
        Self::value_validation_with_color(arg, val, err, ColorChoice::Auto)
    }

    fn value_validation_with_color(
        arg: String,
        val: String,
        err: Box<dyn error::Error + Send + Sync>,
        color: ColorChoice,
    ) -> Self {
        let mut c = Colorizer::new(true, color);

        start_error(&mut c, "Invalid value");

        c.none(" for '");
        c.warning(arg.clone());
        c.none("'");

        c.none(format!(": {}", err));

        Error {
            message: c,
            kind: ErrorKind::ValueValidation,
            info: vec![arg, val, err.to_string()],
            source: Some(err),
            backtrace: Backtrace::new(),
        }
    }

    pub(crate) fn wrong_number_of_values(
        app: &App,
        arg: &Arg,
        num_vals: usize,
        curr_vals: usize,
        usage: String,
    ) -> Self {
        let mut c = Colorizer::new(true, app.get_color());
        let verb = Error::singular_or_plural(curr_vals);

        start_error(&mut c, "The argument '");
        c.warning(arg.to_string());
        c.none("' requires ");
        c.warning(num_vals.to_string());
        c.none(" values, but ");
        c.warning(curr_vals.to_string());
        c.none(format!(" {} provided", verb));
        put_usage(&mut c, usage);
        try_help(app, &mut c);

        Error {
            message: c,
            kind: ErrorKind::WrongNumberOfValues,
            info: vec![arg.to_string(), curr_vals.to_string(), num_vals.to_string()],
            source: None,
            backtrace: Backtrace::new(),
        }
    }

    pub(crate) fn unexpected_multiple_usage(app: &App, arg: &Arg, usage: String) -> Self {
        let mut c = Colorizer::new(true, app.get_color());
        let arg = arg.to_string();

        start_error(&mut c, "The argument '");
        c.warning(arg.clone());
        c.none("' was provided more than once, but cannot be used multiple times");
        put_usage(&mut c, usage);
        try_help(app, &mut c);

        Error {
            message: c,
            kind: ErrorKind::UnexpectedMultipleUsage,
            info: vec![arg],
            source: None,
            backtrace: Backtrace::new(),
        }
    }

    pub(crate) fn unknown_argument(
        app: &App,
        arg: String,
        did_you_mean: Option<(String, Option<String>)>,
        usage: String,
    ) -> Self {
        let mut c = Colorizer::new(true, app.get_color());

        start_error(&mut c, "Found argument '");
        c.warning(arg.clone());
        c.none("' which wasn't expected, or isn't valid in this context");

        if let Some((flag, subcmd)) = did_you_mean {
            let flag = format!("--{}", flag);
            c.none("\n\n\tDid you mean ");

            if let Some(subcmd) = subcmd {
                c.none("to put '");
                c.good(flag);
                c.none("' after the subcommand '");
                c.good(subcmd);
                c.none("'?");
            } else {
                c.none("'");
                c.good(flag);
                c.none("'?");
            }
        }

        // If the user wants to supply things like `--a-flag` or `-b` as a value,
        // suggest `--` for disambiguation.
        if arg.starts_with('-') {
            c.none(format!(
                "\n\n\tIf you tried to supply `{}` as a value rather than a flag, use `-- {}`",
                arg, arg
            ));
        }

        put_usage(&mut c, usage);
        try_help(app, &mut c);

        Error {
            message: c,
            kind: ErrorKind::UnknownArgument,
            info: vec![arg],
            source: None,
            backtrace: Backtrace::new(),
        }
    }

    pub(crate) fn unnecessary_double_dash(app: &App, arg: String, usage: String) -> Self {
        let mut c = Colorizer::new(true, app.get_color());

        start_error(&mut c, "Found argument '");
        c.warning(arg.clone());
        c.none("' which wasn't expected, or isn't valid in this context");

        c.none(format!(
            "\n\n\tIf you tried to supply `{}` as a subcommand, remove the '--' before it.",
            arg
        ));
        put_usage(&mut c, usage);
        try_help(app, &mut c);

        Error {
            message: c,
            kind: ErrorKind::UnknownArgument,
            info: vec![arg],
            source: None,
            backtrace: Backtrace::new(),
        }
    }

    pub(crate) fn argument_not_found_auto(arg: String) -> Self {
        let mut c = Colorizer::new(true, ColorChoice::Auto);

        start_error(&mut c, "The argument '");
        c.warning(arg.clone());
        c.none("' wasn't found");

        Error {
            message: c,
            kind: ErrorKind::ArgumentNotFound,
            info: vec![arg],
            source: None,
            backtrace: Backtrace::new(),
        }
    }

    /// Deprecated, see [`App::error`]
    ///
    /// [`App::error`]: crate::App::error
    #[deprecated(since = "3.0.0", note = "Replaced with `App::error`")]
    pub fn with_description(description: String, kind: ErrorKind) -> Self {
        let mut c = Colorizer::new(true, ColorChoice::Auto);

        start_error(&mut c, description);
        Error::new(c, kind)
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        let mut c = Colorizer::new(true, ColorChoice::Auto);
        start_error(&mut c, e.to_string());
        Error::new(c, ErrorKind::Io)
    }
}

impl From<fmt::Error> for Error {
    fn from(e: fmt::Error) -> Self {
        let mut c = Colorizer::new(true, ColorChoice::Auto);
        start_error(&mut c, e.to_string());
        Error::new(c, ErrorKind::Format)
    }
}

impl error::Error for Error {
    #[allow(trivial_casts)]
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as _)
    }
}

#[cfg(feature = "debug")]
#[derive(Debug)]
struct Backtrace(backtrace::Backtrace);

#[cfg(feature = "debug")]
impl Backtrace {
    fn new() -> Self {
        Self(backtrace::Backtrace::new())
    }
}

#[cfg(feature = "debug")]
impl Display for Backtrace {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // `backtrace::Backtrace` uses `Debug` instead of `Display`
        write!(f, "{:?}", self.0)
    }
}

#[cfg(not(feature = "debug"))]
#[derive(Debug)]
struct Backtrace;

#[cfg(not(feature = "debug"))]
impl Backtrace {
    fn new() -> Self {
        Self
    }
}

#[cfg(not(feature = "debug"))]
impl Display for Backtrace {
    fn fmt(&self, _: &mut Formatter) -> fmt::Result {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    /// Check `clap::Error` impls Send and Sync.
    mod clap_error_impl_send_sync {
        use crate::Error;
        trait Foo: std::error::Error + Send + Sync + 'static {}
        impl Foo for Error {}
    }
}
