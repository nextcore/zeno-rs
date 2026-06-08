<div align="center">

<img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white" />
<img src="https://img.shields.io/badge/version-0.1.0-orange?style=for-the-badge" />
<img src="https://img.shields.io/badge/license-MIT-blue?style=for-the-badge" />
<img src="https://img.shields.io/badge/tests-26%20passing-brightgreen?style=for-the-badge" />
<img src="https://img.shields.io/badge/unsafe-0%25-success?style=for-the-badge" />

# ⚡ zeno-rs

### The Blazing-Fast Embedded Scripting & Blade Templating Engine for Rust

> Write logic. Render views. Ship faster.  
> Drop-in Blade templating + a full scripting runtime — embedded directly in your Rust app.

[Getting Started](#-quick-start) · [Blade Directives](#-blade-directives) · [Components](#-html-components) · [Examples](#-axum-example)

</div>

---

## Why zeno-rs?

Most Rust template engines give you one thing: a way to interpolate variables into HTML. **zeno-rs gives you a complete runtime.**

| | zeno-rs | Tera | Handlebars | MiniJinja |
|---|:---:|:---:|:---:|:---:|
| Blade-compatible syntax | ✅ | ❌ | ❌ | ❌ |
| HTML components (`<x-component>`) | ✅ | ❌ | ❌ | ❌ |
| Embedded scripting runtime | ✅ | ❌ | ❌ | ❌ |
| Layout inheritance (`@extends`) | ✅ | ✅ | ❌ | ✅ |
| Zero unsafe code in core | ✅ | ✅ | ✅ | ✅ |
| Custom slot/handler system | ✅ | ❌ | ❌ | ❌ |
| Built-in OpenAPI/Swagger docs | ✅ | ❌ | ❌ | ❌ |

If you're building a web application in Rust and want **Laravel-grade templating power** without the PHP overhead — this is it.

---

## ✨ Features

- 🎨 **Full Blade Syntax** — `@if`, `@foreach`, `@forelse`, `@extends`, `@section`, `@yield`, `@include`, `@push`, `@stack`, `@class`, `@method`, `@csrf` and more
- 🧩 **HTML Components** — `<x-alert>`, `<x-card>` with named slots, dynamic props, and isolated scopes
- ⚡ **ZenoLang Scripting** — Built-in expression language with variables, loops, functions, try/catch, switch, and pattern matching
- 🔌 **Pluggable Slots** — Register your own handlers in pure Rust and call them from templates
- 📄 **OpenAPI Generator** — Auto-generate API documentation from your routes
- 🛡️ **Zero External Deps in Core** — `zenocore` compiles with zero dependencies
- 🧪 **Production Tested** — Full test suite covering transpiler, executor, and slot behavior

---

## 📦 Workspace

```
zeno-rs/
├── crates/
│   ├── zenocore/        # 🔩 Core: lexer, parser, executor, scope — zero dependencies
│   ├── zeno-blade/      # 🎨 Blade transpiler + HTML component executor
│   ├── zeno-std/        # 🧰 Standard library: math, date, string, money
│   ├── zeno-apidoc/     # 📄 OpenAPI 3.0 spec + Swagger UI
│   └── zenoengine/      # 📦 Batteries-included facade
└── examples/
    └── web_server/      # 🚀 Axum example with Swagger UI
```

---

## 🚀 Quick Start

### Installation

```toml
# Cargo.toml
[dependencies]
zenoengine = { git = "https://github.com/nextcore/zeno-rs" }
```

Or pick only what you need:

```toml
[dependencies]
zenocore  = { git = "https://github.com/nextcore/zeno-rs" }
zeno-blade = { git = "https://github.com/nextcore/zeno-rs" }
```

### Execute a Script

```rust
use zenoengine::{new_engine, parser::parse_string, executor::Context, scope::{Scope, Value}};

let engine = new_engine();
let mut ctx = Context::new();
let scope = Scope::new(None);

scope.set("name", Value::String("World".to_string()));

let script = r#"
    if: $name == 'World' {
        then: { set: $greeting = "Hello, World! 🌍" }
        else: { set: $greeting = "Hello, stranger." }
    }
"#;

let root = parse_string(script, "main.zl").unwrap();
engine.execute(&mut ctx, &root, &scope).unwrap();

println!("{}", scope.get("greeting").unwrap().to_string_coerce());
// Hello, World! 🌍
```

### Render a Blade Template

```rust
use std::sync::Mutex;
use zenoengine::{new_engine, executor::Context, scope::{Scope, Value}};
use zeno_blade::{register_blade_slots, slots::HtmlBuffer, transpiler::parse_string};

let mut engine = new_engine();
register_blade_slots(&mut engine);

let mut ctx = Context::new();
ctx.set("httpWriter", HtmlBuffer(Mutex::new(String::new())));

let scope = Scope::new(None);
scope.set("_view_root", Value::String("resources/views".to_string()));
scope.set("title", Value::String("Dashboard".to_string()));

let node = parse_string("view.blade: 'dashboard'", "main.zl").unwrap();
engine.execute(&mut ctx, &node, &scope).unwrap();

let html = ctx.get::<HtmlBuffer>("httpWriter").unwrap();
println!("{}", html.0.lock().unwrap());
```

---

## 🎨 Blade Directives

Write templates that feel just like Laravel — because they are:

```html
@extends('layouts.app')

@section('content')

<h1>Welcome, {{ $user }}!</h1>

{{-- Comments are never rendered --}}

@if($role == 'admin')
    <span class="badge">Admin</span>
@elseif($role == 'moderator')
    <span class="badge">Mod</span>
@else
    <span class="badge">User</span>
@endif

@forelse($posts as $post)
    <article>
        <h2>{{ $post }}</h2>
    </article>
@empty
    <p>No posts yet. Start writing!</p>
@endforelse

<form method="POST" action="/update">
    @csrf
    @method('PUT')
    <button type="submit">Save</button>
</form>

@endsection

@push('scripts')
    <script src="/app.js"></script>
@endpush
```

### Full Directive Reference

| Directive | Description |
|-----------|-------------|
| `{{ $var }}` | Echo (HTML-escaped) |
| `{!! $raw !!}` | Echo raw/unescaped |
| `@if` / `@elseif` / `@else` / `@endif` | Conditional |
| `@foreach` / `@endforeach` | Standard loop |
| `@forelse` / `@empty` / `@endforelse` | Loop with empty fallback |
| `@extends('layout')` | Layout inheritance |
| `@section('name')` / `@endsection` | Define section |
| `@yield('name')` | Output section |
| `@include('partial')` | Include partial view |
| `@push('stack')` / `@stack('stack')` | Stacked content blocks |
| `@class(['cls' => $cond])` | Conditional CSS classes |
| `@method('PUT')` | HTTP method spoofing |
| `@csrf` | CSRF hidden input |
| `{{-- comment --}}` | Blade comment |

---

## 🧩 HTML Components

Define reusable, scoped components — just like Vue or Laravel Blade components.

**`resources/views/components/alert.blade.zl`:**
```html
<div @class(['alert', 'alert-danger' => $is_danger, 'alert-success' => $is_success])>
    <strong>{{ $header }}</strong>
    <p>{{ $slot }}</p>
</div>
```

**Use it anywhere:**
```html
<x-alert :is_danger="true">
    <x-slot name="header">Access Denied</x-slot>
    You don't have permission to view this page.
</x-alert>
```

**Output:**
```html
<div class="alert alert-danger">
    <strong>Access Denied</strong>
    <p>You don't have permission to view this page.</p>
</div>
```

Props are **automatically isolated** — the component gets its own scope. No pollution, no surprises.

---

## 🧰 ZenoLang Script Reference

ZenoLang powers the logic layer — an indented, readable scripting language:

```yaml
# Variables & types
set: $name = "Andi"
set: $score = 95
set: $active = true
set: $tags = ['rust', 'fast', 'safe']

# Conditionals
if: $score >= 90 {
  then: { set: $grade = "A" }
  elseif: $score >= 80 { set: $grade = "B" }
  else: { set: $grade = "C" }
}

# Loops with metadata
for: $tags {
  as: $tag
  do: {
    log: "$loop.iteration. $tag"
  }
}

# Forelse
forelse: $users {
  as: $user
  do: { log: $user }
  forelse_empty: { log: "No users found." }
}

# Functions
fn: add {
  params: [$a, $b]
  do: { return: $a + $b }
}
set: $result = add(10, 32)   # 42

# Error handling
try {
  do: { http.get: 'https://api.example.com/data' }
  catch: { log: "Failed: $error" }
}

# Switch
switch: $role {
  case 'admin': { log: "Full access" }
  case 'editor': { log: "Edit access" }
  default: { log: "Read only" }
}
```

---

## 🔌 Custom Slots

Extend the engine with your own handlers — call them from any template or script:

```rust
use std::sync::Arc;
use zenocore::{executor::Engine, slots::SlotMeta};

fn register_my_slots(engine: &mut Engine) {
    engine.register(
        "db.find",
        Arc::new(|engine, _ctx, node, scope| {
            let table = engine.resolve_shorthand_value(node, scope).to_string_coerce();
            // ... query your database
            scope.set("result", Value::String(format!("Queried {}", table)));
            Ok(())
        }),
        SlotMeta {
            description: "Query a database table".to_string(),
            ..Default::default()
        },
    );
}
```

```yaml
# In any template or script:
db.find: 'users'
log: $result   # Queried users
```

---

## 🚀 Axum Example

A full Axum web server with ZenoLang execution endpoint and Swagger UI — ready to run:

```bash
git clone https://github.com/nextcore/zeno-rs
cd zeno-rs
cargo run -p web_server_example
```

```
🚀 ZenoEngine Axum web server running at http://127.0.0.1:3000
📖 Swagger UI available at http://127.0.0.1:3000/docs
```

**Endpoints:**

| Method | Path | Description |
|--------|------|-------------|
| `POST` | `/execute` | Execute a ZenoLang script |
| `GET` | `/docs` | Swagger UI |
| `GET` | `/openapi.json` | OpenAPI 3.0 spec |

---

## 📄 OpenAPI / Swagger

Auto-generate API documentation from your routes with zero config:

```rust
use zenoengine::apidoc::{APIRegistry, RouteDoc};

let registry = APIRegistry::global();
registry.register("POST", "/users", RouteDoc {
    summary: "Create User".to_string(),
    tags: vec!["Users".to_string()],
    // ...
});

// GET /openapi.json → full OpenAPI 3.0 spec
// GET /docs         → interactive Swagger UI
```

---

## 🏗️ Building & Testing

```bash
# Clone
git clone https://github.com/nextcore/zeno-rs
cd zeno-rs

# Build entire workspace
cargo build

# Run all tests
cargo test --all

# Run specific crate tests
cargo test -p zeno-blade
```

**Requirements:** Rust **1.85+** (Edition 2024)

---

## 🔗 Ecosystem

| Repository | Language | Description |
|-----------|----------|-------------|
| [nextcore/zeno-go](https://github.com/nextcore/zeno-go) | Go | The original ZenoEngine implementation |
| [nextcore/zeno-rs](https://github.com/nextcore/zeno-rs) | Rust | This repository — Rust port |

Templates written for `zeno-go` are **100% compatible** with `zeno-rs`. Switch backends, keep your views.

---

## 📝 License

MIT © [NextCore](https://github.com/nextcore)

---

<div align="center">

**Built with ❤️ in Rust · Zero Unsafe · Zero Bloat · Full Power**

⭐ If zeno-rs saves you time, consider giving it a star!

</div>
