# Implementation status: desktop + MCP + OpenFOAM vertical slice

## Architecture

```text
Desktop / MCP client / AI agent
             |
             v
        bridge-studio
             |
             +------> bridge-mcp
             |            |
             v            v
            CIM       cfd-core
             |            |
             +------------+
                          v
                  cfd-openfoam adapter
                          |
                          v
                    OpenFOAM case
```

The MCP layer is protocol-facing. CFD semantics live in `cfd-core`; OpenFOAM-specific translation and execution live in `cfd-openfoam`. The desktop application is a thin engineering UI over the same canonical study model.

## Implemented

- Rust workspace with `cfd-core`, `cfd-openfoam`, `bridge-mcp`, and `bridge-studio`.
- Solver-neutral `CfdStudy`, `MisterInjection`, `FluidMaterial`, and `DropletDistribution` contracts.
- Explicit droplet-distribution provenance.
- Study validation for fluid properties, injections, and transient timestep.
- Reproducible OpenFOAM case directory creation with the canonical study serialized to `system/study.json`.
- MCP JSON-lines bridge with study creation, validation, preparation, run, and results commands.
- Initial native desktop GUI showing the CFD study, mister injection, validation state, and OpenFOAM preparation action.
- Flatpak manifest, desktop metadata, icon, CI build, and local Ubuntu installation instructions.
- Physical pump/valve actuation is not exposed by the bridge.

## Current limitations

- `cfd-openfoam` still invokes `foamRun -help` as an execution smoke test; it does not yet create solver dictionaries, mesh, injection models, or perform a physical CFD solve.
- Desktop GUI is an initial engineering-hub shell, not yet a full CAD viewport/editor.
- The GUI does not yet spawn/connect to `bridge-mcp`; both binaries are packaged together so that integration can be added without changing the distribution boundary.
- OpenFOAM is not bundled yet.

## Next increments

1. Real OpenFOAM dictionaries, mesh generation, injection definitions, and solver selection.
2. Async run/status/stop and structured logs.
3. Post-processing into normalized `CfdRunResult` metrics.
4. GUI-to-MCP process/IPC integration.
5. OpenCADStudio adapter/plugin IPC boundary.
6. Blender semantic extraction into the same CIM.
7. Parameter sweeps and `cfd.compare_runs`.

## Engineering boundary

P0 treats mister atomization as calibrated injection data. The adapter must not infer droplet size from pressure alone and must preserve the source of each droplet distribution.
