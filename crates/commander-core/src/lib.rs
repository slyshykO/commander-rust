//! This crate is using for `commander_rust`.
//!
//! Only `Application`, `Cli`, `Raw` you will use.
//!
//! `Application` will be returned by macro `run!()`.
//! It's readonly. You can get application information through it.
//! `Application` contains all information you defined using `#[option]` ,`#[command]` and `#[entry]`.
//! See `Application` for more details.
//!
//! `Cli` is an interface of CLI. You can get all argument of options through it.
//! `Cli` offered two convenient method to get argument of options.
//! They are `get(idx: &str) -> Raw` and `get_or<T: From<Raw>>(&self, idx: &str, d: T) -> T`.
//! See `Cli` for more details.
//!
//!
//! `Raw` is encapsulation of something. It a sequence of String.
//! You can regard it as `Raw(<Vec<String>>)`. In fact, it is.
//! `Raw` is using for types convert.
//! Any type implemented `From<Raw>` can be types of command processing functions' parameter.
//! For example, `Vec<i32>` implemented `From<Raw>`. So you can use it like `fn method(v: Vec<i32>)`.
//! But `slice` is not implemented `From<Raw>`, so you can not use it like `fn method(s: [i32])`.
//! Once type implemented `From<Raw>`, you can covert it's type using `let right: bool = raw.into()`.
//!


mod raw;
mod fmt;
mod pattern;

use std::ops::Index;
use std::collections::HashMap;

use pattern::{ Pattern, PatternType };
pub use raw::Raw;
use std::process::exit;

/// The type of argument.
///
/// they are:
/// - <rs>    -- RequiredSingle
/// - [os]    -- OptionalSingle
/// - <rm...> -- RequiredMultiple
/// - [om...] -- OptionalMultiple
///
///
/// For most of the times, you will not use it.
///
///
#[doc(hidden)]
#[derive(PartialEq, Eq)]
#[derive(Debug, Clone)]
pub enum ArgumentType {
    RequiredSingle,
    OptionalSingle,
    RequiredMultiple,
    OptionalMultiple,
}

/// Represents a parameter.
///
/// For example, `<dir>` will represents as
/// ```ignore
/// Argument {
///     name: "dir",
///     ty: ArgumentType::RequiredSingle,
/// }
/// ```
///
/// For most of the time, you will not use it.
#[doc(hidden)]
#[derive(PartialEq, Eq)]
#[derive(Clone)]
pub struct Argument {
    pub name: String,
    pub ty: ArgumentType,
}

/// Represents an application.
///
/// Application is what? Application is generated from your code.
/// If you use `#[command]`, application will get a `Command`.
/// If you use `#[options]`, application will get a `Options`.
/// If you write descriptions in your `Cargo.toml`, application will get a `desc`.
/// If you write version in your `Cargo.toml`, application will get a `ver`.
///
/// For most of the time, you will use all of them.
///
/// And we offer a way to get the only application of your CLI.
/// Using `commander_rust::run!()`(instead of `commander_rust::run()`, it's a proc_macro) to get it.
///
/// # Note
/// It's generated by `commander_rust`, and it should be readonly.
///
pub struct Application {
    pub name: String,
    pub desc: String,
    pub cmds: Vec<Command>,
    pub opts: Vec<Options>,
    pub direct_args: Vec<Argument>,
}

/// Represents a instance defined by `#[command]`.
///
/// For example, `#[command(rmir <dir> [others...], "remove files")]` will generate a instance like:
/// ```ignore
/// Command {
///     name: "rmdir",
///     args: [
///         Argument {
///             name: "dir",
///             ty: ArgumentType::RequiredSingle,
///         },
///         Argument {
///             name: "others",
///             ty: ArgumentType::OptionalMultiple,
///         }
///     ],
///     desc: Some("remove files"),
///     opts: ...
/// }
/// ```
///
/// `opts` is determined by `#[option]` before `#[command]`.
///
/// # Note
/// `#[command]` should be and only should be defined after all `#[option]`.
/// It means:
/// ```ignore
/// // correct
/// #[option(...)]
/// #[command(test ...)]
/// fn test(...) {}
///
/// // fault
/// #[command(test ...)]
/// #[option(...)]
/// fn test(...) {}
/// ```
/// And the name in `#[command]` have to be same as the name of corresponding functions.
/// In this example, they are `test`.
/// For most of the time, you will not use it.
///
#[doc(hidden)]
pub struct Command {
    pub name: String,
    pub args: Vec<Argument>,
    pub desc: Option<String>,
    pub opts: Vec<Options>,
}

/// Represents a instance defined by `#[option]`.
///
/// # Note
/// `#[option]` only accepts up to one argument. And one `#[command]` can accept many `#[option]`.
/// It's similar with `Command`. See `Command` for more detail.
/// For most of the time, you will not use it.
#[derive(Debug)]
#[doc(hidden)]
pub struct Options {
    pub short: String,
    pub long: String,
    pub arg: Option<Argument>,
    pub desc: Option<String>,
}

/// A divided set of user's inputs.
///
///
/// For example, when you input `[pkg-name] rmdir /test/ -rf`, `commander_rust` will generate something like
/// ```ignore
/// [
///     Instance {
///         name: "rmdir",
///         args: ["/test/"],
///     },
///     Instance {
///         name: "r",
///         args: vec![],
///     },
///     Instance {
///         name: "f",
///         args: vec![],
///     }
/// ]
/// ```
/// For most of the time, you will not use it.
#[derive(Debug, Eq, PartialEq)]
#[doc(hidden)]
pub struct Instance {
    pub name: String,
    pub args: Vec<String>,
}

/// `Cmd` is not `Command`. It's defined by the user's input.
///
/// `name` is the first elements of inputs if the first element is one of the name of any `Command`.
/// `raws` is followed element after the first element, until something like `-rf`,`--simple=yes` appears.
/// `raws` is `Vec<Raw>`, because we one command maybe has more than one arguments.
/// See `Raw` for more details.
/// `opt_raws` is a `HashMap`. It stores elements user input that `Options` might use.
/// It's hard to understand what `Cmd' is for. Many people get confused.
/// But fortunately, for most of the time(even forever), you will not use it.
#[derive(Debug)]
#[doc(hidden)]
pub struct Cmd {
    pub name: String,
    pub raws: Vec<Raw>,
    pub opt_raws: HashMap<String, Raw>,
}


/// Something like `Cmd`.
///
/// But unfortunately, you will use it frequently.
///
/// `commander_rust` will generate a instance of `Application` according your code. It happens in compile-time.
/// `commander_rust` will generate a instance of `Cli` according user's input. It happens in run-time.

/// What' the difference?
/// Content in `Application` will be replaced by something concrete through user's input.
/// For example, If your code is like this:
/// ```ignore
/// #[option(-r, --recursive [dir...], "recursively")]
/// #[command(rmdir <dir> [otherDirs...], "remove files and directories")]
/// fn rmdir(dir: i32, other_dirs: Option<Vec<bool>>, cli: Cli) {
///     let r: bool = cli.get("recursive").into();
/// }
/// ```
/// Let's see. The last argument of function is `Cli` type(you can miss it).
/// So when we want to do something if `--recursive` is offered by user, how can we?
/// You just need to code like `let r: ? = cli.get("recursive").into()`,
/// then you can get contents of `recursive` options if user has inputted it.
///
/// That's why `Cli` will be used frequently.
///
#[derive(Debug)]
pub struct Cli {
    pub cmd: Option<Cmd>,
    pub global_raws: HashMap<String, Raw>,
    pub direct_args: Vec<Raw>,
}

impl Application {
    /// Deriving `#[option(-h, --help, "output usage information")]`
    /// and `#[option(-V, --version, "output the version number")]` for all `Command` and `Application`.
    /// Dont use it!
    #[doc(hidden)]
    pub fn derive(&mut self) {
        self.opts.push(Options {
            short: String::from("h"),
            long: String::from("help"),
            arg: None,
            desc: Some(String::from("output usage information")),
        });
        self.opts.push(Options {
            short: String::from("V"),
            long: String::from("version"),
            arg: None,
            desc: Some(String::from("output the version number")),
        });
        self.derive_cmds();
    }

    /// Deriving `#[option(-h, --help, "output usage information")]`
    /// and `#[option(-V, --version, "output the version number")]` for all `Command`.
    /// Dont use it!
    #[doc(hidden)]
    fn derive_cmds(&mut self) {
        for cmd in &mut self.cmds {
            cmd.derive();
        }
    }

    pub fn contains_key(&self, idx: &str) -> bool {
        for opt in self.opts.iter() {
            if opt.long == idx || opt.short == idx {
                return true;
            }
        }

        for cmd in self.cmds.iter() {
            for opt in cmd.opts.iter() {
                if opt.long == idx || opt.short == idx {
                    return true;
                }
            }
        }

        false
    }
}

impl Command {
    /// Deriving `#[option(-h, --help, "output usage information")]`
    /// and `#[option(-V, --version, "output the version number")]` for `Command`.
    /// Dont use it!
    #[doc(hidden)]
    pub fn derive(&mut self) {
        self.opts.push(Options {
            short: String::from("h"),
            long: String::from("help"),
            arg: None,
            desc: Some(String::from("output usage information")),
        });
    }
}

impl Cli {
    /// Create a empty `Cli`.
    #[doc(hidden)]
    pub fn empty() -> Cli {
        Cli {
            cmd: None,
            global_raws: HashMap::new(),
            direct_args: vec![],
        }
    }

    /// Get the content of `Options`.
    /// `Options` has two types, one is private, the other is global. Of course they are same.
    /// Private means they belong to the command.
    /// Global means they belong to the global.
    ///
    /// Private is more weight than global.
    pub fn get(&self, idx: &str) -> Raw {
        if self.cmd.is_some() && self.cmd.as_ref().unwrap().has(idx) {
            self.cmd.as_ref().unwrap()[idx].clone()
        } else if self.global_raws.contains_key(idx) {
            self.global_raws[idx].clone()
        } else {
            Raw::new(vec![])
        }
    }

    /// Getting contents of `Options`. if `idx` dont exist, return `default`.
    pub fn get_or<T: From<Raw>>(&self, idx: &str, d: T) -> T {
        if self.has(idx) {
            self.get(idx).into()
        } else {
            d
        }
    }

    /// Get contents of `Options`. if `idx` dont exist, call f.
    ///
    /// f should return a value of type T.
    pub fn get_or_else<T: From<Raw>, F>(&self, idx: &str, f: F) -> T
    where F: FnOnce() -> T {
        if self.has(idx) {
            self.get(idx).into()
        } else {
            f()
        }
    }

    /// Check user input a option or not.
    pub fn has(&self, idx: &str) -> bool {
        (self.cmd.is_some() && self.cmd.as_ref().unwrap().has(idx)) || self.global_raws.contains_key(idx)
    }

    /// Inner function, dont use it.
    #[doc(hidden)]
    pub fn from(instances: &Vec<Instance>, app: &Application) -> Option<Cli> {
        if instances.is_empty() {
            None
        } else {
            let cmd = Cmd::from(instances, &app.cmds);
            let mut global_raws = HashMap::new();

            // if sub-command is offered, add options that doesn't exist in this sub-command into global_raws
            // if it doesn't, all options should belong to the global_raws
            // both case, options should be checked that if they are defined or not
            // for using easily, short & long are pushed into global_raws, so does `Cmd`
            for ins in instances.iter() {
                let matched = app.opts.iter().find(|o| o.long == ins.name || o.short == ins.name);

                if let Some(matched) = matched {
                    if let Some(cmd) = &cmd {
                        if !cmd.has(&matched.long) {
                            let raw = Raw::divide_opt(ins, &matched.arg);

                            global_raws.insert(matched.long.clone(), raw.clone());
                            global_raws.insert(matched.short.clone(), raw);
                        }
                    } else {
                        let raw = Raw::divide_opt(ins, &matched.arg);

                        global_raws.insert(matched.long.clone(), raw.clone());
                        global_raws.insert(matched.short.clone(), raw);
                    }
                }
            }

            Some(Cli {
                cmd,
                global_raws,
                direct_args: {
                    if instances[0].is_empty() && !instances[0].args.is_empty() {
                        Raw::divide_cmd(&instances[0], &app.direct_args)
                    } else {
                        vec![]
                    }
                }
            })
        }
    }

    #[doc(hidden)]
    pub fn get_raws(&self) -> Vec<Raw> {
        if let Some(cmd) = &self.cmd {
            cmd.raws.clone()
        } else {
            vec![]
        }
    }

    /// Get the name of the only command inputted by user.
    ///
    /// For Exanple, If user input `[pkg-name] rmdir -rf ./*`,
    /// then the name is `rmdir`.
    ///
    #[doc(hidden)]
    pub fn get_name(&self) -> String {
        if let Some(cmd) = &self.cmd {
            cmd.name.clone()
        } else {
            String::new()
        }
    }
}

impl Cmd {
    /// Create a `Cmd` using offered name.
    #[doc(hidden)]
    fn new(name: String) -> Cmd {
        Cmd {
            name,
            raws: vec![],
            opt_raws: HashMap::new(),
        }
    }

    #[doc(hidden)]
    fn push(&mut self, arg: Raw) {
        self.raws.push(arg);
    }

    #[doc(hidden)]
    fn insert(&mut self, key: String, arg: Raw) {
        if !self.opt_raws.contains_key(&key) {
            self.opt_raws.insert(key, arg);
        }
    }

    #[doc(hidden)]
    fn append(&mut self, raws: Vec<Raw>) {
        raws.into_iter().for_each(|r| self.push(r));
    }

    fn get_cmd_idx(instances: &Vec<Instance>, commands: &Vec<Command>) -> Option<usize> {
        for (idx, ins) in instances.iter().enumerate() {
            if commands.iter().any(|c| c.name == ins.name) {
                return Some(idx);
            }
        }

        None
    }

    /// Check user input a option or not. Used by `Cli::has`.
    ///
    /// Dont use it.
    #[doc(hidden)]
    pub fn has(&self, idx: &str) -> bool {
        self.opt_raws.contains_key(idx)
    }

    /// Inner function, don't use it.
    #[doc(hidden)]
    pub fn from(instances: &Vec<Instance>, commands: &Vec<Command>) -> Option<Cmd> {
        let mut result = Cmd::new(String::new());

        if instances.is_empty() {
            None
        } else {
            let idx = Cmd::get_cmd_idx(instances, commands);
            let head;
            let n;

            if let Some(idx) = idx {
                head = instances.get(idx).unwrap();
                n = idx + 1;
            } else {
                return None;
            }

            let cmd = commands.iter().find(|c| c.name == head.name);

            // user calls sub-command or not
            if let Some(sub_cmd) = cmd {
                let raws = Raw::divide_cmd(head, &sub_cmd.args);

                result.name = sub_cmd.name.clone();
                // get raws of arguments
                result.append(raws);

                // get all raws of all options
                for ins in instances.iter().skip(n) {
                    let matched = sub_cmd.opts.iter().find(|o| (o.long == ins.name || o.short == ins.name));

                    if let Some(matched) = matched {
                        let raw = Raw::divide_opt(ins, &matched.arg);

                        result.insert(matched.long.clone(), raw.clone());
                        result.insert(matched.short.clone(), raw);
                    }
                }

                Some(result)
            } else {
                None
            }
        }
    }
}

impl Index<&str> for Cmd {
    type Output = Raw;

    fn index(&self, idx: &str) -> &Raw {
        &self.opt_raws[idx]
    }
}

impl Instance {
    /// Check instance is empty or not.
    #[doc(hidden)]
    pub fn is_empty(&self) -> bool {
        self.name.is_empty()
    }

    /// Create an empty `Instance`.
    #[doc(hidden)]
    pub fn empty() -> Instance {
        Instance {
            name: String::new(),
            args: vec![],
        }
    }

    /// Create an `Instance` using offered name.
    #[doc(hidden)]
    pub fn new(name: &str) -> Instance {
        Instance {
            name: String::from(name),
            args: vec![],
        }
    }
}

pub fn normalize(args: Vec<String>, app: &Application) -> Vec<Instance> {
    let mut instances = vec![];
    let mut head = Instance::empty();
    let mut args = args.into_iter().skip(1);
    let mut flag = false;

    while let Some(arg) = args.next() {
        let reg = Pattern::match_str(&arg);

        match reg.ty {
            PatternType::Stmt => {
                if app.contains_key(reg.groups[0]) {
                    let mut all_opts: Vec<&str> = reg.groups[1].split_terminator(" ").collect();

                    if !head.is_empty() || (!head.args.is_empty() && instances.is_empty()) {
                        instances.push(head);
                    }

                    head = Instance::empty();
                    all_opts.dedup_by(|a, b| a == b);
                    all_opts.retain(|x| !x.is_empty());

                    instances.push(Instance {
                        name: String::from(reg.groups[0]),
                        args: all_opts.into_iter().map(|x| String::from(x)).collect(),
                    });
                } else {
                    eprintln!("Unknown option: --{}", reg.groups[0]);
                    exit(-1);
                }
            },
            PatternType::Short => {
                let mut all_opts: Vec<&str> = reg.groups[0].split("").collect();

                all_opts.dedup_by(|a, b| a == b);
                all_opts.retain(|x| !x.is_empty());

                if !head.is_empty() || (!head.args.is_empty() && instances.is_empty()) {
                    instances.push(head);
                }

                for x in all_opts.into_iter() {
                    if x.len() == 1 {
                        if app.contains_key(x) {
                            instances.push(Instance::new(x));
                        } else {
                            eprintln!("Unknown option: -{}", x);
                            exit(-1);
                        }
                    }
                }

                head = instances.pop().unwrap_or(Instance::empty());
            },
            PatternType::Long => {
                if app.contains_key(reg.groups[0]) {
                    if !head.is_empty() || (!head.args.is_empty() && instances.is_empty()) {
                        instances.push(head);
                    }

                    head = Instance::new(reg.groups[0]);
                } else {
                    eprintln!("Unknown option: --{}", reg.groups[0]);
                    exit(-1);
                }
            },
            PatternType::Word => {
                if app.cmds.iter().any(|c| c.name == arg) && !flag {
                    if !head.is_empty() || (!head.args.is_empty() && instances.is_empty()) {
                        instances.push(head);
                    }

                    head = Instance::new(&arg);
                    flag = true;
                } else {
                    head.args.push(arg);
                }
            },
            _ => {
                head.args.push(arg);
            },
        }
    }

    if !head.is_empty() || (!head.args.is_empty() && instances.is_empty()) {
        instances.push(head);
    }

    instances
}