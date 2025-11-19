use ainakan::Ainakan;
use std::sync::LazyLock;

static AINAKAN: LazyLock<Ainakan> = LazyLock::new(|| unsafe { Ainakan::obtain() });

fn main() {
    let device_manager = ainakan::DeviceManager::obtain(&AINAKAN);
    let local_device = device_manager.get_local_device().unwrap();
    let processes = local_device.enumerate_processes();

    for process in processes {
        println!("{} {:?}", process.get_name(), process.get_pid());
    }
}
