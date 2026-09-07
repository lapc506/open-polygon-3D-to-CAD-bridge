import express from 'express';
import { WebSocketServer } from 'ws';
import { ModbusTCPClient } from './modbus-client.js';

const app = express();
app.use(express.static('public'));
const server = app.listen(process.env.PORT || 3000);
const wss = new WebSocketServer({ server });

const plc = new ModbusTCPClient({
  host: process.env.PLC_HOST || '127.0.0.1',
  port: Number(process.env.PLC_PORT || 502),
  unitId: Number(process.env.PLC_UNIT_ID || 1),
});

wss.on('connection', ws => {
  ws.send(JSON.stringify({ type: 'connection', state: plc.connected ? 'connected' : 'simulation' }));
  ws.on('message', async raw => {
    try {
      const command = JSON.parse(raw.toString());
      if (command.type === 'force-input') await plc.setVirtualInput(command.name, Boolean(command.value));
      if (command.type === 'set-position') await plc.setPosition(command.value);
    } catch (error) {
      ws.send(JSON.stringify({ type: 'error', message: error.message }));
    }
  });
});

setInterval(async () => {
  const state = await plc.readState();
  const payload = JSON.stringify({ type: 'plc-state', ...state });
  for (const ws of wss.clients) if (ws.readyState === ws.OPEN) ws.send(payload);
}, 100);

console.log(`OpenPLC bridge listening on http://127.0.0.1:${process.env.PORT || 3000}`);
