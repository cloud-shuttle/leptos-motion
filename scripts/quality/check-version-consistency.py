#!/usr/bin/env python3
"""
Check that all crate versions are consistent with the workspace version.
"""

import toml
import sys
import os

def main():
    # Read workspace Cargo.toml
    with open("Cargo.toml", "r") as f:
        workspace = toml.load(f)

    workspace_version = workspace["workspace"]["package"]["version"]

    # Check all crate Cargo.toml files
    crates = [
        "leptos-motion-core",
        "leptos-motion-dom",
        "leptos-motion-gestures",
        "leptos-motion-layout",
        "leptos-motion-scroll",
        "leptos-motion-macros",
        "leptos-motion"
    ]

    for crate in crates:
        crate_path = f"crates/{crate}/Cargo.toml"
        if not os.path.exists(crate_path):
            print(f"Warning: {crate_path} not found, skipping...")
            continue

        with open(crate_path, "r") as f:
            crate_toml = toml.load(f)

        crate_version = crate_toml["package"]["version"]

        # Handle workspace version references
        if isinstance(crate_version, dict) and "workspace" in crate_version:
            # This is a workspace version reference, which is correct
            continue
        elif crate_version != workspace_version:
            print(f"Version mismatch: {crate} has {crate_version}, workspace has {workspace_version}")
            sys.exit(1)

    print("All versions are consistent!")
    return 0

if __name__ == "__main__":
    sys.exit(main())
