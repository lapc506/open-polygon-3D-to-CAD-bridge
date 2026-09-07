use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum CfdError {
    #[error("invalid CFD study: {0}")]
    InvalidStudy(String),
    #[error("missing parameter: {0}")]
    MissingParameter(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Point3(pub f64, pub f64, pub f64);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vector3(pub f64, pub f64, pub f64);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FluidMaterial {
    pub name: String,
    pub density_kg_m3: f64,
    pub viscosity_pa_s: f64,
    pub surface_tension_n_m: Option<f64>,
    pub temperature_k: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DropletDistribution {
    pub d10_m: Option<f64>,
    pub d50_m: Option<f64>,
    pub d90_m: Option<f64>,
    pub source: DistributionSource,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DistributionSource {
    Manufacturer,
    Experimental,
    Calibrated,
    Assumed,
    SolverDerived,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MisterInjection {
    pub entity_id: Uuid,
    pub position: Point3,
    pub direction: Vector3,
    pub pressure_pa: Option<f64>,
    pub flow_rate_kg_s: f64,
    pub distribution: DropletDistribution,
    pub duty_cycle: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CfdStudy {
    pub id: Uuid,
    pub name: String,
    pub fluid: FluidMaterial,
    pub gravity_m_s2: Vector3,
    pub transient: bool,
    pub timestep_s: Option<f64>,
    pub injections: Vec<MisterInjection>,
    pub parameters: BTreeMap<String, f64>,
}

impl CfdStudy {
    pub fn validate(&self) -> Result<(), CfdError> {
        if self.injections.is_empty() {
            return Err(CfdError::InvalidStudy("at least one mister injection is required".into()));
        }
        if self.fluid.density_kg_m3 <= 0.0 || self.fluid.viscosity_pa_s <= 0.0 {
            return Err(CfdError::InvalidStudy("fluid density and viscosity must be positive".into()));
        }
        if self.transient && self.timestep_s.unwrap_or(0.0) <= 0.0 {
            return Err(CfdError::InvalidStudy("transient studies require a positive timestep".into()));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CfdRunResult {
    pub run_id: Uuid,
    pub study_id: Uuid,
    pub status: RunStatus,
    pub case_path: String,
    pub metrics: BTreeMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RunStatus {
    Prepared,
    Running,
    Completed,
    Failed,
}

pub trait CfdBackend {
    fn prepare(&self, study: &CfdStudy) -> Result<CfdRunResult, CfdError>;
    fn run(&self, run: &CfdRunResult) -> Result<CfdRunResult, CfdError>;
    fn results(&self, run: &CfdRunResult) -> Result<CfdRunResult, CfdError>;
}
