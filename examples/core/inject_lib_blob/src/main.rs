use ainakan::{Ainakan, Inject};
use std::sync::LazyLock;

static AINAKAN: LazyLock<Ainakan> = LazyLock::new(|| unsafe { Ainakan::obtain() });

fn main() {
    let device_manager = ainakan::DeviceManager::obtain(&AINAKAN);
    let local_device = device_manager.get_local_device();
    let args: Vec<String> = std::env::args().collect();
    let pid = args[1].parse().unwrap();

    if let Ok(mut device) = local_device {
        println!("[*] Ainakan version: {}", ainakan::Ainakan::version());
        println!("[*] Device name: {}", device.get_name());

        let script_source = include_bytes!("../../../../target/release/libinject_example.so");
        let id = device
            .inject_library_blob_sync(pid, script_source, "injected_function", "w00t")
            .unwrap();

        println!("*** Injected, id={}", id);
    }
}
