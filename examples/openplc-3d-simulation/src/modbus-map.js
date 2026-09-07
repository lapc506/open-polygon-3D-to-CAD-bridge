export const MODBUS_MAP = Object.freeze({
  forward: { address: 0, iec: '%QX0.0', kind: 'coil' },
  reverse: { address: 1, iec: '%QX0.1', kind: 'coil' },
  position: { address: 0, iec: '%QW0', kind: 'holding-register' },
  home: { address: 0, iec: '%IX0.0', kind: 'discrete-input' },
  end: { address: 1, iec: '%IX0.1', kind: 'discrete-input' },
});

// These offsets are a project-local contract, not universal OpenPLC offsets.
// The OpenPLC project/runtime configuration must use the same mapping.
export function validatePositionMm(value) {
  if (!Number.isFinite(value)) throw new TypeError('position must be finite');
  return Math.max(0, Math.min(100, value));
}
