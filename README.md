<div align="center">

<img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white" />
<img src="https://img.shields.io/badge/version-0.2.0-orange?style=for-the-badge" />
<img src="https://img.shields.io/badge/license-Apache--2.0-blue?style=for-the-badge" />
<img src="https://img.shields.io/badge/crates.io-v0.2.0-fc8d62?style=for-the-badge&logo=rust" />
<img src="https://img.shields.io/badge/tests-27%2F27%20passing-brightgreen?style=for-the-badge" />
<img src="https://img.shields.io/badge/Blade-compatible-blueviolet?style=for-the-badge" />
<img src="https://img.shields.io/badge/plugins-native%20C--ABI-ff6b35?style=for-the-badge" />

# ⚡ zeno-rs

### High-Performance Laravel Blade Template Engine & ZenoLang Scripting Engine for Rust

> Write familiar `@if`, `@foreach`, `@extends`, `{{ $var }}`, and `<x-component>` Blade templates — executed directly at native Rust speed.

[Overview](#-overview) · [vs Tera](#-zeno-blade-vs-tera) · [Quickstart](#-quickstart) · [Native Plugins](#-native-rust-dynamic-plugins-so--dylib--dll) · [Blade Reference](#-blade-directives) · [Components](#-html-components) · [Hot Reload](#%EF%B8%8F-template-loading--hot-reload)

</div>

---

## 🌟 Overview

**`zeno-rs`** brings the developer experience of Laravel Blade templating to the Rust ecosystem. If you are familiar with Blade syntax, you can use your existing `.blade.zl` views seamlessly in Rust web services (such as Axum, Actix, or Tower) without learning a new template syntax.

### Key Features

- **100% Blade Compatible**: Supports `@extends`, `@section`, `@yield`, `@include`, `@forelse`, `@csrf`, `@method`, and `<x-component>` tags.
- **Smart Hot Reload**: Per-file `mtime` cache invalidation so template edits take effect instantly in development without restarting the server.
- **Embedded ZenoLang Execution**: Full scripting engine with 50+ built-in slots for string manipulation, math calculations, arrays, and maps.
- **Native Dynamic Plugin System**: Load compiled Rust shared libraries (`.so`, `.dylib`, `.dll`) at runtime via FFI without recompiling the core engine.
- **Lightweight Footprint**: Single binary deployment with minimal memory consumption (~2-5 MB under load).

---

## 🆚 zeno-blade vs Tera

Tera is a popular template engine in the Rust ecosystem inspired by Jinja2/Django. **`zeno-blade`** is designed specifically for teams that prefer Laravel Blade syntax or require granular per-file hot reloading.

### Technical & DX Comparison

| Feature | zeno-blade | Tera | Notes |
|---|:---:|:---:|---|
| 🔥 **Per-file Hot Reload** | ✅ | ❌ | zeno-blade re-parses only modified files on save |
| 🎨 **Laravel Blade Syntax** | ✅ | ❌ | Direct support for `@extends`, `@section`, `{{ $var }}` |
| 🧩 **HTML Components (<x-component>)** | ✅ | ❌ | Native component encapsulation with slots |
| 📐 **Layout Inheritance** | ✅ | ✅ | Both support layout inheritance & block yields |
| 🔁 **Empty Loop Fallback (`@forelse`)** | ✅ | ❌ | Direct support for `@forelse ... @empty ... @endforelse` |
| 🎯 **Conditional Classes (`@class`)** | ✅ | ❌ | Directive for dynamic CSS class lists |
| 🔐 **Form Directives (`@csrf`, `@method`)** | ✅ | ❌ | Built-in form helpers |
| 🔌 **Native C-ABI Dynamic Plugins (`.so`)** | ✅ | ❌ | Load `.so`/`.dylib` Rust extensions at runtime |
| 🧰 **Built-in Utility Suite** | ✅ | ❌ | 50+ built-in math, string, array, map slots |
| 📄 **OpenAPI / Swagger UI Integration** | ✅ | ❌ | Bundled `zeno-apidoc` crate |

---

## 🔥 Workspace Architecture

**`zeno-rs`** is organized as a modular workspace:

```
zeno-rs/
├── crates/
│   ├── zenocore/             # 🔩 Core engine: lexer, parser, executor, scope, dynamic plugins
│   ├── zeno-blade/           # 🎨 Blade engine — transpiles .blade.zl → AST → HTML
│   ├── zeno-std/             # 🧰 Standard library: math, date, string, money
│   ├── zeno-apidoc/          # 📄 OpenAPI 3.0 spec + Swagger UI generator
│   ├── zenoengine/           # 📦 Batteries-included facade (recommended starting point)
│   └── zeno-plugin-example/  # 🔌 Example native C-ABI Rust dynamic plugin (.so)
└── examples/
    └── web_server/           # 🚀 Axum web server integration example
```

> Templates are **100% portable** between Go ([`zeno-go`](https://github.com/nextcore/zeno-go)) and Rust (`zeno-rs`) backends.

---

## ⚡ Quickstart

### 1. Add Dependencies

Add `zenoengine` to your `Cargo.toml`:

```toml
[dependencies]
zenoengine = "0.2"   # batteries-included facade
zenocore   = "0.2"   # core engine + plugin system
zeno-blade  = "0.2"  # or just the Blade engine
```

### 2. Render Blade Templates in Rust

```rust
use zenoengine::{new_engine, executor::Context, scope::{Scope, Value}};
use zeno_blade::{register_blade_slots, slots::HtmlBuffer};
use zenocore::parser::parse_string;

let engine = new_engine();
register_blade_slots(&engine);

let mut ctx = Context::new();
ctx.set("httpWriter", HtmlBuffer(std::sync::Mutex::new(String::new())));

let scope = Scope::new(None);
scope.set("_view_root", Value::String("resources/views".to_string()));
scope.set("user",  Value::String("Alex".to_string()));
scope.set("title", Value::String("Dashboard".to_string()));

let node = parse_string("view.blade: 'dashboard'", "main.zl").unwrap();
engine.execute(&mut ctx, &node, &scope).unwrap();

let html = ctx.get::<HtmlBuffer>("httpWriter").unwrap();
println!("{}", html.0.lock().unwrap()); // Rendered HTML output
```

### 3. Write Your Blade Template (`resources/views/dashboard.blade.zl`)

```html
@extends('layouts.app')

@section('content')
  <h1>Welcome back, {{ $user }}!</h1>

  @if($role == 'admin')
    <span class="badge badge-admin">Admin</span>
  @endif

  @forelse($posts as $post)
    <article><h2>{{ $post }}</h2></article>
  @empty
    <p>No recent posts.</p>
  @endforelse
@endsection
```

---

## 🔌 Native Rust Dynamic Plugins (`.so` / `.dylib` / `.dll`)

`zeno-rs` includes a **Native Dynamic Plugin System**. You can compile custom Rust code into a shared library (`.so`, `.dylib`, `.dll`) and load it dynamically into ZenoCore at runtime.

### 1. Write Plugin Crate (`cdylib`)

```rust
// Cargo.toml -> [lib] crate-type = ["cdylib"]
use zenocore::{Engine, Value, SlotMeta};
use std::sync::Arc;

#[unsafe(no_mangle)]
pub extern "C" fn zeno_plugin_init(engine: &Engine) {
    engine.register(
        "custom.sha256",
        Arc::new(|engine, _ctx, node, scope| {
            let input = engine.resolve_shorthand_value(node, scope).to_string_coerce();
            let hash = format!("{:x}", md5::compute(input.as_bytes()));
            scope.set("hash_result", Value::String(hash));
            Ok(())
        }),
        SlotMeta::default(),
    );
}
```

### 2. Load & Execute in ZenoLang Script (`.zl`)

```yaml
# Load shared library dynamically at runtime
plugin.load: './plugins/libcustom_plugin.so'

# Execute custom slot registered by the plugin
custom.sha256: 'secret data' {
    as: $hash
}

log: $hash
```

---

## 🎨 Blade Directives

| Directive | Description |
|-----------|-------------|
| `{{ $var }}` | Escaped output |
| `{!! $raw !!}` | Unescaped raw HTML output |
| `@if` / `@elseif` / `@else` / `@endif` | Conditional branches |
| `@foreach` / `@endforeach` | Array/list iteration |
| `@forelse` / `@empty` / `@endforelse` | Iteration with empty fallback block |
| `@extends('layout')` | Extend base template layout |
| `@section` / `@endsection` | Define content section |
| `@yield('name')` | Render content section in layout |
| `@include('partial')` | Include partial template |
| `@push('stack')` / `@stack('stack')` | Push to named stack |
| `@class(['cls' => $cond])` | Dynamic CSS class builder |
| `@csrf` / `@method('PUT')` | Form helpers |
| `{{-- comment --}}` | Server-side comment |

---

## 🧩 HTML Components

Encapsulate UI into reusable Blade components (`<x-component>`):

**Component definition** — `resources/views/components/alert.blade.zl`:
```html
<div @class(['alert', 'alert-danger' => $is_danger, 'alert-success' => $is_success])>
    <strong>{{ $header }}</strong>
    <p>{{ $slot }}</p>
</div>
```

**Usage:**
```html
<x-alert :is_danger="true">
    <x-slot name="header">Notice</x-slot>
    Your session will expire shortly.
</x-alert>
```

---

## ⚙️ Template Loading & Hot Reload

In development, `zeno-blade` checks the `mtime` timestamp of template files on each request. Only files that have been modified are re-parsed into AST. All unchanged templates remain cached in memory, providing instant feedback without restarting the server.

---

## 🧰 Built-in Slots Suite

ZenoCore includes 50+ built-in slots:

- **Logic**: `if` (`&&`, `||`, `==`, `!=`, `>`, `<`, `>=`, `<=`), `for`, `while`, `try`, `var`
- **String**: `string.trim`, `upper`, `lower`, `split`, `replace`, `contains`, `starts_with`, `ends_with`, `len`, `concat`, `substr`, `format`
- **Math**: `math.add`, `sub`, `mul`, `div`, `mod`, `pow`, `sqrt`, `abs`, `ceil`, `floor`, `round`, `min`, `max`, `clamp`, `random`
- **Collections**: `array.push`, `pop`, `shift`, `unshift`, `slice`, `reverse`, `sort`, `unique`, `contains`, `map.set`, `get`, `delete`, `merge`, `keys`, `values`, `has`, `entries`
- **Utilities**: `log`, `print`, `coalesce`, `cast.to_int/float/string/bool`, `include`, `util.datetime`, `util.timestamp`, `util.uuid`, `util.env`

---

## 🏗️ Build & Test

```bash
git clone https://github.com/nextcore/zeno-rs
cd zeno-rs

# Build plugin example
cargo build --package zeno-plugin-example

# Run workspace unit tests
cargo test
```

**Requirements:** Rust **1.85+** (Edition 2024)

---

## 📝 License

Apache 2.0 © [NextCore](https://github.com/nextcore)

---

<div align="center">

**Laravel Blade DX powered by Rust Performance.**

⭐ Star `zeno-rs` on GitHub if you find this project useful!

</div>
