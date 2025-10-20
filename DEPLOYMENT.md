# Web Deployment Instructions

This document explains how to deploy the Predators and Prey ecology simulator to GitHub Pages.

## Overview

The project is now configured to build for WebAssembly (WASM) and run in web browsers. However, the GitHub Actions workflow file needs to be added manually due to permission restrictions.

## Manual Setup Required

### Step 1: Add the GitHub Actions Workflow

You need to manually create the workflow file in your repository:

1. Go to https://github.com/maccam912/predators-and-prey
2. Click "Add file" → "Create new file"
3. Enter the path: `.github/workflows/deploy.yml`
4. Copy and paste the contents from the section below
5. Commit directly to your `main` or `master` branch

### Workflow File Contents

Create `.github/workflows/deploy.yml` with this content:

```yaml
name: Deploy to GitHub Pages

on:
  push:
    branches: [main, master]
  workflow_dispatch:

permissions:
  contents: write
  pages: write
  id-token: write

concurrency:
  group: "pages"
  cancel-in-progress: false

jobs:
  build-and-deploy:
    runs-on: ubuntu-latest
    timeout-minutes: 30

    steps:
      - name: Checkout repository
        uses: actions/checkout@v4

      - name: Install Rust toolchain
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: wasm32-unknown-unknown

      - name: Cache cargo registry
        uses: actions/cache@v4
        with:
          path: ~/.cargo/registry
          key: ${{ runner.os }}-cargo-registry-${{ hashFiles('**/Cargo.lock') }}
          restore-keys: |
            ${{ runner.os }}-cargo-registry-

      - name: Cache cargo index
        uses: actions/cache@v4
        with:
          path: ~/.cargo/git
          key: ${{ runner.os }}-cargo-index-${{ hashFiles('**/Cargo.lock') }}
          restore-keys: |
            ${{ runner.os }}-cargo-index-

      - name: Cache cargo build
        uses: actions/cache@v4
        with:
          path: target
          key: ${{ runner.os }}-cargo-build-target-${{ hashFiles('**/Cargo.lock') }}
          restore-keys: |
            ${{ runner.os }}-cargo-build-target-

      - name: Install wasm-bindgen-cli
        run: cargo install wasm-bindgen-cli --version 0.2.99

      - name: Build for WASM
        run: cargo build --release --target wasm32-unknown-unknown

      - name: Generate WASM bindings
        run: |
          wasm-bindgen --no-typescript --target web \
            --out-dir ./out/ \
            --out-name "predators-and-prey" \
            ./target/wasm32-unknown-unknown/release/predators-and-prey.wasm

      - name: Copy index.html to output
        run: cp index.html ./out/

      - name: Copy assets to output (if exists)
        run: |
          if [ -d "assets" ]; then
            cp -r assets ./out/
          fi

      - name: Setup Pages
        uses: actions/configure-pages@v4

      - name: Upload artifact
        uses: actions/upload-pages-artifact@v3
        with:
          path: './out'

      - name: Deploy to GitHub Pages
        id: deployment
        uses: actions/deploy-pages@v4
```

### Step 2: Enable GitHub Pages

After adding the workflow file:

1. Go to **Settings** → **Pages** in your repository
2. Under "Source", select **GitHub Actions**
3. Save the settings

### Step 3: Trigger Deployment

The workflow will automatically run when you:
- Push to the `main` or `master` branch
- Manually trigger it from the Actions tab

## Local Testing

To test the WASM build locally:

```bash
# Add WASM target (if not already added)
rustup target add wasm32-unknown-unknown

# Build for WASM
cargo build --release --target wasm32-unknown-unknown

# Install wasm-bindgen-cli
cargo install wasm-bindgen-cli

# Generate bindings
wasm-bindgen --no-typescript --target web \
  --out-dir ./out/ \
  --out-name "predators-and-prey" \
  ./target/wasm32-unknown-unknown/release/predators-and-prey.wasm

# Copy HTML
cp index.html ./out/

# Serve locally (requires a local server)
# You can use Python's http.server or any other local web server
cd out && python3 -m http.server 8080
```

Then open http://localhost:8080 in your browser.

## After Deployment

Once deployed, your ecology simulator will be available at:

**https://maccam912.github.io/predators-and-prey/**

## Changes Made

The following files were modified to support web deployment:

- **Cargo.toml**: Added WASM build profiles and `getrandom` "js" feature
- **src/main.rs**: Configured WindowPlugin for web with canvas targeting
- **index.html**: Created web page with proper Bevy canvas configuration
- **src/systems/movement.rs**: Fixed clippy warnings
