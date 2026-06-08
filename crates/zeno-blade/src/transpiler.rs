use zenocore::Node;

fn find_balanced_paren(s: &str) -> Option<usize> {
    let mut depth = 0;
    for (i, c) in s.char_indices() {
        if c == '(' {
            depth += 1;
        } else if c == ')' {
            depth -= 1;
            if depth == 0 {
                return Some(i);
            }
        }
    }
    None
}

fn find_end_forelse(s: &str) -> Option<usize> {
    let mut depth = 0;
    let mut pos = 0;
    let bytes = s.as_bytes();
    while pos < bytes.len() {
        if s[pos..].starts_with("@forelse") {
            depth += 1;
            pos += 8;
        } else if s[pos..].starts_with("@endforelse") {
            if depth == 0 {
                return Some(pos);
            }
            depth -= 1;
            pos += 11;
        } else {
            pos += 1;
        }
    }
    None
}

fn find_end_component(s: &str, tag_name: &str) -> Option<usize> {
    let closing = format!("</{}", tag_name);
    let start = format!("<{}", tag_name);
    
    let mut depth = 0;
    let mut pos = 0;
    let bytes = s.as_bytes();
    
    while pos < bytes.len() {
        if s[pos..].starts_with(&start) {
            depth += 1;
            pos += start.len();
        } else if s[pos..].starts_with(&closing) {
            if depth == 0 {
                return Some(pos);
            }
            depth -= 1;
            pos += closing.len();
        } else {
            pos += 1;
        }
    }
    None
}

fn parse_blade_attributes(raw: &str, filename: &str) -> Vec<Node> {
    let mut nodes = Vec::new();
    let raw = raw.trim();
    let chars: Vec<char> = raw.chars().collect();
    let n = chars.len();
    let mut i = 0;

    while i < n {
        // Skip spaces
        while i < n && chars[i] == ' ' {
            i += 1;
        }
        if i >= n {
            break;
        }

        // Read Key
        let key_start = i;
        while i < n && chars[i] != '=' && chars[i] != ' ' {
            i += 1;
        }
        let key: String = chars[key_start..i].iter().collect();

        let val = if i < n && chars[i] == '=' {
            i += 1; // skip =
            if i < n && (chars[i] == '"' || chars[i] == '\'') {
                let quote = chars[i];
                i += 1;
                let val_start = i;
                while i < n && chars[i] != quote {
                    i += 1;
                }
                let v: String = chars[val_start..i].iter().collect();
                if i < n {
                    i += 1;
                } // skip quote
                v
            } else {
                let val_start = i;
                while i < n && chars[i] != ' ' {
                    i += 1;
                }
                chars[val_start..i].iter().collect()
            }
        } else {
            "true".to_string()
        };

        nodes.push(Node {
            name: key,
            value: Some(val),
            children: Vec::new(),
            line: 1,
            col: 1,
            filename: filename.to_string(),
        });
    }

    nodes
}


fn find_end_directive(s: &str, start_dir: &str, end_dir: &str) -> Option<usize> {
    let mut depth = 0;
    let mut pos = 0;
    let bytes = s.as_bytes();
    while pos < bytes.len() {
        if s[pos..].starts_with(start_dir) {
            depth += 1;
            pos += start_dir.len();
        } else if s[pos..].starts_with(end_dir) {
            if depth == 0 {
                return Some(pos);
            }
            depth -= 1;
            pos += end_dir.len();
        } else {
            pos += 1;
        }
    }
    None
}

fn find_end_if(s: &str) -> Option<(usize, &'static str)> {
    let mut depth = 0;
    let mut pos = 0;
    while pos < s.len() {
        if s[pos..].starts_with("@if") {
            depth += 1;
            pos += 3;
        } else if s[pos..].starts_with("@endif") {
            if depth == 0 {
                return Some((pos, "endif"));
            }
            depth -= 1;
            pos += 6;
        } else if s[pos..].starts_with("@else") {
            if depth == 0 {
                return Some((pos, "else"));
            }
            pos += 5;
        } else {
            pos += 1;
        }
    }
    None
}

fn split_blade_args(s: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut depth = 0;
    let mut last_split = 0;
    for (i, c) in s.char_indices() {
        if c == '[' || c == '(' {
            depth += 1;
        } else if c == ']' || c == ')' {
            depth -= 1;
        } else if c == ',' && depth == 0 {
            args.push(s[last_split..i].trim().to_string());
            last_split = i + c.len_utf8();
        }
    }
    args.push(s[last_split..].trim().to_string());
    args
}

fn parse_blade_data(s: &str, filename: &str) -> Option<Node> {
    let s = s.trim();
    if s.starts_with('[') && s.ends_with(']') {
        let inner = &s[1..s.len() - 1];
        let pairs = split_blade_args(inner);
        let mut data_node = Node {
            name: "data_map".to_string(),
            value: None,
            children: Vec::new(),
            line: 1,
            col: 1,
            filename: filename.to_string(),
        };
        for pair in pairs {
            if pair.is_empty() {
                continue;
            }
            let parts: Vec<&str> = pair.split("=>").collect();
            if parts.len() == 2 {
                let key = parts[0].trim().trim_matches(|c| c == '\'' || c == '"').to_string();
                let val_raw = parts[1].trim();
                let mut val_node = if val_raw.starts_with('$') {
                    Node {
                        name: "var".to_string(),
                        value: Some(val_raw.to_string()),
                        children: Vec::new(),
                        line: 1,
                        col: 1,
                        filename: filename.to_string(),
                    }
                } else {
                    Node {
                        name: "literal".to_string(),
                        value: Some(val_raw.trim_matches(|c| c == '\'' || c == '"').to_string()),
                        children: Vec::new(),
                        line: 1,
                        col: 1,
                        filename: filename.to_string(),
                    }
                };
                val_node.name = key;
                data_node.children.push(val_node);
            } else if parts.len() == 1 {
                let val_raw = parts[0].trim();
                let key = val_raw.trim_matches(|c| c == '\'' || c == '"').to_string();
                let val_node = Node {
                    name: key,
                    value: Some("true".to_string()),
                    children: Vec::new(),
                    line: 1,
                    col: 1,
                    filename: filename.to_string(),
                };
                data_node.children.push(val_node);
            }
        }
        Some(data_node)
    } else if s.starts_with('$') {
        Some(Node {
            name: "data_var".to_string(),
            value: Some(s.to_string()),
            children: Vec::new(),
            line: 1,
            col: 1,
            filename: filename.to_string(),
        })
    } else {
        None
    }
}

pub fn transpile_blade_native(content: &str, filename: &str) -> Result<Node, String> {
    let mut root = Node {
        name: "do".to_string(),
        value: None,
        children: Vec::new(),
        line: 1,
        col: 1,
        filename: filename.to_string(),
    };

    let mut pos = 0;
    let mut extends_file: Option<String> = None;

    while pos < content.len() {
        let next_trigger = content[pos..].find(|c| c == '@' || c == '{' || c == '<');
        let next_idx = match next_trigger {
            Some(idx) => pos + idx,
            None => {
                let text = &content[pos..];
                if !text.is_empty() {
                    root.children.push(Node {
                        name: "__native_write".to_string(),
                        value: Some(text.to_string()),
                        children: Vec::new(),
                        line: 1,
                        col: 1,
                        filename: filename.to_string(),
                    });
                }
                break;
            }
        };

        if next_idx > pos {
            let text = &content[pos..next_idx];
            root.children.push(Node {
                name: "__native_write".to_string(),
                value: Some(text.to_string()),
                children: Vec::new(),
                line: 1,
                col: 1,
                filename: filename.to_string(),
            });
        }

        pos = next_idx;

        if content[pos..].starts_with("<x-") {
            let mut tag_end = None;
            let mut in_quote = false;
            let mut quote_char = ' ';
            let chars: Vec<char> = content[pos..].chars().collect();
            for (i, &c) in chars.iter().enumerate() {
                if in_quote {
                    if c == quote_char {
                        in_quote = false;
                    }
                } else {
                    if c == '"' || c == '\'' {
                        in_quote = true;
                        quote_char = c;
                    }
                    if c == '>' {
                        tag_end = Some(i);
                        break;
                    }
                }
            }

            let tag_end = match tag_end {
                Some(idx) => idx,
                None => {
                    root.children.push(Node {
                        name: "__native_write".to_string(),
                        value: Some("<".to_string()),
                        children: Vec::new(),
                        line: 1,
                        col: 1,
                        filename: filename.to_string(),
                    });
                    pos += 1;
                    continue;
                }
            };

            let tag_len_in_bytes: usize = chars[..tag_end + 1].iter().map(|c| c.len_utf8()).sum();
            let full_tag = &content[pos..pos + tag_len_in_bytes];
            let is_self_closing = full_tag.ends_with("/>");

            let mut inner = full_tag;
            if is_self_closing {
                inner = inner.strip_suffix("/>").unwrap_or(inner);
            } else {
                inner = inner.strip_suffix(">").unwrap_or(inner);
            }
            inner = inner.strip_prefix("<").unwrap_or(inner);
            inner = inner.trim();

            let parts: Vec<&str> = inner.splitn(2, ' ').collect();
            let tag_name = parts[0];
            let attrs_raw = if parts.len() > 1 { parts[1] } else { "" };

            let mut node_name = "view.component".to_string();
            let mut node_value = tag_name.strip_prefix("x-").unwrap_or(tag_name).to_string();

            if tag_name == "x-slot" {
                node_name = "slot".to_string();
                node_value = String::new();
            }

            let mut comp_node = Node {
                name: node_name.clone(),
                value: Some(node_value),
                children: Vec::new(),
                line: 1,
                col: 1,
                filename: filename.to_string(),
            };

            let attr_nodes = parse_blade_attributes(attrs_raw, filename);
            comp_node.children.extend(attr_nodes.clone());

            if node_name == "slot" {
                for attr in &attr_nodes {
                    if attr.name == "name" {
                        comp_node.value = attr.value.clone();
                    }
                }
            }

            if is_self_closing {
                root.children.push(comp_node);
                pos += tag_len_in_bytes;
            } else {
                let block_start = pos + tag_len_in_bytes;
                let block_end = match find_end_component(&content[block_start..], tag_name) {
                    Some(idx) => idx,
                    None => return Err(format!("unclosed component {}", tag_name)),
                };

                let absolute_block_end = block_start + block_end;
                let body_content = &content[block_start..absolute_block_end];
                let body_node = transpile_blade_native(body_content, filename)?;

                if node_name == "slot" {
                    comp_node.children.extend(body_node.children);
                } else {
                    let mut default_slot_node = Node {
                        name: "default_slot".to_string(),
                        value: None,
                        children: Vec::new(),
                        line: 1,
                        col: 1,
                        filename: filename.to_string(),
                    };

                    for child in body_node.children {
                        if child.name == "slot" {
                            comp_node.children.push(child);
                        } else {
                            default_slot_node.children.push(child);
                        }
                    }

                    if !default_slot_node.children.is_empty() {
                        comp_node.children.push(default_slot_node);
                    }
                }

                root.children.push(comp_node);

                let end_tag_scan = &content[absolute_block_end..];
                if let Some(end_tag_close) = end_tag_scan.find('>') {
                    pos = absolute_block_end + end_tag_close + 1;
                } else {
                    let closing_len = 2 + tag_name.len() + 1;
                    pos = absolute_block_end + closing_len;
                }
            }
        } else if content[pos..].starts_with("{{--") {
            if let Some(end_comment) = content[pos..].find("--}}") {
                pos += end_comment + 4;
            } else {
                pos = content.len();
            }
        } else if content[pos..].starts_with("{!!") {
            if let Some(end_echo) = content[pos..].find("!!}") {
                let var_val = content[pos + 3..pos + end_echo].trim().to_string();
                root.children.push(Node {
                    name: "__native_write".to_string(),
                    value: Some(var_val),
                    children: Vec::new(),
                    line: 1,
                    col: 1,
                    filename: filename.to_string(),
                });
                pos += end_echo + 3;
            } else {
                root.children.push(Node {
                    name: "__native_write".to_string(),
                    value: Some("{!!".to_string()),
                    children: Vec::new(),
                    line: 1,
                    col: 1,
                    filename: filename.to_string(),
                });
                pos += 3;
            }
        } else if content[pos..].starts_with("{{") {
            if let Some(end_echo) = content[pos..].find("}}") {
                let var_val = content[pos + 2..pos + end_echo].trim().to_string();
                root.children.push(Node {
                    name: "__native_write_safe".to_string(),
                    value: Some(var_val),
                    children: Vec::new(),
                    line: 1,
                    col: 1,
                    filename: filename.to_string(),
                });
                pos += end_echo + 2;
            } else {
                root.children.push(Node {
                    name: "__native_write".to_string(),
                    value: Some("{{".to_string()),
                    children: Vec::new(),
                    line: 1,
                    col: 1,
                    filename: filename.to_string(),
                });
                pos += 2;
            }
        } else if content[pos..].starts_with("@csrf") {
            root.children.push(Node {
                name: "__native_write".to_string(),
                value: Some("$csrf_field".to_string()),
                children: Vec::new(),
                line: 1,
                col: 1,
                filename: filename.to_string(),
            });
            pos += 5;
        } else if content[pos..].starts_with("@extends") {
            let start_paren = content[pos..].find('(');
            let end_paren = find_balanced_paren(&content[pos..]);
            if let (Some(sp), Some(ep)) = (start_paren, end_paren) {
                let val_raw = content[pos + sp + 1..pos + ep].trim().trim_matches(|c| c == '\'' || c == '"').to_string();
                extends_file = Some(val_raw);
                pos += ep + 1;
            } else {
                pos += 8;
            }
        } else if content[pos..].starts_with("@yield") {
            let start_paren = content[pos..].find('(');
            let end_paren = find_balanced_paren(&content[pos..]);
            if let (Some(sp), Some(ep)) = (start_paren, end_paren) {
                let val_raw = content[pos + sp + 1..pos + ep].trim().trim_matches(|c| c == '\'' || c == '"').to_string();
                root.children.push(Node {
                    name: "section.yield".to_string(),
                    value: Some(val_raw),
                    children: Vec::new(),
                    line: 1,
                    col: 1,
                    filename: filename.to_string(),
                });
                pos += ep + 1;
            } else {
                pos += 6;
            }
        } else if content[pos..].starts_with("@stack") {
            let start_paren = content[pos..].find('(');
            let end_paren = find_balanced_paren(&content[pos..]);
            if let (Some(sp), Some(ep)) = (start_paren, end_paren) {
                let val_raw = content[pos + sp + 1..pos + ep].trim().trim_matches(|c| c == '\'' || c == '"').to_string();
                root.children.push(Node {
                    name: "view.stack".to_string(),
                    value: Some(val_raw),
                    children: Vec::new(),
                    line: 1,
                    col: 1,
                    filename: filename.to_string(),
                });
                pos += ep + 1;
            } else {
                pos += 6;
            }
        } else if content[pos..].starts_with("@section") {
            let start_paren = content[pos..].find('(');
            let end_paren = find_balanced_paren(&content[pos..]);
            if let (Some(sp), Some(ep)) = (start_paren, end_paren) {
                let val_raw = content[pos + sp + 1..pos + ep].trim().trim_matches(|c| c == '\'' || c == '"').to_string();
                let block_start = pos + ep + 1;
                if let Some(block_end) = find_end_directive(&content[block_start..], "@section", "@endsection") {
                    let body_content = &content[block_start..block_start + block_end];
                    let body_node = transpile_blade_native(body_content, filename)?;
                    
                    root.children.push(Node {
                        name: "section.define".to_string(),
                        value: Some(val_raw),
                        children: vec![Node {
                            name: "do".to_string(),
                            value: None,
                            children: body_node.children,
                            line: 1,
                            col: 1,
                            filename: filename.to_string(),
                        }],
                        line: 1,
                        col: 1,
                        filename: filename.to_string(),
                    });
                    pos = block_start + block_end + 11;
                } else {
                    return Err("unclosed @section".to_string());
                }
            } else {
                pos += 8;
            }
        } else if content[pos..].starts_with("@push") {
            let start_paren = content[pos..].find('(');
            let end_paren = find_balanced_paren(&content[pos..]);
            if let (Some(sp), Some(ep)) = (start_paren, end_paren) {
                let val_raw = content[pos + sp + 1..pos + ep].trim().trim_matches(|c| c == '\'' || c == '"').to_string();
                let block_start = pos + ep + 1;
                if let Some(block_end) = find_end_directive(&content[block_start..], "@push", "@endpush") {
                    let body_content = &content[block_start..block_start + block_end];
                    let body_node = transpile_blade_native(body_content, filename)?;
                    
                    root.children.push(Node {
                        name: "view.push".to_string(),
                        value: Some(val_raw),
                        children: vec![Node {
                            name: "do".to_string(),
                            value: None,
                            children: body_node.children,
                            line: 1,
                            col: 1,
                            filename: filename.to_string(),
                        }],
                        line: 1,
                        col: 1,
                        filename: filename.to_string(),
                    });
                    pos = block_start + block_end + 8;
                } else {
                    return Err("unclosed @push".to_string());
                }
            } else {
                pos += 5;
            }
        } else if content[pos..].starts_with("@include") {
            let start_paren = content[pos..].find('(');
            let end_paren = find_balanced_paren(&content[pos..]);
            if let (Some(sp), Some(ep)) = (start_paren, end_paren) {
                let args_raw = &content[pos + sp + 1..pos + ep];
                let args = split_blade_args(args_raw);
                if !args.is_empty() {
                    let view_name = args[0].trim().trim_matches(|c| c == '\'' || c == '"').to_string();
                    let mut include_node = Node {
                        name: "view.include".to_string(),
                        value: Some(view_name),
                        children: Vec::new(),
                        line: 1,
                        col: 1,
                        filename: filename.to_string(),
                    };
                    if args.len() > 1 {
                        if let Some(data_node) = parse_blade_data(&args[1], filename) {
                            include_node.children.push(data_node);
                        }
                    }
                    root.children.push(include_node);
                }
                pos += ep + 1;
            } else {
                pos += 8;
            }
        } else if content[pos..].starts_with("@foreach") {
            let start_paren = content[pos..].find('(');
            let end_paren = find_balanced_paren(&content[pos..]);
            if let (Some(sp), Some(ep)) = (start_paren, end_paren) {
                let def_raw = &content[pos + sp + 1..pos + ep];
                let parts: Vec<&str> = def_raw.split(" as ").collect();
                if parts.len() == 2 {
                    let list_var = parts[0].trim().to_string();
                    let item_var = parts[1].trim().trim_start_matches('$').to_string();
                    let block_start = pos + ep + 1;
                    if let Some(block_end) = find_end_directive(&content[block_start..], "@foreach", "@endforeach") {
                        let body_content = &content[block_start..block_start + block_end];
                        let body_node = transpile_blade_native(body_content, filename)?;
                        
                        root.children.push(Node {
                            name: "for".to_string(),
                            value: Some(list_var),
                            children: vec![
                                Node {
                                    name: "as".to_string(),
                                    value: Some(item_var),
                                    children: Vec::new(),
                                    line: 1,
                                    col: 1,
                                    filename: filename.to_string(),
                                },
                                Node {
                                    name: "do".to_string(),
                                    value: None,
                                    children: body_node.children,
                                    line: 1,
                                    col: 1,
                                    filename: filename.to_string(),
                                }
                            ],
                            line: 1,
                            col: 1,
                            filename: filename.to_string(),
                        });
                        pos = block_start + block_end + 11;
                    } else {
                        return Err("unclosed @foreach".to_string());
                    }
                } else {
                    pos += 8;
                }
            } else {
                pos += 8;
            }
        } else if content[pos..].starts_with("@forelse") {
            let start_paren = content[pos..].find('(');
            let end_paren = find_balanced_paren(&content[pos..]);
            if let (Some(sp), Some(ep)) = (start_paren, end_paren) {
                let def_raw = &content[pos + sp + 1..pos + ep];
                let parts: Vec<&str> = def_raw.split(" as ").collect();
                if parts.len() == 2 {
                    let list_var = parts[0].trim().to_string();
                    let item_var = parts[1].trim().trim_start_matches('$').to_string();
                    
                    let block_start = pos + ep + 1;
                    let block_end = match find_end_forelse(&content[block_start..]) {
                        Some(idx) => idx,
                        None => return Err("unclosed @forelse".to_string()),
                    };
                    
                    let absolute_block_end = block_start + block_end;
                    let full_block_content = &content[block_start..absolute_block_end];
                    
                    // Custom scan for @empty
                    let mut empty_pos = None;
                    let mut d = 0;
                    let mut scan_pos = 0;
                    let bytes = full_block_content.as_bytes();
                    while scan_pos < bytes.len() {
                        if full_block_content[scan_pos..].starts_with("@empty") {
                            if d == 0 {
                                empty_pos = Some(scan_pos);
                                break;
                            }
                        } else if full_block_content[scan_pos..].starts_with("@foreach") 
                            || full_block_content[scan_pos..].starts_with("@forelse") {
                            d += 1;
                        } else if full_block_content[scan_pos..].starts_with("@endforeach") 
                            || full_block_content[scan_pos..].starts_with("@endforelse") {
                            d -= 1;
                        }
                        scan_pos += 1;
                    }
                    
                    let (body_content, empty_content) = match empty_pos {
                        Some(pos) => (
                            &full_block_content[..pos],
                            Some(&full_block_content[pos + 6..])
                        ),
                        None => (full_block_content, None),
                    };
                    
                    let body_node = transpile_blade_native(body_content, filename)?;
                    
                    let mut forelse_node = Node {
                        name: "forelse".to_string(),
                        value: Some(list_var),
                        children: vec![
                            Node {
                                name: "as".to_string(),
                                value: Some(item_var),
                                children: Vec::new(),
                                line: 1,
                                col: 1,
                                filename: filename.to_string(),
                            },
                            Node {
                                name: "do".to_string(),
                                value: None,
                                children: body_node.children,
                                line: 1,
                                col: 1,
                                filename: filename.to_string(),
                            }
                        ],
                        line: 1,
                        col: 1,
                        filename: filename.to_string(),
                    };
                    
                    if let Some(ec) = empty_content {
                        let empty_node = transpile_blade_native(ec, filename)?;
                        forelse_node.children.push(Node {
                            name: "forelse_empty".to_string(),
                            value: None,
                            children: empty_node.children,
                            line: 1,
                            col: 1,
                            filename: filename.to_string(),
                        });
                    }
                    
                    root.children.push(forelse_node);
                    pos = absolute_block_end + 11;
                } else {
                    return Err("invalid @forelse format".to_string());
                }
            } else {
                pos += 8;
            }
        } else if content[pos..].starts_with("@class") {
            let start_paren = content[pos..].find('(');
            let end_paren = find_balanced_paren(&content[pos..]);
            if let (Some(sp), Some(ep)) = (start_paren, end_paren) {
                let args_raw = &content[pos + sp + 1..pos + ep];
                if let Some(data_node) = parse_blade_data(args_raw, filename) {
                    root.children.push(Node {
                        name: "view.class".to_string(),
                        value: None,
                        children: vec![data_node],
                        line: 1,
                        col: 1,
                        filename: filename.to_string(),
                    });
                }
                pos += ep + 1;
            } else {
                pos += 6;
            }
        } else if content[pos..].starts_with("@method") {
            let start_paren = content[pos..].find('(');
            let end_paren = find_balanced_paren(&content[pos..]);
            if let (Some(sp), Some(ep)) = (start_paren, end_paren) {
                let method_raw = content[pos + sp + 1..pos + ep].trim().trim_matches(|c| c == '\'' || c == '"').to_string();
                let html_output = format!(r#"<input type="hidden" name="_method" value="{}">"#, method_raw);
                root.children.push(Node {
                    name: "__native_write".to_string(),
                    value: Some(html_output),
                    children: Vec::new(),
                    line: 1,
                    col: 1,
                    filename: filename.to_string(),
                });
                pos += ep + 1;
            } else {
                pos += 7;
            }
        } else if content[pos..].starts_with("@if") {
            let start_paren = content[pos..].find('(');
            let end_paren = find_balanced_paren(&content[pos..]);
            if let (Some(sp), Some(ep)) = (start_paren, end_paren) {
                let cond_raw = content[pos + sp + 1..pos + ep].trim().to_string();
                let block_start = pos + ep + 1;
                if let Some((block_end, match_type)) = find_end_if(&content[block_start..]) {
                    let true_content = &content[block_start..block_start + block_end];
                    let true_node = transpile_blade_native(true_content, filename)?;
                    
                    let mut else_node: Option<Node> = None;
                    if match_type == "else" {
                        let else_start = block_start + block_end + 5;
                        if let Some((else_end, match_type2)) = find_end_if(&content[else_start..]) {
                            if match_type2 != "endif" {
                                return Err("unclosed @else (expected @endif)".to_string());
                            }
                            let else_content = &content[else_start..else_start + else_end];
                            let else_body = transpile_blade_native(else_content, filename)?;
                            else_node = Some(Node {
                                name: "else".to_string(),
                                value: None,
                                children: else_body.children,
                                line: 1,
                                col: 1,
                                filename: filename.to_string(),
                            });
                            pos = else_start + else_end + 6;
                        } else {
                            return Err("unclosed @else".to_string());
                        }
                    } else {
                        pos = block_start + block_end + 6;
                    }

                    let mut if_children = vec![Node {
                        name: "then".to_string(),
                        value: None,
                        children: true_node.children,
                        line: 1,
                        col: 1,
                        filename: filename.to_string(),
                    }];
                    if let Some(en) = else_node {
                        if_children.push(en);
                    }

                    root.children.push(Node {
                        name: "if".to_string(),
                        value: Some(cond_raw),
                        children: if_children,
                        line: 1,
                        col: 1,
                        filename: filename.to_string(),
                    });
                } else {
                    return Err("unclosed @if".to_string());
                }
            } else {
                pos += 3;
            }
        } else if content[pos..].starts_with("@zeno") {
            let block_start = pos + 5;
            if let Some(block_end) = content[block_start..].find("@endzeno") {
                let code_raw = &content[block_start..block_start + block_end];
                match zenocore::parser::parse_string(code_raw, filename) {
                    Ok(parsed_node) => {
                        root.children.extend(parsed_node.children);
                    }
                    Err(e) => {
                        return Err(format!("compile error in @zeno block: {:?}", e));
                    }
                }
                pos = block_start + block_end + 8;
            } else {
                return Err("unclosed @zeno".to_string());
            }
        } else {
            let c = &content[pos..pos + 1];
            root.children.push(Node {
                name: "__native_write".to_string(),
                value: Some(c.to_string()),
                children: Vec::new(),
                line: 1,
                col: 1,
                filename: filename.to_string(),
            });
            pos += 1;
        }
    }

    if let Some(ext_file) = extends_file {
        root.children.push(Node {
            name: "view.extends".to_string(),
            value: Some(ext_file),
            children: Vec::new(),
            line: 1,
            col: 1,
            filename: filename.to_string(),
        });
    }

    Ok(root)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transpile_simple() {
        let blade = "Hello {{ $name }}!";
        let node = transpile_blade_native(blade, "test.blade.zl").unwrap();
        assert_eq!(node.name, "do");
        assert_eq!(node.children.len(), 3);
        assert_eq!(node.children[0].name, "__native_write");
        assert_eq!(node.children[0].value, Some("Hello ".to_string()));
        assert_eq!(node.children[1].name, "__native_write_safe");
        assert_eq!(node.children[1].value, Some("$name".to_string()));
        assert_eq!(node.children[2].name, "__native_write");
        assert_eq!(node.children[2].value, Some("!".to_string()));
    }

    #[test]
    fn test_transpile_if() {
        let blade = "@if($cond)Yes @else No @endif";
        let node = transpile_blade_native(blade, "test.blade.zl").unwrap();
        assert_eq!(node.name, "do");
        assert_eq!(node.children.len(), 1);
        let if_node = &node.children[0];
        assert_eq!(if_node.name, "if");
        assert_eq!(if_node.value, Some("$cond".to_string()));
        assert_eq!(if_node.children.len(), 2);
        assert_eq!(if_node.children[0].name, "then");
        assert_eq!(if_node.children[0].children[0].value, Some("Yes ".to_string()));
        assert_eq!(if_node.children[1].name, "else");
        assert_eq!(if_node.children[1].children[0].value, Some(" No ".to_string()));
    }

    #[test]
    fn test_transpile_forelse() {
        let blade = "@forelse($items as $item)\n    Item: {{ $item }}\n@empty\n    No items.\n@endforelse";
        let node = transpile_blade_native(blade, "test.blade.zl").unwrap();
        println!("{:#?}", node);
        assert_eq!(node.children.len(), 1);
        let forelse_node = &node.children[0];
        assert_eq!(forelse_node.name, "forelse");
        assert_eq!(forelse_node.value, Some("$items".to_string()));
        // children: [as, do, forelse_empty]
        let has_as = forelse_node.children.iter().any(|c| c.name == "as");
        let has_do = forelse_node.children.iter().any(|c| c.name == "do");
        let has_empty = forelse_node.children.iter().any(|c| c.name == "forelse_empty");
        assert!(has_as, "Missing 'as' child");
        assert!(has_do, "Missing 'do' child");
        assert!(has_empty, "Missing 'forelse_empty' child");
    }
}
