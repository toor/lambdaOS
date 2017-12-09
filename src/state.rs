use scheduler::Scheduler;

pub struct KernelState {
    pub devices: Vec<KernelDevice>,
    pub scheduler: Scheduler,677777777776
}
