use crate::vm::Vm;

pub struct Runtime {
    vm: Vm,
}

impl Runtime {
    pub fn new() -> crate::error::Result<Self> {
        let vm = Vm::new()?;
        Ok(Runtime { vm })
    }

    pub fn execute_file(&self, filename: &str) -> crate::error::Result<()> {
        self.vm.run_file(filename)?;
        self.vm.run_pending_jobs()?;
        Ok(())
    }

    pub fn execute_source(&self, source: &str) -> crate::error::Result<()> {
        self.vm.run_source(source)?;
        self.vm.run_pending_jobs()?;
        Ok(())
    }
}
