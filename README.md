# mono-rt

Dynamic bindings to the Mono runtime, designed for process injection into Unity games and other
Mono-hosted applications on Windows.

Rather than starting a new JIT domain, this crate attaches to a Mono runtime that is already
running in the target process. All exports are resolved at runtime via `GetModuleHandleW` and
`GetProcAddress`, so no import library or compile-time link to `mono.dll` is needed.

## Platform support

| Platform | Status |
|---|---|
| Windows | Supported |
| Linux | Planned but contributions welcome! |

The core binding layer is platform-agnostic; only `init()` uses a Windows-specific API to locate
the loaded module. A Linux port would replace that with `dlopen`/`dlsym` and is a self-contained
change.

## Getting started

Add the dependency:

```toml
[dependencies]
mono-rt = "0.1.0"
# or, for the latest commit:
mono-rt = { git = "https://github.com/theo-abel/mono-rt" }
```

Then in your injected code:

```rust
use mono_rt::prelude::*;

// 1. Resolve exports from the already-loaded Mono DLL.
//    Common names: "mono.dll" (Unity <= 2017), "mono-2.0-bdwgc.dll" (Unity 2018+)
mono_rt::init("mono-2.0-bdwgc.dll")?;

// 2. Attach the current thread. Keep the guard alive for the duration of all Mono work.
let _guard = unsafe { MonoThreadGuard::attach()? };

// 3. Navigate the assembly graph.
let image = MonoImage::find("Assembly-CSharp")?.expect("assembly not loaded");
let class = image.class_from_name("", "PlayerController")?.expect("class not found");

// 4. Look up a method and invoke it.
let method = class.method("Respawn", None)?.expect("method not found");
let domain = MonoDomain::root()?.expect("no root domain");
let obj = class.new_object(domain)?.expect("allocation failed");
let result = unsafe { method.invoke_with(obj.as_ptr(), &[Value::Bool(true)])? };

// 5. Read a field value directly from a live instance.
let hp_field = class.field("m_health")?.expect("field not found");
let offset = hp_field.offset()?;
// let hp: f32 = unsafe { mono_rt::read_field(obj_ptr, offset) };
```

## Threading model

Mono requires every thread that calls into the runtime to be registered with the garbage
collector. `MonoThreadGuard::attach()` handles this registration, and the guard automatically
deregisters the thread when dropped.

All handle types (`MonoClass`, `MonoObject`, `MonoMethod`, ...) are `!Send + !Sync`, they are
bound to the thread on which they were obtained. The compiler prevents them from crossing thread
boundaries silently. If you do need to transfer a handle to another thread where you can
guarantee both threads are attached, you can opt in with an explicit `unsafe impl Send`.

## Runtime API coverage

The table below reflects the current state. The goal is to cover the APIs most relevant to game
modding and runtime inspection; lower-level or rarely-needed functions can be added as needed.

| Area | Covered | Not yet covered |
|---|---|---|
| Initialization | `init`, thread attach/detach, root domain | domain unload, domain creation |
| Assembly | open by path, enumerate loaded, get image | get by name, get list from domain |
| Image | find by name, class lookup | enumerate all classes |
| Class | field/method by name, field/method enumeration, type descriptor, vtable, object allocation | parent class, interfaces, properties, events, nested types |
| Field | offset, name, type, static read | instance write, static write |
| Method | invoke (raw + typed `Value`), name | full signature, parameter types, return type, flags |
| Object | unbox | get class, get type, clone |
| String | create from `&str`, convert to `String` | |
| Array | length, element address | create, set element |
| Type | kind (`TypeKind` enum), boxing | is_valuetype, get_class |
| GC | — | pinned handles (`gc_handle_new`/`free`) |
| Exceptions | surface as `MonoError::ManagedException` | inspect message, stack trace |

## Safety

`unsafe` appears in two places in the public API:

- `MonoThreadGuard::attach()` : you assert that the returned guard will be dropped on the same
  thread that called `attach`.
- `MonoMethod::invoke` / `invoke_with` : you assert that the object and argument types match the
  method's actual signature, which Mono does not validate at the call site.
- `read_field` / `write_field` : you assert that the offset and type `T` are correct for the
  target field, and that the object pointer is live.

Everything else - null checks, CString conversion, error propagation - is handled by the library.

## Integration tests

The crate ships a standalone test binary (`mono-rt-integration`) that exercises every public
API layer against a real, live Mono runtime. Unlike the unit tests, this binary must run
against an actual Mono DLL, it is not part of `cargo nextest run`.

### Prerequisites

You need a `mono-2.0-bdwgc.dll` (Unity 2018+) or `mono-2.0-sgen.dll` (standard Mono
install) already on disk. The binary never modifies the DLL or the process it targets; it
only loads it in-process to call the inspection API.

### Running

Use the `test-integration` recipe. The first argument is the path to the Mono DLL; the
second (optional) argument is the directory that contains `mscorlib.dll`. The second
argument is only needed when the DLL comes from a game whose managed assemblies are not
stored under the standard `lib/mono/4.5/` layout next to the runtime.

**Standard Mono installation** (`choco install mono` or the official installer):

```powershell
just test-integration "C:\Program Files\Mono\bin\mono-2.0-sgen.dll"
```

**Unity game DLL** (assemblies live in `<Game>_Data\Managed\`):

```powershell
just test-integration `
    "C:\path\to\game\MonoBleedingEdge\EmbedRuntime\mono-2.0-bdwgc.dll" `
    "C:\path\to\game\GameName_Data\Managed"
```

A passing run prints one `[PASS]` line per test and exits with code 0:

```
[PASS] init_succeeds
[PASS] root_domain_is_some
...
[PASS] thread_guard_attach_drop
--- 16 passed, 0 failed ---
```

## Credits

Some inspiration was drawn from the [mono-rs](https://github.com/b4rti/mono-rs) project, particularly the public API design. Shoot out to Bartosz for paving the way!

## License

GPL-3.0-only. See [LICENSE](LICENSE).
