# ------------------------------
# Settings
# ------------------------------

set windows-shell := ["powershell.exe", "-c"]
set allow-duplicate-recipes

# ------------------------------
# Requirements
# ------------------------------

cargo := require("cargo")

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

[doc("Run all tests.")]
[group("test")]
test package=build_package:
    {{ cargo }} test {{ if package == "" { "--workspace" } else { "-p " + package } }}

[doc("Build project. Use release parameter for release builds. Use package parameter to build a specific package.")]
[group("build")]
build profile=build_profile package=build_package:
    {{ cargo }} build \
        {{ if profile == "release" { "--release" } else { "" } }} \
        --all-targets \
        {{ if package == "" { "--workspace" } else { "-p " + package } }}
