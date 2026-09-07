use cfd_core::{CfdBackend, CfdStudy, FluidMaterial, MisterInjection, Point3, Vector3, DropletDistribution, DistributionSource};
use cfd_openfoam::OpenFoamBackend;
use eframe::egui;
use uuid::Uuid;

struct StudioApp {
    study: CfdStudy,
    prepared: bool,
    status: String,
}

impl Default for StudioApp {
    fn default() -> Self {
        Self {
            study: demo_study(),
            prepared: false,
            status: "Ready".into(),
        }
    }
}

fn demo_study() -> CfdStudy {
    CfdStudy {
        id: Uuid::new_v4(),
        name: "aeroponic-demo".into(),
        fluid: FluidMaterial {
            name: "nutrient-solution".into(),
            density_kg_m3: 1000.0,
            viscosity_pa_s: 0.001,
            surface_tension_n_m: Some(0.072),
            temperature_k: Some(293.15),
        },
        gravity_m_s2: Vector3(0.0, 0.0, -9.81),
        transient: true,
        timestep_s: Some(0.001),
        injections: vec![MisterInjection {
            entity_id: Uuid::new_v4(),
            position: Point3(0.0, 0.0, 1.0),
            direction: Vector3(0.0, 0.0, -1.0),
            pressure_pa: Some(300_000.0),
            flow_rate_kg_s: 0.00002,
            distribution: DropletDistribution {
                d10_m: Some(10e-6),
                d50_m: Some(30e-6),
                d90_m: Some(60e-6),
                source: DistributionSource::Assumed,
            },
            duty_cycle: Some(1.0),
        }],
        parameters: Default::default(),
    }
}

impl eframe::App for StudioApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("header").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Open Polygon CAD Bridge");
                ui.separator();
                ui.label("Desktop engineering hub");
            });
        });

        egui::SidePanel::left("navigation").show(ctx, |ui| {
            ui.heading("Project");
            ui.label("Aeroponic CFD");
            ui.separator();
            ui.label("CIM");
            ui.label("CFD Study");
            ui.label("OpenFOAM");
            ui.label("MCP Bridge");
            ui.separator();
            ui.small("P0: post-atomization transport");
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("CFD Study");
            ui.label(format!("Study: {}", self.study.name));
            ui.label(format!("ID: {}", self.study.id));
            ui.separator();

            egui::Grid::new("study-grid").show(ui, |ui| {
                ui.label("Fluid");
                ui.label(&self.study.fluid.name);
                ui.end_row();
                ui.label("Density");
                ui.label(format!("{:.1} kg/m³", self.study.fluid.density_kg_m3));
                ui.end_row();
                ui.label("Viscosity");
                ui.label(format!("{:.4} Pa·s", self.study.fluid.viscosity_pa_s));
                ui.end_row();
                ui.label("Transient");
                ui.label(self.study.transient.to_string());
                ui.end_row();
                ui.label("Timestep");
                ui.label(format!("{:.4} s", self.study.timestep_s.unwrap_or_default()));
                ui.end_row();
            });

            ui.separator();
            ui.heading("Mister injection");
            let injection = &self.study.injections[0];
            ui.label(format!("Pressure: {:.0} kPa", injection.pressure_pa.unwrap_or_default() / 1000.0));
            ui.label(format!("Mass flow: {:.6} kg/s", injection.flow_rate_kg_s));
            ui.label(format!("Droplets D10 / D50 / D90: {:.0} / {:.0} / {:.0} µm",
                injection.distribution.d10_m.unwrap_or_default() * 1e6,
                injection.distribution.d50_m.unwrap_or_default() * 1e6,
                injection.distribution.d90_m.unwrap_or_default() * 1e6));
            ui.label("Distribution source: Assumed (must be calibrated before validation)");

            ui.separator();
            if ui.button("Validate study").clicked() {
                self.status = match self.study.validate() {
                    Ok(()) => "Study valid".into(),
                    Err(e) => format!("Validation failed: {e}"),
                };
            }
            if ui.button("Prepare OpenFOAM case").clicked() {
                let backend = OpenFoamBackend::new("runs/openfoam");
                match backend.prepare(&self.study) {
                    Ok(run) => {
                        self.prepared = true;
                        self.status = format!("Prepared case: {}", run.case_path);
                    }
                    Err(e) => self.status = format!("Prepare failed: {e}"),
                }
            }
            if self.prepared {
                ui.label("Case prepared");
            }
            ui.separator();
            ui.strong(&self.status);
        });
    }
}

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "Open Polygon CAD Bridge",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(StudioApp::default()))),
    )
}
