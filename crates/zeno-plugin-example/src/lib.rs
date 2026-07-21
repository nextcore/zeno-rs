use zenocore::{Engine, Value, SlotMeta, InputMeta};
use std::collections::HashMap;
use std::sync::Arc;

/// Dynamic Plugin Initialization Entrypoint.
/// ZenoCore looks for this symbol via `libloading` when `plugin.load` is called.
#[unsafe(no_mangle)]
pub extern "C" fn zeno_plugin_init(engine: &Engine) {
    // Register custom plugin slot: example.hello
    engine.register(
        "example.hello",
        Arc::new(|engine, _ctx, node, scope| {
            let mut name = String::new();
            if let Some(ref v) = node.value {
                let v_clean = v.trim();
                if (v_clean.starts_with('"') && v_clean.ends_with('"')) || (v_clean.starts_with('\'') && v_clean.ends_with('\'')) {
                    name = v_clean[1..v_clean.len()-1].to_string();
                } else if v_clean.starts_with('$') {
                    name = scope.get(&v_clean[1..]).unwrap_or(Value::Nil).to_string_coerce();
                } else {
                    name = v_clean.to_string();
                }
            }
            for c in &node.children {
                if c.name == "val" || c.name == "name" {
                    name = engine.resolve_shorthand_value(c, scope).to_string_coerce();
                }
            }
            let greeting = if name.is_empty() {
                "Hello from Native Rust Plugin!".to_string()
            } else {
                format!("Hello, {}! (Powered by Native Rust Plugin)", name)
            };
            
            let mut target = "plugin_message".to_string();
            for c in &node.children {
                if c.name == "as" {
                    if let Some(ref cv) = c.value {
                        target = cv.trim_start_matches('$').to_string();
                    }
                }
            }

            scope.set(&target, Value::String(greeting));
            Ok(())
        }),
        SlotMeta {
            description: "Example native Rust plugin slot returning a greeting.".to_string(),
            example: "example.hello: 'World'\n  as: $msg".to_string(),
            inputs: {
                let mut m = HashMap::new();
                m.insert("val".to_string(), InputMeta { description: "Name to greet".to_string(), required: false, r#type: "string".to_string() });
                m.insert("as".to_string(), InputMeta { description: "Output variable".to_string(), required: false, r#type: "string".to_string() });
                m
            },
            required_blocks: Vec::new(),
            value_type: String::new(),
        },
    );

    // Register custom plugin slot: example.add_tax
    engine.register(
        "example.add_tax",
        Arc::new(|engine, _ctx, node, scope| {
            let mut price = 0.0;
            let mut rate = 0.11; // Default 11% VAT
            let mut target = "total_price".to_string();

            if let Some(ref v) = node.value {
                let v_clean = v.trim();
                if v_clean.starts_with('$') {
                    price = scope.get(&v_clean[1..]).unwrap_or(Value::Nil).to_float();
                } else if let Ok(f) = v_clean.parse::<f64>() {
                    price = f;
                }
            }

            for c in &node.children {
                if c.name == "price" || c.name == "val" {
                    price = engine.resolve_shorthand_value(c, scope).to_float();
                }
                if c.name == "rate" || c.name == "vat" {
                    rate = engine.resolve_shorthand_value(c, scope).to_float();
                }
                if c.name == "as" {
                    if let Some(ref cv) = c.value {
                        target = cv.trim_start_matches('$').to_string();
                    }
                }
            }

            let total = price * (1.0 + rate);
            scope.set(&target, Value::Float(total));
            Ok(())
        }),
        SlotMeta {
            description: "Calculate price with tax rate in native Rust plugin.".to_string(),
            example: "example.add_tax: 100.0\n  rate: 0.11\n  as: $total".to_string(),
            inputs: {
                let mut m = HashMap::new();
                m.insert("price".to_string(), InputMeta { description: "Base price".to_string(), required: false, r#type: "number".to_string() });
                m.insert("rate".to_string(), InputMeta { description: "Tax rate fraction".to_string(), required: false, r#type: "number".to_string() });
                m.insert("as".to_string(), InputMeta { description: "Output variable".to_string(), required: false, r#type: "string".to_string() });
                m
            },
            required_blocks: Vec::new(),
            value_type: String::new(),
        },
    );
}
