# Ubuntu Flatpak development build

The project now has an initial desktop executable, `bridge-studio`, and a Flatpak manifest that packages it together with the MCP bridge.

## Local build

Install Flatpak tooling and the Freedesktop 25.08 SDK, then from the repository root:

```bash
cargo build --release -p bridge-studio -p bridge-mcp
flatpak-builder --force-clean --repo=repo build-dir packaging/flatpak/com.openpolygon.CadBridge.yml
flatpak build-bundle repo open-polygon-cad-bridge.flatpak com.openpolygon.CadBridge
```

Install the resulting bundle for the current user:

```bash
flatpak install --user ./open-polygon-cad-bridge.flatpak
flatpak run com.openpolygon.CadBridge
```

The desktop entry launches `bridge-studio`; `bridge-mcp` is installed inside the same sandbox for the upcoming GUI-to-MCP integration.

## Current scope

The GUI is an engineering-hub scaffold. It can display the canonical aeroponic CFD study, validate it, and prepare an OpenFOAM case through the adapter. The OpenFOAM adapter is still a smoke-test scaffold and does not yet perform a physical CFD solve.

The Flatpak follows the normal Flatpak model of a sandboxed application/runtime and can be installed per-user. See the official Flatpak command reference for bundle/install semantics.
