pub mod parameter {

	use std::env::args;
    use std::marker::Copy;
    use std::ops::RangeInclusive;
    use std::option::Option::None;
    use std::option::Option::Some;
	use std::process::exit;
	use std::result::Result;
	use std::string::String;
    use std::vec::Vec;
	use std::fmt::Display;

	#[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum ParameterType {
        NUMBER,
        STRING,
        ENUM,
        BOOLEAN,
    }

	pub trait ParameterValue: Display {}
	impl ParameterValue for String {}
	impl ParameterValue for u64 {}
	impl ParameterValue for bool {}

	#[derive(Debug, Copy, Clone)]
    pub struct Parameter<'a, T: ParameterValue> {
		long_names: &'a Vec<String>,
        short_names: &'a Vec<String>,

        description: &'a String,
        types: &'a Vec<ParameterType>,

        default_value: &'a T,
        value: &'a Option<T>,

        allowed_number_values: &'a Option<Vec<RangeInclusive<u64>>>,
        allowed_string_values: &'a Option<Vec<String>>,
		string_check_function: &'a Option<fn(String) -> Result<String, String>>,
    }

    impl<T: ParameterValue> Parameter<'_, T> {
		fn get_value(&self) -> T {
            self.value.unwrap_or(*self.default_value.clone())
        }

		fn set_value(&self, value: &String) -> Result<(), &'static str> {
			None
				.or(self.has(&ParameterType::NUMBER)
					.and_then(|_| value.parse::<u64>().ok())
					.and_then(|v| Some(self.value = &Some(v)))
					.and_then(|_| Some(())))
				.or(self.has(&ParameterType::BOOLEAN)
					.and_then(|_| {
						match value.as_str() {
							"true" => Some(self.value = &Some(true)),
							"false" => Some(self.value = &Some(false)),
							_ => None,
						}
					})
					.and_then(|_| Some(())))
				.or(self.has(&ParameterType::ENUM)
					.and_then(|_| self.allowed_string_values.unwrap().contains(value).then_some(()))
					.and_then(|_| Some(self.value = &Some(*value)))
					.and_then(|_| Some(())))
				.or(self.has(&ParameterType::STRING)
					.and_then(|_| self.string_check_function.unwrap()(*value).ok())
					.and_then(|_| Some(self.value = &Some(*value)))
					.and_then(|_| Some(())))
				.ok_or("Value isn't valid")
        }

		fn get_long_names(&self) -> String {
			self.long_names
				.iter()
				.map(|name| String::from("--") + name)
				.collect::<Vec<String>>()
				.join(", ")
		}

		fn get_short_names(&self) -> String {
			self.short_names
				.iter()
				.map(|name| format!("-{}", name))
				.collect::<Vec<String>>()
				.join(", ")
		}

		fn is(&self, name: &String) -> bool {
			self.long_names.contains(name) || self.short_names.contains(name)
		}

		fn has(&self, t: &ParameterType) -> Option<()> {
			self.types.contains(t).then_some(())
		}
    }

    impl<T: ParameterValue> ToString for Parameter<'_, T> {
        fn to_string(&self) -> String {
            return format!(
				"names: {}\naliases: {}\ndescription: {}\ndefault value: {}",
                self.get_long_names(),
                self.get_short_names(),
                self.description,
                self.default_value
            );
        }
    }

    pub fn get_parameters<'a, T: ParameterValue>() -> Vec<Parameter<'a, T>> {
        let mut parameters: Vec<Parameter<T>> = vec![
            Parameter {
				long_names: &vec![String::from("min-password-length")],
                short_names: &vec![String::from("minpl")],
				
                description: &String::from("The minimum length of the passwords"),
                types: &vec![ParameterType::NUMBER],

                default_value: &String::from("6"), // TODO
                value: &Option::None,

                allowed_number_values: &Some(vec![RangeInclusive::new(1, 8)]),
                allowed_string_values: &Some(vec![]),
				string_check_function: &None,
            },
            Parameter {
				long_names: &vec![String::from("max-password-length")],
                short_names: &vec![String::from("maxpl")],
				
                description: &String::from("The maximum length of the passwords"),
                types: &vec![ParameterType::NUMBER],

                default_value: &String::from("8"), // TODO
                value: &Option::None,

                allowed_number_values: &Some(vec![RangeInclusive::new(6, 12)]),
                allowed_string_values: &Some(vec![]),
				string_check_function: &Option::None,
            },
            Parameter {
				long_names: &vec![String::from("password-file")],
                short_names: &vec![String::from("pf")],

                description: &String::from("The file of the path to store the passwords to"),
                types: &vec![ParameterType::STRING],

                default_value: &String::from("./passwords.txt"),
                value: &Option::None,

                allowed_number_values: &Some(vec![RangeInclusive::new(6, 12)]),
                allowed_string_values: &Some(vec![]),
				string_check_function: &Option::None,
            },
        ];

        parameters = args()
            .skip(1)
            .filter(|arg| arg.trim_start_matches("-") != arg)
            .map(|arg| arg.trim_start_matches("-").to_string())
            .map(|arg| arg.split_once("=").unwrap_or((arg.as_str(), "")))
            .fold(parameters, |mut parameters, current| {
                parameters
					.iter_mut()
					.find(|parameter| parameter.is(&String::from(current.0)))
					.and_then(|param| Some(param.set_value(&String::from(current.1))));
				return parameters;
            });

		if args().len() == 0 {
			print_help(&parameters);
			exit(0);
		}
		return parameters;
    }

	pub fn print_help<T: ParameterValue>(parameters: &Vec<Parameter<T>>) {
		println!("");
	}
}
