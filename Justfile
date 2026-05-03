# ------------------------------
# Settings
# ------------------------------

set windows-shell := ["powershell.exe", "-c"]
set allow-duplicate-recipes

# ------------------------------
# Requirements
# ------------------------------

cargo := require("cargo")
git_bin := require("git")
git := "& \"" + git_bin + "\"" # thanks powershell >:(

# ------------------------------
# Variables
# ------------------------------

build_profile := "dev"
build_package := ""

# ------------------------------
# Aliases
# ------------------------------

alias fmt := format
alias check := lint

# ------------------------------
# Tasks
# ------------------------------

[doc("Default task - will be run when no task is specified.")]
default:
    @just --list

[confirm("Are you sure you want to clean the build directory? (y/n)")]
[doc("Clean the Cargo build directory.")]
clean package=build_package:
    {{ cargo }} clean {{ if package == "" { "" } else { "-p " + package } }}

[doc("Format the code using rustfmt.")]
[group("fmt")]
format package=build_package:
    {{ cargo }} fmt {{ if package == "" { "--all" } else { "-p " + package } }} -- --emit=files

[doc("Run strict static analysis using Clippy.")]
[group("lint")]
lint package=build_package:
    {{ cargo }} clippy \
        --all-targets \
        --all-features \
        {{ if package == "" { "--all" } else { "-p " + package } }}

[doc("Check for unused dependencies.")]
[group("lint")]
udeps:
    {{ cargo }} udeps --all-targets --backend depinfo

[doc("Run all tests.")]
[group("test")]
test package=build_package:
    {{ cargo }} nextest run \
        {{ if package == "" { "--workspace" } else { "-p " + package } }} \
        --all-features \
        --all-targets

[doc("Run crate documentation tests.")]
[group("docs")]
[group("test")]
test-docs package=build_package:
    {{ cargo }} test {{ if package == "" { "--all" } else { "-p " + package } }} --doc

[doc("Run the integration test suite against a live Mono runtime.\
    \nArguments:\
    \n  mono_dll        : path to the Mono runtime DLL (mono-2.0-bdwgc.dll or equivalent)\
    \n  assemblies_path : directory containing mscorlib.dll; required when the DLL comes\
    \n                    from a Unity game whose assemblies are not under lib/mono/4.5/")]
[group("test")]
test-integration mono_dll assemblies_path="":
    $env:MONO_PATH = "{{ assemblies_path }}"; \
        {{ cargo }} run --features integration-tests --bin mono-rt-integration \
        -- "{{ mono_dll }}" \
        "tests\fixtures\MonoRtFixture.dll"

[doc("Bump the crate version in Cargo.toml and README.md, then regenerate Cargo.lock.\nExample: just bump 0.3.0")]
[group("release")]
bump version:
    $c = (Get-Content Cargo.toml -Raw) -replace '(?m)^version = "\d+\.\d+\.\d+"', 'version = "{{ version }}"'; [IO.File]::WriteAllText((Join-Path $PWD "Cargo.toml"), $c)
    $r = (Get-Content README.md -Raw) -replace 'mono-rt = "\d+\.\d+\.\d+"', 'mono-rt = "{{ version }}"'; [IO.File]::WriteAllText((Join-Path $PWD "README.md"), $r)
    {{ cargo }} generate-lockfile

[doc("Create a git tag for the current version and push it to the remote repository.")]
[group("release")]
release version:
    {{ git }} tag -a "v{{ version }}" -m "Release {{ version }}"
    {{ git }} push origin "v{{ version }}"

[doc("Build project. Use release parameter for release builds. Use package parameter to build a specific package.")]
[group("build")]
build profile=build_profile package=build_package:
    {{ cargo }} build \
        {{ if profile == "release" { "--release" } else { "" } }} \
        --all-targets \
        {{ if package == "" { "--workspace" } else { "-p " + package } }}

[doc("Build Rust crates documentation")]
[group("docs")]
build-rustdocs package=build_package:
    {{ cargo }} doc {{ if package == "" { "--all" } else { "-p " + package } }} --no-deps --document-private-items
