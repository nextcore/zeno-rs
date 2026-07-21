use crate::executor::{Engine, InputMeta, SlotMeta};
use crate::scope::Value;
use std::collections::HashMap;
use std::sync::Arc;
use super::resolve_node_value;

pub fn register(engine: &mut Engine) {
    // ==========================================
    // MATH.ADD
    // ==========================================
    engine.register(
        "math.add",
        Arc::new(|engine, _ctx, node, scope| {
            let mut a = Value::Nil;
            let mut b = Value::Nil;
            let mut target = "math_result".to_string();

            if node.value.is_some() {
                a = resolve_node_value(engine, node, scope);
            }

            for c in &node.children {
                if c.name == "a" || c.name == "val" || c.name == "value" {
                    a = engine.resolve_shorthand_value(c, scope);
                }
                if c.name == "b" || c.name == "by" || c.name == "add" {
                    b = engine.resolve_shorthand_value(c, scope);
                }
                if c.name == "as" {
                    if let Some(ref cv) = c.value {
                        target = cv.trim_start_matches('$').to_string();
                    }
                }
            }

            let result = if matches!(a, Value::Float(_)) || matches!(b, Value::Float(_)) {
                Value::Float(a.to_float() + b.to_float())
            } else {
                Value::Int(a.to_int() + b.to_int())
            };

            scope.set(&target, result);
            Ok(())
        }),
        SlotMeta {
            description: "Add two numbers.".to_string(),
            example: "math.add: $count\n  by: 1\n  as: $count".to_string(),
            inputs: {
                let mut m = HashMap::new();
                m.insert("a".to_string(), InputMeta { description: "First number".to_string(), required: false, r#type: "number".to_string() });
                m.insert("b".to_string(), InputMeta { description: "Second number".to_string(), required: false, r#type: "number".to_string() });
                m.insert("by".to_string(), InputMeta { description: "Second number (alias)".to_string(), required: false, r#type: "number".to_string() });
                m.insert("as".to_string(), InputMeta { description: "Output variable".to_string(), required: false, r#type: "string".to_string() });
                m
            },
            required_blocks: Vec::new(),
            value_type: String::new(),
        },
    );

    // ==========================================
    // MATH.SUB
    // ==========================================
    engine.register(
        "math.sub",
        Arc::new(|engine, _ctx, node, scope| {
            let mut a = Value::Nil;
            let mut b = Value::Nil;
            let mut target = "math_result".to_string();

            if node.value.is_some() {
                a = resolve_node_value(engine, node, scope);
            }

            for c in &node.children {
                if c.name == "a" || c.name == "val" || c.name == "value" {
                    a = engine.resolve_shorthand_value(c, scope);
                }
                if c.name == "b" || c.name == "by" || c.name == "sub" {
                    b = engine.resolve_shorthand_value(c, scope);
                }
                if c.name == "as" {
                    if let Some(ref cv) = c.value {
                        target = cv.trim_start_matches('$').to_string();
                    }
                }
            }

            let result = if matches!(a, Value::Float(_)) || matches!(b, Value::Float(_)) {
                Value::Float(a.to_float() - b.to_float())
            } else {
                Value::Int(a.to_int() - b.to_int())
            };

            scope.set(&target, result);
            Ok(())
        }),
        SlotMeta {
            description: "Subtract two numbers.".to_string(),
            example: "math.sub: $total\n  by: $discount\n  as: $final_price".to_string(),
            inputs: {
                let mut m = HashMap::new();
                m.insert("a".to_string(), InputMeta { description: "First number".to_string(), required: false, r#type: "number".to_string() });
                m.insert("b".to_string(), InputMeta { description: "Second number".to_string(), required: false, r#type: "number".to_string() });
                m.insert("by".to_string(), InputMeta { description: "Second number (alias)".to_string(), required: false, r#type: "number".to_string() });
                m.insert("as".to_string(), InputMeta { description: "Output variable".to_string(), required: false, r#type: "string".to_string() });
                m
            },
            required_blocks: Vec::new(),
            value_type: String::new(),
        },
    );

    // ==========================================
    // MATH.MUL
    // ==========================================
    engine.register(
        "math.mul",
        Arc::new(|engine, _ctx, node, scope| {
            let mut a = Value::Nil;
            let mut b = Value::Nil;
            let mut target = "math_result".to_string();

            if node.value.is_some() {
                a = resolve_node_value(engine, node, scope);
            }

            for c in &node.children {
                if c.name == "a" || c.name == "val" || c.name == "value" {
                    a = engine.resolve_shorthand_value(c, scope);
                }
                if c.name == "b" || c.name == "by" || c.name == "mul" {
                    b = engine.resolve_shorthand_value(c, scope);
                }
                if c.name == "as" {
                    if let Some(ref cv) = c.value {
                        target = cv.trim_start_matches('$').to_string();
                    }
                }
            }

            let result = if matches!(a, Value::Float(_)) || matches!(b, Value::Float(_)) {
                Value::Float(a.to_float() * b.to_float())
            } else {
                Value::Int(a.to_int() * b.to_int())
            };

            scope.set(&target, result);
            Ok(())
        }),
        SlotMeta {
            description: "Multiply two numbers.".to_string(),
            example: "math.mul: $price\n  by: $quantity\n  as: $total".to_string(),
            inputs: {
                let mut m = HashMap::new();
                m.insert("a".to_string(), InputMeta { description: "First number".to_string(), required: false, r#type: "number".to_string() });
                m.insert("b".to_string(), InputMeta { description: "Second number".to_string(), required: false, r#type: "number".to_string() });
                m.insert("by".to_string(), InputMeta { description: "Second number (alias)".to_string(), required: false, r#type: "number".to_string() });
                m.insert("as".to_string(), InputMeta { description: "Output variable".to_string(), required: false, r#type: "string".to_string() });
                m
            },
            required_blocks: Vec::new(),
            value_type: String::new(),
        },
    );

    // ==========================================
    // MATH.DIV
    // ==========================================
    engine.register(
        "math.div",
        Arc::new(|engine, _ctx, node, scope| {
            let mut a = Value::Nil;
            let mut b = Value::Nil;
            let mut target = "math_result".to_string();

            if node.value.is_some() {
                a = resolve_node_value(engine, node, scope);
            }

            for c in &node.children {
                if c.name == "a" || c.name == "val" || c.name == "value" {
                    a = engine.resolve_shorthand_value(c, scope);
                }
                if c.name == "b" || c.name == "by" || c.name == "div" {
                    b = engine.resolve_shorthand_value(c, scope);
                }
                if c.name == "as" {
                    if let Some(ref cv) = c.value {
                        target = cv.trim_start_matches('$').to_string();
                    }
                }
            }

            let divisor = b.to_float();
            if divisor == 0.0 {
                return Err(crate::diagnostic::Diagnostic {
                    r#type: "error".to_string(),
                    message: "math.div: division by zero".to_string(),
                    filename: node.filename.clone(),
                    line: node.line,
                    col: node.col,
                    slot: Some("math.div".to_string()),
                });
            }

            // Return float result
            scope.set(&target, Value::Float(a.to_float() / divisor));
            Ok(())
        }),
        SlotMeta {
            description: "Divide two numbers. Returns a float. Errors on division by zero.".to_string(),
            example: "math.div: $total\n  by: $count\n  as: $average".to_string(),
            inputs: {
                let mut m = HashMap::new();
                m.insert("a".to_string(), InputMeta { description: "Dividend".to_string(), required: false, r#type: "number".to_string() });
                m.insert("b".to_string(), InputMeta { description: "Divisor".to_string(), required: false, r#type: "number".to_string() });
                m.insert("by".to_string(), InputMeta { description: "Divisor (alias)".to_string(), required: false, r#type: "number".to_string() });
                m.insert("as".to_string(), InputMeta { description: "Output variable".to_string(), required: false, r#type: "string".to_string() });
                m
            },
            required_blocks: Vec::new(),
            value_type: String::new(),
        },
    );

    // ==========================================
    // MATH.MOD
    // ==========================================
    engine.register(
        "math.mod",
        Arc::new(|engine, _ctx, node, scope| {
            let mut a = Value::Nil;
            let mut b = Value::Nil;
            let mut target = "math_result".to_string();

            if node.value.is_some() {
                a = resolve_node_value(engine, node, scope);
            }

            for c in &node.children {
                if c.name == "a" || c.name == "val" || c.name == "value" {
                    a = engine.resolve_shorthand_value(c, scope);
                }
                if c.name == "b" || c.name == "by" || c.name == "mod" {
                    b = engine.resolve_shorthand_value(c, scope);
                }
                if c.name == "as" {
                    if let Some(ref cv) = c.value {
                        target = cv.trim_start_matches('$').to_string();
                    }
                }
            }

            let divisor = b.to_int();
            if divisor == 0 {
                return Err(crate::diagnostic::Diagnostic {
                    r#type: "error".to_string(),
                    message: "math.mod: modulo by zero".to_string(),
                    filename: node.filename.clone(),
                    line: node.line,
                    col: node.col,
                    slot: Some("math.mod".to_string()),
                });
            }

            scope.set(&target, Value::Int(a.to_int() % divisor));
            Ok(())
        }),
        SlotMeta {
            description: "Get the remainder of integer division.".to_string(),
            example: "math.mod: $index\n  by: 2\n  as: $remainder".to_string(),
            inputs: {
                let mut m = HashMap::new();
                m.insert("a".to_string(), InputMeta { description: "Dividend".to_string(), required: false, r#type: "int".to_string() });
                m.insert("b".to_string(), InputMeta { description: "Divisor".to_string(), required: false, r#type: "int".to_string() });
                m.insert("by".to_string(), InputMeta { description: "Divisor (alias)".to_string(), required: false, r#type: "int".to_string() });
                m.insert("as".to_string(), InputMeta { description: "Output variable".to_string(), required: false, r#type: "string".to_string() });
                m
            },
            required_blocks: Vec::new(),
            value_type: String::new(),
        },
    );

    // ==========================================
    // MATH.POW
    // ==========================================
    engine.register(
        "math.pow",
        Arc::new(|engine, _ctx, node, scope| {
            let mut base = Value::Nil;
            let mut exp = Value::Nil;
            let mut target = "math_result".to_string();

            if node.value.is_some() {
                base = resolve_node_value(engine, node, scope);
            }

            for c in &node.children {
                if c.name == "base" || c.name == "val" || c.name == "a" {
                    base = engine.resolve_shorthand_value(c, scope);
                }
                if c.name == "exp" || c.name == "exponent" || c.name == "b" {
                    exp = engine.resolve_shorthand_value(c, scope);
                }
                if c.name == "as" {
                    if let Some(ref cv) = c.value {
                        target = cv.trim_start_matches('$').to_string();
                    }
                }
            }

            scope.set(&target, Value::Float(base.to_float().powf(exp.to_float())));
            Ok(())
        }),
        SlotMeta {
            description: "Raise base to the power of exponent.".to_string(),
            example: "math.pow: 2\n  exp: 10\n  as: $result".to_string(),
            inputs: {
                let mut m = HashMap::new();
                m.insert("base".to_string(), InputMeta { description: "Base".to_string(), required: false, r#type: "number".to_string() });
                m.insert("exp".to_string(), InputMeta { description: "Exponent".to_string(), required: false, r#type: "number".to_string() });
                m.insert("as".to_string(), InputMeta { description: "Output variable".to_string(), required: false, r#type: "string".to_string() });
                m
            },
            required_blocks: Vec::new(),
            value_type: String::new(),
        },
    );

    // ==========================================
    // MATH.SQRT
    // ==========================================
    engine.register(
        "math.sqrt",
        Arc::new(|engine, _ctx, node, scope| {
            let mut val = Value::Nil;
            let mut target = "sqrt_result".to_string();

            if node.value.is_some() {
                val = resolve_node_value(engine, node, scope);
            }

            for c in &node.children {
                if c.name == "val" || c.name == "value" {
                    val = engine.resolve_shorthand_value(c, scope);
                }
                if c.name == "as" {
                    if let Some(ref cv) = c.value {
                        target = cv.trim_start_matches('$').to_string();
                    }
                }
            }

            scope.set(&target, Value::Float(val.to_float().sqrt()));
            Ok(())
        }),
        SlotMeta {
            description: "Compute the square root of a number.".to_string(),
            example: "math.sqrt: 144\n  as: $root".to_string(),
            inputs: {
                let mut m = HashMap::new();
                m.insert("val".to_string(), InputMeta { description: "Input number".to_string(), required: false, r#type: "number".to_string() });
                m.insert("as".to_string(), InputMeta { description: "Output variable".to_string(), required: false, r#type: "string".to_string() });
                m
            },
            required_blocks: Vec::new(),
            value_type: String::new(),
        },
    );

    // ==========================================
    // MATH.ABS
    // ==========================================
    engine.register(
        "math.abs",
        Arc::new(|engine, _ctx, node, scope| {
            let mut val = Value::Nil;
            let mut target = "abs_result".to_string();

            if node.value.is_some() {
                val = resolve_node_value(engine, node, scope);
            }

            for c in &node.children {
                if c.name == "val" || c.name == "value" {
                    val = engine.resolve_shorthand_value(c, scope);
                }
                if c.name == "as" {
                    if let Some(ref cv) = c.value {
                        target = cv.trim_start_matches('$').to_string();
                    }
                }
            }

            let result = match val {
                Value::Int(i) => Value::Int(i.abs()),
                _ => Value::Float(val.to_float().abs()),
            };

            scope.set(&target, result);
            Ok(())
        }),
        SlotMeta {
            description: "Compute the absolute value of a number.".to_string(),
            example: "math.abs: $temperature\n  as: $magnitude".to_string(),
            inputs: {
                let mut m = HashMap::new();
                m.insert("val".to_string(), InputMeta { description: "Input number".to_string(), required: false, r#type: "number".to_string() });
                m.insert("as".to_string(), InputMeta { description: "Output variable".to_string(), required: false, r#type: "string".to_string() });
                m
            },
            required_blocks: Vec::new(),
            value_type: String::new(),
        },
    );

    // ==========================================
    // MATH.CEIL
    // ==========================================
    engine.register(
        "math.ceil",
        Arc::new(|engine, _ctx, node, scope| {
            let mut val = Value::Nil;
            let mut target = "ceil_result".to_string();

            if node.value.is_some() {
                val = resolve_node_value(engine, node, scope);
            }

            for c in &node.children {
                if c.name == "val" || c.name == "value" {
                    val = engine.resolve_shorthand_value(c, scope);
                }
                if c.name == "as" {
                    if let Some(ref cv) = c.value {
                        target = cv.trim_start_matches('$').to_string();
                    }
                }
            }

            scope.set(&target, Value::Int(val.to_float().ceil() as i64));
            Ok(())
        }),
        SlotMeta {
            description: "Round a float up to the nearest integer.".to_string(),
            example: "math.ceil: 4.1\n  as: $rounded_up".to_string(),
            inputs: {
                let mut m = HashMap::new();
                m.insert("val".to_string(), InputMeta { description: "Input number".to_string(), required: false, r#type: "number".to_string() });
                m.insert("as".to_string(), InputMeta { description: "Output variable".to_string(), required: false, r#type: "string".to_string() });
                m
            },
            required_blocks: Vec::new(),
            value_type: String::new(),
        },
    );

    // ==========================================
    // MATH.FLOOR
    // ==========================================
    engine.register(
        "math.floor",
        Arc::new(|engine, _ctx, node, scope| {
            let mut val = Value::Nil;
            let mut target = "floor_result".to_string();

            if node.value.is_some() {
                val = resolve_node_value(engine, node, scope);
            }

            for c in &node.children {
                if c.name == "val" || c.name == "value" {
                    val = engine.resolve_shorthand_value(c, scope);
                }
                if c.name == "as" {
                    if let Some(ref cv) = c.value {
                        target = cv.trim_start_matches('$').to_string();
                    }
                }
            }

            scope.set(&target, Value::Int(val.to_float().floor() as i64));
            Ok(())
        }),
        SlotMeta {
            description: "Round a float down to the nearest integer.".to_string(),
            example: "math.floor: 4.9\n  as: $rounded_down".to_string(),
            inputs: {
                let mut m = HashMap::new();
                m.insert("val".to_string(), InputMeta { description: "Input number".to_string(), required: false, r#type: "number".to_string() });
                m.insert("as".to_string(), InputMeta { description: "Output variable".to_string(), required: false, r#type: "string".to_string() });
                m
            },
            required_blocks: Vec::new(),
            value_type: String::new(),
        },
    );

    // ==========================================
    // MATH.ROUND
    // ==========================================
    engine.register(
        "math.round",
        Arc::new(|engine, _ctx, node, scope| {
            let mut val = Value::Nil;
            let mut decimals: u32 = 0;
            let mut target = "round_result".to_string();

            if node.value.is_some() {
                val = resolve_node_value(engine, node, scope);
            }

            for c in &node.children {
                if c.name == "val" || c.name == "value" {
                    val = engine.resolve_shorthand_value(c, scope);
                }
                if c.name == "decimals" || c.name == "precision" {
                    decimals = engine.resolve_shorthand_value(c, scope).to_int().max(0) as u32;
                }
                if c.name == "as" {
                    if let Some(ref cv) = c.value {
                        target = cv.trim_start_matches('$').to_string();
                    }
                }
            }

            let f = val.to_float();
            if decimals == 0 {
                scope.set(&target, Value::Int(f.round() as i64));
            } else {
                let factor = 10f64.powi(decimals as i32);
                let rounded = (f * factor).round() / factor;
                scope.set(&target, Value::Float(rounded));
            }

            Ok(())
        }),
        SlotMeta {
            description: "Round a float to the nearest integer or to N decimal places.".to_string(),
            example: "math.round: $price\n  decimals: 2\n  as: $rounded_price".to_string(),
            inputs: {
                let mut m = HashMap::new();
                m.insert("val".to_string(), InputMeta { description: "Input number".to_string(), required: false, r#type: "number".to_string() });
                m.insert("decimals".to_string(), InputMeta { description: "Decimal places".to_string(), required: false, r#type: "int".to_string() });
                m.insert("as".to_string(), InputMeta { description: "Output variable".to_string(), required: false, r#type: "string".to_string() });
                m
            },
            required_blocks: Vec::new(),
            value_type: String::new(),
        },
    );

    // ==========================================
    // MATH.MIN
    // ==========================================
    engine.register(
        "math.min",
        Arc::new(|engine, _ctx, node, scope| {
            let mut a = Value::Nil;
            let mut b = Value::Nil;
            let mut target = "min_result".to_string();

            if node.value.is_some() {
                a = resolve_node_value(engine, node, scope);
            }

            for c in &node.children {
                if c.name == "a" || c.name == "val" || c.name == "value" {
                    a = engine.resolve_shorthand_value(c, scope);
                }
                if c.name == "b" {
                    b = engine.resolve_shorthand_value(c, scope);
                }
                if c.name == "as" {
                    if let Some(ref cv) = c.value {
                        target = cv.trim_start_matches('$').to_string();
                    }
                }
            }

            let result = if a.to_float() <= b.to_float() { a } else { b };
            scope.set(&target, result);
            Ok(())
        }),
        SlotMeta {
            description: "Return the minimum of two numbers.".to_string(),
            example: "math.min: $a\n  b: $b\n  as: $minimum".to_string(),
            inputs: {
                let mut m = HashMap::new();
                m.insert("a".to_string(), InputMeta { description: "First number".to_string(), required: false, r#type: "number".to_string() });
                m.insert("b".to_string(), InputMeta { description: "Second number".to_string(), required: false, r#type: "number".to_string() });
                m.insert("as".to_string(), InputMeta { description: "Output variable".to_string(), required: false, r#type: "string".to_string() });
                m
            },
            required_blocks: Vec::new(),
            value_type: String::new(),
        },
    );

    // ==========================================
    // MATH.MAX
    // ==========================================
    engine.register(
        "math.max",
        Arc::new(|engine, _ctx, node, scope| {
            let mut a = Value::Nil;
            let mut b = Value::Nil;
            let mut target = "max_result".to_string();

            if node.value.is_some() {
                a = resolve_node_value(engine, node, scope);
            }

            for c in &node.children {
                if c.name == "a" || c.name == "val" || c.name == "value" {
                    a = engine.resolve_shorthand_value(c, scope);
                }
                if c.name == "b" {
                    b = engine.resolve_shorthand_value(c, scope);
                }
                if c.name == "as" {
                    if let Some(ref cv) = c.value {
                        target = cv.trim_start_matches('$').to_string();
                    }
                }
            }

            let result = if a.to_float() >= b.to_float() { a } else { b };
            scope.set(&target, result);
            Ok(())
        }),
        SlotMeta {
            description: "Return the maximum of two numbers.".to_string(),
            example: "math.max: $score\n  b: 100\n  as: $capped_score".to_string(),
            inputs: {
                let mut m = HashMap::new();
                m.insert("a".to_string(), InputMeta { description: "First number".to_string(), required: false, r#type: "number".to_string() });
                m.insert("b".to_string(), InputMeta { description: "Second number".to_string(), required: false, r#type: "number".to_string() });
                m.insert("as".to_string(), InputMeta { description: "Output variable".to_string(), required: false, r#type: "string".to_string() });
                m
            },
            required_blocks: Vec::new(),
            value_type: String::new(),
        },
    );

    // ==========================================
    // MATH.CLAMP
    // ==========================================
    engine.register(
        "math.clamp",
        Arc::new(|engine, _ctx, node, scope| {
            let mut val = Value::Nil;
            let mut min_val = f64::NEG_INFINITY;
            let mut max_val = f64::INFINITY;
            let mut target = "clamp_result".to_string();

            if node.value.is_some() {
                val = resolve_node_value(engine, node, scope);
            }

            for c in &node.children {
                if c.name == "val" || c.name == "value" {
                    val = engine.resolve_shorthand_value(c, scope);
                }
                if c.name == "min" {
                    min_val = engine.resolve_shorthand_value(c, scope).to_float();
                }
                if c.name == "max" {
                    max_val = engine.resolve_shorthand_value(c, scope).to_float();
                }
                if c.name == "as" {
                    if let Some(ref cv) = c.value {
                        target = cv.trim_start_matches('$').to_string();
                    }
                }
            }

            let clamped = val.to_float().clamp(min_val, max_val);
            // Return int if result is a whole number and input was int
            if matches!(val, Value::Int(_)) && clamped == clamped as i64 as f64 {
                scope.set(&target, Value::Int(clamped as i64));
            } else {
                scope.set(&target, Value::Float(clamped));
            }
            Ok(())
        }),
        SlotMeta {
            description: "Clamp a number between min and max values.".to_string(),
            example: "math.clamp: $value\n  min: 0\n  max: 100\n  as: $clamped".to_string(),
            inputs: {
                let mut m = HashMap::new();
                m.insert("val".to_string(), InputMeta { description: "Input number".to_string(), required: false, r#type: "number".to_string() });
                m.insert("min".to_string(), InputMeta { description: "Minimum value".to_string(), required: false, r#type: "number".to_string() });
                m.insert("max".to_string(), InputMeta { description: "Maximum value".to_string(), required: false, r#type: "number".to_string() });
                m.insert("as".to_string(), InputMeta { description: "Output variable".to_string(), required: false, r#type: "string".to_string() });
                m
            },
            required_blocks: Vec::new(),
            value_type: String::new(),
        },
    );

    // ==========================================
    // MATH.RANDOM
    // ==========================================
    engine.register(
        "math.random",
        Arc::new(|engine, _ctx, node, scope| {
            let mut min: i64 = 0;
            let mut max: i64 = 100;
            let mut target = "random_result".to_string();

            for c in &node.children {
                if c.name == "min" || c.name == "from" {
                    min = engine.resolve_shorthand_value(c, scope).to_int();
                }
                if c.name == "max" || c.name == "to" {
                    max = engine.resolve_shorthand_value(c, scope).to_int();
                }
                if c.name == "as" {
                    if let Some(ref cv) = c.value {
                        target = cv.trim_start_matches('$').to_string();
                    }
                }
            }

            if max <= min {
                scope.set(&target, Value::Int(min));
                return Ok(());
            }

            // Simple pseudo-random using system time nanos
            let nanos = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.subsec_nanos())
                .unwrap_or(42) as i64;

            let range = max - min + 1;
            let rand_val = min + (nanos.abs() % range);
            scope.set(&target, Value::Int(rand_val));
            Ok(())
        }),
        SlotMeta {
            description: "Generate a pseudo-random integer between min and max (inclusive).".to_string(),
            example: "math.random {\n  min: 1\n  max: 6\n  as: $dice_roll\n}".to_string(),
            inputs: {
                let mut m = HashMap::new();
                m.insert("min".to_string(), InputMeta { description: "Minimum value (default: 0)".to_string(), required: false, r#type: "int".to_string() });
                m.insert("max".to_string(), InputMeta { description: "Maximum value (default: 100)".to_string(), required: false, r#type: "int".to_string() });
                m.insert("as".to_string(), InputMeta { description: "Output variable".to_string(), required: false, r#type: "string".to_string() });
                m
            },
            required_blocks: Vec::new(),
            value_type: String::new(),
        },
    );
}
