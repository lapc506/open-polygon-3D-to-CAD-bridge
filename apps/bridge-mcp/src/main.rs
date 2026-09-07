use anyhow::Result;
use cfd_core::{CfdBackend, CfdStudy, FluidMaterial, MisterInjection, Point3, Vector3, DropletDistribution, DistributionSource};
use cfd_openfoam::OpenFoamBackend;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::io::{self, BufRead, Write};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
struct Request { id: Value, method: String, #[serde(default)] params: Value }

#[derive(Debug, Serialize)]
struct Response { id: Value, result: Option<Value>, error: Option<Value> }

fn tools() -> Value {
    json!({"tools":[
        {"name":"model.inspect","description":"Inspect the canonical engineering model."},
        {"name":"model.validate","description":"Validate a CFD study before execution."},
        {"name":"cfd.create_study","description":"Create a solver-neutral aeroponic CFD study."},
        {"name":"cfd.prepare","description":"Generate a reproducible OpenFOAM case from a CFD study."},
        {"name":"cfd.run","description":"Execute the configured OpenFOAM backend for a prepared case."},
        {"name":"cfd.get_results","description":"Return normalized CFD run results."}
    ]})
}

fn demo_study() -> CfdStudy {
    CfdStudy {
        id: Uuid::new_v4(), name: "aeroponic-demo".into(),
        fluid: FluidMaterial { name: "nutrient-solution".into(), density_kg_m3: 1000.0, viscosity_pa_s: 0.001, surface_tension_n_m: Some(0.072), temperature_k: Some(293.15) },
        gravity_m_s2: Vector3(0.0, 0.0, -9.81), transient: true, timestep_s: Some(0.001),
        injections: vec![MisterInjection {
            entity_id: Uuid::new_v4(), position: Point3(0.0, 0.0, 1.0), direction: Vector3(0.0, 0.0, -1.0),
            pressure_pa: Some(300_000.0), flow_rate_kg_s: 0.00002,
            distribution: DropletDistribution { d10_m: Some(10e-6), d50_m: Some(30e-6), d90_m: Some(60e-6), source: DistributionSource::Assumed }, duty_cycle: Some(1.0),
        }], parameters: Default::default(),
    }
}

fn handle(req: &Request) -> Result<Value> {
    match req.method.as_str() {
        "tools/list" => Ok(tools()),
        "cfd.create_study" => Ok(serde_json::to_value(demo_study())?),
        "cfd.prepare" => {
            let study: CfdStudy = req.params.get("study").cloned().map(serde_json::from_value).transpose()?.unwrap_or_else(|| Ok(demo_study()))?;
            let backend = OpenFoamBackend::new("runs/openfoam");
            Ok(serde_json::to_value(backend.prepare(&study).map_err(|e| anyhow::anyhow!(e))?)?)
        }
        "model.validate" => {
            let study: CfdStudy = req.params.get("study").cloned().map(serde_json::from_value).transpose()?.unwrap_or_else(|| Ok(demo_study()))?;
            Ok(json!({"valid": study.validate().is_ok()}))
        }
        "model.inspect" => Ok(json!({"status":"scaffold","canonical_model":"CIM","simulation_backend":"OpenFOAM","physical_actuation":false})),
        _ => Ok(json!({"error":"method not implemented"})),
    }
}

fn main() -> Result<()> {
    let stdin = io::stdin(); let mut stdout = io::stdout();
    for line in stdin.lock().lines() {
        let line = line?; if line.trim().is_empty() { continue; }
        let req: Request = serde_json::from_str(&line)?; let id = req.id.clone();
        let response = match handle(&req) {
            Ok(result) => Response { id, result: Some(result), error: None },
            Err(error) => Response { id, result: None, error: Some(json!({"message": error.to_string()})) },
        };
        writeln!(stdout, "{}", serde_json::to_string(&response)?)?; stdout.flush()?;
    }
    Ok(())
}
