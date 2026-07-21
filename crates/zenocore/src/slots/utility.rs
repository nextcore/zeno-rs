use crate::diagnostic::Diagnostic;
use crate::executor::{Engine, InputMeta, SlotMeta};
use crate::parser::Node;
use crate::scope::{Scope, Value};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use super::{resolve_node_value, parse_json, serialize_json, parse_duration, FunctionRegistry};

pub fn register(engine: &mut Engine) {
    // ==========================================
    // FN (Define Function)
    // ==========================================
    engine.register(
        "fn",
        Arc::new(|_engine, ctx, node, _scope| {
            let func_name = node.value.clone().unwrap_or_default();
            let func_clean = func_name.trim_start_matches('$').to_string();
            if func_clean.is_empty() {
                return Err(Diagnostic {
                    r#type: "error".to_string(),
                    message: "fn: function name is required".to_string(),
                    filename: node.filename.clone(),
                    line: node.line,
                    col: node.col,
                    slot: Some("fn".to_string()),
                });
            }

            let registry = if let Some(reg) = ctx.get::<FunctionRegistry>("functions") {
                reg
            } else {
                ctx.set("functions", FunctionRegistry { functions: Mutex::new(HashMap::new()) });
                ctx.get::<FunctionRegistry>("functions").ok_or_else(|| Diagnostic {
                    r#type: "error".to_string(),
                    message: "fn: failed to initialize function registry".to_string(),
                    filename: node.filename.clone(),
                    line: node.line,
                    col: node.col,
                    slot: Some("fn".to_string()),
                })?
            };

            registry.functions.lock().unwrap().insert(func_clean, node.clone());
            Ok(())
        }),
        SlotMeta {
            description: "Define a reusable function code block.".to_string(),
            example: "fn: hitung_gaji {\n  log: $gaji\n}".to_string(),
            inputs: HashMap::new(),
            required_blocks: Vec::new(),
            value_type: String::new(),
        },
    );

    // ==========================================
    // CALL (Invoke Function)
    // ==========================================
    engine.register(
        "call",
        Arc::new(|engine, ctx, node, scope| {
            let func_name = node.value.clone().unwrap_or_default();
            let func_clean = func_name.trim_start_matches('$').to_string();
            if func_clean.is_empty() {
                return Err(Diagnostic {
                    r#type: "error".to_string(),
                    message: "call: function name is required".to_string(),
                    filename: node.filename.clone(),
                    line: node.line,
                    col: node.col,
                    slot: Some("call".to_string()),
                });
            }

            let registry = ctx.get::<FunctionRegistry>("functions").ok_or_else(|| Diagnostic {
                r#type: "error".to_string(),
                message: format!("call: function '{}' not found (no functions registered)", func_clean),
                filename: node.filename.clone(),
                line: node.line,
                col: node.col,
                slot: Some("call".to_string()),
            })?;

            let func_node = {
                let funcs = registry.functions.lock().unwrap();
                funcs.get(&func_clean).cloned().ok_or_else(|| Diagnostic {
                    r#type: "error".to_string(),
                    message: format!("call: function '{}' not found", func_clean),
                    filename: node.filename.clone(),
                    line: node.line,
                    col: node.col,
                    slot: Some("call".to_string()),
                })?
            };

            for child in &func_node.children {
                engine.execute(ctx, child, scope)?;
            }

            Ok(())
        }),
        SlotMeta {
            description: "Call a registered function code block.".to_string(),
            example: "call: hitung_gaji".to_string(),
            inputs: HashMap::new(),
            required_blocks: Vec::new(),
            value_type: String::new(),
        },
    );

    // ==========================================
    // JSON.PARSE
    // ==========================================
    engine.register(
        "json.parse",
        Arc::new(|engine, _ctx, node, scope| {
            let mut json_str = String::new();
            let mut target = "json_result".to_string();

            if node.value.is_some() {
                json_str = resolve_node_value(engine, node, scope).to_string_coerce();
            }

            for c in &node.children {
                if c.name == "val" || c.name == "value" {
                    json_str = engine.resolve_shorthand_value(c, scope).to_string_coerce();
                }
                if c.name == "as" {
                    if let Some(ref cv) = c.value {
                        target = cv.trim_start_matches('$').to_string();
                    }
                }
            }

            if json_str.is_empty() {
                return Err(Diagnostic {
                    r#type: "error".to_string(),
                    message: "json.parse: input value is empty".to_string(),
                    filename: node.filename.clone(),
                    line: node.line,
                    col: node.col,
                    slot: Some("json.parse".to_string()),
                });
            }

            let val = parse_json(&json_str).map_err(|e| Diagnostic {
                r#type: "error".to_string(),
                message: format!("json.parse: invalid json format: {}", e),
                filename: node.filename.clone(),
                line: node.line,
                col: node.col,
                slot: Some("json.parse".to_string()),
            })?;

            scope.set(&target, val);
            Ok(())
        }),
        SlotMeta {
            description: "Parse a JSON string into a structured Value.".to_string(),
            example: "json.parse: $response_body\n  as: $data".to_string(),
            inputs: {
                let mut m = HashMap::new();
                m.insert("val".to_string(), InputMeta { description: "JSON string".to_string(), required: false, r#type: "string".to_string() });
                m.insert("value".to_string(), InputMeta { description: "JSON string".to_string(), required: false, r#type: "string".to_string() });
                m.insert("as".to_string(), InputMeta { description: "Target variable".to_string(), required: false, r#type: "any".to_string() });
                m
            },
            required_blocks: Vec::new(),
            value_type: String::new(),
        },
    );

    // ==========================================
    // JSON.STRINGIFY
    // ==========================================
    engine.register(
        "json.stringify",
        Arc::new(|engine, _ctx, node, scope| {
            let mut val = Value::Nil;
            let mut target = "json_string".to_string();

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

            let serialized = serialize_json(&val);
            scope.set(&target, Value::String(serialized));
            Ok(())
        }),
        SlotMeta {
            description: "Serialize a structured Value into a JSON string.".to_string(),
            example: "json.stringify: $data\n  as: $json_str".to_string(),
            inputs: {
                let mut m = HashMap::new();
                m.insert("val".to_string(), InputMeta { description: "Value to serialize".to_string(), required: false, r#type: "any".to_string() });
                m.insert("value".to_string(), InputMeta { description: "Value to serialize".to_string(), required: false, r#type: "any".to_string() });
                m.insert("as".to_string(), InputMeta { description: "Target variable".to_string(), required: false, r#type: "any".to_string() });
                m
            },
            required_blocks: Vec::new(),
            value_type: String::new(),
        },
    );

    // ==========================================
    // TIME.SLEEP
    // ==========================================
    engine.register(
        "time.sleep",
        Arc::new(|engine, _ctx, node, scope| {
            let mut duration_str = String::new();

            if node.value.is_some() {
                duration_str = resolve_node_value(engine, node, scope).to_string_coerce();
            }

            for c in &node.children {
                if c.name == "duration" || c.name == "val" {
                    duration_str = engine.resolve_shorthand_value(c, scope).to_string_coerce();
                }
            }

            if duration_str.is_empty() {
                return Err(Diagnostic {
                    r#type: "error".to_string(),
                    message: "time.sleep: duration is required".to_string(),
                    filename: node.filename.clone(),
                    line: node.line,
                    col: node.col,
                    slot: Some("time.sleep".to_string()),
                });
            }

            let dur = parse_duration(&duration_str).ok_or_else(|| Diagnostic {
                r#type: "error".to_string(),
                message: format!("time.sleep: invalid duration format '{}'", duration_str),
                filename: node.filename.clone(),
                line: node.line,
                col: node.col,
                slot: Some("time.sleep".to_string()),
            })?;

            std::thread::sleep(dur);
            Ok(())
        }),
        SlotMeta {
            description: "Pause execution for a duration.".to_string(),
            example: "time.sleep: '1s'".to_string(),
            inputs: {
                let mut m = HashMap::new();
                m.insert("duration".to_string(), InputMeta { description: "Duration string (e.g. 1s, 500ms)".to_string(), required: false, r#type: "string".to_string() });
                m.insert("val".to_string(), InputMeta { description: "Duration string (e.g. 1s, 500ms)".to_string(), required: false, r#type: "string".to_string() });
                m
            },
            required_blocks: Vec::new(),
            value_type: String::new(),
        },
    );
    // ==========================================
    // IF (conditional branch with then/else)
    // ==========================================
    engine.register(
        "if",
        Arc::new(|engine, ctx, node, scope| {
            let cond_val = if let Some(ref val_str) = node.value {
                evaluate_condition(engine, val_str, scope)
            } else {
                false
            };

            let mut then_node: Option<&Node> = None;
            let mut else_node: Option<&Node> = None;

            for child in &node.children {
                if child.name == "then" {
                    then_node = Some(child);
                } else if child.name == "else" {
                    else_node = Some(child);
                }
            }

            if cond_val {
                if let Some(then_n) = then_node {
                    for child in &then_n.children {
                        engine.execute(ctx, child, scope)?;
                    }
                }
            } else if let Some(else_n) = else_node {
                for child in &else_n.children {
                    engine.execute(ctx, child, scope)?;
                }
            }

            Ok(())
        }),
        SlotMeta {
            description: "Conditional branch. Supports `&&`, `||`, `==`, `!=`, `>`, `<`, `>=`, `<=`.".to_string(),
            example: "if: '$role == admin' {\n  then: { log: 'welcome admin' }\n  else: { log: 'access denied' }\n}".to_string(),
            inputs: {
                let mut m = HashMap::new();
                m.insert("then".to_string(), InputMeta { description: "Execute if condition is true".to_string(), required: false, r#type: "any".to_string() });
                m.insert("else".to_string(), InputMeta { description: "Execute if condition is false".to_string(), required: false, r#type: "any".to_string() });
                m
            },
            required_blocks: Vec::new(),
            value_type: String::new(),
        },
    );

    // ==========================================
    // LOG
    // ==========================================
    engine.register(
        "log",
        Arc::new(|engine, _ctx, node, scope| {
            let val = if node.value.is_some() {
                resolve_node_value(engine, node, scope)
            } else {
                let mut map = HashMap::new();
                for child in &node.children {
                    let child_val = engine.resolve_shorthand_value(child, scope);
                    map.insert(child.name.clone(), child_val);
                }
                if map.is_empty() {
                    Value::Nil
                } else {
                    Value::Map(map)
                }
            };
            println!("[ZenoLang Log] {}", val.to_string_coerce());
            Ok(())
        }),
        SlotMeta {
            description: "Print a value to stdout for debugging purposes.".to_string(),
            example: "log: $message".to_string(),
            inputs: HashMap::new(),
            required_blocks: Vec::new(),
            value_type: String::new(),
        },
    );

    // ==========================================
    // CAST SLOTS
    // ==========================================
    engine.register(
        "cast.to_int",
        Arc::new(|engine, _ctx, node, scope| {
            let mut val = Value::Nil;
            let mut target = "cast_result".to_string();

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

            scope.set(&target, Value::Int(val.to_int()));
            Ok(())
        }),
        SlotMeta {
            description: "Cast a value to an integer.".to_string(),
            example: "cast.to_int: $price_str\n  as: $price".to_string(),
            inputs: {
                let mut m = HashMap::new();
                m.insert("val".to_string(), InputMeta { description: "Input value".to_string(), required: false, r#type: "any".to_string() });
                m.insert("as".to_string(), InputMeta { description: "Output variable".to_string(), required: false, r#type: "string".to_string() });
                m
            },
            required_blocks: Vec::new(),
            value_type: String::new(),
        },
    );

    engine.register(
        "cast.to_float",
        Arc::new(|engine, _ctx, node, scope| {
            let mut val = Value::Nil;
            let mut target = "cast_result".to_string();

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

            scope.set(&target, Value::Float(val.to_float()));
            Ok(())
        }),
        SlotMeta {
            description: "Cast a value to a float.".to_string(),
            example: "cast.to_float: $count\n  as: $float_count".to_string(),
            inputs: {
                let mut m = HashMap::new();
                m.insert("val".to_string(), InputMeta { description: "Input value".to_string(), required: false, r#type: "any".to_string() });
                m.insert("as".to_string(), InputMeta { description: "Output variable".to_string(), required: false, r#type: "string".to_string() });
                m
            },
            required_blocks: Vec::new(),
            value_type: String::new(),
        },
    );

    engine.register(
        "cast.to_string",
        Arc::new(|engine, _ctx, node, scope| {
            let mut val = Value::Nil;
            let mut target = "cast_result".to_string();

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

            scope.set(&target, Value::String(val.to_string_coerce()));
            Ok(())
        }),
        SlotMeta {
            description: "Cast a value to a string.".to_string(),
            example: "cast.to_string: $count\n  as: $count_str".to_string(),
            inputs: {
                let mut m = HashMap::new();
                m.insert("val".to_string(), InputMeta { description: "Input value".to_string(), required: false, r#type: "any".to_string() });
                m.insert("as".to_string(), InputMeta { description: "Output variable".to_string(), required: false, r#type: "string".to_string() });
                m
            },
            required_blocks: Vec::new(),
            value_type: String::new(),
        },
    );

    engine.register(
        "cast.to_bool",
        Arc::new(|engine, _ctx, node, scope| {
            let mut val = Value::Nil;
            let mut target = "cast_result".to_string();

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

            scope.set(&target, Value::Bool(val.to_bool()));
            Ok(())
        }),
        SlotMeta {
            description: "Cast a value to a boolean.".to_string(),
            example: "cast.to_bool: $flag_str\n  as: $is_enabled".to_string(),
            inputs: {
                let mut m = HashMap::new();
                m.insert("val".to_string(), InputMeta { description: "Input value".to_string(), required: false, r#type: "any".to_string() });
                m.insert("as".to_string(), InputMeta { description: "Output variable".to_string(), required: false, r#type: "string".to_string() });
                m
            },
            required_blocks: Vec::new(),
            value_type: String::new(),
        },
    );

    // ==========================================
    // COALESCE (null coalescing with explicit slot)
    // ==========================================
    engine.register(
        "coalesce",
        Arc::new(|engine, _ctx, node, scope| {
            let mut val = Value::Nil;
            let mut def = Value::Nil;
            let mut target = "coalesce_result".to_string();

            if node.value.is_some() {
                val = resolve_node_value(engine, node, scope);
            }

            for c in &node.children {
                if c.name == "val" || c.name == "value" {
                    val = engine.resolve_shorthand_value(c, scope);
                }
                if c.name == "default" || c.name == "def" || c.name == "else" {
                    def = engine.resolve_shorthand_value(c, scope);
                }
                if c.name == "as" {
                    if let Some(ref cv) = c.value {
                        target = cv.trim_start_matches('$').to_string();
                    }
                }
            }

            let is_nil = match &val {
                Value::Nil => true,
                Value::String(s) => s.is_empty(),
                _ => false,
            };

            let result = if is_nil { def } else { val };
            scope.set(&target, result);
            Ok(())
        }),
        SlotMeta {
            description: "Return val if it's non-nil/non-empty, otherwise return default.".to_string(),
            example: "coalesce: $name\n  default: 'Anonymous'\n  as: $display_name".to_string(),
            inputs: {
                let mut m = HashMap::new();
                m.insert("val".to_string(), InputMeta { description: "Primary value".to_string(), required: false, r#type: "any".to_string() });
                m.insert("default".to_string(), InputMeta { description: "Fallback value".to_string(), required: false, r#type: "any".to_string() });
                m.insert("def".to_string(), InputMeta { description: "Fallback value (alias)".to_string(), required: false, r#type: "any".to_string() });
                m.insert("as".to_string(), InputMeta { description: "Output variable".to_string(), required: false, r#type: "string".to_string() });
                m
            },
            required_blocks: Vec::new(),
            value_type: String::new(),
        },
    );

    // ==========================================
    // INCLUDE (execute an external .zl file)
    // ==========================================
    engine.register(
        "include",
        Arc::new(|engine, ctx, node, scope| {
            let path = resolve_node_value(engine, node, scope).to_string_coerce();
            if path.is_empty() {
                return Err(Diagnostic {
                    r#type: "error".to_string(),
                    message: "include: file path is required".to_string(),
                    filename: node.filename.clone(),
                    line: node.line,
                    col: node.col,
                    slot: Some("include".to_string()),
                });
            }

            let content = std::fs::read_to_string(&path).map_err(|e| Diagnostic {
                r#type: "error".to_string(),
                message: format!("include: failed to read '{}': {}", path, e),
                filename: node.filename.clone(),
                line: node.line,
                col: node.col,
                slot: Some("include".to_string()),
            })?;

            let parsed_node = crate::parser::parse_string(&content, &path).map_err(|e| Diagnostic {
                r#type: "error".to_string(),
                message: format!("include: failed to parse '{}': {:?}", path, e),
                filename: node.filename.clone(),
                line: node.line,
                col: node.col,
                slot: Some("include".to_string()),
            })?;

            engine.execute(ctx, &parsed_node, scope)
        }),
        SlotMeta {
            description: "Include and execute another ZenoLang file.".to_string(),
            example: "include: 'helpers/auth.zl'".to_string(),
            inputs: HashMap::new(),
            required_blocks: Vec::new(),
            value_type: String::new(),
        },
    );

    // ==========================================
    // UTIL.DATETIME
    // ==========================================
    engine.register(
        "util.datetime",
        Arc::new(|engine, _ctx, node, scope| {
            let mut format = "%Y-%m-%d %H:%M:%S".to_string();
            let mut target = "datetime_result".to_string();

            for c in &node.children {
                if c.name == "format" || c.name == "fmt" {
                    format = engine.resolve_shorthand_value(c, scope).to_string_coerce();
                }
                if c.name == "as" {
                    if let Some(ref cv) = c.value {
                        target = cv.trim_start_matches('$').to_string();
                    }
                }
            }

            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default();

            // Simple formatted datetime using unix timestamp arithmetic
            let ts = now.as_secs();
            let secs = ts % 60;
            let mins = (ts / 60) % 60;
            let hours = (ts / 3600) % 24;
            let days_since_epoch = ts / 86400;
            // Simplified gregorian calendar calculation
            let (year, month, day) = days_to_ymd(days_since_epoch);

            let formatted = format
                .replace("%Y", &format!("{:04}", year))
                .replace("%m", &format!("{:02}", month))
                .replace("%d", &format!("{:02}", day))
                .replace("%H", &format!("{:02}", hours))
                .replace("%M", &format!("{:02}", mins))
                .replace("%S", &format!("{:02}", secs));

            scope.set(&target, Value::String(formatted));
            Ok(())
        }),
        SlotMeta {
            description: "Get current datetime. Format: %Y=year, %m=month, %d=day, %H=hour, %M=min, %S=sec.".to_string(),
            example: "util.datetime {\n  format: '%Y-%m-%d'\n  as: $today\n}".to_string(),
            inputs: {
                let mut m = HashMap::new();
                m.insert("format".to_string(), InputMeta { description: "strftime format string".to_string(), required: false, r#type: "string".to_string() });
                m.insert("as".to_string(), InputMeta { description: "Output variable".to_string(), required: false, r#type: "string".to_string() });
                m
            },
            required_blocks: Vec::new(),
            value_type: String::new(),
        },
    );

    // ==========================================
    // UTIL.TIMESTAMP
    // ==========================================
    engine.register(
        "util.timestamp",
        Arc::new(|_engine, _ctx, node, scope| {
            let mut target = "timestamp".to_string();

            for c in &node.children {
                if c.name == "as" {
                    if let Some(ref cv) = c.value {
                        target = cv.trim_start_matches('$').to_string();
                    }
                }
            }

            let ts = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0);

            scope.set(&target, Value::Int(ts as i64));
            Ok(())
        }),
        SlotMeta {
            description: "Get current Unix timestamp (seconds since epoch).".to_string(),
            example: "util.timestamp {\n  as: $ts\n}".to_string(),
            inputs: {
                let mut m = HashMap::new();
                m.insert("as".to_string(), InputMeta { description: "Output variable".to_string(), required: false, r#type: "string".to_string() });
                m
            },
            required_blocks: Vec::new(),
            value_type: String::new(),
        },
    );

    // ==========================================
    // UTIL.UUID (Generate a UUID v4-like string)
    // ==========================================
    engine.register(
        "util.uuid",
        Arc::new(|_engine, _ctx, node, scope| {
            let mut target = "uuid".to_string();

            for c in &node.children {
                if c.name == "as" {
                    if let Some(ref cv) = c.value {
                        target = cv.trim_start_matches('$').to_string();
                    }
                }
            }

            // Generate a pseudo UUID v4 from system time + simple mixing
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default();
            let a = now.as_nanos() as u64;
            let b = now.subsec_nanos() as u64 ^ (a >> 16);
            let c_val = (a ^ b.wrapping_mul(0x9e3779b97f4a7c15)) as u64;
            let d = b ^ a.wrapping_mul(0x6c62272e07bb0142);

            let uuid = format!(
                "{:08x}-{:04x}-4{:03x}-{:04x}-{:012x}",
                (a & 0xFFFFFFFF) as u32,
                ((b >> 16) & 0xFFFF) as u16,
                (c_val & 0xFFF) as u16,
                (0x8000 | (d & 0x3FFF)) as u16,
                (a ^ d) & 0xFFFFFFFFFFFF
            );

            scope.set(&target, Value::String(uuid));
            Ok(())
        }),
        SlotMeta {
            description: "Generate a UUID v4-like unique identifier string.".to_string(),
            example: "util.uuid {\n  as: $id\n}".to_string(),
            inputs: {
                let mut m = HashMap::new();
                m.insert("as".to_string(), InputMeta { description: "Output variable".to_string(), required: false, r#type: "string".to_string() });
                m
            },
            required_blocks: Vec::new(),
            value_type: String::new(),
        },
    );

    // ==========================================
    // UTIL.ENV (Read environment variables)
    // ==========================================
    engine.register(
        "util.env",
        Arc::new(|engine, _ctx, node, scope| {
            let mut key = String::new();
            let mut default_val = Value::Nil;
            let mut target = "env_result".to_string();

            if node.value.is_some() {
                key = resolve_node_value(engine, node, scope).to_string_coerce();
            }

            for c in &node.children {
                if c.name == "key" || c.name == "name" {
                    key = engine.resolve_shorthand_value(c, scope).to_string_coerce();
                }
                if c.name == "default" || c.name == "def" {
                    default_val = engine.resolve_shorthand_value(c, scope);
                }
                if c.name == "as" {
                    if let Some(ref cv) = c.value {
                        target = cv.trim_start_matches('$').to_string();
                    }
                }
            }

            let val = std::env::var(&key)
                .map(Value::String)
                .unwrap_or(default_val);

            scope.set(&target, val);
            Ok(())
        }),
        SlotMeta {
            description: "Read an environment variable by name.".to_string(),
            example: "util.env: 'DATABASE_URL'\n  default: 'sqlite:./dev.db'\n  as: $db_url".to_string(),
            inputs: {
                let mut m = HashMap::new();
                m.insert("key".to_string(), InputMeta { description: "Environment variable name".to_string(), required: false, r#type: "string".to_string() });
                m.insert("default".to_string(), InputMeta { description: "Default value if not set".to_string(), required: false, r#type: "any".to_string() });
                m.insert("as".to_string(), InputMeta { description: "Output variable".to_string(), required: false, r#type: "string".to_string() });
                m
            },
            required_blocks: Vec::new(),
            value_type: String::new(),
        },
    );

    // ==========================================
    // PRINT (alias for log, outputs without prefix)
    // ==========================================
    engine.register(
        "print",
        Arc::new(|engine, _ctx, node, scope| {
            let val = if node.value.is_some() {
                resolve_node_value(engine, node, scope)
            } else {
                let mut parts = Vec::new();
                for child in &node.children {
                    if child.name == "val" || child.name == "value" {
                        parts.push(engine.resolve_shorthand_value(child, scope).to_string_coerce());
                    }
                }
                Value::String(parts.join(""))
            };
            println!("{}", val.to_string_coerce());
            Ok(())
        }),
        SlotMeta {
            description: "Print a value to stdout without any prefix.".to_string(),
            example: "print: 'Hello, World!'".to_string(),
            inputs: HashMap::new(),
            required_blocks: Vec::new(),
            value_type: String::new(),
        },
    );

    // ==========================================
    // PLUGIN.LOAD (Load Native Rust Dynamic Library Plugin)
    // ==========================================
    #[cfg(feature = "plugins")]
    engine.register(
        "plugin.load",
        Arc::new(|engine, _ctx, node, scope| {
            let path = resolve_node_value(engine, node, scope).to_string_coerce();
            if path.is_empty() {
                return Err(Diagnostic {
                    r#type: "error".to_string(),
                    message: "plugin.load: library path is required".to_string(),
                    filename: node.filename.clone(),
                    line: node.line,
                    col: node.col,
                    slot: Some("plugin.load".to_string()),
                });
            }

            unsafe { engine.load_plugin(&path) }
        }),
        SlotMeta {
            description: "Load a native Rust dynamic library (.so/.dylib/.dll) plugin into ZenoCore.".to_string(),
            example: "plugin.load: './plugins/libcustom.so'".to_string(),
            inputs: {
                let mut m = HashMap::new();
                m.insert("val".to_string(), InputMeta { description: "Path to shared library file".to_string(), required: false, r#type: "string".to_string() });
                m
            },
            required_blocks: Vec::new(),
            value_type: String::new(),
        },
    );
}

/// Evaluate a condition expression string against the current scope.
/// Supports: `&&`, `||`, `==`, `!=`, `>`, `<`, `>=`, `<=`, variable resolution, and boolean coercion.
fn evaluate_condition(engine: &Engine, expr: &str, scope: &Arc<Scope>) -> bool {
    let expr = expr.trim();

    // Strip surrounding quotes
    let expr = if (expr.starts_with('"') && expr.ends_with('"'))
        || (expr.starts_with('\'') && expr.ends_with('\''))
    {
        &expr[1..expr.len() - 1]
    } else {
        expr
    };

    let expr = expr.trim();
    if expr.is_empty() {
        return false;
    }

    // OR operator (lower precedence)
    if expr.contains("||") {
        for part in expr.split("||") {
            if evaluate_condition(engine, part.trim(), scope) {
                return true;
            }
        }
        return false;
    }

    // AND operator
    if expr.contains("&&") {
        for part in expr.split("&&") {
            if !evaluate_condition(engine, part.trim(), scope) {
                return false;
            }
        }
        return true;
    }

    // Comparison operators (order matters: check 2-char ops first)
    let ops = ["==", "!=", ">=", "<=", ">", "<"];
    for op in &ops {
        if let Some(pos) = expr.find(op) {
            let left_str = expr[..pos].trim();
            let right_str = expr[pos + op.len()..].trim();

            let left_val = resolve_cond_value(engine, left_str, scope);
            let right_val = resolve_cond_value(engine, right_str, scope);

            return match *op {
                "==" => left_val.to_string_coerce() == right_val.to_string_coerce(),
                "!=" => left_val.to_string_coerce() != right_val.to_string_coerce(),
                ">" => left_val.to_float() > right_val.to_float(),
                "<" => left_val.to_float() < right_val.to_float(),
                ">=" => left_val.to_float() >= right_val.to_float(),
                "<=" => left_val.to_float() <= right_val.to_float(),
                _ => false,
            };
        }
    }

    // Bare value — resolve and coerce to bool
    resolve_cond_value(engine, expr, scope).to_bool()
}

fn resolve_cond_value(_engine: &Engine, s: &str, scope: &Arc<Scope>) -> Value {
    let s = s.trim();
    if s.starts_with('$') {
        return scope.get(&s[1..]).unwrap_or(Value::Nil);
    }
    if (s.starts_with('"') && s.ends_with('"')) || (s.starts_with('\'') && s.ends_with('\'')) {
        return Value::String(s[1..s.len() - 1].to_string());
    }
    if s == "true" { return Value::Bool(true); }
    if s == "false" { return Value::Bool(false); }
    if s == "null" || s == "nil" { return Value::Nil; }
    if let Ok(i) = s.parse::<i64>() { return Value::Int(i); }
    if let Ok(f) = s.parse::<f64>() { return Value::Float(f); }
    Value::String(s.to_string())
}

/// Convert days since Unix epoch (1970-01-01) to (year, month, day).
fn days_to_ymd(days: u64) -> (u64, u64, u64) {
    // Simplified gregorian calculation
    let mut d = days as i64;
    let mut year = 1970i64;

    loop {
        let leap = (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0);
        let days_in_year = if leap { 366 } else { 365 };
        if d < days_in_year {
            break;
        }
        d -= days_in_year;
        year += 1;
    }

    let leap = (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0);
    let month_days: [i64; 12] = [31, if leap { 29 } else { 28 }, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

    let mut month = 1i64;
    for &md in &month_days {
        if d < md {
            break;
        }
        d -= md;
        month += 1;
    }

    (year as u64, month as u64, (d + 1) as u64)
}

