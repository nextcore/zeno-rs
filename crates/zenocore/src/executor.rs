use crate::diagnostic::Diagnostic;
use crate::parser::Node;
use crate::scope::{Scope, Value};
use std::any::Any;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Default)]
pub struct Context {
    values: HashMap<String, Arc<dyn Any + Send + Sync>>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn set<T: Any + Send + Sync>(&mut self, key: &str, val: T) {
        self.values.insert(key.to_string(), Arc::new(val));
    }

    pub fn get<T: Any + Send + Sync>(&self, key: &str) -> Option<Arc<T>> {
        self.values
            .get(key)
            .and_then(|any| any.clone().downcast::<T>().ok())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputMeta {
    pub description: String,
    pub required: bool,
    pub r#type: String, // "string", "int", "bool", etc
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SlotMeta {
    pub description: String,
    pub example: String,
    pub inputs: HashMap<String, InputMeta>,
    pub required_blocks: Vec<String>,
    pub value_type: String,
}

pub type HandlerFn = Arc<dyn Fn(&Engine, &mut Context, &Node, &Arc<Scope>) -> Result<(), Diagnostic> + Send + Sync>;

pub struct Engine {
    pub registry: std::sync::Mutex<HashMap<String, HandlerFn>>,
    pub docs: std::sync::Mutex<HashMap<String, SlotMeta>>,
    #[cfg(feature = "plugins")]
    pub plugins: std::sync::Mutex<Vec<Arc<libloading::Library>>>,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            registry: std::sync::Mutex::new(HashMap::new()),
            docs: std::sync::Mutex::new(HashMap::new()),
            #[cfg(feature = "plugins")]
            plugins: std::sync::Mutex::new(Vec::new()),
        }
    }

    pub fn register(&self, name: &str, handler: HandlerFn, meta: SlotMeta) {
        self.registry.lock().unwrap().insert(name.to_string(), handler);
        self.docs.lock().unwrap().insert(name.to_string(), meta);
    }

    #[cfg(feature = "plugins")]
    pub unsafe fn load_plugin<P: AsRef<std::path::Path>>(&self, path: P) -> Result<(), Diagnostic> {
        let path_ref = path.as_ref();
        let lib = unsafe { libloading::Library::new(path_ref) }.map_err(|e| Diagnostic {
            r#type: "error".to_string(),
            message: format!("plugin.load: failed to load library '{:?}': {}", path_ref, e),
            filename: String::new(),
            line: 0,
            col: 0,
            slot: Some("plugin.load".to_string()),
        })?;

        let lib_arc = Arc::new(lib);

        type PluginInitFn = unsafe extern "C" fn(&Engine);
        let symbol: libloading::Symbol<PluginInitFn> = unsafe { lib_arc.get(b"zeno_plugin_init\0") }.map_err(|e| Diagnostic {
            r#type: "error".to_string(),
            message: format!("plugin.load: symbol 'zeno_plugin_init' not found in '{:?}': {}", path_ref, e),
            filename: String::new(),
            line: 0,
            col: 0,
            slot: Some("plugin.load".to_string()),
        })?;

        unsafe { symbol(self) };
        self.plugins.lock().unwrap().push(lib_arc);
        Ok(())
    }

    pub fn resolve_shorthand_value(&self, node: &Node, scope: &Arc<Scope>) -> Value {
        // A. If has children, treat as List (if all keys are empty or numeric) or Map
        if !node.children.is_empty() {
            let is_list = node.children.iter().all(|c| c.name.is_empty() || c.name.parse::<usize>().is_ok());
            if is_list {
                let mut sorted_children = node.children.clone();
                sorted_children.sort_by_key(|c| c.name.parse::<usize>().unwrap_or(0));
                let list = sorted_children.iter().map(|c| self.resolve_shorthand_value(c, scope)).collect();
                return Value::List(list);
            } else {
                let mut map = HashMap::new();
                for child in &node.children {
                    map.insert(child.name.clone(), self.resolve_shorthand_value(child, scope));
                }
                return Value::Map(map);
            }
        }

        // B. Get raw string value
        let mut val_str = match &node.value {
            Some(v) => {
                let mut s = v.trim().to_string();
                if s.ends_with(',') {
                    s = s[..s.len() - 1].trim().to_string();
                }
                s
            }
            None => return Value::Nil,
        };

        // C. Check String Literal (double or single quotes)
        if val_str.len() >= 2 && (
            (val_str.starts_with('"') && val_str.ends_with('"')) ||
            (val_str.starts_with('\'') && val_str.ends_with('\''))
        ) {
            let inner = &val_str[1..val_str.len() - 1];
            if inner.contains("${") {
                return Value::String(interpolate_str(inner, scope));
            }
            return Value::String(inner.to_string());
        }

        // D. Check bracket notation index normalization (e.g. $list[0] -> $list.0)
        if val_str.contains('[') && val_str.contains(']') {
            val_str = val_str.replace('[', ".").replace(']', "");
        }

        // E. Check Null-coalescing (??)
        if val_str.starts_with('$') && val_str.contains("??") {
            let parts: Vec<&str> = val_str.splitn(2, "??").collect();
            if parts.len() == 2 {
                let v1 = parts[0].trim();
                let v2 = parts[1].trim();

                let res1 = self.resolve_shorthand_value(&Node {
                    name: String::new(),
                    value: Some(v1.to_string()),
                    children: Vec::new(),
                    line: node.line,
                    col: node.col,
                    filename: node.filename.clone(),
                }, scope);

                if res1 != Value::Nil && !res1.to_string_coerce().is_empty() {
                    return res1;
                }

                return self.resolve_shorthand_value(&Node {
                    name: String::new(),
                    value: Some(v2.to_string()),
                    children: Vec::new(),
                    line: node.line,
                    col: node.col,
                    filename: node.filename.clone(),
                }, scope);
            }
        }

        // F. Check Ternary Operator ( ? and  : )
        if val_str.starts_with('$') && val_str.contains(" ? ") && val_str.contains(" : ") {
            let parts: Vec<&str> = val_str.splitn(2, " ? ").collect();
            if parts.len() == 2 {
                let cond_str = parts[0].trim();
                let rest_parts: Vec<&str> = parts[1].splitn(2, " : ").collect();
                if rest_parts.len() == 2 {
                    let true_str = rest_parts[0].trim();
                    let false_str = rest_parts[1].trim();

                    let cond_val = self.resolve_shorthand_value(&Node {
                        name: String::new(),
                        value: Some(cond_str.to_string()),
                        children: Vec::new(),
                        line: node.line,
                        col: node.col,
                        filename: node.filename.clone(),
                    }, scope);

                    if cond_val.to_bool() {
                        return self.resolve_shorthand_value(&Node {
                            name: String::new(),
                            value: Some(true_str.to_string()),
                            children: Vec::new(),
                            line: node.line,
                            col: node.col,
                            filename: node.filename.clone(),
                        }, scope);
                    } else {
                        return self.resolve_shorthand_value(&Node {
                            name: String::new(),
                            value: Some(false_str.to_string()),
                            children: Vec::new(),
                            line: node.line,
                            col: node.col,
                            filename: node.filename.clone(),
                        }, scope);
                    }
                }
            }
        }

        // G. Check Variable Reference ($other)
        if val_str.starts_with('$') {
            let key = &val_str[1..];
            if let Some(val) = scope.get(key) {
                return val;
            }
        }

        // H. Fallback: Parse to appropriate primitive type or return raw string
        if let Ok(i) = val_str.parse::<i64>() {
            Value::Int(i)
        } else if let Ok(f) = val_str.parse::<f64>() {
            Value::Float(f)
        } else if let Ok(b) = val_str.parse::<bool>() {
            Value::Bool(b)
        } else {
            Value::String(val_str.clone())
        }
    }

    pub fn validate_value_type(&self, val: &Value, expected_type: &str, node: &Node, slot_name: &str) -> Result<(), Diagnostic> {
        let is_valid = match expected_type {
            "string" => matches!(val, Value::String(_)),
            "int" | "integer" => match val {
                Value::Int(_) => true,
                Value::Float(f) => *f == (*f as i64) as f64,
                Value::String(s) => s.parse::<i64>().is_ok(),
                _ => false,
            },
            "bool" | "boolean" => match val {
                Value::Bool(_) => true,
                Value::String(s) => {
                    let s_lower = s.to_lowercase();
                    s_lower == "true" || s_lower == "false" || s_lower == "1" || s_lower == "0"
                }
                _ => false,
            },
            "float" | "number" => match val {
                Value::Int(_) | Value::Float(_) => true,
                Value::String(s) => s.parse::<f64>().is_ok(),
                _ => false,
            },
            "list" | "array" => matches!(val, Value::List(_)) || match val {
                Value::String(s) => s.starts_with('[') && s.ends_with(']'),
                _ => false,
            },
            "map" | "object" => matches!(val, Value::Map(_)) || match val {
                Value::String(s) => s.starts_with('{') && s.ends_with('}'),
                _ => false,
            },
            _ => true,
        };

        if !is_valid {
            let mut attr_name = node.name.clone();
            if attr_name == slot_name {
                attr_name = "(main value)".to_string();
            }
            return Err(Diagnostic {
                r#type: "error".to_string(),
                message: format!(
                    "validation error: type mismatch for '{}'. Expected {}, got {:?}",
                    attr_name, expected_type, val
                ),
                filename: node.filename.clone(),
                line: node.line,
                col: node.col,
                slot: Some(slot_name.to_string()),
            });
        }
        Ok(())
    }

    pub fn execute(&self, ctx: &mut Context, node: &Node, scope: &Arc<Scope>) -> Result<(), Diagnostic> {
        // Catch panics to match Go's immortal runtime recovery behavior.
        // We use AssertUnwindSafe because we capture references and raw pointers.
        let self_safe = std::panic::AssertUnwindSafe(self);
        let mut ctx_safe = std::panic::AssertUnwindSafe(ctx);
        let node_safe = std::panic::AssertUnwindSafe(node);
        let scope_safe = std::panic::AssertUnwindSafe(scope);

        let result = std::panic::catch_unwind(move || {
            let this = *self_safe;
            let n = *node_safe;
            let s = *scope_safe;

            this.execute_internal(&mut **ctx_safe, n, s)
        });

        match result {
            Ok(execution_res) => execution_res,
            Err(panic_err) => {
                // Try to extract panic message
                let message = if let Some(s) = panic_err.downcast_ref::<&str>() {
                    s.to_string()
                } else if let Some(s) = panic_err.downcast_ref::<String>() {
                    s.clone()
                } else {
                    "unknown panic".to_string()
                };

                Err(Diagnostic {
                    r#type: "panic".to_string(),
                    message: format!("PANIC: {}", message),
                    filename: node.filename.clone(),
                    line: node.line,
                    col: node.col,
                    slot: Some(node.name.clone()),
                })
            }
        }
    }

    fn execute_internal(&self, ctx: &mut Context, node: &Node, scope: &Arc<Scope>) -> Result<(), Diagnostic> {
        // A. Check Variable Shorthand ($var: value)
        if node.name.len() > 1 && node.name.starts_with('$') {
            let var_name = &node.name[1..];
            let val = self.resolve_shorthand_value(node, scope);
            scope.set(var_name, val);
            return Ok(());
        }

        // B. Check registered slot handler
        let handler_opt = self.registry.lock().unwrap().get(&node.name).cloned();
        if let Some(handler) = handler_opt {
            // 1. Perform Validation if metadata exists
            let meta_opt = self.docs.lock().unwrap().get(&node.name).cloned();
            if let Some(meta) = meta_opt {
                // a. Check Unknown Attributes
                if !meta.inputs.is_empty() {
                    let allow_any = meta.inputs.contains_key("*") || meta.inputs.contains_key("*(any)");
                    if !allow_any {
                        for child in &node.children {
                            if child.name == "do" || child.name == "then" || child.name == "else" || child.name == "catch" || child.name.is_empty() {
                                continue;
                            }
                            if child.name.contains('.') {
                                continue;
                            }
                            if !meta.inputs.contains_key(&child.name) {
                                return Err(Diagnostic {
                                    r#type: "error".to_string(),
                                    message: format!("validation error: unknown attribute '{}'", child.name),
                                    filename: child.filename.clone(),
                                    line: child.line,
                                    col: child.col,
                                    slot: Some(node.name.clone()),
                                });
                            }
                        }
                    }
                }

                // b. Check Required main value or attributes
                for (name, input) in &meta.inputs {
                    if name == "(value)" {
                        if input.required {
                            let empty = node.value.is_none() || node.value.as_ref().unwrap().is_empty();
                            if empty {
                                return Err(Diagnostic {
                                    r#type: "error".to_string(),
                                    message: format!("validation error: missing required main value for slot '{}'", node.name),
                                    filename: node.filename.clone(),
                                    line: node.line,
                                    col: node.col,
                                    slot: Some(node.name.clone()),
                                });
                            }
                        }
                        // Validate main value type if present
                        if !input.r#type.is_empty() && input.r#type != "any" {
                            let val = self.resolve_shorthand_value(node, scope);
                            if val != Value::Nil {
                                self.validate_value_type(&val, &input.r#type, node, &node.name)?;
                            }
                        }
                        continue;
                    }

                    // Check attribute requirement
                    let found_child = node.children.iter().find(|c| c.name == *name);
                    if input.required {
                        if found_child.is_none() {
                            return Err(Diagnostic {
                                r#type: "error".to_string(),
                                message: format!("validation error: missing required attribute '{}'", name),
                                filename: node.filename.clone(),
                                line: node.line,
                                col: node.col,
                                slot: Some(node.name.clone()),
                            });
                        }
                        // Validate type
                        let child_node = found_child.unwrap();
                        if !input.r#type.is_empty() && input.r#type != "any" {
                            let val = self.resolve_shorthand_value(child_node, scope);
                            self.validate_value_type(&val, &input.r#type, child_node, &node.name)?;
                        }
                    } else if let Some(child_node) = found_child {
                        // Validate optional attribute type if present
                        if !input.r#type.is_empty() && input.r#type != "any" {
                            let val = self.resolve_shorthand_value(child_node, scope);
                            self.validate_value_type(&val, &input.r#type, child_node, &node.name)?;
                        }
                    }
                }

                // c. Check Required Blocks
                for block_name in &meta.required_blocks {
                    let found_block = node.children.iter().any(|c| c.name == *block_name);
                    if !found_block {
                        return Err(Diagnostic {
                            r#type: "error".to_string(),
                            message: format!("validation error: missing required block '{}:'", block_name),
                            filename: node.filename.clone(),
                            line: node.line,
                            col: node.col,
                            slot: Some(node.name.clone()),
                        });
                    }
                }
            }

            // 2. Call the registered handler
            return handler(self, ctx, node, scope);
        }

        // C. Fallback: Execute child nodes recursively
        for child in &node.children {
            self.execute(ctx, child, scope)?;
        }

        Ok(())
    }
}

/// Expand `${var}` and `${$var}` interpolation patterns inside a string using scope values.
/// Supports dot-notation references (e.g. `${user.name}`).
pub fn interpolate_str(s: &str, scope: &Arc<Scope>) -> String {
    let mut result = String::with_capacity(s.len());
    let mut last_idx = 0;

    while let Some(start_offset) = s[last_idx..].find("${") {
        let abs_start = last_idx + start_offset;
        result.push_str(&s[last_idx..abs_start]);

        if let Some(end_offset) = s[abs_start..].find('}') {
            let abs_end = abs_start + end_offset;
            let key_raw = s[abs_start + 2..abs_end].trim();
            // Support both ${name} and ${$name}
            let key = key_raw.trim_start_matches('$');
            if let Some(val) = scope.get(key) {
                result.push_str(&val.to_string_coerce());
            }
            last_idx = abs_end + 1;
        } else {
            // Unclosed brace — emit as-is and stop
            result.push_str(&s[abs_start..]);
            return result;
        }
    }
    result.push_str(&s[last_idx..]);
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse_string;

    #[test]
    fn test_executor_shorthand_and_flow() {
        let engine = Engine::new();
        let log_called = Arc::new(std::sync::Mutex::new(Vec::new()));
        let log_clone = log_called.clone();

        engine.register(
            "log",
            Arc::new(move |_engine, _ctx, node, scope| {
                // Get resolved value
                let val = if let Some(ref v) = node.value {
                    if v.starts_with('$') {
                        let var_name = &v[1..];
                        scope.get(var_name).unwrap().to_string_coerce()
                    } else if v.starts_with('"') && v.ends_with('"') {
                        v[1..v.len()-1].to_string()
                    } else {
                        v.clone()
                    }
                } else {
                    String::new()
                };
                log_clone.lock().unwrap().push(val);
                Ok(())
            }),
            SlotMeta {
                description: "Logs message".to_string(),
                example: "log: 'hello'".to_string(),
                inputs: HashMap::new(),
                required_blocks: Vec::new(),
                value_type: "string".to_string(),
            },
        );

        let code = r#"
            $name: "Budi"
            log: $name
        "#;
        let root = parse_string(code, "test.zl").unwrap();
        let mut ctx = Context::new();
        let scope = Scope::new(None);

        engine.execute(&mut ctx, &root, &scope).unwrap();
        assert_eq!(log_called.lock().unwrap().as_slice(), &["Budi".to_string()]);
    }

    #[test]
    fn test_executor_panic_recovery() {
        let engine = Engine::new();
        engine.register(
            "panic_slot",
            Arc::new(|_engine, _ctx, _node, _scope| {
                panic!("intentional panic");
            }),
            SlotMeta {
                description: "".to_string(),
                example: "".to_string(),
                inputs: HashMap::new(),
                required_blocks: Vec::new(),
                value_type: "".to_string(),
            },
        );

        let root = parse_string("panic_slot", "test.zl").unwrap();
        let mut ctx = Context::new();
        let scope = Scope::new(None);

        let res = engine.execute(&mut ctx, &root, &scope);
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert_eq!(err.r#type, "panic");
        assert!(err.message.contains("intentional panic"));
    }

    #[test]
    fn test_executor_trailing_comma() {
        let engine = Engine::new();
        let root = parse_string("$connections: $connections_str ,", "test.zl").unwrap();
        let mut ctx = Context::new();
        let scope = Scope::new(None);
        scope.set("connections_str", Value::String("5".to_string()));

        engine.execute(&mut ctx, &root, &scope).unwrap();
        assert_eq!(scope.get("connections").unwrap().to_string_coerce(), "5");
    }
}
