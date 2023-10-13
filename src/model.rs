use std::fmt::Debug;

use crate::environment::Environment;

pub trait Evaluatable: ToString + Debug {
    fn evaluate<'a>(&'a self, environment: &'a Environment) -> Result<Primitive, String>;
}

enum OperationType {
    Arithmetic,
    Logical,
    Bitwise,
    Equality,
    Relational,
    FloatToInt,
    IntToFloat
}

impl ToString for OperationType {
    fn to_string(&self) -> String {
        match self {
            Self::Arithmetic => String::from("Arithmetic"),
            Self::Logical => String::from("Logical"),
            Self::Bitwise => String::from("Bitwise"),
            Self::Equality => String::from("Equality"),
            Self::Relational => String::from("Relational"),
            Self::FloatToInt => String::from("Float to Int"),
            Self::IntToFloat => String::from("Int to Float"),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Primitive {
    Integer(i32),
    Float(f32),
    Boolean(bool),
    String(String),
}

impl ToString for Primitive {
    fn to_string(&self) -> String {
        match self {
            Primitive::Integer(val) => format!("{val}"),
            Primitive::Float(val) => format!("{val}"),
            Primitive::Boolean(val) => format!("{val}"),
            Primitive::String(val) => format!("\"{val}\""),
        }
    }
}

impl Evaluatable for Primitive {
    fn evaluate(&self, _environment: &Environment) -> Result<Primitive, String> {
        match self {
            Primitive::Integer(val) => Ok(Primitive::Integer(val.clone())),
            Primitive::Float(val) => Ok(Primitive::Float(val.clone())),
            Primitive::Boolean(val) => Ok(Primitive::Boolean(val.clone())),
            Primitive::String(val) => Ok(Primitive::String(val.clone())),
        }
    }
}

impl Primitive {
    fn type_string(&self) -> String {
        match self {
            Primitive::Integer(_) => String::from("Integer"),
            Primitive::Float(_) => String::from("Float"),
            Primitive::Boolean(_) => String::from("Boolean"),
            Primitive::String(_) => String::from("String"),
        }
    }

    fn is_numeric(&self) -> bool {
        match self {
            Self::Integer(_) => true,
            Self::Float(_) => true,
            _ => false
        }
    }

    fn is_integer(&self) -> bool {
        match self {
            Self::Integer(_) => true,
            _ => false
        }
    }

    fn _is_float(&self) -> bool {
        match self {
            Self::Float(_) => true,
            _ => false
        }
    }

    fn is_boolean(&self) -> bool {
        match self {
            Self::Boolean(_) => true,
            _ => false
        }
    }

    fn _is_string(&self) -> bool {
        match self {
            Self::String(_) => true,
            _ => false
        }
    }

    fn get_int_value(&self) -> Result<i32, String> {
        match self {
            Self::Integer(val) => Ok(val.clone()),
            _ => Err(format!("Invalid type to get integer value: {}", self.type_string()))
        }
    }

    fn get_float_value(&self) -> Result<f32, String> {
        match self {
            Self::Float(val) => Ok(val.clone()),
            _ => Err(format!("Invalid type to get float value: {}", self.type_string()))
        }
    }
    
    fn get_boolean_value(&self) -> Result<bool, String> {
        match self {
            Self::Boolean(val) => Ok(val.clone()),
            _ => Err(format!("Invalid type to get Boolean value: {}", self.type_string()))
        }
    }

    fn get_string_value<'a>(&'a self) -> Result<&'a str, String> {
        match self {
            Self::String(val) => Ok(val),
            _ => Err(format!("Invalid type to get String value: {}", self.type_string()))
        }
    }
}

#[derive(Debug)]
pub enum Operation {
    Add(Box <dyn Evaluatable>, Box <dyn Evaluatable>),
    Subtract(Box <dyn Evaluatable>, Box <dyn Evaluatable>),
    Multiply(Box <dyn Evaluatable>, Box <dyn Evaluatable>),
    Divide(Box <dyn Evaluatable>, Box <dyn Evaluatable>),
    Modulus(Box <dyn Evaluatable>, Box <dyn Evaluatable>),
    Power(Box <dyn Evaluatable>, Box <dyn Evaluatable>),
    LogicalAnd(Box <dyn Evaluatable>, Box <dyn Evaluatable>),
    LogicalOr(Box <dyn Evaluatable>, Box <dyn Evaluatable>),
    LogicalNot(Box <dyn Evaluatable>),
    BitwiseAnd(Box <dyn Evaluatable>, Box <dyn Evaluatable>),
    BitwiseOr(Box <dyn Evaluatable>, Box <dyn Evaluatable>),
    BitwiseXor(Box <dyn Evaluatable>, Box <dyn Evaluatable>),
    BitwiseNot(Box <dyn Evaluatable>),
    LeftShift(Box <dyn Evaluatable>, Box <dyn Evaluatable>),
    RightShift(Box <dyn Evaluatable>, Box <dyn Evaluatable>),
    Equals(Box <dyn Evaluatable>, Box <dyn Evaluatable>),
    NotEquals(Box <dyn Evaluatable>, Box <dyn Evaluatable>),
    LessThan(Box <dyn Evaluatable>, Box <dyn Evaluatable>),
    LessThanOrEqual(Box <dyn Evaluatable>, Box <dyn Evaluatable>),
    GreaterThan(Box <dyn Evaluatable>, Box <dyn Evaluatable>),
    GreaterThanOrEqual(Box <dyn Evaluatable>, Box <dyn Evaluatable>),
    FloatToInt(Box <dyn Evaluatable>),
    IntToFloat(Box <dyn Evaluatable>),
}

impl ToString for Operation {
    fn to_string(&self) -> String {
        match self {
            Self::Add(val1, val2) => format!("({}) + ({})", val1.to_string(), val2.to_string()),
            Self::Subtract(val1, val2) => format!("({}) - ({})", val1.to_string(), val2.to_string()),
            Self::Multiply(val1, val2) => format!("({}) * ({})", val1.to_string(), val2.to_string()),
            Self::Divide(val1, val2) => format!("({}) / ({})", val1.to_string(), val2.to_string()),
            Self::Modulus(val1, val2) => format!("({}) % ({})", val1.to_string(), val2.to_string()),
            Self::Power(val1, val2) => format!("({}) ** ({})", val1.to_string(), val2.to_string()),

            Self::LogicalAnd(val1, val2) => format!("({}) && ({})", val1.to_string(), val2.to_string()),
            Self::LogicalOr(val1, val2) => format!("({}) || ({})", val1.to_string(), val2.to_string()),
            Self::LogicalNot(val1) => format!("!({})", val1.to_string()),

            Self::BitwiseAnd(val1, val2) => format!("({}) & ({})", val1.to_string(), val2.to_string()),
            Self::BitwiseOr(val1, val2) => format!("({}) | ({})", val1.to_string(), val2.to_string()),
            Self::BitwiseXor(val1, val2) => format!("({}) ^ ({})", val1.to_string(), val2.to_string()),
            Self::BitwiseNot(val1) => format!("~({})", val1.to_string()),
            Self::LeftShift(val1, val2) => format!("({}) << ({})", val1.to_string(), val2.to_string()),
            Self::RightShift(val1, val2) => format!("({}) >> ({})", val1.to_string(), val2.to_string()),

            Self::Equals(val1, val2) => format!("({}) == ({})", val1.to_string(), val2.to_string()),
            Self::NotEquals(val1, val2) => format!("({}) != ({})", val1.to_string(), val2.to_string()),

            Self::LessThan(val1, val2) => format!("({}) < ({})", val1.to_string(), val2.to_string()),
            Self::LessThanOrEqual(val1, val2) => format!("({}) <= ({})", val1.to_string(), val2.to_string()),
            Self::GreaterThan(val1, val2) => format!("({}) > ({})", val1.to_string(), val2.to_string()),
            Self::GreaterThanOrEqual(val1, val2) => format!("({}) >= ({})", val1.to_string(), val2.to_string()),

            Self::FloatToInt(val1) => format!("FloatToInt({})", val1.to_string()),
            Self::IntToFloat(val1) => format!("IntToFloat({})", val1.to_string()),
        }
    }
}

impl Evaluatable for Operation {
    fn evaluate<'a>(&'a self, environment: &'a Environment) -> Result<Primitive, String> {
        let (result1, result2) = get_results(self, environment);

        let results = unpack_results(result1, result2);
        let val1: Primitive;
        let val2_option: Option<Primitive>;

        match results {
            Ok(vals) => (val1, val2_option) = vals,
            Err(e) => return Err(e)
        }

        match self.get_type() {
            OperationType::Arithmetic => {
                arithmetic(self, &val1, &val2_option.unwrap())
            }
            OperationType::Logical => {
                logic(self, &val1, &val2_option)   
            }
            OperationType::Bitwise => {
                bitwise(self, &val1, &val2_option)
            }
            OperationType::Equality => {
                equality(self, &val1, &val2_option.unwrap())
            }
            OperationType::Relational => {
                relation(self, &val1, &val2_option.unwrap())
            }
            OperationType::FloatToInt => {
                cast_to_integer(&val1)
            }
            OperationType::IntToFloat => {
                cast_to_float(&val1)
            }
        }
    }
}


fn type_mismatch_error<'a>(val1: &Primitive, val2_option: &Option<Primitive>, operation: &Operation) -> Result<Primitive, String> {
    if let Some(val2) = val2_option {
        Err(format!("Incompatible types: {} and {} for {} operation",val1.type_string(), val2.type_string(), operation.get_type().to_string()))
    } else {
        Err(format!("Incompatible type: {} for {} operation", val1.type_string(), operation.get_type().to_string()))
    }   
}

impl Operation {
    fn get_type(&self) -> OperationType {
        match self {
            Self::Add(_, _) => OperationType::Arithmetic,
            Self::Subtract(_, _) => OperationType::Arithmetic,
            Self::Multiply(_, _) => OperationType::Arithmetic,
            Self::Divide(_, _) => OperationType::Arithmetic,
            Self::Modulus(_, _) => OperationType::Arithmetic,
            Self::Power(_, _) => OperationType::Arithmetic,

            Self::LogicalAnd(_, _) => OperationType::Logical,
            Self::LogicalOr(_, _) => OperationType::Logical,
            Self::LogicalNot(_) => OperationType::Logical,

            Self::BitwiseAnd(_, _) => OperationType::Bitwise,
            Self::BitwiseOr(_, _) => OperationType::Bitwise,
            Self::BitwiseXor(_, _) => OperationType::Bitwise,
            Self::BitwiseNot(_) => OperationType::Bitwise,
            Self::LeftShift(_, _) => OperationType::Bitwise,
            Self::RightShift(_, _) => OperationType::Bitwise,

            Self::Equals(_, _) => OperationType::Equality,
            Self::NotEquals(_, _) => OperationType::Equality,

            Self::LessThan(_, _) => OperationType::Relational,
            Self::LessThanOrEqual(_, _) => OperationType::Relational,
            Self::GreaterThan(_, _) => OperationType::Relational,
            Self::GreaterThanOrEqual(_, _) => OperationType::Relational,

            Self::FloatToInt(_) => OperationType::FloatToInt,
            Self::IntToFloat(_) => OperationType::IntToFloat,
        }
    }
}

/// Address of given cell
#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
pub struct CellAddress(pub i32, pub i32);

impl ToString for CellAddress {
    fn to_string(&self) -> String {
        format!("({}, {})", self.0, self.1)
    }
}

/// Value of given cell
#[derive(Debug, Clone)]
pub struct CellValue(pub i32, pub i32);

impl Evaluatable for CellValue {
    fn evaluate<'a>(&'a self, environment: &'a Environment) -> Result<Primitive, String> {
        let value = environment.grid().get_cell(&CellAddress(self.0, self.1));
        match value {
            Some(result) => match result {
                Ok(val) => val.evaluate(environment),
                Err(err) => Err(err.to_string()),
            }
            None => Err(format!("Value for cell ({}, {}) not found", self.0, self.1)),
        }
    }
}

impl ToString for CellValue {
    fn to_string(&self) -> String {
        format!("({}, {})", self.0, self.1)
    }
}

#[derive(Debug)]
pub enum Statistics {
    Max(CellAddress, CellAddress),
    Min(CellAddress, CellAddress),
    Mean(CellAddress, CellAddress),
    Sum(CellAddress, CellAddress),
}

impl ToString for Statistics {
    fn to_string(&self) -> String {
        match self {
            Self::Max(cell1, cell2) => format!("max({}, {})", cell1.to_string(), cell2.to_string()),
            Self::Min(cell1, cell2) => format!("min({}, {})", cell1.to_string(), cell2.to_string()),
            Self::Mean(cell1, cell2) => format!("mean({}, {})", cell1.to_string(), cell2.to_string()),
            Self::Sum(cell1, cell2) => format!("sum({}, {})", cell1.to_string(), cell2.to_string()),
        }
    }
}

impl Evaluatable for Statistics {
    fn evaluate<'a>(&'a self, environment: &'a Environment) -> Result<Primitive, String> {
        let top_left_cell: &'a CellAddress;
        let bot_right_cell: &'a CellAddress;

        match self {
            Self::Max(val1, val2) => {
                top_left_cell = val1;
                bot_right_cell = val2;
            }
            Self::Min(val1, val2) => {
                top_left_cell = val1;
                bot_right_cell = val2;
            }
            Self::Mean(val1, val2) => {
                top_left_cell = val1;
                bot_right_cell = val2;
            }
            Self::Sum(val1, val2) => {
                top_left_cell = val1;
                bot_right_cell = val2;
            }
        }

        if top_left_cell.0 > bot_right_cell.0 || top_left_cell.1 > bot_right_cell.1 {
            Err(format!("Invalid Address for Statistics Function"))
        } else {
            let cells: Vec<CellAddress> = get_cells(top_left_cell, bot_right_cell);
            let mut cell_vals: Vec<Primitive> = Vec::new();
            
            for cell in &cells {
                let result = environment.grid().get_cell(&cell);

                match result {
                    Some(result) => {
                        let evaluated_val = match result {
                            Ok(val) => val.evaluate(environment),
                            Err(err) => return Err(err.to_string()),
                        };

                        match evaluated_val {
                            Ok(primitive_val) => {
                                if !primitive_val.is_numeric() {
                                    return Err(format!("Value in cell ({},{}) is not numeric", cell.0, cell.1));
                                }

                                cell_vals.push(primitive_val);
                            }
                            Err(e) => {
                                return Err(e);
                            }
                        }
                    }
                    None => return Err(format!("Value for cell ({}, {}) not found", cell.0, cell.1)),
                }
            }

            match self {
                Self::Max(_, _) => max(&cell_vals, environment),
                Self::Min(_, _) => min(&cell_vals, environment),
                Self::Mean(_, _) => mean(&cell_vals),
                Self::Sum(_, _) => Ok(Primitive::Float(sum(&cell_vals))),
            }
        }
    }
}

fn mean<'a>(cell_vals: &Vec<Primitive>) -> Result<Primitive, String> {
    Ok(Primitive::Float(sum(&cell_vals) / cell_vals.len() as f32))
}

fn sum(cell_vals: &Vec<Primitive>) -> f32 {
    let mut total = 0.0;
    for val in cell_vals {
        total += coerce_to_float(&val).unwrap().get_float_value().unwrap();
    }
    total
}

fn max<'a>(vals: &Vec<Primitive>, _environment: &'a Environment) -> Result<Primitive, String> {
    let mut max_val = Primitive::Integer(i32::MIN);
    for val in vals {
        if coerce_to_float(&val).unwrap().get_float_value() > coerce_to_float(&max_val).unwrap().get_float_value() {
            max_val = val.clone();
        }
    }
    Ok(max_val)
}

fn min<'a>(vals: &Vec<Primitive>, _environment: &'a Environment) -> Result<Primitive, String> {
    let mut min_val = Primitive::Integer(i32::MAX);
    for val in vals {
        if coerce_to_float(&val).unwrap().get_float_value() < coerce_to_float(&min_val).unwrap().get_float_value() {
            min_val = val.clone();
        }
    }
    Ok(min_val)
}

fn get_cells(top_left_cell: &CellAddress, bot_right_cell: &CellAddress) -> Vec<CellAddress> {
    let mut cells: Vec<CellAddress> = Vec::new();

    for y in top_left_cell.1..=bot_right_cell.1 {
        for x in top_left_cell.0..=bot_right_cell.0 {
            cells.push(CellAddress(x, y));
        }
    }

    cells
}

fn get_results<'a>(operation: &'a Operation, environment: &'a Environment) -> (Result<Primitive, String>, Option<Result<Primitive, String>>) {
    let val1: Result<Primitive, String>;
    let val2: Option<Result<Primitive, String>>;

    match operation {
        Operation::Add(v1, v2) => {
            val1 = v1.evaluate(environment);
            val2 = Some(v2.evaluate(environment));
        }
        Operation::Subtract(v1, v2) => {
            val1 = v1.evaluate(environment);
            val2 = Some(v2.evaluate(environment));
        }
        Operation::Multiply(v1, v2) => {
            val1 = v1.evaluate(environment);
            val2 = Some(v2.evaluate(environment));
        }
        Operation::Divide(v1, v2) => {
            val1 = v1.evaluate(environment);
            val2 = Some(v2.evaluate(environment));
        }
        Operation::Modulus(v1, v2) => {
            val1 = v1.evaluate(environment);
            val2 = Some(v2.evaluate(environment));
        }
        Operation::Power(v1, v2) => {
            val1 = v1.evaluate(environment);
            val2 = Some(v2.evaluate(environment));
        }

        Operation::LogicalAnd(v1, v2) => {
            val1 = v1.evaluate(environment);
            val2 = Some(v2.evaluate(environment));
        }
        Operation::LogicalOr(v1, v2) => {
            val1 = v1.evaluate(environment);
            val2 = Some(v2.evaluate(environment));
        }
        Operation::LogicalNot(v1) => {
            val1 = v1.evaluate(environment);
            val2 = None;
        }


        Operation::BitwiseAnd(v1, v2) => {
            val1 = v1.evaluate(environment);
            val2 = Some(v2.evaluate(environment));
        }
        Operation::BitwiseOr(v1, v2) => {
            val1 = v1.evaluate(environment);
            val2 = Some(v2.evaluate(environment));
        }
        Operation::BitwiseXor(v1, v2) => {
            val1 = v1.evaluate(environment);
            val2 = Some(v2.evaluate(environment));
        }
        Operation::BitwiseNot(v1) => {
            val1 = v1.evaluate(environment);
            val2 = None;
        }
        Operation::RightShift(v1, v2) => {
            val1 = v1.evaluate(environment);
            val2 = Some(v2.evaluate(environment));
        }
        Operation::LeftShift(v1, v2) => {
            val1 = v1.evaluate(environment);
            val2 = Some(v2.evaluate(environment));
        }


        Operation::Equals(v1, v2) => {
            val1 = v1.evaluate(environment);
            val2 = Some(v2.evaluate(environment));
        }
        Operation::NotEquals(v1, v2) => {
            val1 = v1.evaluate(environment);
            val2 = Some(v2.evaluate(environment));
        }
        Operation::GreaterThan(v1, v2) => {
            val1 = v1.evaluate(environment);
            val2 = Some(v2.evaluate(environment));
        }
        Operation::GreaterThanOrEqual(v1, v2) => {
            val1 = v1.evaluate(environment);
            val2 = Some(v2.evaluate(environment));
        }
        Operation::LessThan(v1, v2) => {
            val1 = v1.evaluate(environment);
            val2 = Some(v2.evaluate(environment));
        }
        Operation::LessThanOrEqual(v1, v2) => {
            val1 = v1.evaluate(environment);
            val2 = Some(v2.evaluate(environment));
        }

        Operation::FloatToInt(v1) => {
            val1 = v1.evaluate(environment);
            val2 = None;
        }
        Operation::IntToFloat(v1) => {
            val1 = v1.evaluate(environment);
            val2 = None;
        }
    }

    (val1, val2)
}

fn unpack_results<'a> (
    result1: Result<Primitive, String>,
    result2: Option<Result<Primitive, String>>,
) -> Result<(Primitive, Option<Primitive>), String> {
    let result1_unpacked: Option<Primitive>;
    let result2_unpacked: Option<Option<Primitive>>;
    let mut error = String::from("");

    match result1 {
        Ok(v) => result1_unpacked = Some(v),
        Err(e) => {
            error = e;
            result1_unpacked = None;
        }
    }

    match result2 {
        Some(result) => match result {
            Ok(v) => result2_unpacked = Some(Some(v)),
            Err(e) => {
                if error.is_empty() {
                    error = e;
                }
                result2_unpacked = None;
            }
        },
        None => result2_unpacked = None,
    }

    let result: Result<(Primitive, Option<Primitive>), String>;

    if error.is_empty() {
        result =  Ok((result1_unpacked.unwrap(), result2_unpacked.unwrap_or_else(|| {
            None
        })));
    } else {
        result = Err(error)
    }

    result
}

fn arithmetic<'a>(operation: &Operation, val1: &Primitive, val2: &Primitive) -> Result<Primitive, String> {
    if val1.is_integer() && val2.is_integer() {
        let v1 = val1.get_int_value().unwrap();
        let v2 = val2.get_int_value().unwrap();

        match operation {
            Operation::Add(_, _) => Ok(Primitive::Integer(v1 + v2)),
            Operation::Subtract(_, _) => Ok(Primitive::Integer(v1 - v2)),
            Operation::Multiply(_, _) => Ok(Primitive::Integer(v1 * v2)),
            Operation::Divide(_, _) => {
                if v2 == 0 {
                    Err(format!("Division by 0"))
                } else {
                    Ok(Primitive::Integer(v1 / v2))
                }
            }
            Operation::Modulus(_, _) => Ok(Primitive::Integer(v1 % v2)),
            Operation::Power(_, _) => {
                if v2 < 0 {
                    Err(format!("Integer exponent cannot be less than 0 for integer base"))
                } else if v1 == 0 && v2 == 0 {
                    Err(format!("Cannot calculate 0 ^ 0"))
                } else {
                    Ok(Primitive::Integer(v1.pow(v2 as u32)))
                }
            }
            _ => panic!("Unexpected Arithmetic Operation"),
        }
    } else if val1.is_numeric() && val2.is_numeric() { // i.e. Because of the previous check, if
        // both are numeric, one must be a float
        let v1 = coerce_to_float(val1).unwrap().get_float_value().unwrap();
        let v2 = coerce_to_float(val2).unwrap().get_float_value().unwrap();

        match operation {
            Operation::Add(_, _) => Ok(Primitive::Float(v1 + v2)),
            Operation::Subtract(_, _) => Ok(Primitive::Float(v1 - v2)),
            Operation::Multiply(_, _) => Ok(Primitive::Float(v1 * v2)),
            Operation::Divide(_, _) => {
                if v2 + 1.0 < 1.0 + f32::EPSILON && v2 > -f32::EPSILON {
                    Err(format!("Division by 0"))
                } else {
                    Ok(Primitive::Float(v1 / v2))
                }
            }
            Operation::Modulus(_, _) => Ok(Primitive::Float(v1 % v2)),
            Operation::Power(_, _) => {
                if v1 + 1.0 < 1.0 + f32::EPSILON  && v2 + 1.0 < 1.0 + f32::EPSILON {
                    Err(format!("Cannot calculate 0 ^ 0"))
                } else {
                    Ok(Primitive::Float(v1.powf(v2)))
                }
            }
            _ => panic!("Unexpected Arithmetic Type"),
        }
    } else {
        type_mismatch_error(val1, &Some(val2.clone()), operation)
    }
}

fn logic<'a>(operation: &Operation, val1: &Primitive, val2_option: &Option<Primitive>) -> Result<Primitive, String> {
    let val2 = val2_option.as_ref().unwrap_or_else(|| {
        &Primitive::Boolean(false)
    });

    if !val1.is_boolean() || !val2.is_boolean() {
        return type_mismatch_error(val1, val2_option, operation);
    }

    let v1 = val1.get_boolean_value().unwrap();
    let v2 = val2.get_boolean_value().unwrap();

    Ok(match operation {
        Operation::LogicalNot(_) => Primitive::Boolean(!v1),
        Operation::LogicalAnd(_, _) => Primitive::Boolean(v1 && v2),
        Operation::LogicalOr(_, _) => Primitive::Boolean(v1 || v2),
        _ => panic!("Unexpected Logical type"),
    })
}

fn bitwise<'a>(operation: &Operation, val1: &Primitive, val2_option: &Option<Primitive>) -> Result<Primitive, String> {
    let val2 = val2_option.as_ref().unwrap_or_else(|| {
        &Primitive::Integer(0)
    });

    if !val1.is_integer() || val2.is_boolean() {
        return type_mismatch_error(val1, val2_option, operation);
    }

    let v1 = val1.get_int_value().unwrap();
    let v2 = val2.get_int_value().unwrap();

    Ok(match operation {
        Operation::BitwiseNot(_) => Primitive::Integer(!v1),
        Operation::BitwiseAnd(_, _) => Primitive::Integer(v1 & v2),
        Operation::BitwiseOr(_, _) => Primitive::Integer(v1 | v2),
        Operation::BitwiseXor(_, _) => Primitive::Integer(v1 ^ v2),
        Operation::LeftShift(_, _) => Primitive::Integer(v1 << v2),
        Operation::RightShift(_, _) => Primitive::Integer(v1 >> v2),
        _ => panic!("Unexpected Bitwise Type"),
    })
}

fn equality<'a>(operation: &Operation, val1: &Primitive, val2: &Primitive) -> Result<Primitive, String> {
    if !val1.type_string().eq(&val2.type_string()) {
        Ok(Primitive::Boolean(false))
    } else {
        let mut result = match val1 {
            Primitive::Integer(v1) => Primitive::Boolean(v1.clone() == val2.get_int_value().unwrap()),
            Primitive::Float(v1) => Primitive::Boolean((v1 - val2.get_float_value().unwrap()).abs() < f32::EPSILON),
            Primitive::String(v1) => Primitive::Boolean(v1.eq(&val2.get_string_value().unwrap())),
            Primitive::Boolean(v1) => Primitive::Boolean(v1 == &val2.get_boolean_value().unwrap()),
        };

        if let Operation::NotEquals(_, _) = operation {
            result = Primitive::Boolean(!result.get_boolean_value().unwrap());
        }

        Ok(result)
    }
}

fn relation<'a>(operation: &Operation, val1: &Primitive, val2: &Primitive) -> Result<Primitive, String> {
    if val1.is_integer() && val2.is_integer() {
        let v1 = val1.get_int_value().unwrap();
        let v2 = val2.get_int_value().unwrap();

        Ok(match operation {
            Operation::LessThan(_, _) => Primitive::Boolean(v1 < v2),
            Operation::LessThanOrEqual(_, _) => Primitive::Boolean(v1 <= v2),
            Operation::GreaterThan(_, _) => Primitive::Boolean(v1 > v2),
            Operation::GreaterThanOrEqual(_, _) => Primitive::Boolean(v1 >= v2),
            _ => panic!("Unexpected Relational Operation"),
        })
   } else if val1.is_numeric() && val2.is_numeric() { // i.e. Because of the previous check, if
        // both are numeric, one must be a float
        let v1 = coerce_to_float(val1).unwrap().get_float_value().unwrap();
        let v2 = coerce_to_float(val2).unwrap().get_float_value().unwrap();

        Ok(match operation {
            Operation::LessThan(_, _) => Primitive::Boolean(v1 < v2),
            Operation::LessThanOrEqual(_, _) => Primitive::Boolean((v1 < v2) || (v1 - v2).abs() < f32::EPSILON),
            Operation::GreaterThan(_, _) => Primitive::Boolean(v1 > v2),
            Operation::GreaterThanOrEqual(_, _) => Primitive::Boolean((v1 < v2) || (v1 - v2).abs() < f32::EPSILON),
            _ => panic!("Unexpected Relational Operation"),
        })
    } else {
        type_mismatch_error(val1, &Some(val2.clone()), operation)
    }
}

fn cast_to_integer<'a>(float: &Primitive) -> Result<Primitive, String> {
    if let Primitive::Float(val) = float {
        Ok(Primitive::Integer(val.clone() as i32))
    } else {
        Err(format!("Cannot cast type: {}", float.type_string()))
    }
}

fn cast_to_float<'a>(integer: &Primitive) -> Result<Primitive, String> {
    if let Primitive::Integer(val) = integer {
        Ok(Primitive::Float(val.clone() as f32))
    } else {
        Err(format!("Cannot cast type: {}", integer.type_string()))
    }
}

fn coerce_to_float<'a>(num: &Primitive) -> Result<Primitive, String> {
    match num {
        Primitive::Float(val) => Ok(Primitive::Float(val.clone())),
        Primitive::Integer(_) => cast_to_float(num),
        _ => Err(format!("Cannot coerce type {} to float", num.type_string()))
    }
}
