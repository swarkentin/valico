
use serialize::json::{Json, ToJson};
use helpers::{single_validation_error};
use ValicoResult;

pub trait SingleParamValidator {
	fn validate(&self, &Json) -> ValicoResult<()>;
}

pub struct AllowedValuesValidator {
	allowed_values: Vec<Json>
}

impl AllowedValuesValidator {
	pub fn new(values: Vec<Json>) -> AllowedValuesValidator {
		AllowedValuesValidator {
			allowed_values: values
		}
	}
}

impl SingleParamValidator for AllowedValuesValidator {
	fn validate(&self, val: &Json) -> ValicoResult<()> {
		let mut matched = false;
		for allowed_value in self.allowed_values.iter() {
			if (val == allowed_value) { matched = true; }
		}

		if matched {
			Ok(())
		} else {
			Err(single_validation_error(format!("Value {} is not among allowed list", val)))
		}
	}
}

pub struct RejectedValuesValidator {
	rejected_values: Vec<Json>
}

impl RejectedValuesValidator {
	pub fn new(values: Vec<Json>) -> RejectedValuesValidator {
		RejectedValuesValidator {
			rejected_values: values
		}
	}
}

impl SingleParamValidator for RejectedValuesValidator {
	fn validate(&self, val: &Json) -> ValicoResult<()> {
		let mut matched = false;
		for rejected_value in self.rejected_values.iter() {
			if (val == rejected_value) { matched = true; }
		}

		if matched {
			Err(single_validation_error(format!("Value {} is among reject list", val)))
		} else {
			Ok(())
		}
	}
}

pub struct FunctionValidator {
	validator: fn(&Json) -> Result<(), String>
}

impl FunctionValidator {
	pub fn new(validator: fn(&Json) -> Result<(), String>) -> FunctionValidator {
		FunctionValidator {
			validator: validator
		}
	}
}

impl SingleParamValidator for FunctionValidator {
	fn validate(&self, val: &Json) -> ValicoResult<()> {
		let validator = self.validator;
		match validator(val) {
			Ok(()) => Ok(()),
			Err(err) => Err(single_validation_error(err))
		}
	}
}