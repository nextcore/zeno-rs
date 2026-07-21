use crate::executor::{Engine, InputMeta, SlotMeta, interpolate_str};
use crate::scope::Value;
use std::collections::HashMap;
use std::sync::Arc;
use super::resolve_node_value;

pub fn register(engine: &mut Engine) {
    // ==========================================
    // STRING.TRIM
    // ==========================================
    engine.register(
        "string.trim",
        Arc::new(|engine, _ctx, node, scope| {
            let mut val = String::new();
            let mut target = "trimmed".to_string();

            if node.value.is_some() {
                val = resolve_node_value(engine, node, scope).to_string_coerce();
            }

            for c in &node.children {
                if c.name == "val" || c.name == "value" || c.name == "str" {
                    val = engine.resolve_shorthand_value(c, scope).to_string_coerce();
                }
                if c.name == "as" {
                    if let Some(ref cv) = c.value {
                        target = cv.trim_start_matches('$').to_string();
                    }
                }
            }

            scope.set(&target, Value::String(val.trim().to_string()));
            Ok(())
        }),
        SlotMeta {
            description: "Remove leading and trailing whitespace from a string.".to_string(),
            example: "string.trim: $name\n  as: $clean_name".to_string(),
            inputs: {
                let mut m = HashMap::new();
                m.insert("val".to_string(), InputMeta { description: "Input string".to_string(), required: false, r#type: "string".to_string() });
                m.insert("as".to_string(), InputMeta { description: "Output variable".to_string(), required: false, r#type: "string".to_string() });
                m
            },
            required_blocks: Vec::new(),
            value_type: String::new(),
        },
    );

    // ==========================================
    // STRING.UPPER
    // ==========================================
    engine.register(
        "string.upper",
        Arc::new(|engine, _ctx, node, scope| {
            let mut val = String::new();
            let mut target = "upper_result".to_string();

            if node.value.is_some() {
                val = resolve_node_value(engine, node, scope).to_string_coerce();
            }

            for c in &node.children {
                if c.name == "val" || c.name == "value" || c.name == "str" {
                    val = engine.resolve_shorthand_value(c, scope).to_string_coerce();
                }
                if c.name == "as" {
                    if let Some(ref cv) = c.value {
                        target = cv.trim_start_matches('$').to_string();
                    }
                }
            }

            scope.set(&target, Value::String(val.to_uppercase()));
            Ok(())
        }),
        SlotMeta {
            description: "Convert string to uppercase.".to_string(),
            example: "string.upper: $name\n  as: $upper_name".to_string(),
            inputs: {
                let mut m = HashMap::new();
                m.insert("val".to_string(), InputMeta { description: "Input string".to_string(), required: false, r#type: "string".to_string() });
                m.insert("as".to_string(), InputMeta { description: "Output variable".to_string(), required: false, r#type: "string".to_string() });
                m
            },
            required_blocks: Vec::new(),
            value_type: String::new(),
        },
    );

    // ==========================================
    // STRING.LOWER
    // ==========================================
    engine.register(
        "string.lower",
        Arc::new(|engine, _ctx, node, scope| {
            let mut val = String::new();
            let mut target = "lower_result".to_string();

            if node.value.is_some() {
                val = resolve_node_value(engine, node, scope).to_string_coerce();
            }

            for c in &node.children {
                if c.name == "val" || c.name == "value" || c.name == "str" {
                    val = engine.resolve_shorthand_value(c, scope).to_string_coerce();
                }
                if c.name == "as" {
                    if let Some(ref cv) = c.value {
                        target = cv.trim_start_matches('$').to_string();
                    }
                }
            }

            scope.set(&target, Value::String(val.to_lowercase()));
            Ok(())
        }),
        SlotMeta {
            description: "Convert string to lowercase.".to_string(),
            example: "string.lower: $name\n  as: $lower_name".to_string(),
            inputs: {
                let mut m = HashMap::new();
                m.insert("val".to_string(), InputMeta { description: "Input string".to_string(), required: false, r#type: "string".to_string() });
                m.insert("as".to_string(), InputMeta { description: "Output variable".to_string(), required: false, r#type: "string".to_string() });
                m
            },
            required_blocks: Vec::new(),
            value_type: String::new(),
        },
    );

    // ==========================================
    // STRING.LEN
    // ==========================================
    engine.register(
        "string.len",
        Arc::new(|engine, _ctx, node, scope| {
            let mut val = String::new();
            let mut target = "str_len".to_string();

            if node.value.is_some() {
                val = resolve_node_value(engine, node, scope).to_string_coerce();
            }

            for c in &node.children {
                if c.name == "val" || c.name == "value" || c.name == "str" {
                    val = engine.resolve_shorthand_value(c, scope).to_string_coerce();
                }
                if c.name == "as" {
                    if let Some(ref cv) = c.value {
                        target = cv.trim_start_matches('$').to_string();
                    }
                }
            }

            scope.set(&target, Value::Int(val.chars().count() as i64));
            Ok(())
        }),
        SlotMeta {
            description: "Get the character length of a string.".to_string(),
            example: "string.len: $text\n  as: $char_count".to_string(),
            inputs: {
                let mut m = HashMap::new();
                m.insert("val".to_string(), InputMeta { description: "Input string".to_string(), required: false, r#type: "string".to_string() });
                m.insert("as".to_string(), InputMeta { description: "Output variable".to_string(), required: false, r#type: "string".to_string() });
                m
            },
            required_blocks: Vec::new(),
            value_type: String::new(),
        },
    );

    // ==========================================
    // STRING.SPLIT
    // ==========================================
    engine.register(
        "string.split",
        Arc::new(|engine, _ctx, node, scope| {
            let mut val = String::new();
            let mut sep = ",".to_string();
            let mut target = "split_result".to_string();

            if node.value.is_some() {
                val = resolve_node_value(engine, node, scope).to_string_coerce();
            }

            for c in &node.children {
                if c.name == "val" || c.name == "value" || c.name == "str" {
                    val = engine.resolve_shorthand_value(c, scope).to_string_coerce();
                }
                if c.name == "sep" || c.name == "separator" || c.name == "by" {
                    sep = engine.resolve_shorthand_value(c, scope).to_string_coerce();
                }
                if c.name == "as" {
                    if let Some(ref cv) = c.value {
                        target = cv.trim_start_matches('$').to_string();
                    }
                }
            }

            let parts: Vec<Value> = val.split(sep.as_str())
                .map(|s| Value::String(s.to_string()))
                .collect();
            scope.set(&target, Value::List(parts));
            Ok(())
        }),
        SlotMeta {
            description: "Split a string by a separator into an array.".to_string(),
            example: "string.split: $csv_line\n  sep: ','\n  as: $fields".to_string(),
            inputs: {
                let mut m = HashMap::new();
                m.insert("val".to_string(), InputMeta { description: "Input string".to_string(), required: false, r#type: "string".to_string() });
                m.insert("sep".to_string(), InputMeta { description: "Separator".to_string(), required: false, r#type: "string".to_string() });
                m.insert("separator".to_string(), InputMeta { description: "Separator".to_string(), required: false, r#type: "string".to_string() });
                m.insert("by".to_string(), InputMeta { description: "Separator".to_string(), required: false, r#type: "string".to_string() });
                m.insert("as".to_string(), InputMeta { description: "Output variable".to_string(), required: false, r#type: "string".to_string() });
                m
            },
            required_blocks: Vec::new(),
            value_type: String::new(),
        },
    );

    // ==========================================
    // STRING.REPLACE
    // ==========================================
    engine.register(
        "string.replace",
        Arc::new(|engine, _ctx, node, scope| {
            let mut val = String::new();
            let mut find = String::new();
            let mut replace_with = String::new();
            let mut target = "replaced".to_string();

            if node.value.is_some() {
                val = resolve_node_value(engine, node, scope).to_string_coerce();
            }

            for c in &node.children {
                if c.name == "val" || c.name == "value" || c.name == "str" {
                    val = engine.resolve_shorthand_value(c, scope).to_string_coerce();
                }
                if c.name == "find" || c.name == "from" || c.name == "search" {
                    find = engine.resolve_shorthand_value(c, scope).to_string_coerce();
                }
                if c.name == "with" || c.name == "replace" || c.name == "to" {
                    replace_with = engine.resolve_shorthand_value(c, scope).to_string_coerce();
                }
                if c.name == "as" {
                    if let Some(ref cv) = c.value {
                        target = cv.trim_start_matches('$').to_string();
                    }
                }
            }

            let result = val.replace(find.as_str(), &replace_with);
            scope.set(&target, Value::String(result));
            Ok(())
        }),
        SlotMeta {
            description: "Replace all occurrences of a substring in a string.".to_string(),
            example: "string.replace: $text\n  find: 'old'\n  with: 'new'\n  as: $result".to_string(),
            inputs: {
                let mut m = HashMap::new();
                m.insert("val".to_string(), InputMeta { description: "Input string".to_string(), required: false, r#type: "string".to_string() });
                m.insert("find".to_string(), InputMeta { description: "String to find".to_string(), required: false, r#type: "string".to_string() });
                m.insert("with".to_string(), InputMeta { description: "Replacement string".to_string(), required: false, r#type: "string".to_string() });
                m.insert("as".to_string(), InputMeta { description: "Output variable".to_string(), required: false, r#type: "string".to_string() });
                m
            },
            required_blocks: Vec::new(),
            value_type: String::new(),
        },
    );

    // ==========================================
    // STRING.CONTAINS
    // ==========================================
    engine.register(
        "string.contains",
        Arc::new(|engine, _ctx, node, scope| {
            let mut val = String::new();
            let mut needle = String::new();
            let mut target = "contains_result".to_string();

            if node.value.is_some() {
                val = resolve_node_value(engine, node, scope).to_string_coerce();
            }

            for c in &node.children {
                if c.name == "val" || c.name == "value" || c.name == "str" {
                    val = engine.resolve_shorthand_value(c, scope).to_string_coerce();
                }
                if c.name == "needle" || c.name == "find" || c.name == "search" {
                    needle = engine.resolve_shorthand_value(c, scope).to_string_coerce();
                }
                if c.name == "as" {
                    if let Some(ref cv) = c.value {
                        target = cv.trim_start_matches('$').to_string();
                    }
                }
            }

            scope.set(&target, Value::Bool(val.contains(needle.as_str())));
            Ok(())
        }),
        SlotMeta {
            description: "Check if a string contains a substring.".to_string(),
            example: "string.contains: $text\n  needle: 'hello'\n  as: $has_hello".to_string(),
            inputs: {
                let mut m = HashMap::new();
                m.insert("val".to_string(), InputMeta { description: "Input string".to_string(), required: false, r#type: "string".to_string() });
                m.insert("needle".to_string(), InputMeta { description: "Substring to search for".to_string(), required: false, r#type: "string".to_string() });
                m.insert("as".to_string(), InputMeta { description: "Output variable (bool)".to_string(), required: false, r#type: "string".to_string() });
                m
            },
            required_blocks: Vec::new(),
            value_type: String::new(),
        },
    );

    // ==========================================
    // STRING.STARTS_WITH
    // ==========================================
    engine.register(
        "string.starts_with",
        Arc::new(|engine, _ctx, node, scope| {
            let mut val = String::new();
            let mut prefix = String::new();
            let mut target = "starts_with_result".to_string();

            if node.value.is_some() {
                val = resolve_node_value(engine, node, scope).to_string_coerce();
            }

            for c in &node.children {
                if c.name == "val" || c.name == "value" || c.name == "str" {
                    val = engine.resolve_shorthand_value(c, scope).to_string_coerce();
                }
                if c.name == "prefix" || c.name == "with" {
                    prefix = engine.resolve_shorthand_value(c, scope).to_string_coerce();
                }
                if c.name == "as" {
                    if let Some(ref cv) = c.value {
                        target = cv.trim_start_matches('$').to_string();
                    }
                }
            }

            scope.set(&target, Value::Bool(val.starts_with(prefix.as_str())));
            Ok(())
        }),
        SlotMeta {
            description: "Check if a string starts with a prefix.".to_string(),
            example: "string.starts_with: $url\n  prefix: 'https'\n  as: $is_secure".to_string(),
            inputs: {
                let mut m = HashMap::new();
                m.insert("val".to_string(), InputMeta { description: "Input string".to_string(), required: false, r#type: "string".to_string() });
                m.insert("prefix".to_string(), InputMeta { description: "Prefix to check".to_string(), required: false, r#type: "string".to_string() });
                m.insert("as".to_string(), InputMeta { description: "Output variable (bool)".to_string(), required: false, r#type: "string".to_string() });
                m
            },
            required_blocks: Vec::new(),
            value_type: String::new(),
        },
    );

    // ==========================================
    // STRING.ENDS_WITH
    // ==========================================
    engine.register(
        "string.ends_with",
        Arc::new(|engine, _ctx, node, scope| {
            let mut val = String::new();
            let mut suffix = String::new();
            let mut target = "ends_with_result".to_string();

            if node.value.is_some() {
                val = resolve_node_value(engine, node, scope).to_string_coerce();
            }

            for c in &node.children {
                if c.name == "val" || c.name == "value" || c.name == "str" {
                    val = engine.resolve_shorthand_value(c, scope).to_string_coerce();
                }
                if c.name == "suffix" || c.name == "with" {
                    suffix = engine.resolve_shorthand_value(c, scope).to_string_coerce();
                }
                if c.name == "as" {
                    if let Some(ref cv) = c.value {
                        target = cv.trim_start_matches('$').to_string();
                    }
                }
            }

            scope.set(&target, Value::Bool(val.ends_with(suffix.as_str())));
            Ok(())
        }),
        SlotMeta {
            description: "Check if a string ends with a suffix.".to_string(),
            example: "string.ends_with: $filename\n  suffix: '.rs'\n  as: $is_rust_file".to_string(),
            inputs: {
                let mut m = HashMap::new();
                m.insert("val".to_string(), InputMeta { description: "Input string".to_string(), required: false, r#type: "string".to_string() });
                m.insert("suffix".to_string(), InputMeta { description: "Suffix to check".to_string(), required: false, r#type: "string".to_string() });
                m.insert("as".to_string(), InputMeta { description: "Output variable (bool)".to_string(), required: false, r#type: "string".to_string() });
                m
            },
            required_blocks: Vec::new(),
            value_type: String::new(),
        },
    );

    // ==========================================
    // STRING.CONCAT
    // ==========================================
    engine.register(
        "string.concat",
        Arc::new(|engine, _ctx, node, scope| {
            let mut parts: Vec<String> = Vec::new();
            let mut target = "concat_result".to_string();
            let mut sep = String::new();

            for c in &node.children {
                if c.name == "val" || c.name == "value" || c.name == "str" || c.name == "part" {
                    parts.push(engine.resolve_shorthand_value(c, scope).to_string_coerce());
                }
                if c.name == "sep" || c.name == "separator" {
                    sep = engine.resolve_shorthand_value(c, scope).to_string_coerce();
                }
                if c.name == "as" {
                    if let Some(ref cv) = c.value {
                        target = cv.trim_start_matches('$').to_string();
                    }
                }
            }

            // If node has value, prepend it
            if let Some(ref v) = node.value {
                let resolved = engine.resolve_shorthand_value(node, scope).to_string_coerce();
                if !resolved.is_empty() {
                    parts.insert(0, resolved);
                } else {
                    parts.insert(0, v.clone());
                }
            }

            scope.set(&target, Value::String(parts.join(&sep)));
            Ok(())
        }),
        SlotMeta {
            description: "Concatenate multiple strings with an optional separator.".to_string(),
            example: "string.concat {\n  part: $first\n  part: ' '\n  part: $last\n  as: $full_name\n}".to_string(),
            inputs: {
                let mut m = HashMap::new();
                m.insert("val".to_string(), InputMeta { description: "String part".to_string(), required: false, r#type: "string".to_string() });
                m.insert("part".to_string(), InputMeta { description: "String part".to_string(), required: false, r#type: "string".to_string() });
                m.insert("sep".to_string(), InputMeta { description: "Separator between parts".to_string(), required: false, r#type: "string".to_string() });
                m.insert("as".to_string(), InputMeta { description: "Output variable".to_string(), required: false, r#type: "string".to_string() });
                m
            },
            required_blocks: Vec::new(),
            value_type: String::new(),
        },
    );

    // ==========================================
    // STRING.SUBSTR
    // ==========================================
    engine.register(
        "string.substr",
        Arc::new(|engine, _ctx, node, scope| {
            let mut val = String::new();
            let mut start: usize = 0;
            let mut length: Option<usize> = None;
            let mut target = "substr_result".to_string();

            if node.value.is_some() {
                val = resolve_node_value(engine, node, scope).to_string_coerce();
            }

            for c in &node.children {
                if c.name == "val" || c.name == "value" || c.name == "str" {
                    val = engine.resolve_shorthand_value(c, scope).to_string_coerce();
                }
                if c.name == "start" || c.name == "from" {
                    start = engine.resolve_shorthand_value(c, scope).to_int().max(0) as usize;
                }
                if c.name == "len" || c.name == "length" || c.name == "count" {
                    length = Some(engine.resolve_shorthand_value(c, scope).to_int().max(0) as usize);
                }
                if c.name == "as" {
                    if let Some(ref cv) = c.value {
                        target = cv.trim_start_matches('$').to_string();
                    }
                }
            }

            let chars: Vec<char> = val.chars().collect();
            let total = chars.len();
            let actual_start = start.min(total);
            let result: String = match length {
                Some(len) => chars[actual_start..].iter().take(len).collect(),
                None => chars[actual_start..].iter().collect(),
            };

            scope.set(&target, Value::String(result));
            Ok(())
        }),
        SlotMeta {
            description: "Extract a substring from a string.".to_string(),
            example: "string.substr: $text\n  start: 0\n  len: 10\n  as: $excerpt".to_string(),
            inputs: {
                let mut m = HashMap::new();
                m.insert("val".to_string(), InputMeta { description: "Input string".to_string(), required: false, r#type: "string".to_string() });
                m.insert("start".to_string(), InputMeta { description: "Start index".to_string(), required: false, r#type: "int".to_string() });
                m.insert("len".to_string(), InputMeta { description: "Length to extract".to_string(), required: false, r#type: "int".to_string() });
                m.insert("as".to_string(), InputMeta { description: "Output variable".to_string(), required: false, r#type: "string".to_string() });
                m
            },
            required_blocks: Vec::new(),
            value_type: String::new(),
        },
    );

    // ==========================================
    // STRING.FORMAT (Template interpolation)
    // ==========================================
    engine.register(
        "string.format",
        Arc::new(|engine, _ctx, node, scope| {
            let mut template = String::new();
            let mut target = "formatted".to_string();

            if node.value.is_some() {
                template = resolve_node_value(engine, node, scope).to_string_coerce();
            }

            for c in &node.children {
                if c.name == "val" || c.name == "value" || c.name == "template" {
                    template = engine.resolve_shorthand_value(c, scope).to_string_coerce();
                }
                if c.name == "as" {
                    if let Some(ref cv) = c.value {
                        target = cv.trim_start_matches('$').to_string();
                    }
                }
            }

            let result = interpolate_str(&template, scope);
            scope.set(&target, Value::String(result));
            Ok(())
        }),
        SlotMeta {
            description: "Format a template string by interpolating ${var} expressions from scope.".to_string(),
            example: "string.format: 'Hello, ${name}!'\n  as: $greeting".to_string(),
            inputs: {
                let mut m = HashMap::new();
                m.insert("val".to_string(), InputMeta { description: "Template string".to_string(), required: false, r#type: "string".to_string() });
                m.insert("template".to_string(), InputMeta { description: "Template string".to_string(), required: false, r#type: "string".to_string() });
                m.insert("as".to_string(), InputMeta { description: "Output variable".to_string(), required: false, r#type: "string".to_string() });
                m
            },
            required_blocks: Vec::new(),
            value_type: String::new(),
        },
    );

    // ==========================================
    // STRING.INDEX_OF
    // ==========================================
    engine.register(
        "string.index_of",
        Arc::new(|engine, _ctx, node, scope| {
            let mut val = String::new();
            let mut needle = String::new();
            let mut target = "index_result".to_string();

            if node.value.is_some() {
                val = resolve_node_value(engine, node, scope).to_string_coerce();
            }

            for c in &node.children {
                if c.name == "val" || c.name == "value" || c.name == "str" {
                    val = engine.resolve_shorthand_value(c, scope).to_string_coerce();
                }
                if c.name == "needle" || c.name == "find" || c.name == "search" {
                    needle = engine.resolve_shorthand_value(c, scope).to_string_coerce();
                }
                if c.name == "as" {
                    if let Some(ref cv) = c.value {
                        target = cv.trim_start_matches('$').to_string();
                    }
                }
            }

            let idx = val.find(needle.as_str()).map(|i| i as i64).unwrap_or(-1);
            scope.set(&target, Value::Int(idx));
            Ok(())
        }),
        SlotMeta {
            description: "Find the index of the first occurrence of a substring (-1 if not found).".to_string(),
            example: "string.index_of: $text\n  needle: 'hello'\n  as: $idx".to_string(),
            inputs: {
                let mut m = HashMap::new();
                m.insert("val".to_string(), InputMeta { description: "Input string".to_string(), required: false, r#type: "string".to_string() });
                m.insert("needle".to_string(), InputMeta { description: "Substring to find".to_string(), required: false, r#type: "string".to_string() });
                m.insert("as".to_string(), InputMeta { description: "Output variable (int)".to_string(), required: false, r#type: "string".to_string() });
                m
            },
            required_blocks: Vec::new(),
            value_type: String::new(),
        },
    );

    // ==========================================
    // STRING.TRIM_START / STRING.TRIM_END
    // ==========================================
    engine.register(
        "string.trim_start",
        Arc::new(|engine, _ctx, node, scope| {
            let mut val = String::new();
            let mut target = "trimmed".to_string();

            if node.value.is_some() {
                val = resolve_node_value(engine, node, scope).to_string_coerce();
            }
            for c in &node.children {
                if c.name == "val" || c.name == "value" || c.name == "str" {
                    val = engine.resolve_shorthand_value(c, scope).to_string_coerce();
                }
                if c.name == "as" {
                    if let Some(ref cv) = c.value {
                        target = cv.trim_start_matches('$').to_string();
                    }
                }
            }

            scope.set(&target, Value::String(val.trim_start().to_string()));
            Ok(())
        }),
        SlotMeta {
            description: "Remove leading whitespace from a string.".to_string(),
            example: "string.trim_start: $text\n  as: $result".to_string(),
            inputs: HashMap::new(),
            required_blocks: Vec::new(),
            value_type: String::new(),
        },
    );

    engine.register(
        "string.trim_end",
        Arc::new(|engine, _ctx, node, scope| {
            let mut val = String::new();
            let mut target = "trimmed".to_string();

            if node.value.is_some() {
                val = resolve_node_value(engine, node, scope).to_string_coerce();
            }
            for c in &node.children {
                if c.name == "val" || c.name == "value" || c.name == "str" {
                    val = engine.resolve_shorthand_value(c, scope).to_string_coerce();
                }
                if c.name == "as" {
                    if let Some(ref cv) = c.value {
                        target = cv.trim_start_matches('$').to_string();
                    }
                }
            }

            scope.set(&target, Value::String(val.trim_end().to_string()));
            Ok(())
        }),
        SlotMeta {
            description: "Remove trailing whitespace from a string.".to_string(),
            example: "string.trim_end: $text\n  as: $result".to_string(),
            inputs: HashMap::new(),
            required_blocks: Vec::new(),
            value_type: String::new(),
        },
    );

    // ==========================================
    // STRING.REPEAT
    // ==========================================
    engine.register(
        "string.repeat",
        Arc::new(|engine, _ctx, node, scope| {
            let mut val = String::new();
            let mut count: usize = 1;
            let mut target = "repeated".to_string();

            if node.value.is_some() {
                val = resolve_node_value(engine, node, scope).to_string_coerce();
            }
            for c in &node.children {
                if c.name == "val" || c.name == "value" || c.name == "str" {
                    val = engine.resolve_shorthand_value(c, scope).to_string_coerce();
                }
                if c.name == "count" || c.name == "times" || c.name == "n" {
                    count = engine.resolve_shorthand_value(c, scope).to_int().max(0) as usize;
                }
                if c.name == "as" {
                    if let Some(ref cv) = c.value {
                        target = cv.trim_start_matches('$').to_string();
                    }
                }
            }

            scope.set(&target, Value::String(val.repeat(count)));
            Ok(())
        }),
        SlotMeta {
            description: "Repeat a string N times.".to_string(),
            example: "string.repeat: '-'\n  count: 20\n  as: $divider".to_string(),
            inputs: {
                let mut m = HashMap::new();
                m.insert("val".to_string(), InputMeta { description: "Input string".to_string(), required: false, r#type: "string".to_string() });
                m.insert("count".to_string(), InputMeta { description: "Number of repetitions".to_string(), required: false, r#type: "int".to_string() });
                m.insert("as".to_string(), InputMeta { description: "Output variable".to_string(), required: false, r#type: "string".to_string() });
                m
            },
            required_blocks: Vec::new(),
            value_type: String::new(),
        },
    );
}
