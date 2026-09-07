# Flatpak packaging

This directory contains the experimental Ubuntu/Flatpak packaging for the
current MCP/OpenFOAM bridge vertical slice.

## Local build

The current binary is the `bridge-mcp` JSON-lines scaffold, not yet a native
GUI. The desktop entry therefore launches it in a terminal. This packaging is
intended to validate installation, sandboxing, and distribution before the
full `bridge-studio` desktop UI is added.

From the repository root:

```bash
cargo build --release -p bridge-mcp
flatpak-builder --user --install --force-clean build-dir \
  packaging/flatpak/com.openpolygon.CadBridge.yml
flatpak run com.openpolygon.CadBridge
```

To create a portable bundle:

```bash
flatpak-builder --force-clean --repo=repo build-dir \
  packaging/flatpak/com.openpolygon.CadBridge.yml
flatpak build-bundle repo open-polygon-cad-bridge.flatpak \
  com.openpolygon.CadBridge
```

The GitHub Actions workflow also builds the bundle and publishes it as a
workflow artifact.

## Current limitation

OpenFOAM is not bundled yet. The current CFD adapter is still a scaffold, so
this package validates the application/bridge packaging boundary rather than
shipping a complete CFD desktop product.
