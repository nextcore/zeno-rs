use crate::diagnostic::Diagnostic;
use crate::executor::{Engine, InputMeta, SlotMeta};
use crate::scope::Value;
use std::collections::HashMap;
use std::sync::Arc;
use super::resolve_node_value;

pub fn register(engine: &mut Engine) {
    // ==========================================
    // ARRAY.PUSH
    // ==========================================
    engine.register(
        "array.push",
        Arc::new(|engine, _ctx, node, scope| {
            let mut target_name = String::new();
            let mut items = Vec::new();

            if let Some(ref val) = node.value {
                target_name = val.trim_start_matches('$').to_string();
            }

            for c in &node.children {
                if c.name == "in" || c.name == "list" {
                    if let Some(ref cv) = c.value {
                        target_name = cv.trim_start_matches('$').to_string();
                    }
                }
                if c.name == "val" || c.name == "value" || c.name == "item" {
                    items.push(engine.resolve_shorthand_value(c, scope));
                }
            }

            if target_name.is_empty() {
                return Err(Diagnostic {
                    r#type: "error".to_string(),
                    message: "array.push: target list not specified".to_string(),
                    filename: node.filename.clone(),
                    line: node.line,
                    col: node.col,
                    slot: Some("array.push".to_string()),
                });
            }

            let mut list = match scope.get(&target_name) {
                Some(Value::List(l)) => l.clone(),
                Some(Value::Nil) | None => Vec::new(),
                Some(other) => vec![other],
            };

            list.extend(items);
            scope.set(&target_name, Value::List(list));
            Ok(())
        }),
        SlotMeta {
            description: "Add one or more items to the end of an array.".to_string(),
            example: "array.push: $my_list\n  val: 'New Item'".to_string(),
            inputs: {
                let mut m = HashMap::new();
                m.insert("in".to_string(), InputMeta { description: "Target list".to_string(), required: false, r#type: "string".to_string() });
                m.insert("list".to_string(), InputMeta { description: "Target list".to_string(), required: false, r#type: "string".to_string() });
                m.insert("val".to_string(), InputMeta { description: "Value to push".to_string(), required: false, r#type: "any".to_string() });
                m.insert("value".to_string(), InputMeta { description: "Value to push".to_string(), required: false, r#type: "any".to_string() });
                m.insert("item".to_string(), InputMeta { description: "Value to push".to_string(), required: false, r#type: "any".to_string() });
                m
            },
            required_blocks: Vec::new(),
            value_type: String::new(),
        },
    );

    // ==========================================
    // COLLECTIONS.GET
    // ==========================================
    engine.register(
        "collections.get",
        Arc::new(|engine, _ctx, node, scope| {
            let mut list = Vec::new();
            let mut index = 0;
            let mut target = "item".to_string();

            if node.value.is_some() {
                list = resolve_node_value(engine, node, scope).to_list();
            }

            for c in &node.children {
                if c.name == "in" || c.name == "list" {
                    list = engine.resolve_shorthand_value(c, scope).to_list();
                }
                if c.name == "index" || c.name == "i" {
                    index = engine.resolve_shorthand_value(c, scope).to_int() as usize;
                }
                if c.name == "as" {
                    if let Some(ref cv) = c.value {
                        target = cv.trim_start_matches('$').to_string();
                    }
                }
            }

            if list.is_empty() {
                scope.set(&target, Value::Nil);
                return Ok(());
            }

            if index >= list.len() {
                return Err(Diagnostic {
                    r#type: "error".to_string(),
                    message: "collections.get: index out of bounds".to_string(),
                    filename: node.filename.clone(),
                    line: node.line,
                    col: node.col,
                    slot: Some("collections.get".to_string()),
                });
            }

            scope.set(&target, list[index].clone());
            Ok(())
        }),
        SlotMeta {
            description: "Get item from array at index.".to_string(),
            example: "collections.get: $list\n  index: 0\n  as: $item".to_string(),
            inputs: {
                let mut m = HashMap::new();
                m.insert("in".to_string(), InputMeta { description: "Source list".to_string(), required: false, r#type: "list".to_string() });
                m.insert("list".to_string(), InputMeta { description: "Source list".to_string(), required: false, r#type: "list".to_string() });
                m.insert("index".to_string(), InputMeta { description: "Index".to_string(), required: true, r#type: "int".to_string() });
                m.insert("i".to_string(), InputMeta { description: "Index".to_string(), required: false, r#type: "int".to_string() });
                m.insert("as".to_string(), InputMeta { description: "Target variable".to_string(), required: false, r#type: "any".to_string() });
                m
            },
            required_blocks: Vec::new(),
            value_type: String::new(),
        },
    );

    // ==========================================
    // ARRAY.POP
    // ==========================================
    engine.register(
        "array.pop",
        Arc::new(|_engine, _ctx, node, scope| {
            let mut target_name = String::new();
            let mut dst_name = "popped_item".to_string();

            if let Some(ref val) = node.value {
                target_name = val.trim_start_matches('$').to_string();
            }

            for c in &node.children {
                if c.name == "in" || c.name == "list" {
                    if let Some(ref cv) = c.value {
                        target_name = cv.trim_start_matches('$').to_string();
                    }
                }
                if c.name == "as" {
                    if let Some(ref cv) = c.value {
                        dst_name = cv.trim_start_matches('$').to_string();
                    }
                }
            }

            if target_name.is_empty() {
                return Err(Diagnostic {
                    r#type: "error".to_string(),
                    message: "array.pop: target list not specified".to_string(),
                    filename: node.filename.clone(),
                    line: node.line,
                    col: node.col,
                    slot: Some("array.pop".to_string()),
                });
            }

            let current_val = scope.get(&target_name).unwrap_or(Value::Nil);
            let mut list = current_val.to_list();

            if list.is_empty() {
                scope.set(&dst_name, Value::Nil);
                return Ok(());
            }

            let popped = list.pop().unwrap();
            scope.set(&target_name, Value::List(list));
            scope.set(&dst_name, popped);
            Ok(())
        }),
        SlotMeta {
            description: "Remove and return the last item of an array.".to_string(),
            example: "array.pop: $stack\n  as: $item".to_string(),
            inputs: {
                let mut m = HashMap::new();
                m.insert("in".to_string(), InputMeta { description: "Source list".to_string(), required: false, r#type: "string".to_string() });
                m.insert("list".to_string(), InputMeta { description: "Source list".to_string(), required: false, r#type: "string".to_string() });
                m.insert("as".to_string(), InputMeta { description: "Target variable".to_string(), required: false, r#type: "any".to_string() });
                m
            },
            required_blocks: Vec::new(),
            value_type: String::new(),
        },
    );

    // ==========================================
    // ARRAY.JOIN
    // ==========================================
    engine.register(
        "array.join",
        Arc::new(|engine, _ctx, node, scope| {
            let mut list = Vec::new();
            let mut separator = ",".to_string();
            let mut target = "joined_string".to_string();

            if node.value.is_some() {
                list = resolve_node_value(engine, node, scope).to_list();
            }

            for c in &node.children {
                if c.name == "list" {
                    list = engine.resolve_shorthand_value(c, scope).to_list();
                }
                if c.name == "sep" || c.name == "separator" {
                    separator = engine.resolve_shorthand_value(c, scope).to_string_coerce();
                }
                if c.name == "as" {
                    if let Some(ref cv) = c.value {
                        target = cv.trim_start_matches('$').to_string();
                    }
                }
            }

            let str_list: Vec<String> = list.iter().map(|item| item.to_string_coerce()).collect();
            scope.set(&target, Value::String(str_list.join(&separator)));
            Ok(())
        }),
        SlotMeta {
            description: "Join array elements into a string with a separator.".to_string(),
            example: "array.join: $tags\n  sep: ', '\n  as: $tag_str".to_string(),
            inputs: {
                let mut m = HashMap::new();
                m.insert("list".to_string(), InputMeta { description: "Source list".to_string(), required: false, r#type: "list".to_string() });
                m.insert("sep".to_string(), InputMeta { description: "Separator".to_string(), required: false, r#type: "string".to_string() });
                m.insert("separator".to_string(), InputMeta { description: "Separator".to_string(), required: false, r#type: "string".to_string() });
                m.insert("as".to_string(), InputMeta { description: "Target variable".to_string(), required: false, r#type: "any".to_string() });
                m
            },
            required_blocks: Vec::new(),
            value_type: String::new(),
        },
    );

    // ==========================================
    // MAP.SET
    // ==========================================
    engine.register(
        "map.set",
        Arc::new(|engine, _ctx, node, scope| {
            let mut target_name = String::new();

            if let Some(ref val) = node.value {
                target_name = val.trim_start_matches('$').to_string();
            }

            for c in &node.children {
                if c.name == "map" {
                    if let Some(ref cv) = c.value {
                        target_name = cv.trim_start_matches('$').to_string();
                    }
                }
            }

            if target_name.is_empty() {
                return Err(Diagnostic {
                    r#type: "error".to_string(),
                    message: "map.set: target map not specified".to_string(),
                    filename: node.filename.clone(),
                    line: node.line,
                    col: node.col,
                    slot: Some("map.set".to_string()),
                });
            }

            let mut map_val = match scope.get(&target_name) {
                Some(Value::Map(m)) => m.clone(),
                _ => HashMap::new(),
            };

            let mut explicit_key = String::new();
            let mut explicit_val = Value::Nil;
            let mut has_explicit = false;

            for c in &node.children {
                if c.name == "key" {
                    explicit_key = engine.resolve_shorthand_value(c, scope).to_string_coerce();
                    has_explicit = true;
                } else if c.name == "val" || c.name == "value" {
                    explicit_val = engine.resolve_shorthand_value(c, scope);
                } else if c.name != "map" {
                    map_val.insert(c.name.clone(), engine.resolve_shorthand_value(c, scope));
                }
            }

            if has_explicit && !explicit_key.is_empty() {
                map_val.insert(explicit_key, explicit_val);
            }

            scope.set(&target_name, Value::Map(map_val));
            Ok(())
        }),
        SlotMeta {
            description: "Set values in a map/object.".to_string(),
            example: "map.set: $user\n  age: 30".to_string(),
            inputs: {
                let mut m = HashMap::new();
                m.insert("map".to_string(), InputMeta { description: "Target map".to_string(), required: false, r#type: "string".to_string() });
                m.insert("key".to_string(), InputMeta { description: "Key".to_string(), required: false, r#type: "string".to_string() });
                m.insert("val".to_string(), InputMeta { description: "Value".to_string(), required: false, r#type: "any".to_string() });
                m.insert("value".to_string(), InputMeta { description: "Value to push".to_string(), required: false, r#type: "any".to_string() });
                m.insert("*".to_string(), InputMeta { description: "Dynamic properties".to_string(), required: false, r#type: "any".to_string() });
                m
            },
            required_blocks: Vec::new(),
            value_type: String::new(),
        },
    );

    // ==========================================
    // MAP.KEYS
    // ==========================================
    engine.register(
        "map.keys",
        Arc::new(|engine, _ctx, node, scope| {
            let mut val = Value::Nil;
            let mut target = "keys".to_string();

            if node.value.is_some() {
                val = resolve_node_value(engine, node, scope);
            }

            for c in &node.children {
                if c.name == "map" {
                    val = engine.resolve_shorthand_value(c, scope);
                }
                if c.name == "as" {
                    if let Some(ref cv) = c.value {
                        target = cv.trim_start_matches('$').to_string();
                    }
                }
            }

            let keys_list = match val {
                Value::Map(m) => m.keys().map(|k| Value::String(k.clone())).collect(),
                _ => Vec::new(),
            };

            scope.set(&target, Value::List(keys_list));
            Ok(())
        }),
        SlotMeta {
            description: "Get all keys of a map.".to_string(),
            example: "map.keys: $user\n  as: $fields".to_string(),
            inputs: {
                let mut m = HashMap::new();
                m.insert("map".to_string(), InputMeta { description: "Source map".to_string(), required: false, r#type: "map".to_string() });
                m.insert("as".to_string(), InputMeta { description: "Target variable".to_string(), required: false, r#type: "any".to_string() });
                m
            },
            required_blocks: Vec::new(),
            value_type: String::new(),
        },
    );

    // ==========================================
    // LEN
    // ==========================================
    engine.register(
        "len",
        Arc::new(|engine, _ctx, node, scope| {
            let mut val = Value::Nil;
            let mut target = "len".to_string();

            if node.value.is_some() {
                val = resolve_node_value(engine, node, scope);
            }

            for c in &node.children {
                if c.name == "as" {
                    if let Some(ref cv) = c.value {
                        target = cv.trim_start_matches('$').to_string();
                    }
                }
                if c.name == "in" || c.name == "list" || c.name == "val" || c.name == "value" {
                    val = engine.resolve_shorthand_value(c, scope);
                }
            }

            let length = match val {
                Value::Nil => 0,
                Value::String(s) => s.len() as i64,
                Value::List(l) => l.len() as i64,
                Value::Map(m) => m.len() as i64,
                _ => 0,
            };

            scope.set(&target, Value::Int(length));
            Ok(())
        }),
        SlotMeta {
            description: "Get the length of a string, array, or map.".to_string(),
            example: "len: $my_list\n  as: $count".to_string(),
            inputs: {
                let mut m = HashMap::new();
                m.insert("in".to_string(), InputMeta { description: "Collection".to_string(), required: false, r#type: "any".to_string() });
                m.insert("list".to_string(), InputMeta { description: "Collection".to_string(), required: false, r#type: "any".to_string() });
                m.insert("val".to_string(), InputMeta { description: "Collection".to_string(), required: false, r#type: "any".to_string() });
                m.insert("value".to_string(), InputMeta { description: "Collection".to_string(), required: false, r#type: "any".to_string() });
                m.insert("as".to_string(), InputMeta { description: "Target variable".to_string(), required: false, r#type: "any".to_string() });
                m
            },
            required_blocks: Vec::new(),
            value_type: String::new(),
        },
    );

    // ==========================================
    // ARRAY.REVERSE
    // ==========================================
    engine.register(
        "array.reverse",
        Arc::new(|_engine, _ctx, node, scope| {
            let mut target_name = String::new();
            if let Some(ref val) = node.value {
                target_name = val.trim_start_matches('$').to_string();
            }
            for c in &node.children {
                if c.name == "in" || c.name == "list" {
                    if let Some(ref cv) = c.value { target_name = cv.trim_start_matches('$').to_string(); }
                }
            }
            if target_name.is_empty() {
                return Err(crate::diagnostic::Diagnostic { r#type: "error".to_string(), message: "array.reverse: target list not specified".to_string(), filename: node.filename.clone(), line: node.line, col: node.col, slot: Some("array.reverse".to_string()) });
            }
            let mut list = scope.get(&target_name).unwrap_or(crate::scope::Value::Nil).to_list();
            list.reverse();
            scope.set(&target_name, crate::scope::Value::List(list));
            Ok(())
        }),
        SlotMeta { description: "Reverse an array in-place.".to_string(), example: "array.reverse: $items".to_string(), inputs: HashMap::new(), required_blocks: Vec::new(), value_type: String::new() },
    );

    // ==========================================
    // ARRAY.UNIQUE
    // ==========================================
    engine.register(
        "array.unique",
        Arc::new(|engine, _ctx, node, scope| {
            let mut list = Vec::new();
            let mut target = "unique_list".to_string();
            if node.value.is_some() { list = resolve_node_value(engine, node, scope).to_list(); }
            for c in &node.children {
                if c.name == "in" || c.name == "list" { list = engine.resolve_shorthand_value(c, scope).to_list(); }
                if c.name == "as" { if let Some(ref cv) = c.value { target = cv.trim_start_matches('$').to_string(); } }
            }
            let mut seen = Vec::new();
            let mut unique = Vec::new();
            for item in list {
                let s = item.to_string_coerce();
                if !seen.contains(&s) { seen.push(s); unique.push(item); }
            }
            scope.set(&target, crate::scope::Value::List(unique));
            Ok(())
        }),
        SlotMeta { description: "Remove duplicate values from an array.".to_string(), example: "array.unique: $tags\n  as: $unique_tags".to_string(), inputs: { let mut m = HashMap::new(); m.insert("as".to_string(), InputMeta { description: "Output".to_string(), required: false, r#type: "any".to_string() }); m }, required_blocks: Vec::new(), value_type: String::new() },
    );

    // ==========================================
    // ARRAY.SHIFT
    // ==========================================
    engine.register(
        "array.shift",
        Arc::new(|_engine, _ctx, node, scope| {
            let mut target_name = String::new();
            let mut dst = "shifted_item".to_string();
            if let Some(ref val) = node.value { target_name = val.trim_start_matches('$').to_string(); }
            for c in &node.children {
                if c.name == "in" || c.name == "list" { if let Some(ref cv) = c.value { target_name = cv.trim_start_matches('$').to_string(); } }
                if c.name == "as" { if let Some(ref cv) = c.value { dst = cv.trim_start_matches('$').to_string(); } }
            }
            let mut list = scope.get(&target_name).unwrap_or(crate::scope::Value::Nil).to_list();
            if list.is_empty() { scope.set(&dst, crate::scope::Value::Nil); return Ok(()); }
            let first = list.remove(0);
            scope.set(&target_name, crate::scope::Value::List(list));
            scope.set(&dst, first);
            Ok(())
        }),
        SlotMeta { description: "Remove and return the first element.".to_string(), example: "array.shift: $queue\n  as: $item".to_string(), inputs: { let mut m = HashMap::new(); m.insert("as".to_string(), InputMeta { description: "Output".to_string(), required: false, r#type: "any".to_string() }); m }, required_blocks: Vec::new(), value_type: String::new() },
    );

    // ==========================================
    // ARRAY.UNSHIFT
    // ==========================================
    engine.register(
        "array.unshift",
        Arc::new(|engine, _ctx, node, scope| {
            let mut target_name = String::new();
            let mut item = crate::scope::Value::Nil;
            if let Some(ref val) = node.value { target_name = val.trim_start_matches('$').to_string(); }
            for c in &node.children {
                if c.name == "in" || c.name == "list" { if let Some(ref cv) = c.value { target_name = cv.trim_start_matches('$').to_string(); } }
                if c.name == "val" || c.name == "value" || c.name == "item" { item = engine.resolve_shorthand_value(c, scope); }
            }
            let mut list = scope.get(&target_name).unwrap_or(crate::scope::Value::Nil).to_list();
            list.insert(0, item);
            scope.set(&target_name, crate::scope::Value::List(list));
            Ok(())
        }),
        SlotMeta { description: "Prepend an item to an array.".to_string(), example: "array.unshift: $queue\n  val: $new_item".to_string(), inputs: { let mut m = HashMap::new(); m.insert("val".to_string(), InputMeta { description: "Item".to_string(), required: false, r#type: "any".to_string() }); m }, required_blocks: Vec::new(), value_type: String::new() },
    );

    // ==========================================
    // ARRAY.SLICE
    // ==========================================
    engine.register(
        "array.slice",
        Arc::new(|engine, _ctx, node, scope| {
            let mut list = Vec::new();
            let mut start: usize = 0;
            let mut end: Option<usize> = None;
            let mut target = "sliced_list".to_string();
            if node.value.is_some() { list = resolve_node_value(engine, node, scope).to_list(); }
            for c in &node.children {
                if c.name == "in" || c.name == "list" { list = engine.resolve_shorthand_value(c, scope).to_list(); }
                if c.name == "start" || c.name == "from" { start = engine.resolve_shorthand_value(c, scope).to_int().max(0) as usize; }
                if c.name == "end" || c.name == "to" { end = Some(engine.resolve_shorthand_value(c, scope).to_int().max(0) as usize); }
                if c.name == "as" { if let Some(ref cv) = c.value { target = cv.trim_start_matches('$').to_string(); } }
            }
            let total = list.len();
            let s = start.min(total);
            let e = end.unwrap_or(total).min(total);
            let sliced = if s < e { list[s..e].to_vec() } else { Vec::new() };
            scope.set(&target, crate::scope::Value::List(sliced));
            Ok(())
        }),
        SlotMeta { description: "Extract a portion of an array.".to_string(), example: "array.slice: $items\n  start: 0\n  end: 5\n  as: $page".to_string(), inputs: { let mut m = HashMap::new(); m.insert("start".to_string(), InputMeta { description: "Start index".to_string(), required: false, r#type: "int".to_string() }); m.insert("end".to_string(), InputMeta { description: "End index".to_string(), required: false, r#type: "int".to_string() }); m.insert("as".to_string(), InputMeta { description: "Output".to_string(), required: false, r#type: "any".to_string() }); m }, required_blocks: Vec::new(), value_type: String::new() },
    );

    // ==========================================
    // ARRAY.CONTAINS
    // ==========================================
    engine.register(
        "array.contains",
        Arc::new(|engine, _ctx, node, scope| {
            let mut list = Vec::new();
            let mut needle = crate::scope::Value::Nil;
            let mut target = "contains_result".to_string();
            if node.value.is_some() { list = resolve_node_value(engine, node, scope).to_list(); }
            for c in &node.children {
                if c.name == "in" || c.name == "list" { list = engine.resolve_shorthand_value(c, scope).to_list(); }
                if c.name == "needle" || c.name == "val" || c.name == "value" { needle = engine.resolve_shorthand_value(c, scope); }
                if c.name == "as" { if let Some(ref cv) = c.value { target = cv.trim_start_matches('$').to_string(); } }
            }
            let ns = needle.to_string_coerce();
            let found = list.iter().any(|i| i.to_string_coerce() == ns);
            scope.set(&target, crate::scope::Value::Bool(found));
            Ok(())
        }),
        SlotMeta { description: "Check if an array contains a value.".to_string(), example: "array.contains: $roles\n  needle: 'admin'\n  as: $is_admin".to_string(), inputs: { let mut m = HashMap::new(); m.insert("needle".to_string(), InputMeta { description: "Value to find".to_string(), required: false, r#type: "any".to_string() }); m.insert("as".to_string(), InputMeta { description: "Output (bool)".to_string(), required: false, r#type: "any".to_string() }); m }, required_blocks: Vec::new(), value_type: String::new() },
    );

    // ==========================================
    // ARRAY.SORT
    // ==========================================
    engine.register(
        "array.sort",
        Arc::new(|engine, _ctx, node, scope| {
            let mut target_name = String::new();
            let mut desc = false;
            let mut by_key = String::new();
            if let Some(ref val) = node.value { target_name = val.trim_start_matches('$').to_string(); }
            for c in &node.children {
                if c.name == "in" || c.name == "list" { if let Some(ref cv) = c.value { target_name = cv.trim_start_matches('$').to_string(); } }
                if c.name == "order" || c.name == "dir" { let v = engine.resolve_shorthand_value(c, scope).to_string_coerce(); desc = v == "desc" || v == "descending"; }
                if c.name == "by" || c.name == "key" { by_key = engine.resolve_shorthand_value(c, scope).to_string_coerce(); }
            }
            let mut list = scope.get(&target_name).unwrap_or(crate::scope::Value::Nil).to_list();
            if by_key.is_empty() {
                list.sort_by(|a, b| { let cmp = a.to_string_coerce().cmp(&b.to_string_coerce()); if desc { cmp.reverse() } else { cmp } });
            } else {
                let bk = by_key.clone();
                list.sort_by(|a, b| {
                    let av = match a { crate::scope::Value::Map(m) => m.get(&bk).map(|v| v.to_string_coerce()).unwrap_or_default(), _ => a.to_string_coerce() };
                    let bv = match b { crate::scope::Value::Map(m) => m.get(&bk).map(|v| v.to_string_coerce()).unwrap_or_default(), _ => b.to_string_coerce() };
                    let cmp = av.cmp(&bv);
                    if desc { cmp.reverse() } else { cmp }
                });
            }
            scope.set(&target_name, crate::scope::Value::List(list));
            Ok(())
        }),
        SlotMeta { description: "Sort an array in-place.".to_string(), example: "array.sort: $users\n  by: 'name'\n  order: 'asc'".to_string(), inputs: { let mut m = HashMap::new(); m.insert("by".to_string(), InputMeta { description: "Map key to sort by".to_string(), required: false, r#type: "string".to_string() }); m.insert("order".to_string(), InputMeta { description: "asc or desc".to_string(), required: false, r#type: "string".to_string() }); m }, required_blocks: Vec::new(), value_type: String::new() },
    );

    // ==========================================
    // MAP.GET
    // ==========================================
    engine.register(
        "map.get",
        Arc::new(|engine, _ctx, node, scope| {
            let mut map_val = crate::scope::Value::Nil;
            let mut key = String::new();
            let mut default_val = crate::scope::Value::Nil;
            let mut target = "map_get_result".to_string();
            if node.value.is_some() { map_val = resolve_node_value(engine, node, scope); }
            for c in &node.children {
                if c.name == "map" { map_val = engine.resolve_shorthand_value(c, scope); }
                if c.name == "key" || c.name == "field" { key = engine.resolve_shorthand_value(c, scope).to_string_coerce(); }
                if c.name == "default" || c.name == "def" { default_val = engine.resolve_shorthand_value(c, scope); }
                if c.name == "as" { if let Some(ref cv) = c.value { target = cv.trim_start_matches('$').to_string(); } }
            }
            let result = if let crate::scope::Value::Map(ref m) = map_val { m.get(&key).cloned().unwrap_or(default_val) } else { default_val };
            scope.set(&target, result);
            Ok(())
        }),
        SlotMeta { description: "Get a value from a map by key.".to_string(), example: "map.get: $user\n  key: 'name'\n  default: 'Anonymous'\n  as: $name".to_string(), inputs: { let mut m = HashMap::new(); m.insert("key".to_string(), InputMeta { description: "Key".to_string(), required: false, r#type: "string".to_string() }); m.insert("default".to_string(), InputMeta { description: "Default".to_string(), required: false, r#type: "any".to_string() }); m.insert("as".to_string(), InputMeta { description: "Output".to_string(), required: false, r#type: "any".to_string() }); m }, required_blocks: Vec::new(), value_type: String::new() },
    );

    // ==========================================
    // MAP.DELETE
    // ==========================================
    engine.register(
        "map.delete",
        Arc::new(|engine, _ctx, node, scope| {
            let mut target_name = String::new();
            let mut key = String::new();
            if let Some(ref val) = node.value { target_name = val.trim_start_matches('$').to_string(); }
            for c in &node.children {
                if c.name == "map" { if let Some(ref cv) = c.value { target_name = cv.trim_start_matches('$').to_string(); } }
                if c.name == "key" || c.name == "field" { key = engine.resolve_shorthand_value(c, scope).to_string_coerce(); }
            }
            if target_name.is_empty() || key.is_empty() { return Ok(()); }
            if let Some(crate::scope::Value::Map(mut m)) = scope.get(&target_name) {
                m.remove(&key);
                scope.set(&target_name, crate::scope::Value::Map(m));
            }
            Ok(())
        }),
        SlotMeta { description: "Remove a key from a map.".to_string(), example: "map.delete: $data\n  key: 'password'".to_string(), inputs: { let mut m = HashMap::new(); m.insert("key".to_string(), InputMeta { description: "Key to remove".to_string(), required: false, r#type: "string".to_string() }); m }, required_blocks: Vec::new(), value_type: String::new() },
    );

    // ==========================================
    // MAP.MERGE
    // ==========================================
    engine.register(
        "map.merge",
        Arc::new(|engine, _ctx, node, scope| {
            let mut base: HashMap<String, crate::scope::Value> = HashMap::new();
            let mut overlay: HashMap<String, crate::scope::Value> = HashMap::new();
            let mut target = "merged_map".to_string();
            if node.value.is_some() { if let crate::scope::Value::Map(m) = resolve_node_value(engine, node, scope) { base = m; } }
            for c in &node.children {
                if c.name == "base" || c.name == "a" { if let crate::scope::Value::Map(m) = engine.resolve_shorthand_value(c, scope) { base = m; } }
                if c.name == "with" || c.name == "b" || c.name == "overlay" { if let crate::scope::Value::Map(m) = engine.resolve_shorthand_value(c, scope) { overlay = m; } }
                if c.name == "as" { if let Some(ref cv) = c.value { target = cv.trim_start_matches('$').to_string(); } }
            }
            for (k, v) in overlay { base.insert(k, v); }
            scope.set(&target, crate::scope::Value::Map(base));
            Ok(())
        }),
        SlotMeta { description: "Merge two maps. Keys in 'with' override the base map.".to_string(), example: "map.merge: $defaults\n  with: $overrides\n  as: $config".to_string(), inputs: { let mut m = HashMap::new(); m.insert("base".to_string(), InputMeta { description: "Base map".to_string(), required: false, r#type: "map".to_string() }); m.insert("with".to_string(), InputMeta { description: "Overlay map".to_string(), required: false, r#type: "map".to_string() }); m.insert("as".to_string(), InputMeta { description: "Output".to_string(), required: false, r#type: "any".to_string() }); m }, required_blocks: Vec::new(), value_type: String::new() },
    );

    // ==========================================
    // MAP.VALUES
    // ==========================================
    engine.register(
        "map.values",
        Arc::new(|engine, _ctx, node, scope| {
            let mut val = crate::scope::Value::Nil;
            let mut target = "values".to_string();
            if node.value.is_some() { val = resolve_node_value(engine, node, scope); }
            for c in &node.children {
                if c.name == "map" { val = engine.resolve_shorthand_value(c, scope); }
                if c.name == "as" { if let Some(ref cv) = c.value { target = cv.trim_start_matches('$').to_string(); } }
            }
            let values_list = if let crate::scope::Value::Map(m) = val {
                let mut pairs: Vec<_> = m.into_iter().collect();
                pairs.sort_by(|a, b| a.0.cmp(&b.0));
                pairs.into_iter().map(|(_, v)| v).collect()
            } else { Vec::new() };
            scope.set(&target, crate::scope::Value::List(values_list));
            Ok(())
        }),
        SlotMeta { description: "Get all values from a map as a list.".to_string(), example: "map.values: $config\n  as: $vals".to_string(), inputs: { let mut m = HashMap::new(); m.insert("as".to_string(), InputMeta { description: "Output".to_string(), required: false, r#type: "any".to_string() }); m }, required_blocks: Vec::new(), value_type: String::new() },
    );

    // ==========================================
    // MAP.HAS
    // ==========================================
    engine.register(
        "map.has",
        Arc::new(|engine, _ctx, node, scope| {
            let mut map_val = crate::scope::Value::Nil;
            let mut key = String::new();
            let mut target = "map_has_result".to_string();
            if node.value.is_some() { map_val = resolve_node_value(engine, node, scope); }
            for c in &node.children {
                if c.name == "map" { map_val = engine.resolve_shorthand_value(c, scope); }
                if c.name == "key" || c.name == "field" { key = engine.resolve_shorthand_value(c, scope).to_string_coerce(); }
                if c.name == "as" { if let Some(ref cv) = c.value { target = cv.trim_start_matches('$').to_string(); } }
            }
            let has = if let crate::scope::Value::Map(ref m) = map_val { m.contains_key(&key) } else { false };
            scope.set(&target, crate::scope::Value::Bool(has));
            Ok(())
        }),
        SlotMeta { description: "Check if a map contains a specific key.".to_string(), example: "map.has: $config\n  key: 'debug'\n  as: $has_debug".to_string(), inputs: { let mut m = HashMap::new(); m.insert("key".to_string(), InputMeta { description: "Key to check".to_string(), required: false, r#type: "string".to_string() }); m.insert("as".to_string(), InputMeta { description: "Output (bool)".to_string(), required: false, r#type: "any".to_string() }); m }, required_blocks: Vec::new(), value_type: String::new() },
    );

    // ==========================================
    // MAP.ENTRIES
    // ==========================================
    engine.register(
        "map.entries",
        Arc::new(|engine, _ctx, node, scope| {
            let mut val = crate::scope::Value::Nil;
            let mut target = "entries".to_string();
            if node.value.is_some() { val = resolve_node_value(engine, node, scope); }
            for c in &node.children {
                if c.name == "map" { val = engine.resolve_shorthand_value(c, scope); }
                if c.name == "as" { if let Some(ref cv) = c.value { target = cv.trim_start_matches('$').to_string(); } }
            }
            let entries = if let crate::scope::Value::Map(m) = val {
                let mut pairs: Vec<_> = m.into_iter().collect();
                pairs.sort_by(|a, b| a.0.cmp(&b.0));
                pairs.into_iter().map(|(k, v)| {
                    let mut entry = HashMap::new();
                    entry.insert("key".to_string(), crate::scope::Value::String(k));
                    entry.insert("value".to_string(), v);
                    crate::scope::Value::Map(entry)
                }).collect()
            } else { Vec::new() };
            scope.set(&target, crate::scope::Value::List(entries));
            Ok(())
        }),
        SlotMeta { description: "Get all key-value pairs as a list of {key, value} maps.".to_string(), example: "map.entries: $config\n  as: $pairs".to_string(), inputs: { let mut m = HashMap::new(); m.insert("as".to_string(), InputMeta { description: "Output".to_string(), required: false, r#type: "any".to_string() }); m }, required_blocks: Vec::new(), value_type: String::new() },
    );
}
