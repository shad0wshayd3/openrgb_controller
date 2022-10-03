#![windows_subsystem = "windows"]

use log::{ info, error };
use openrgb::OpenRGB;
use windows::Win32::{
    System::Power::DEVICE_NOTIFY_CALLBACK,
    System::Power::DEVICE_NOTIFY_SUBSCRIBE_PARAMETERS,
    System::Power::HPOWERNOTIFY,
    System::Power::RegisterSuspendResumeNotification,
    UI::WindowsAndMessaging::PBT_APMSUSPEND,
    UI::WindowsAndMessaging::PBT_APMRESUMEAUTOMATIC,
};

#[tokio::main]
async fn openrgb_connect(a_wake: bool) {
    match OpenRGB::connect().await {
        Ok(client) => {
            if a_wake {
                info!("Resuming");
                let _ = client.load_profile("ORGBC_Resume").await;
            }
            else {
                info!("Sleeping");
                let _ = client.load_profile("ORGBC_Sleep").await;
            }
        }
        Err(e) => {
            error!("Error: {}", e);
        }
    }
}

fn main() {
    let _ = initialize_log();
    match register_callback() {
        Ok(_) => {
            info!("Registered callback");
            loop {
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
        },
        Err(e) => {
            error!("Failed to register callback: {}", e);
        }
    }
}

fn initialize_log() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Info)
        .chain(fern::log_file("openrgb_controller.log")?)
        .apply()?;

    return Ok(());
}

fn register_callback() -> windows::core::Result<HPOWERNOTIFY> {
    let mut params = DEVICE_NOTIFY_SUBSCRIBE_PARAMETERS {
        Callback: Some(sleep_callback),
        Context: std::ptr::null_mut(),
    };

    unsafe
    {
        let handle: *mut core::ffi::c_void = &mut params as *mut _ as *mut core::ffi::c_void;
        let handle: windows::Win32::Foundation::HANDLE = std::mem::transmute(handle);
        return RegisterSuspendResumeNotification(handle, DEVICE_NOTIFY_CALLBACK.0);
    }
}

unsafe extern "system" fn sleep_callback(_: *const core::ffi::c_void, a_type: u32, _: *const core::ffi::c_void) -> u32 {
    match a_type {
        PBT_APMSUSPEND => {
            openrgb_connect(false);
        },
        PBT_APMRESUMEAUTOMATIC => {
            openrgb_connect(true);
        },
        _ => {}
    }

    return 0;
}
