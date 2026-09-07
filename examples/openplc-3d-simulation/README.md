# OpenPLC 3D simulation

Browser simulation for the reference linear axis. Node owns the Modbus TCP boundary and exposes state over WebSocket; the browser never connects directly to Modbus.

## Mapping

| IEC 61131-3 | Modbus address | Meaning |
|---|---:|---|
| `%QX0.0` | Coil 0 | forward |
| `%QX0.1` | Coil 1 | reverse |
| `%QW0` | Holding register 0 | position mm |
| `%IX0.0` | Discrete input 0 | home |
| `%IX0.1` | Discrete input 1 | end |

These offsets are project-local and must match the OpenPLC runtime configuration.
