<div align="center">

<img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white" />
<img src="https://img.shields.io/badge/version-0.2.0-orange?style=for-the-badge" />
<img src="https://img.shields.io/badge/license-Apache--2.0-blue?style=for-the-badge" />
<img src="https://img.shields.io/badge/crates.io-v0.2.0-fc8d62?style=for-the-badge&logo=rust" />
<img src="https://img.shields.io/badge/tests-27%2F27%20passing-brightgreen?style=for-the-badge" />
<img src="https://img.shields.io/badge/Blade-compatible-blueviolet?style=for-the-badge" />
<img src="https://img.shields.io/badge/plugins-native%20C--ABI-ff6b35?style=for-the-badge" />

# ⚡ zeno-rs

### Your Laravel Blade Templates & ZenoLang Scripts. Now Running in Rust.

> You already know `@if`, `@foreach`, `@extends`, `{{ $var }}`, and `<x-component>`.  
> **You don't need to learn a new template language. You need a faster, extensible runtime.**

[Why Switch?](#-why-leave-php) · [vs Tera](#-zeno-blade-vs-tera--why-blade-wins) · [Quickstart](#-2-minute-migration) · [Native Plugins](#-native-rust-dynamic-plugins-so--dylib--dll) · [Blade Reference](#-blade-directives) · [Components](#-html-components) · [Hot Reload](#%EF%B8%8F-template-loading--hot-reload)

</div>

---

## 🤔 Why Leave PHP?

You love Laravel. The DX is excellent, the ecosystem is mature, and Blade is genuinely good.  
But at some point, every Laravel project hits the same wall:

| Problem | PHP/Laravel | zeno-rs (Rust) |
|---|---|---|
| Memory per request | ~20–50 MB (FPM workers) | ~2–5 MB (single binary) |
| Cold start | Opcache warm-up required | Instant — binary is pre-compiled |
| Concurrency | Process-per-request (FPM) or Swoole | Native async with Tokio / Axum |
| Deployment | PHP runtime + Composer + env | Single static binary, zero deps |
| Dynamic Extensions | Recompile PHP C extensions | **Load native `.so`/`.dylib` plugins dynamically** ✅ |
| Template syntax | Laravel Blade | **Identical Blade syntax** ✅ |

The catch with every other Rust web framework: **you have to throw away your templates.**  
Tera, Handlebars, MiniJinja — none of them speak Blade.

**zeno-rs does.** Your `.blade.zl` files work as-is.

---

## 🆚 zeno-blade vs Tera — Why Blade Wins

Tera is the most popular Rust template engine. It's solid, well-documented, and widely used.  
But if you're a Laravel developer — or if you care about developer experience — it falls short in ways that matter every day.

### The Hot Reload Problem (This Is the Big One)

Here's what your workflow looks like when you change a template:

**With Tera:**
```rust
// Option A: Restart the server every time.
// Option B: Call full_reload() — which re-reads and re-parses EVERY template.
tera.full_reload()?; // ← nukes the entire cache, re-parses all files
```
Tera has no per-file invalidation. Change one file → invalidate everything → re-parse everything.  
On a project with 50+ templates, this adds latency to every dev refresh.

**With zeno-blade:**
```
Edit one template → Save → Refresh browser

✅ Only that one file is re-parsed (mtime check = 1 syscall)
✅ Every other template stays in RAM untouched
✅ Zero manual reload call needed
✅ Zero restart needed
```

zeno-blade uses **mtime-based per-file cache invalidation**:  
check the file's last-modified timestamp on every request, re-parse only when it changes.  
It's the best of both worlds — RAM speed when nothing changed, instant pickup when you saved.

### Full Feature Comparison

| Feature | zeno-blade | Tera | Notes |
|---|:---:|:---:|---|
| 🔥 **Hot reload — auto, per-file** | ✅ | ❌ | Tera: call full_reload() to nuke entire cache |
| 🎨 **Laravel Blade syntax** | ✅ | ❌ | Tera uses Jinja2 / Django-like syntax |
| 🧩 **HTML components (<x-component>)** | ✅ | ❌ | Tera has no component system |
| 📐 **Layout inheritance (@extends)** | ✅ | ✅ | Both support @extends / @section / @yield |
| 🔁 **Loop with empty fallback (@forelse)** | ✅ | ❌ | No forelse equivalent in Tera |
| 🎯 **Conditional CSS classes (@class)** | ✅ | ❌ | Laravel-style @class directive |
| 🔐 **Form helpers (@csrf, @method)** | ✅ | ❌ | Tera has no form helpers |
| 🧠 **Embedded scripting (ZenoLang)** | ✅ | ❌ | Full scripting runtime built-in |
| 🔌 **Native C-ABI Dynamic Plugins (`.so`)** | ✅ | ❌ | Load `.so`/`.dylib` Rust extensions at runtime |
| 🧰 **Rich Built-in Slots (Math/String/Array)** | ✅ | ❌ | 50+ built-in math, string, array, map slots |
| 📄 **Built-in OpenAPI / Swagger UI** | ✅ | ❌ | Bundled in the zeno-rs workspace |
| 🛡️ **Zero unsafe code in core** | ✅ | ✅ | Thread-safe Mutex architecture |
| 📦 **Maturity / ecosystem** | 🆕 | ✅ | Tera has a larger community — honest trade-off |

---

## 🔥 What Exactly Is This?

**`zeno-rs`** is a Rust workspace (monorepo) containing:

```
zeno-rs/
├── crates/
│   ├── zenocore/             # 🔩 Core engine: lexer, parser, executor, scope, dynamic plugins
│   ├── zeno-blade/           # 🎨 THE Blade engine — transpiles .blade.zl → AST → HTML
│   ├── zeno-std/             # 🧰 Standard library: math, date, string, money
│   ├── zeno-apidoc/          # 📄 OpenAPI 3.0 spec + Swagger UI
│   ├── zenoengine/           # 📦 Batteries-included facade (start here)
│   └── zeno-plugin-example/  # 🔌 Example native C-ABI Rust dynamic plugin (.so)
└── examples/
    └── web_server/           # 🚀 Full Axum web server, ready to run
```

> **`zeno-blade`** is the star of the show — a full Blade engine living inside `zeno-rs`.  
> Templates are **100% portable** between Go (`zeno-go`) and Rust backends.

---

## ⚡ 2-Minute Migration

### Step 1 — Add to `Cargo.toml`

```toml
[dependencies]
zenoengine = "0.2"   # batteries-included facade
zenocore   = "0.2"   # core engine + native plugin system
zeno-blade  = "0.2"  # or just the Blade engine
```

All crates are published on **[crates.io](https://crates.io)**. No git URLs needed.

### Step 2 — Point it at your existing views directory

```rust
use zenoengine::{new_engine, executor::Context, scope::{Scope, Value}};
use zeno_blade::{register_blade_slots, slots::HtmlBuffer};
use zenocore::parser::parse_string;

let engine = new_engine();
register_blade_slots(&engine);

let mut ctx = Context::new();
ctx.set("httpWriter", HtmlBuffer(std::sync::Mutex::new(String::new())));

let scope = Scope::new(None);
scope.set("_view_root", Value::String("resources/views".to_string())); // 👈 same path
scope.set("user",  Value::String("Andi".to_string()));
scope.set("title", Value::String("Dashboard".to_string()));

let node = parse_string("view.blade: 'dashboard'", "main.zl").unwrap();
engine.execute(&mut ctx, &node, &scope).unwrap();

let html = ctx.get::<HtmlBuffer>("httpWriter").unwrap();
println!("{}", html.0.lock().unwrap()); // ← your rendered HTML
```

---

## 🔌 Native Rust Dynamic Plugins (`.so` / `.dylib` / `.dll`)

`zeno-rs` includes a **Native Dynamic Plugin System**. You can compile custom Rust code into a shared library (`.so`, `.dylib`, `.dll`) and load it dynamically into ZenoCore at runtime without recompiling your main server!

### 1. Write & Compile Plugin (`cdylib`)

```rust
// Cargo.toml -> [lib] crate-type = ["cdylib"]
use zenocore::{Engine, Value, SlotMeta, InputMeta};
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

### 2. Load and Use in ZenoLang Script (`.zl`)

```yaml
# Load shared library dynamically at runtime
plugin.load: './plugins/libcustom_plugin.so'

# Call new custom slot registered by the plugin
custom.sha256: 'my secret payload' {
    as: $hash
}

log: $hash
```

---

## 🎨 Blade Directives

Full directive support, identical to Laravel Blade:

```html
@extends('layouts.app')

@section('content')

<h1>Welcome, {{ $user }}!</h1>

{{-- Comments never appear in output --}}

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

### Directive Reference

| Directive | Laravel Blade | zeno-blade |
|-----------|:---:|:---:|
| `{{ $var }}` — escaped echo | ✅ | ✅ |
| `{!! $raw !!}` — raw echo | ✅ | ✅ |
| `@if` / `@elseif` / `@else` / `@endif` | ✅ | ✅ |
| `@foreach` / `@endforeach` | ✅ | ✅ |
| `@forelse` / `@empty` / `@endforelse` | ✅ | ✅ |
| `@extends('layout')` | ✅ | ✅ |
| `@section` / `@endsection` | ✅ | ✅ |
| `@yield('name')` | ✅ | ✅ |
| `@include('partial')` | ✅ | ✅ |
| `@push('stack')` / `@stack('stack')` | ✅ | ✅ |
| `@class(['cls' => $cond])` | ✅ | ✅ |
| `@method('PUT')` | ✅ | ✅ |
| `@csrf` | ✅ | ✅ |
| `{{-- comment --}}` | ✅ | ✅ |

---

## 🧩 HTML Components

Identical to Laravel Blade components — `<x-component>` with named slots and dynamic props.

**Define once** — `resources/views/components/alert.blade.zl`:
```html
<div @class(['alert', 'alert-danger' => $is_danger, 'alert-success' => $is_success])>
    <strong>{{ $header }}</strong>
    <p>{{ $slot }}</p>
</div>
```

**Use anywhere — same syntax as Laravel:**
```html
<x-alert :is_danger="true">
    <x-slot name="header">Access Denied</x-slot>
    You don't have permission to view this page.
</x-alert>
```

---

## 🧰 Built-in Slots (50+ Utilities)

ZenoCore includes a comprehensive suite of built-in slots:

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

# Run all 27 tests across workspace
cargo test
```

**Requirements:** Rust **1.85+** (Edition 2024)

---

## 📝 License

Apache 2.0 © [NextCore](https://github.com/nextcore)

---

<div align="center">

**Keep your Blade templates. Ditch the PHP overhead. Ship in Rust.**

⭐ If this saves you a rewrite, give it a star!

</div>
