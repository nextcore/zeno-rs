<div align="center">

<img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white" />
<img src="https://img.shields.io/badge/version-0.1.0-orange?style=for-the-badge" />
<img src="https://img.shields.io/badge/license-MIT-blue?style=for-the-badge" />
<img src="https://img.shields.io/badge/tests-passing-brightgreen?style=for-the-badge" />
<img src="https://img.shields.io/badge/unsafe-0%25-success?style=for-the-badge" />
<img src="https://img.shields.io/badge/Blade-compatible-blueviolet?style=for-the-badge" />
<img src="https://img.shields.io/badge/hot%20reload-built--in-ff6b35?style=for-the-badge" />

# ⚡ zeno-rs

### Your Laravel Blade Templates. Now Running in Rust.

> You already know `@if`, `@foreach`, `@extends`, `{{ $var }}`, and `<x-component>`.  
> **You don't need to learn a new template language. You need a faster runtime.**

[Why Switch?](#-why-leave-php) · [vs Tera](#-zeno-blade-vs-tera--why-blade-wins) · [Quickstart](#-2-minute-migration) · [Blade Reference](#-blade-directives) · [Components](#-html-components) · [Hot Reload](#%EF%B8%8F-template-loading--hot-reload)

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
| 🔌 **Custom handler / slot system** | ✅ | ❌ | Register Rust functions callable from templates |
| 📄 **Built-in OpenAPI / Swagger UI** | ✅ | ❌ | Bundled in the zeno-rs workspace |
| 🛡️ **Zero unsafe code in core** | ✅ | ✅ | Both are memory-safe |
| 📦 **Maturity / ecosystem** | 🆕 | ✅ | Tera has a larger community — honest trade-off |

### Syntax: What You Already Know vs What You'd Have to Learn

```html
<!-- Tera — Jinja2-style, new syntax to learn -->
{% for post in posts %}
  {% if post.featured %}
    <article>{{ post.title | upper }}</article>
  {% endif %}
{% else %}
  <p>No posts.</p>
{% endfor %}
```

```html
{{-- zeno-blade — Laravel Blade, you already know this --}}
@forelse($posts as $post)
  @if($post_featured)
    <article>{{ $post }}</article>
  @endif
@empty
  <p>No posts.</p>
@endforelse
```

If you've written a single Laravel view, you already know how to write zeno-blade templates.

> [!NOTE]
> Tera is an excellent library and the right choice if you're not coming from a Blade background.  
> If you **are** — zeno-blade gives you Laravel's template DX at Rust's performance level.

---

## 🔥 What Exactly Is This?

**`zeno-rs`** is a Rust workspace (monorepo) containing:

```
zeno-rs/
├── crates/
│   ├── zenocore/        # 🔩 Core engine: lexer, parser, executor, scope — zero dependencies
│   ├── zeno-blade/      # 🎨 THE Blade engine — transpiles .blade.zl → AST → HTML
│   ├── zeno-std/        # 🧰 Standard library: math, date, string, money
│   ├── zeno-apidoc/     # 📄 OpenAPI 3.0 spec + Swagger UI
│   └── zenoengine/      # 📦 Batteries-included facade (start here)
└── examples/
    └── web_server/      # 🚀 Full Axum web server, ready to run
```

> **`zeno-blade`** is the star of the show — a full Blade engine living inside `zeno-rs`.  
> It is the Rust sibling of [`nextcore/zeno-go`](https://github.com/nextcore/zeno-go), the original Go implementation.  
> Templates are **100% portable** between Go and Rust backends.

---

## ⚡ 2-Minute Migration

### Step 1 — Add to `Cargo.toml`

```toml
[dependencies]
zenoengine = { git = "https://github.com/nextcore/zeno-rs" }
zeno-blade  = { git = "https://github.com/nextcore/zeno-rs" }
```

### Step 2 — Point it at your existing views directory

```rust
use std::sync::Mutex;
use zenoengine::{new_engine, executor::Context, scope::{Scope, Value}};
use zeno_blade::{register_blade_slots, slots::HtmlBuffer};
use zenocore::parser::parse_string;

let mut engine = new_engine();
register_blade_slots(&mut engine);

let mut ctx = Context::new();
ctx.set("httpWriter", HtmlBuffer(Mutex::new(String::new())));

let scope = Scope::new(None);
scope.set("_view_root", Value::String("resources/views".to_string())); // 👈 same path
scope.set("user",  Value::String("Andi".to_string()));
scope.set("title", Value::String("Dashboard".to_string()));

let node = parse_string("view.blade: 'dashboard'", "main.zl").unwrap();
engine.execute(&mut ctx, &node, &scope).unwrap();

let html = ctx.get::<HtmlBuffer>("httpWriter").unwrap();
println!("{}", html.0.lock().unwrap()); // ← your rendered HTML
```

### Step 3 — Your existing Blade templates work unchanged

```html
{{-- resources/views/dashboard.blade.zl — no changes needed --}}
@extends('layouts.app')

@section('content')
  <h1>Welcome, {{ $user }}!</h1>

  @if($role == 'admin')
    <span class="badge badge-danger">Admin</span>
  @endif

  @forelse($posts as $post)
    <article><h2>{{ $post }}</h2></article>
  @empty
    <p>No posts yet.</p>
  @endforelse
@endsection
```

That's it. **No rewrite. No new syntax. Just a faster runtime.**

---

## 🎨 Blade Directives

> **`zeno-blade`** transpiles `.blade.zl` templates to ZenoLang AST nodes, then executes them against the `zenocore` engine. The result is standard HTML — same as what Laravel would produce.

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

**Output:**
```html
<div class="alert alert-danger">
    <strong>Access Denied</strong>
    <p>You don't have permission to view this page.</p>
</div>
```

Props are **automatically isolated** — each component gets its own scope. No variable pollution.

---

## ⚙️ Template Loading & Hot Reload

> [!IMPORTANT]
> **Hot reload is the #1 reason to choose `zeno-blade` over Tera.**  
> See the [full comparison](#-zeno-blade-vs-tera--why-blade-wins) for details.

Most Rust template engines force a painful choice: either restart the server, or reload **everything** from scratch. `zeno-blade` does neither.

`zeno-blade` uses a **smart mtime-based per-file cache**:

1. Template loads → parsed to AST, stored in RAM. ⚡
2. Next request → check file's `modified time` (one lightweight syscall, no file read).
3. **File unchanged** → serve AST straight from RAM. Zero disk I/O.
4. **File changed** → re-read, re-parse, update cache automatically.

```
Edit template → Save → Refresh browser  ✅  (changes visible instantly)
No edits → Every subsequent request      ✅  (served from RAM, no disk touch)
```

**No env vars. No restart. No `cargo build`. Works out of the box.**

> [!TIP]
> Recompiling Rust is only needed when you change **Rust code** (handlers, slots, business logic).  
> Template changes — layouts, components, partials — are always hot-reloaded automatically.

### Preload mode (strict production)

If you want the server to **fail at startup** rather than at runtime when a template is missing:

```rust
use zeno_blade::transpiler::transpile_blade_native;

// In main() — warm up the entire cache before accepting requests
let views = ["dashboard", "layouts/app", "partials/header"];
for view in &views {
    let path = format!("resources/views/{}.blade.zl", view);
    let content = std::fs::read_to_string(&path)
        .unwrap_or_else(|_| panic!("Missing template at startup: {}", path));
    transpile_blade_native(&content, &path)
        .unwrap_or_else(|e| panic!("Template parse error at startup: {}", e));
}
// All templates pre-loaded. Server ready.
```

---

## 🧰 ZenoLang — The Logic Layer

Beyond Blade, `zeno-rs` includes **ZenoLang** — a readable, indented scripting language that powers the execution layer. You won't write it in templates directly (Blade directives handle that), but it's available for server-side scripts and custom logic:

```yaml
# Variables & types
set: $name = "Andi"
set: $score = 95
set: $tags = ['rust', 'fast', 'safe']

# Conditionals
if: $score >= 90 {
  then: { set: $grade = "A" }
  elseif: $score >= 80 { set: $grade = "B" }
  else: { set: $grade = "C" }
}

# Loops
for: $tags {
  as: $tag
  do: { log: "$loop.iteration. $tag" }
}

# Functions
fn: add {
  params: [$a, $b]
  do: { return: $a + $b }
}

# Error handling
try {
  do: { http.get: 'https://api.example.com/data' }
  catch: { log: "Failed: $error" }
}
```

---

## 🔌 Custom Slots (Extend the Engine)

Register your own handlers in Rust and call them from any template or script — like Laravel's custom Blade directives, but with the full power of Rust:

```rust
use std::sync::Arc;
use zenocore::{Engine, SlotMeta, Value};

fn register_my_slots(engine: &mut Engine) {
    engine.register(
        "db.find",
        Arc::new(|engine, _ctx, node, scope| {
            let table = engine.resolve_shorthand_value(node, scope).to_string_coerce();
            // ... query your database (sqlx, diesel, etc.)
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

Then call it from a template or script:
```yaml
db.find: 'users'
log: $result   # → Queried users
```

---

## 🚀 Axum Example

A full Axum web server with Blade rendering, ZenoLang execution, and Swagger UI — clone and run:

```bash
git clone https://github.com/nextcore/zeno-rs
cd zeno-rs
cargo run -p web_server_example
```

```
🚀 ZenoEngine Axum server running at http://127.0.0.1:3000
📖 Swagger UI at http://127.0.0.1:3000/docs
```

| Method | Path | Description |
|--------|------|-------------|
| `POST` | `/execute` | Execute a ZenoLang script |
| `GET` | `/docs` | Swagger UI |
| `GET` | `/openapi.json` | OpenAPI 3.0 spec |

---

## 📄 OpenAPI / Swagger (Bonus)

Auto-generate API docs from your routes with zero config — something you'd need a separate package for in Laravel:

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

## 🏗️ Build & Test

```bash
git clone https://github.com/nextcore/zeno-rs
cd zeno-rs

# Build all crates
cargo build

# Run all tests
cargo test --all

# Run only Blade engine tests
cargo test -p zeno-blade
```

**Requirements:** Rust **1.85+** (Edition 2024)

---

## 🔗 Ecosystem

| Repository | Language | Description |
|-----------|----------|-------------|
| [nextcore/zeno-go](https://github.com/nextcore/zeno-go) | Go | Original ZenoEngine — Go implementation |
| [nextcore/zeno-rs](https://github.com/nextcore/zeno-rs) | Rust | This repository — Rust port |

Templates written for `zeno-go` are **100% compatible** with `zeno-rs`.  
Same `.blade.zl` files. Same directives. Same component syntax. Different runtime.

---

## 📝 License

Apache 2.0 © [NextCore](https://github.com/nextcore)

---

<div align="center">

**Keep your Blade templates. Ditch the PHP overhead. Ship in Rust.**

> `zeno-rs` is the workspace — [`zeno-blade`](crates/zeno-blade) is the Blade engine inside it.

⭐ If this saves you a rewrite, give it a star!

</div>
