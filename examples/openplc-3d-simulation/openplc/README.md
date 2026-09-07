# OpenPLC reference project

Load `aeroponic_conveyor.st` into an OpenPLC project configured for IEC 61131-3 Structured Text.

## Project-local mapping

- `%QX0.0` → Coil 0 → forward
- `%QX0.1` → Coil 1 → reverse
- `%QW0` → Holding register 0 → position
- `%IX0.0` → Discrete input 0 → home
- `%IX0.1` → Discrete input 1 → end stop
- `%IX0.2` → Discrete input 2 → start

OpenPLC Modbus mappings are configuration-dependent; verify the generated/runtime mapping before connecting the bridge. Port `502` is the default documented endpoint for this example; use an unprivileged alternative such as `1502` when required by the host/container setup.
