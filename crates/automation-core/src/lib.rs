use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type EntityId = Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plc {
    pub id: EntityId,
    pub name: String,
    pub endpoint: ModbusEndpoint,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModbusEndpoint {
    pub host: String,
    pub port: u16,
    pub unit_id: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PlcVariableKind { InputBit, OutputBit, InputWord, OutputWord }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModbusMapping {
    pub name: String,
    pub iec_address: String,
    pub kind: PlcVariableKind,
    pub offset: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualSensor {
    pub id: EntityId,
    pub name: String,
    pub input_mapping: String,
    pub condition: SensorCondition,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SensorCondition { PositionAtOrBelowMm(f64), PositionAtOrAboveMm(f64), Manual(bool) }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Actuator {
    pub id: EntityId,
    pub name: String,
    pub output_mapping: String,
    pub action: ActuatorAction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActuatorAction { TranslateX { speed_mm_s: f64 }, RotateZ { rpm: f64 } }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KinematicJoint {
    pub id: EntityId,
    pub name: String,
    pub axis: [f64; 3],
    pub position_mm: f64,
    pub min_mm: f64,
    pub max_mm: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlcProgram { pub name: String, pub source_path: String }
