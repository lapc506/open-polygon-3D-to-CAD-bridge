# Implementation status: MCP + OpenFOAM vertical slice

## Architecture

```text
MCP client / agent
        |
        v
   bridge-mcp
        |
        v
       CIM
        |
        v
   cfd-core trait
        |
        v
 cfd-openfoam adapter
        |
        v
 OpenFOAM case / solver
```

The MCP layer is intentionally protocol-facing. CFD semantics live in `cfd-core`; OpenFOAM-specific translation and execution live in `cfd-openfoam`.

## Implemented in this slice

- Rust workspace with `cfd-core`, `cfd-openfoam`, and `bridge-mcp`.
- Solver-neutral `CfdStudy`, `MisterInjection`, `FluidMaterial`, and `DropletDistribution` contracts.
- Explicit droplet-distribution provenance.
- Study validation for fluid properties, injections, and transient timestep.
- Reproducible OpenFOAM case directory creation with the canonical study serialized to `system/study.json`.
- MCP-style JSON-lines bridge exposing model inspection/validation and CFD study preparation.
- Physical pump/valve actuation is not exposed by the bridge.

## Next implementation increments

1. Replace the JSON-lines scaffold with the project's production MCP transport.
2. Add real OpenFOAM dictionaries, mesh generation, injection definitions, and solver selection.
3. Add asynchronous run/status/stop handling and structured logs.
4. Parse OpenFOAM post-processing into normalized `CfdRunResult` metrics.
5. Add parameter sweeps and `cfd.compare_runs`.
6. Add the OpenCADStudio adapter/plugin IPC boundary.
7. Add Blender semantic extraction into the same CIM.

## Engineering boundary

P0 treats mister atomization as calibrated injection data. The adapter must not infer droplet size from pressure alone and must preserve the source of each droplet distribution.
