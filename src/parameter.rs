pub mod parameter {

	use std::env::args;
	use std::fmt::{Display, Formatter};
    use std::marker::Copy;
    use std::ops::RangeInclusive;
    use std::option::Option::{None, Some};
	use std::process::exit;
	use std::result::Result;
	use std::string::String;
    use std::vec::Vec;

	use enum_as_inner::EnumAsInner;

	#[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum ParameterType {
        NUMBER,
        STRING,
        ENUM,
        BOOLEAN,
    }

	#[derive(Debug, Clone, PartialEq, Eq, EnumAsInner)]
	pub enum ParameterValue {
		String(String),
		Number(u64),
		Boolean(bool),
	}

	impl Display for ParameterValue {
		fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
			match self {
				ParameterValue::String(_) => write!(f, "{}", self.as_string().unwrap()),
				ParameterValue::Number(_) => write!(f, "{}", self.as_number().unwrap()),
				ParameterValue::Boolean(_) => write!(f, "{}", self.as_boolean().unwrap()),
			}
			
		}
	}

	#[derive(Debug, Clone)]
    pub struct Parameter {
		long_names: Vec<String>,
        short_names: Vec<String>,

        description: String,
        types: Vec<ParameterType>,

        default_value: ParameterValue,
        value: Option<ParameterValue>,

        allowed_number_values: Option<Vec<RangeInclusive<u64>>>,
        allowed_string_values: Option<Vec<String>>,
		string_check_function: Option<fn(String) -> Result<String, String>>,
    }

    impl Parameter {
		pub fn get_value(&self) -> &ParameterValue{
            self.value.as_ref().unwrap_or(&self.default_value)
        }

		pub fn set_value(& mut self, value: &String) -> Result<(), &'static str> {
			None
				.or(self.has(&ParameterType::NUMBER)
					.and_then(|_| value.parse::<u64>().ok())
					.and_then(|v| self.allowed_number_values
						.is_none().then_some(v)
						.or_else(|| self.allowed_number_values.as_ref().unwrap().iter().any(|r| r.contains(&v)).then_some(v)))
					.and_then(|v| Some(self.value = Some(ParameterValue::Number(v))))
					.and_then(|_| Some(())))
				.or(self.has(&ParameterType::BOOLEAN)
					.and_then(|_| {
						match value.as_str() {
							"true" => Some(self.value = Some(ParameterValue::Boolean(true))),
							"false" => Some(self.value = Some(ParameterValue::Boolean(false))),
							_ => None,
						}
					})
					.and_then(|_| Some(())))
				.or(self.has(&ParameterType::ENUM)
					.and_then(|_| self.allowed_string_values.as_ref().unwrap().contains(value).then_some(()))
					.and_then(|_| Some(self.value = Some(ParameterValue::String(value.clone()))))
					.and_then(|_| Some(())))
				.or(self.has(&ParameterType::STRING)
					.and_then(|_| self.string_check_function.unwrap()(value.clone()).ok())
					.and_then(|_| Some(self.value = Some(ParameterValue::String(value.clone()))))
					.and_then(|_| Some(())))
				.ok_or("Value isn't valid")
        }

		pub fn get_long_names(&self) -> String {
			self.long_names
				.iter()
				.map(|name| String::from("--") + name)
				.collect::<Vec<String>>()
				.join(", ")
		}

		pub fn get_short_names(&self) -> String {
			self.short_names
				.iter()
				.map(|name| format!("-{}", name))
				.collect::<Vec<String>>()
				.join(", ")
		}

		pub fn is(&self, name: &String) -> bool {
			self.long_names.contains(name) || self.short_names.contains(name)
		}

		pub fn has(&self, t: &ParameterType) -> Option<()> {
			self.types.contains(t).then_some(())
		}
    }

    impl ToString for Parameter {
        fn to_string(&self) -> String {
            return format!(
				"\t- names: {}\n\t  aliases: {}\n\t  description: {}\n\t  default value: {}",
                self.get_long_names(),
                self.get_short_names(),
                self.description,
                self.default_value
            );
        }
    }

    pub fn get_parameters() -> Vec<Parameter> {
        let parameters = vec![
            Parameter {
				long_names: vec![String::from("min-password-length")],
                short_names: vec![String::from("mip")],
				
                description: String::from("The minimum length of the passwords"),
                types: vec![ParameterType::NUMBER],

                default_value: ParameterValue::Number(6),
                value: None,

                allowed_number_values: Some(vec![RangeInclusive::new(1, 8)]),
                allowed_string_values: None,
				string_check_function: None,
            },
            Parameter {
				long_names: vec![String::from("max-password-length")],
                short_names: vec![String::from("map")],
				
                description: String::from("The maximum length of the passwords"),
                types: vec![ParameterType::NUMBER],

                default_value: ParameterValue::Number(8),
                value: None,

                allowed_number_values: Some(vec![RangeInclusive::new(6, 12)]),
                allowed_string_values: None,
				string_check_function: None,
            },
            Parameter {
				long_names: vec![String::from("password-file")],
                short_names: vec![String::from("pf")],

                description: String::from("The path to the file containing the passwords"),
                types: vec![ParameterType::STRING],

                default_value: ParameterValue::String(String::from("./passwords.txt")),
                value: None,

                allowed_number_values: None,
                allowed_string_values: None,
				string_check_function: None,
            },
            Parameter {
				long_names: vec![String::from("hash-file")],
                short_names: vec![String::from("hf")],

                description: String::from("The path to the file containing the hashes"),
                types: vec![ParameterType::STRING],

                default_value: ParameterValue::String(String::from("./hashes.txt")),
                value: None,

                allowed_number_values: None,
                allowed_string_values: None,
				string_check_function: None,
            },
            Parameter {
				long_names: vec![String::from("help")],
                short_names: vec![String::from("h")],

                description: String::from("Prints this help message"),
                types: vec![ParameterType::BOOLEAN],

                default_value: ParameterValue::Boolean(false),
                value: None,

                allowed_number_values: None,
                allowed_string_values: None,
				string_check_function: None,
            },
        ];

		if (args().len() == 1)
		|| *(get_named_parameter(&parameters, &String::from("help")).unwrap().get_value().as_boolean().unwrap()) {
			print_help(&parameters);
			exit(0);
		}

        return args()
            .skip(1)
            .filter(|arg| arg.trim_start_matches("-") != arg)
            .map(|arg| arg.trim_start_matches("-").to_string())
            .map(|arg| arg.split_once("=")
			.and_then(|t| Some((t.0.to_string(), t.1.to_string())))
			.unwrap_or((arg, String::from(""))))
            .fold(parameters, |mut parameters, current| {
                parameters
					.iter_mut()
					.find(|parameter| parameter.is(&current.0))
					.and_then(|param| Some(param.set_value(&current.1)));
				return parameters;
            });
    }

	pub fn get_named_parameter<'a>(parameters: &'a Vec<Parameter>, name: &String) -> Option<&'a Parameter> {
		parameters.iter().find(|param| param.is(name))
	}

	pub fn print_help(parameters: &Vec<Parameter>) -> () {
		println!(
"NAME
\t{} - {}

SYNOPSIS
\t{} [--<name>=<value>] [-<alias>=<value>] [--<name>] [-<alias>]

DESCRIPTION
\t{}

OPTIONS
{}

AUTHOR
\t{}

COPYRIGHT
\t{}",
			env!("CARGO_BIN_NAME").to_uppercase(),
			env!("CARGO_PKG_NAME"),
			env!("CARGO_BIN_NAME"),
			env!("CARGO_PKG_DESCRIPTION"),
			parameters
				.iter()
				.map(|parameter| parameter.to_string())
				.collect::<Vec<String>>()
				.join("\n\n"),
			env!("CARGO_PKG_AUTHORS"),
			env!("CARGO_PKG_LICENSE"),
		);
	}
}
