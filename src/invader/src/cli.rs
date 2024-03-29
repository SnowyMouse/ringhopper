#![allow(dead_code)] // TODO: fix this

use std::cmp::Ordering;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use ringhopper::primitives::tag::ParseStrictness;
use ringhopper::tag::tree::VirtualTagsDirectory;
use util::get_tty_metadata;

pub struct CommandLineParser {
    description: &'static str,
    usage: &'static str,
    standard_parameters: HashMap<StandardParameterType, Parameter>,
    custom_parameters: HashMap<&'static str, Parameter>,
    extra_parameters: Vec<String>,
    required_extra_parameters: usize,
    on_help: fn(&CommandLineParser) -> Result<(), String>,
    strictness: ParseStrictness
}

#[derive(Clone)]
pub struct CommandLineArgs {
    standard_parameters: HashMap<StandardParameterType, Parameter>,
    custom_parameters: HashMap<&'static str, Parameter>,
    extra_parameters: Vec<String>,
    strictness: ParseStrictness
}

macro_rules! all_args {
    ($parser:expr) => {{
        let standard_parameters = $parser.standard_parameters.iter().map(|m| m.1);
        let custom_parameters = $parser.custom_parameters.iter().map(|m| m.1);
        standard_parameters.chain(custom_parameters)
    }};
}

macro_rules! all_args_mut {
    ($parser:expr) => {{
        let standard_parameters = $parser.standard_parameters.iter_mut().map(|m| m.1);
        let custom_parameters = $parser.custom_parameters.iter_mut().map(|m| m.1);
        standard_parameters.chain(custom_parameters)
    }};
}

impl CommandLineParser {
    pub fn new(description: &'static str, usage: &'static str) -> Self {
        CommandLineParser {
            description,
            usage,
            on_help: |_| Ok(()),
            required_extra_parameters: 0,
            standard_parameters: HashMap::new(),
            custom_parameters: HashMap::new(),
            extra_parameters: Vec::new(),
            strictness: ParseStrictness::Strict
        }
    }

    pub fn add_help(self) -> Self {
        self.add_help_with_callback(|parser| {
            let mut arguments: Vec<&Parameter> = all_args!(parser).collect();
            arguments.sort_by(|a, b| {
                if let Some(a_short) = a.short {
                    if let Some(b_short) = b.short {
                        let lowercase_a = a_short.to_ascii_lowercase();
                        let lowercase_b = b_short.to_ascii_lowercase();
                        return if lowercase_a == lowercase_b {
                            b_short.cmp(&a_short) // prefer lowercase
                        }
                        else {
                            lowercase_a.cmp(&lowercase_b)
                        }
                    }
                    else {
                        return Ordering::Less
                    }
                }
                else if let Some(_) = b.short {
                    return Ordering::Greater
                }
                else {
                    return a.name.cmp(b.name)
                }
            });

            println!("Usage: {}", parser.usage);
            println!();
            println!("{}", parser.description);
            println!();


            let arg_info_width = 40;
            let min_arg_desc_width = 30;
            let tty_width = get_tty_metadata().map(|t| t.width);
            let min_width_for_good_tty = arg_info_width + min_arg_desc_width;

            match tty_width {
                // By default, we want to print args and description on the same line.
                Some(available_width) if available_width >= min_width_for_good_tty => {
                    for a in arguments {
                        let usage = {
                            let shorthand = match a.short {
                                Some(c) => format!("-{c}, "),
                                None => String::new()
                            };

                            let name = a.name;
                            let usage = if a.usage.is_empty() { String::new() } else { format!(" {}", a.usage) };

                            format!("  {shorthand}--{name}{usage}")
                        };

                        let mut current_pos = usage.len();
                        debug_assert!(current_pos < arg_info_width);
                        print!("{usage}");

                        for word in a.description.split(" ") {
                            let word_len = word.len() + 1; // + 1 for the whitespace

                            // if this would overflow the current line, make a new one
                            if current_pos + word_len > available_width {
                                println!();
                                current_pos = 0;
                            }

                            // pad out the left side if we need to
                            if current_pos < arg_info_width {
                                for _ in current_pos..arg_info_width {
                                    print!(" ");
                                }
                                current_pos = arg_info_width;
                            }

                            // include a space at the start so we can spread words apart
                            // by putting it at the start, this allows for an exact fitting of the words per-line
                            // and also guarantees 1 character of padding between description and argument usage
                            print!(" {word}");
                            current_pos += word_len;
                        }

                        println!();
                    }
                },

                // The fallback is putting description below arguments.
                // This is if the terminal is too small, or we cannot determine terminal width.
                //
                // It is better than displaying a bunch of jumbled up text at least...
                _ => {
                    for a in arguments {
                        match a.short {
                            Some(c) => print!("-{c}, "),
                            None => ()
                        };
                        print!("--{} ", a.name);
                        if !a.usage.is_empty() {
                            print!("{} ", a.usage);
                        }
                        println!();
                        println!("  {}", a.description);
                    }
                },
            }

            println!();
            std::process::exit(0);
        })
    }

    /// Testing only!
    fn add_help_with_callback(mut self, callback: fn(&CommandLineParser) -> Result<(), String>) -> Self {
        let p = Parameter {
            values: None,
            name: "help",
            short: Some('h'),
            description: "Show help for this verb.",
            default_values: None,
            value_type: None,
            required: false,
            value_count: 0,
            multiple: false,
            usage: ""
        };

        self.on_help = callback;
        assert!(self.standard_parameters.get(&StandardParameterType::Help).is_none());
        self.standard_parameters.insert(StandardParameterType::Help, p);
        self
    }

    pub fn set_required_extra_parameters(mut self, amount: usize) -> Self {
        self.required_extra_parameters = amount;
        self
    }

    pub fn add_tags(mut self, multiple: bool) -> Self {
        let p = Parameter {
            values: None,
            name: "tags",
            short: Some('t'),
            description: match multiple {
                true => "Add a tags directory. This can be used multiple times, in which case it is in order of precedence. Default: tags",
                false => "Set a tags directory. Default: tags"
            },
            default_values: Some(vec![CommandLineValue::Path(Path::new("tags").to_owned())]),
            value_type: Some(CommandLineValueType::Path),
            required: false,
            value_count: 1,
            usage: "<dir>",
            multiple,
        };

        assert!(self.standard_parameters.get(&StandardParameterType::Tags).is_none());
        self.standard_parameters.insert(StandardParameterType::Tags, p);
        self
    }

    pub fn add_overwrite(mut self) -> Self {
        let p = Parameter {
            values: None,
            name: "overwrite",
            short: Some('o'),
            description: "Overwrite if the output file already exists.",
            default_values: None,
            value_type: None,
            required: false,
            value_count: 0,
            usage: "",
            multiple: false,
        };

        assert!(self.standard_parameters.get(&StandardParameterType::Overwrite).is_none());
        self.standard_parameters.insert(StandardParameterType::Overwrite, p);
        self
    }

    pub fn add_cow_tags(mut self) -> Self {
        let p = Parameter {
            values: None,
            name: "cow",
            short: Some('c'),
            description: "Set a copy-on-write directory for outputting tags.",
            default_values: Some(vec![]),
            value_type: Some(CommandLineValueType::Path),
            required: false,
            value_count: 1,
            usage: "<dir>",
            multiple: false,
        };

        assert!(self.standard_parameters.get(&StandardParameterType::CowTags).is_none());
        self.standard_parameters.insert(StandardParameterType::CowTags, p);
        self
    }

    pub fn add_jobs(mut self) -> Self {
        let p = Parameter {
            values: None,
            name: "jobs",
            short: Some('j'),
            description: "Set number of threads for this task.",
            default_values: None,
            value_type: Some(CommandLineValueType::UInteger),
            required: false,
            value_count: 1,
            usage: "<jobs>",
            multiple: false,
        };

        assert!(self.standard_parameters.get(&StandardParameterType::Jobs).is_none());
        self.standard_parameters.insert(StandardParameterType::Jobs, p);
        self
    }

    pub fn add_data(mut self) -> Self {
        let p = Parameter {
            values: None,
            name: "data",
            short: Some('d'),
            description: "Set a data directory. Default: data",
            default_values: Some(vec![CommandLineValue::Path(Path::new("data").to_owned())]),
            value_type: Some(CommandLineValueType::Path),
            required: false,
            value_count: 1,
            usage: "<dir>",
            multiple: false,
        };

        assert!(self.standard_parameters.get(&StandardParameterType::Data).is_none());
        self.standard_parameters.insert(StandardParameterType::Data, p);
        self
    }

    pub fn add_maps(mut self) -> Self {
        let p = Parameter {
            values: None,
            name: "maps",
            short: Some('m'),
            description: "Set a maps directory. Default: maps",
            default_values: Some(vec![CommandLineValue::Path(Path::new("maps").to_owned())]),
            value_type: Some(CommandLineValueType::Path),
            required: false,
            value_count: 1,
            usage: "<dir>",
            multiple: false,
        };

        assert!(self.standard_parameters.get(&StandardParameterType::Maps).is_none());
        self.standard_parameters.insert(StandardParameterType::Maps, p);
        self
    }

    pub fn add_custom_parameter(mut self, parameter: Parameter) -> Self {
        assert!(parameter.name != "help" && parameter.short != Some('h'));
        assert!(
            self.custom_parameters.iter().find(|(name, param)| *name == &parameter.name || param.short == parameter.short).is_none(),
            "{} conflicts with an existing parameter", parameter.name
        );
        self.custom_parameters.insert(parameter.name, parameter);
        self
    }

    /// Set strictness for get_virtual_tags_directory
    ///
    /// ONLY SET IF YOU *WANT* TO LOAD INVALID TAGS
    pub fn set_strictness(mut self, strictness: ParseStrictness) -> Self {
        self.strictness = strictness;
        self
    }

    /// Testing only!
    fn parse_strs(self, args: &'static [&'static str]) -> Result<CommandLineArgs, String> {
        self.parse(args.iter().map(<&str>::to_string))
    }

    pub fn parse<T>(mut self, mut args: T) -> Result<CommandLineArgs, String> where T: Iterator<Item=String> {
        'arg_iter: loop {
            let next_arg = if let Some(n) = args.next() {
                n
            } else {
                break
            };

            let mut parse_argument = |p: &mut Parameter| -> Result<(), String> {
                if !p.multiple && p.values.is_some() {
                    return Err(format!("Argument parse error: --{} already passed (multiple not allowed for this)", p.name))
                }

                let mut values = Vec::with_capacity(p.value_count);
                for i in 0..p.value_count {
                    let next_argument = match args.next() {
                        Some(n) => n,
                        None => return Err(format!("Argument parse error: Not enough arguments for --{} passed; need {} more", p.name, p.value_count - i))
                    };
                    let parsed_argument = match p.value_type.expect("value type not set for something that takes arguments") {
                        CommandLineValueType::Path => CommandLineValue::Path(next_argument.into()),
                        CommandLineValueType::String => CommandLineValue::String(next_argument),
                        CommandLineValueType::Float => CommandLineValue::Float(next_argument.parse().map_err(|e| format!("Argument parse error: Cannot convert {next_argument} into float: {e}"))?),
                        CommandLineValueType::Short => CommandLineValue::Short(next_argument.parse().map_err(|e| format!("Argument parse error: Cannot convert {next_argument} into short: {e}"))?),
                        CommandLineValueType::UShort => CommandLineValue::UShort(next_argument.parse().map_err(|e| format!("Argument parse error: Cannot convert {next_argument} into ushort: {e}"))?),
                        CommandLineValueType::Integer => CommandLineValue::Integer(next_argument.parse().map_err(|e| format!("Argument parse error: Cannot convert {next_argument} into int: {e}"))?),
                        CommandLineValueType::UInteger => CommandLineValue::UInteger(next_argument.parse().map_err(|e| format!("Argument parse error: Cannot convert {next_argument} into uint: {e}"))?),
                    };
                    values.push(parsed_argument);
                }
                if p.multiple {
                    if let Some(n) = &mut p.values {
                        n.append(&mut values);
                    } else {
                        p.values = Some(values);
                    }
                } else {
                    p.values = Some(values);
                }
                Ok(())
            };

            if next_arg.starts_with("--") {
                let to_match = &next_arg[2..];
                for p in all_args_mut!(self) {
                    if p.name == to_match {
                        if to_match == "help" {
                            (self.on_help)(&self)?;
                        } else {
                            parse_argument(p)?;
                        }
                        continue 'arg_iter
                    }
                }
                return Err(format!("Argument parse error: Argument {next_arg} not recognized"))
            } else if next_arg.starts_with("-") {
                'char_iter: for short in (&next_arg[1..]).chars() {
                    for p in all_args_mut!(self) {
                        if let Some(param_short) = p.short {
                            if param_short == short {
                                if param_short == 'h' {
                                    (self.on_help)(&self)?;
                                } else {
                                    parse_argument(p)?;
                                }
                                continue 'char_iter
                            }
                        }
                    }
                    return Err(format!("Argument parse error: Argument -{short} not recognized"))
                }
            } else {
                if self.required_extra_parameters <= self.extra_parameters.len() {
                    if self.required_extra_parameters == 0 {
                        return Err(format!("Argument parse error: Unexpected extra argument {next_arg}"));
                    }
                    return Err(format!("Argument parse error: Too many extra arguments specified; expected only {}", self.required_extra_parameters));
                }
                self.extra_parameters.push(next_arg);
            }
        }

        if self.extra_parameters.len() != self.required_extra_parameters {
            return Err(format!("Argument parse error: Expected {} extra argument(s), got {} instead", self.required_extra_parameters, self.extra_parameters.len()))
        }

        for i in all_args_mut!(self) {
            if i.default_values.is_some() && i.values.is_none() {
                i.values = std::mem::take(&mut i.default_values);
            }
        }

        for i in all_args!(self) {
            if i.required && i.values.is_none() {
                return Err(format!("Argument parse error: Expected --{} to be set", i.name))
            }
        }

        let require_exists = |what: StandardParameterType| -> Result<(), String> {
            if let Some(n) = self.standard_parameters.get(&what) {
                for i in n.values.as_ref().unwrap() {
                    let path = i.path();
                    if !path.exists() {
                        return Err(format!("Path `{}` does not exist for --{}", path.display(), n.name))
                    }
                }
            }
            Ok(())
        };
        require_exists(StandardParameterType::Tags)?;
        require_exists(StandardParameterType::Data)?;
        require_exists(StandardParameterType::Maps)?;
        require_exists(StandardParameterType::CowTags)?;

        Ok(CommandLineArgs {
            custom_parameters: self.custom_parameters,
            standard_parameters: self.standard_parameters,
            extra_parameters: self.extra_parameters,
            strictness: self.strictness
        })
    }
}

impl CommandLineArgs {
    /// Get the Tags parameter.
    ///
    /// Panics if Tags was not added.
    pub fn get_tags(&self) -> Vec<&Path> {
        self.standard_parameters
            .get(&StandardParameterType::Tags)
            .expect("tags not added as standard parameter")
            .values
            .as_ref()
            .expect("tags should be present even if it's a default")
            .iter()
            .map(CommandLineValue::path)
            .collect()
    }

    /// Create a `VirtualTagsDirectory` instance.
    pub fn get_virtual_tags_directory(&self) -> VirtualTagsDirectory {
        let cow = self.standard_parameters
            .get(&StandardParameterType::CowTags)
            .and_then(|v|
                v.values
                    .as_ref()
                    .expect("cow should be present even if it's a default")
                    .first()
                    .map(|v| v.path().to_path_buf())
            );

        let mut dir = VirtualTagsDirectory::new(self.get_tags().as_slice(), cow).unwrap();
        dir.set_strictness(self.strictness);
        dir
    }

    /// Get the Data parameter.
    ///
    /// Panics if Data was not added.
    pub fn get_data(&self) -> &Path {
        self.standard_parameters
            .get(&StandardParameterType::Data)
            .expect("data not added as standard parameter")
            .values
            .as_ref()
            .expect("data should be present even if it's a default")[0]
            .path()
    }

    /// Get the Maps parameter.
    ///
    /// Panics if Maps was not added.
    pub fn get_maps(&self) -> &Path {
        self.standard_parameters
            .get(&StandardParameterType::Maps)
            .expect("maps not added as standard parameter")
            .values
            .as_ref()
            .expect("maps should be present even if it's a default")[0]
            .path()
    }

    /// Get the Overwrite parameter.
    ///
    /// Panics if Overwrite was not added.
    pub fn get_overwrite(&self) -> bool {
        self.standard_parameters
            .get(&StandardParameterType::Overwrite)
            .expect("overwrite not added as standard parameter")
            .values
            .is_some()
    }

    /// Get the Jobs parameter.
    ///
    /// Panics if Jobs was not added.
    pub fn get_jobs(&self) -> usize {
        self.standard_parameters
            .get(&StandardParameterType::Jobs)
            .expect("jobs not added as standard parameter")
            .values
            .as_ref()
            .map(|v| (v[0].uinteger() as usize).max(1)) // if 0 was passed, set to 1
            .unwrap_or_else(|| {
                std::thread::available_parallelism().map(|t| t.get().max(1)).unwrap_or(1)
            })
    }

    /// Get the custom parameters.
    ///
    /// Panics if not added.
    pub fn get_custom(&self, what: &'static str) -> Option<&[CommandLineValue]> {
        self.custom_parameters.get(what).expect("custom parameter not added on init but expected").values.as_ref().map(Vec::as_slice)
    }

    /// Get the extra parameters.
    pub fn get_extra(&self) -> &[String] {
        self.extra_parameters.as_slice()
    }
}

#[derive(Clone, Default)]
pub struct Parameter {
    name: &'static str,
    short: Option<char>,
    usage: &'static str,
    description: &'static str,
    value_type: Option<CommandLineValueType>,
    values: Option<Vec<CommandLineValue>>,
    default_values: Option<Vec<CommandLineValue>>,
    value_count: usize,
    multiple: bool,
    required: bool
}
impl Parameter {
    pub fn new(
        name: &'static str,
        short: char,
        description: &'static str,
        usage: &'static str,
        value_type: Option<CommandLineValueType>,
        value_count: usize,
        default_values: Option<Vec<CommandLineValue>>,
        multiple: bool,
        required: bool,
    ) -> Parameter {
        debug_assert!(!(value_type.is_none() && value_count != 0), "must set value type iff value count is non-zero");
        debug_assert!(!(value_type.is_some() && value_count == 0), "must set value type iff value count is non-zero");
        debug_assert!(default_values.is_none() || default_values.as_ref().is_some_and(|f| f.len() == value_count), "default values, if set, must equal value count");

        Self {
            name,
            short: Some(short),
            description,
            value_type,
            values: None,
            default_values,
            value_count,
            multiple,
            usage,
            required
        }
    }

    pub fn single(name: &'static str, short: char, description: &'static str, usage: &'static str, value_type: Option<CommandLineValueType>) -> Parameter {
        Parameter::new(name, short, description, usage, value_type, if value_type.is_some() { 1 } else { 0 }, None, false, false)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum StandardParameterType {
    Tags,
    Data,
    Maps,
    Help,
    CowTags,
    Overwrite,
    Jobs,
}

#[derive(Debug, Clone)]
pub enum CommandLineValue {
    Path(PathBuf),
    String(String),
    Integer(i32),
    UInteger(u32),
    Short(i16),
    UShort(u16),
    Float(f32)
}

impl CommandLineValue {
    pub fn path(&self) -> &Path {
        if let CommandLineValue::Path(p) = self {
            p
        }
        else {
            unreachable!()
        }
    }

    pub fn string(&self) -> &str {
        if let CommandLineValue::String(s) = self {
            s
        }
        else {
            unreachable!()
        }
    }

    pub fn integer(&self) -> i32 {
        if let CommandLineValue::Integer(v) = self {
            *v
        }
        else {
            unreachable!()
        }
    }

    pub fn uinteger(&self) -> u32 {
        if let CommandLineValue::UInteger(v) = self {
            *v
        }
        else {
            unreachable!()
        }
    }

    pub fn short(&self) -> i16 {
        if let CommandLineValue::Short(v) = self {
            *v
        }
        else {
            unreachable!()
        }
    }

    pub fn ushort(&self) -> u16 {
        if let CommandLineValue::UShort(v) = self {
            *v
        }
        else {
            unreachable!()
        }
    }

    pub fn float(&self) -> f32 {
        if let CommandLineValue::Float(v) = self {
            *v
        }
        else {
            unreachable!()
        }
    }
}

#[derive(PartialEq, Copy, Clone)]
pub enum CommandLineValueType {
    Path,
    String,
    Integer,
    UInteger,
    Short,
    UShort,
    Float
}

#[cfg(test)]
mod test;
