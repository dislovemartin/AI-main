use common::{Run, RunStatus};
use anyhow::Result;

pub struct VRARSession {
    run: Run,
    scene_data: SceneData,
    hardware_interface: HardwareInterface,
}

impl VRARSession {
    pub fn new(user: String) -> Self {
        Self {
            run: Run::new(user),
            scene_data: SceneData::default(),
            hardware_interface: HardwareInterface::new(),
        }
    }

    pub async fn process_frame(&mut self) -> Result<FrameData> {
        if !self.run.start_run() {
            return Err(anyhow::anyhow!("Failed to start frame processing"));
        }

        // Frame processing logic

        self.run.complete_run(&self.run.get_user())?;
        Ok(FrameData::default())
    }
}
