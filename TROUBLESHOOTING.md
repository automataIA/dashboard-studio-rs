# Troubleshooting Guide

## Common Issues and Solutions

### 1. Browser Error: "Uncaught RangeError: failed to grow table"

**Symptom**: When loading the app in the browser, you see:
```
Uncaught RangeError: failed to grow table
__wbindgen_init_externref_table
```

**Cause**: This error occurs when using **outdated `wasm-opt`** (version 108 or older) that doesn't properly support modern WASM features like bulk-memory operations.

**Solutions**:

#### Solution 1: Use GitHub Actions Deployment (Recommended)
GitHub Actions uses the latest Trunk and wasm-opt, which handle these features correctly:

```bash
# Push to GitHub
git add .
git commit -m "Deploy to GitHub Pages"
git push origin main
```

The workflow in `.github/workflows/deploy.yml` will automatically build and deploy.

#### Solution 2: Upgrade binaryen Locally

Check your current version:
```bash
wasm-opt --version
# Should show version 116 or higher
```

If version is < 116:

**Option A: Install from source**
```bash
# Remove old version
sudo apt remove binaryen

# Install from GitHub releases
wget https://github.com/WebAssembly/binaryen/releases/download/version_116/binaryen-version_116-x86_64-linux.tar.gz
tar xzf binaryen-version_116-x86_64-linux.tar.gz
sudo cp binaryen-version_116/bin/wasm-opt /usr/local/bin/
```

**Option B: Use Docker**
```bash
docker run --rm -v $(pwd):/src -w /src rustwasm/wasm-pack build --release
```

#### Solution 3: Development Mode (Local Testing)
Use development builds which skip wasm-opt:

```bash
trunk serve
# Access at http://localhost:3000/dashboard-studio-rs/
```

### 2. Build Error: "wasm-validator error in function X"

**Symptom**: During `trunk build --release`:
```
[wasm-validator error] unexpected false: Bulk memory operation (bulk memory is disabled)
```

**Cause**: Same as #1 - outdated wasm-opt.

**Solution**: Use one of the solutions from issue #1.

### 3. Tailwind CSS Not Working

**Symptom**: Styles don't apply, DaisyUI components look unstyled.

**Solutions**:

```bash
# Reinstall dependencies
npm install

# Manually rebuild CSS
npm run build:css

# Check output.css was generated
ls -lh public/output.css
```

### 4. Trunk Serve Fails to Start

**Symptom**: `trunk serve` exits with errors.

**Check**:
```bash
# Verify Trunk is installed
trunk --version

# Reinstall if needed
cargo install trunk --force

# Check Node.js version (should be 16+)
node --version
npm --version
```

### 5. WASM File Too Large

**Symptom**: After build, WASM file is > 5MB.

**Solutions**:

Ensure these settings in `Cargo.toml`:
```toml
[profile.release]
opt-level = 'z'      # Maximum size optimization
lto = true           # Link-time optimization
codegen-units = 1
panic = "abort"
```

Check Trunk is running with `--release`:
```bash
trunk build --release
```

### 6. GitHub Pages Shows 404

**Symptom**: Deployed to GitHub Pages but getting 404 errors.

**Solutions**:

1. **Check GitHub Pages settings**:
   - Go to repository Settings → Pages
   - Source should be "main branch" and "/docs folder"
   - Save changes

2. **Verify public_url in Trunk.toml**:
   ```toml
   [build]
   public_url = "/dashboard-studio-rs/"  # Must match repo name
   ```

3. **Check `.nojekyll` file exists**:
   ```bash
   ls docs/.nojekyll
   ```

4. **Force rebuild and push**:
   ```bash
   rm -rf docs dist
   git add .
   git commit -m "Rebuild GitHub Pages"
   git push origin main
   ```

### 7. Browser Console Shows Module Errors

**Symptom**:
```
Failed to fetch dynamically imported module
```

**Cause**: Incorrect `public_url` configuration.

**Solution**:

For GitHub Pages:
```toml
# Trunk.toml
[build]
public_url = "/your-repo-name/"  # Include leading and trailing slashes
```

For custom domain:
```toml
[build]
public_url = "/"
```

### 8. Development Server Hot Reload Not Working

**Symptom**: Changes to `.rs` files don't trigger browser refresh.

**Solutions**:

```bash
# Kill all trunk processes
pkill -f trunk

# Clear Trunk cache
rm -rf .trunk

# Restart server
trunk serve
```

### 9. CSV Upload Not Working

**Symptom**: File upload button doesn't respond.

**Check Browser Console**: Look for errors related to `FileReader` API.

**Solutions**:
- Ensure using modern browser (Chrome 90+, Firefox 88+, Safari 14+)
- Check CSV file encoding (should be UTF-8)
- Verify file size (< 10MB recommended)

### 10. Chart Not Rendering

**Symptom**: Widget appears but chart is blank.

**Debug Steps**:

1. **Check browser console** for ECharts errors
2. **Verify data mapping**:
   - Open widget configuration panel
   - Ensure X/Y axes are mapped to valid columns
   - Check data types (numeric fields for values, text for categories)
3. **Inspect dataset**:
   - Open browser DevTools → Application → Local Storage
   - Look for `dashboard_state` key
   - Verify dataset has data

### 11. Performance Issues with Large Datasets

**Symptom**: Browser becomes unresponsive with large CSV files.

**Solutions**:

- **Limit CSV size** to < 50,000 rows
- **Use data aggregation** in chart configuration
- **Reduce chart count** on canvas (< 10 widgets)
- **Disable animations** in chart style settings

## Getting More Help

If your issue isn't listed here:

1. **Check existing issues**: [GitHub Issues](https://github.com/yourusername/dashboard-studio-rs/issues)
2. **Open a new issue**: Include:
   - Browser version and OS
   - Trunk version (`trunk --version`)
   - wasm-opt version (`wasm-opt --version`)
   - Full error message
   - Steps to reproduce
3. **Join Leptos Discord**: [https://discord.gg/leptos](https://discord.gg/leptos)

## Debugging Commands

Quick reference for debugging:

```bash
# Check versions
trunk --version
wasm-opt --version
rustup show
node --version

# Clean build
cargo clean
rm -rf dist docs .trunk
trunk build

# Verbose build
trunk build --release -v

# Check WASM file size
ls -lh dist/*.wasm

# Inspect bundle
wasm-objdump -x dist/*.wasm | head -50
```

## Environment Variables

Set these for debugging:

```bash
# Verbose Trunk output
export TRUNK_LOG=trace

# Rust backtrace
export RUST_BACKTRACE=1

# WASM logging
export RUST_LOG=debug
```
