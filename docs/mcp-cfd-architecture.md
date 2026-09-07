# MCP Bridge Architecture — CAD + CFD Experimentation

## 1. Purpose

The MCP Bridge is the protocol-facing orchestration layer for the CAD Integration Hub. It exposes typed engineering operations to MCP clients and AI agents while keeping the CIM as the canonical representation of model state.

The bridge shall support both CAD integration and CFD experimentation. The first concrete CAD target is OpenCADStudio; the first CFD target is OpenFOAM.

The primary scientific objective remains **substrate-free aeroponic nutrient-solution delivery to suspended roots**. This is not a greenhouse fogging/climate-control system.

## 2. Architectural Principle

MCP clients shall communicate with the CIM and bridge services rather than directly manipulating OpenCADStudio or OpenFOAM-specific files.

```text
                         MCP Client / AI Agent
                                  |
                                  v
                           +--------------+
                           |  MCP Bridge  |
                           +------+-------+
                                  |
                         Engineering API
                                  |
                                  v
                         CIM / Parametric Model
                         /                    \
                        /                      \
                       v                        v
              OpenCADStudio Adapter         CFD Core
                       |                  /             \
                       v                 v               v
                Plugin / IPC         OpenFOAM       Future CFD
```

This separation prevents the MCP contract from becoming coupled to a particular CAD application or CFD solver.

## 3. Backend Boundaries

### 3.1 OpenCADStudio

The OpenCADStudio integration shall use an adapter boundary and the application's supported plugin/IPC interfaces. The MCP Bridge must not require modifications to OpenCADStudio core for the PoC.

```text
MCP
 |
 v
CIM
 |
 v
OpenCADStudio Adapter
 |
 v
OpenCADStudio Plugin / IPC
 |
 v
OpenCADStudio
```

The adapter is responsible for translating CIM entities and operations into OpenCADStudio-native document/entity operations and for preserving stable CIM entity IDs and provenance.

### 3.2 OpenFOAM

OpenFOAM is the first CFD execution backend for experimentation.

```text
MCP
 |
 v
CIM / CFD Study
 |
 v
CFD Core
 |
 v
OpenFOAM Adapter
 |
 +--> case generation
 +--> mesh configuration
 +--> solver configuration
 +--> execution
 +--> status monitoring
 +--> result extraction
```

The adapter shall generate reproducible OpenFOAM cases from solver-neutral CIM/CFD-study definitions. OpenFOAM-specific dictionaries and implementation details shall remain inside the adapter.

## 4. MCP Tool Contract

The initial contract is grouped by responsibility.

### Model

- `model.inspect` — return model summary, entities, capabilities, and schema version.
- `model.get_entity` — return a typed CIM entity by stable ID.
- `model.set_parameter` — update a validated parametric value.
- `model.validate` — validate schema, units, references, and domain constraints.

### CAD

- `cad.sync` — synchronize a validated CIM model with a selected CAD target.
- `cad.export` — export the current model to a target CAD representation.

### CFD study

- `cfd.create_study` — create a solver-neutral CFD study associated with a CIM model.
- `cfd.set_domain` — configure the fluid domain and domain boundaries.
- `cfd.set_boundary` — configure solver-neutral boundary conditions.
- `cfd.set_nozzle` — configure mister injection properties and droplet population.
- `cfd.set_fluid` — configure nutrient-solution material properties.
- `cfd.mesh` — generate or prepare a mesh through the selected CFD adapter.
- `cfd.run` — execute a reproducible CFD case.
- `cfd.stop` — request termination of an active run.
- `cfd.get_status` — return case/run status.
- `cfd.get_results` — return normalized CFD metrics and references to result artifacts.
- `cfd.compare_runs` — compare multiple simulation configurations.
- `cfd.optimize` — perform a bounded parameter search using the configured objective.

## 5. Canonical Data Flow

A typical experiment shall follow:

```text
Blender
  |
  v
Semantic extraction
  |
  v
CIM
  |
  +--------------------+
  |                    |
  v                    v
CAD sync             CFD Study
  |                    |
  v                    v
OpenCADStudio       OpenFOAM
                       |
                       v
                 Normalized results
                       |
                       v
                 Comparison / objective
                       |
                       v
                 Updated parameters
                       |
                       +----> CIM
```

The CIM is the source of truth for semantic model state. Generated CAD documents, OpenFOAM cases, logs, meshes, and result files are derived artifacts.

## 6. Aeroponic CFD Model Boundary

P0 shall model the post-atomization transport problem rather than detailed internal nozzle breakup.

Each mister may provide:

- pressure;
- flow rate;
- position;
- orientation;
- nozzle identity;
- spray-cone/distribution profile;
- droplet size distribution;
- duty cycle.

Droplet distributions shall be represented independently from pressure and tagged as manufacturer-provided, experimentally measured, calibrated, assumed, or solver-derived.

The CFD study may model:

- airflow;
- gravity;
- droplet trajectories;
- residence time;
- evaporation where supported;
- root deposition/delivery;
- wall/non-target deposition;
- ventilation effects;
- root-zone coverage and dry zones.

The hydraulic network remains a separate concern. Pump, regulator, manifold, pipes, and branches may determine mister pressure and flow, which are then passed into the CFD study as boundary/injection inputs.

## 7. Experimentation Workflow

The MCP Bridge shall support reproducible computational experiments such as:

```text
Experiment: pressure sweep

2.5 bar ----> OpenFOAM case A --+
3.0 bar ----> OpenFOAM case B --+--> normalized metrics
3.5 bar ----> OpenFOAM case C --+
                                  |
                                  v
                           compare_runs
                                  |
                                  v
                           objective score
```

Future experiments may vary:

- mister pressure;
- mister position and orientation;
- nozzle selection;
- droplet distribution;
- mister spacing/count;
- duty cycle;
- airflow/ventilation parameters;
- nutrient-solution properties.

Parameter sweeps and optimization are P1 capabilities. The MCP/CIM boundary and OpenFOAM adapter boundary are architectural P0 requirements so that P1 experimentation does not require redesigning the system.

## 8. Normalized Result Model

CFD adapters shall normalize solver-specific outputs into a common result representation. Initial metrics should include, where supported:

- root-zone delivered/deposited mass;
- root-zone spatial coverage;
- distribution uniformity;
- dry-zone fraction;
- evaporation/loss fraction;
- wall/non-target deposition;
- droplet residence time;
- mister pressure and flow;
- pump-energy proxy or hydraulic cost when available.

Solver-native fields remain available as artifacts or references for CFD-engineering workflows.

## 9. Safety and Control Boundary

P0/P1 experimentation is simulation-only. The MCP Bridge shall not directly actuate pumps, valves, pressure regulators, or other physical equipment.

Future hardware integration must be a separate control boundary with explicit authorization, limits, telemetry, fail-safe behavior, and experimental validation.

## 10. Requirements Allocation

| Requirement | Priority | Verification |
|---|---|---|
| MCP server exposes typed model operations | P0 | Integration test |
| MCP operations operate through validated CIM state | P0 | Negative/validation tests |
| CFD study is solver-neutral before backend translation | P0 | Schema + integration test |
| OpenFOAM adapter generates reproducible cases | P0 | Golden-case test |
| OpenFOAM run/status/result lifecycle is exposed | P1 | End-to-end test |
| OpenCADStudio adapter boundary is defined | P0 | Contract test |
| OpenCADStudio plugin/IPC implementation | P1 | Integration test |
| Parameter sweeps | P1 | End-to-end experiment |
| Run comparison | P1 | Deterministic comparison test |
| Optimization | P1 | Bounded experiment |
| Physical hardware control | Future | Separate safety/control validation |

## 11. Design Rule

**MCP is the interface to capabilities; CIM is the canonical state; adapters own application-specific translation.**

This rule applies equally to OpenCADStudio, OpenFOAM, and future CAD/CFD backends.
