import net from 'node:net';
import { MODBUS_MAP, validatePositionMm } from './modbus-map.js';

export class ModbusTCPClient {
  constructor({ host, port, unitId }) {
    this.host = host; this.port = port; this.unitId = unitId;
    this.connected = false; this.simulation = true;
    this.state = { forward: false, reverse: false, position: 0, home: true, end: false };
    this.socket = null;
  }

  async connect() {
    // Transport integration is intentionally isolated here so the browser never opens Modbus TCP.
    // A production deployment can replace this adapter with jsmodbus while preserving the WS API.
    return this.state;
  }

  async readState() { return { ...this.state, simulation: this.simulation, connected: this.connected }; }

  async setVirtualInput(name, value) {
    if (name === 'home') this.state.home = value;
    if (name === 'end') this.state.end = value;
  }

  async setPosition(value) {
    this.state.position = validatePositionMm(Number(value));
    this.state.home = this.state.position <= 0;
    this.state.end = this.state.position >= 100;
  }
}
