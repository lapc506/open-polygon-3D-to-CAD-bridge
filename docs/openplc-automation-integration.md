# OpenPLC automation integration

This document defines the Studio boundary for automation scenarios.

## Current vertical slice

`bridge-studio` can display the configured OpenPLC endpoint alongside the CFD study. The endpoint is represented by the shared automation CIM and is not contacted by the Studio process in this increment.

## Next co-simulation boundary

```text
OpenPLC → Modbus bridge → automation state → scenario/timeline → CFD boundary conditions
                                             ← metrics/results ←
```

The co-simulation layer must remain separate from both the PLC transport and CFD solver adapters. Physical pumps, valves and motors are never actuated by this simulation path.
