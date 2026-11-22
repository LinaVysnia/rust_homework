use serde::Deserialize;
use std::fs; 
use axum::{Json, extract::State}; 
use std::collections::HashMap; 

#[derive(Debug, Deserialize, Clone)]  //what is deserialisation?
//every calculation file must have input, output and expression
pub struct CalculationConfig { 
    pub inputs: Vec<String>,
    pub output: String,
    pub expression: String,
}

pub fn load_config() -> Result<CalculationConfig, String> {

    let file_path = "calculation.yaml";
    println!("In file {file_path}");
    let contents = fs::read_to_string(file_path)
        .map_err(|e| format!("Failed to read config file: {e}"))?;

    println!("Calculation text from within: \n{contents}");
    
    //Parsing YAML string into the calculation struct
    let config: CalculationConfig = serde_yaml::from_str(&contents)
        .map_err(|e| format!("Something went wrong parting YAML format: {e}"))?;

    validate_config(&config)?;

    Ok(config)
}

//this expression os Result <()... is odd, why ()?
fn validate_config(config: &CalculationConfig) -> Result<(), String> {
    if config.inputs.is_empty() { 
        return Err("YAML configuration must define at least one input variable.".into()); 
    } 

    if config.output.trim().is_empty() {
        return Err("YAML configuration must specify an output variable name.".into()); 
    } 

    if config.expression.trim().is_empty() { 
        return Err("YAML configuration must provide a mathematical expression.".into()); 
    } 
    
    // Optional For the future : Pre-parse the math expression using chosen math library (e.g. evalexpr, meval) // 
    // to verify syntax and ensure all expression variables are declared in config.inputs. 
    
    Ok(())
}

// Axum automatically extracts and parses the POST JSON body
async fn calculate_handler( 
    State(config): State<Arc<CalculationConfig>>,
    Json(payload): Json<HashMap<String, f64>>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // to do
    // 1. Validate variables against config.inputs

    // 2. Evaluate config.expression with payload values. handle errors

    // 3. Return JSON using key config.output . or return error?
}