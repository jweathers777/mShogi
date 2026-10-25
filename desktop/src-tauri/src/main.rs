#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use mshogi_desktop_session::{DesktopCommand, DesktopController, Snapshot};
use std::{
    io,
    path::PathBuf,
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
    thread,
    time::{Duration, Instant},
};
use tauri::{
    AppHandle, Manager, RunEvent, State,
    menu::{AboutMetadata, Menu, MenuItem, PredefinedMenuItem, Submenu},
};

#[derive(Default)]
struct ShutdownState {
    started: AtomicBool,
    ready: AtomicBool,
    observation_finished: AtomicBool,
    deadline: Mutex<Option<Instant>>,
}
fn engine_path() -> io::Result<PathBuf> {
    let executable = std::env::current_exe()?;
    let directory = executable
        .parent()
        .ok_or_else(|| io::Error::other("application directory unavailable"))?;
    let name = if cfg!(windows) {
        "mshogi.exe"
    } else {
        "mshogi"
    };
    Ok(directory.join(name))
}
fn with_controller<T>(
    controller: &Mutex<DesktopController>,
    shutdown: &ShutdownState,
    operation: impl FnOnce(&mut DesktopController) -> T,
) -> Result<T, String> {
    if shutdown.started.load(Ordering::Acquire) {
        return Err("The application is closing; no further game commands are accepted.".into());
    }
    let mut controller = controller
        .lock()
        .map_err(|_| "Native session lock unavailable".to_string())?;
    if shutdown.started.load(Ordering::Acquire) {
        return Err("The application is closing; no further game commands are accepted.".into());
    }
    Ok(operation(&mut controller))
}
#[tauri::command]
fn desktop_snapshot(
    controller: State<'_, Mutex<DesktopController>>,
    shutdown: State<'_, Arc<ShutdownState>>,
) -> Result<Snapshot, String> {
    with_controller(&controller, &shutdown, DesktopController::snapshot)
}
#[tauri::command]
fn desktop_command(
    command: DesktopCommand,
    controller: State<'_, Mutex<DesktopController>>,
    shutdown: State<'_, Arc<ShutdownState>>,
) -> Result<Snapshot, String> {
    with_controller(&controller, &shutdown, |controller| {
        controller.command(command)
    })?
}
fn initial_controller() -> Result<DesktopController, Box<dyn std::error::Error>> {
    #[cfg(feature = "desktop-verification")]
    {
        let fixture = std::env::var("MSHOGI_DESKTOP_FIXTURE").unwrap_or_else(|_| "standard".into());
        DesktopController::verification(engine_path()?, &fixture).map_err(Into::into)
    }
    #[cfg(not(feature = "desktop-verification"))]
    {
        Ok(DesktopController::new(engine_path()?)?)
    }
}
fn desktop_menu(app: &AppHandle) -> tauri::Result<Menu<tauri::Wry>> {
    // Cocoa's predefined Quit calls NSApplication::terminate directly. A custom
    // item routes ordinary Quit/Cmd-Q through the preventable Tauri exit request.
    let quit = MenuItem::with_id(app, "mshogi-quit", "Quit mShogi", true, Some("CmdOrCtrl+Q"))?;
    let application = Submenu::with_items(
        app,
        "mShogi",
        true,
        &[
            &PredefinedMenuItem::about(
                app,
                Some("About mShogi"),
                Some(AboutMetadata {
                    name: Some("mShogi".into()),
                    version: Some(env!("CARGO_PKG_VERSION").into()),
                    ..Default::default()
                }),
            )?,
            &PredefinedMenuItem::separator(app)?,
            #[cfg(target_os = "macos")]
            &PredefinedMenuItem::services(app, None)?,
            #[cfg(target_os = "macos")]
            &PredefinedMenuItem::hide(app, None)?,
            #[cfg(target_os = "macos")]
            &PredefinedMenuItem::hide_others(app, None)?,
            #[cfg(target_os = "macos")]
            &PredefinedMenuItem::show_all(app, None)?,
            &quit,
        ],
    )?;
    let edit = Submenu::with_items(
        app,
        "Edit",
        true,
        &[
            &PredefinedMenuItem::undo(app, None)?,
            &PredefinedMenuItem::redo(app, None)?,
            &PredefinedMenuItem::separator(app)?,
            &PredefinedMenuItem::cut(app, None)?,
            &PredefinedMenuItem::copy(app, None)?,
            &PredefinedMenuItem::paste(app, None)?,
            &PredefinedMenuItem::select_all(app, None)?,
        ],
    )?;
    #[cfg(target_os = "macos")]
    let view = Submenu::with_items(
        app,
        "View",
        true,
        &[&PredefinedMenuItem::fullscreen(app, None)?],
    )?;
    let window = Submenu::with_items(
        app,
        "Window",
        true,
        &[
            &PredefinedMenuItem::minimize(app, None)?,
            &PredefinedMenuItem::maximize(app, None)?,
            &PredefinedMenuItem::close_window(app, None)?,
        ],
    )?;
    Menu::with_items(
        app,
        &[
            &application,
            &edit,
            #[cfg(target_os = "macos")]
            &view,
            &window,
        ],
    )
}
fn signal_shutdown(app: &AppHandle, shutdown: &ShutdownState) -> bool {
    if shutdown.started.swap(true, Ordering::AcqRel) {
        return false;
    }
    if let Ok(mut deadline) = shutdown.deadline.lock() {
        *deadline = Some(Instant::now() + Duration::from_secs(2));
    }
    if let Ok(mut controller) = app.state::<Mutex<DesktopController>>().lock() {
        controller.begin_shutdown();
    } else {
        eprintln!("Native shutdown: session lock unavailable; cleanup is uncertain.");
    }
    true
}
fn observe_shutdown(app: &AppHandle, shutdown: &ShutdownState) {
    let deadline = shutdown
        .deadline
        .lock()
        .ok()
        .and_then(|deadline| *deadline)
        .unwrap_or_else(|| Instant::now() + Duration::from_secs(2));
    let mut acknowledged = false;
    while Instant::now() < deadline {
        acknowledged = app
            .state::<Mutex<DesktopController>>()
            .lock()
            .is_ok_and(|controller| controller.shutdown_acknowledged());
        if acknowledged {
            break;
        }
        thread::sleep(Duration::from_millis(5));
    }
    if !acknowledged {
        eprintln!(
            "Native shutdown: cleanup was not acknowledged within two seconds; process cleanup is uncertain."
        );
    }
}
fn main() {
    let shutdown = Arc::new(ShutdownState::default());
    let app = tauri::Builder::default()
        .manage(shutdown.clone())
        .menu(desktop_menu)
        .on_menu_event(|app, event| {
            if event.id().as_ref() == "mshogi-quit" {
                app.exit(0);
            }
        })
        .setup(|app| {
            app.manage(Mutex::new(initial_controller()?));
            #[cfg(feature = "desktop-verification")]
            if let Some(window) = app.get_webview_window("main") {
                let fixture =
                    std::env::var("MSHOGI_DESKTOP_FIXTURE").unwrap_or_else(|_| "standard".into());
                window.set_title(&format!("mShogi Verification — {fixture} (test state)"))?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![desktop_snapshot, desktop_command])
        .build(tauri::generate_context!())
        .expect("mShogi desktop host failed");
    app.run(move |app, event| match event {
        RunEvent::ExitRequested { api, .. } => {
            if shutdown.ready.load(Ordering::Acquire) {
                return;
            }
            api.prevent_exit();
            // Repeated Quit requests cannot bypass cleanup while stopping.
            if !signal_shutdown(app, &shutdown) {
                return;
            }
            let worker_app = app.clone();
            let worker_shutdown = shutdown.clone();
            if let Err(error) = thread::Builder::new()
                .name("mshogi-desktop-exit".into())
                .spawn(move || {
                    observe_shutdown(&worker_app, &worker_shutdown);
                    worker_shutdown
                        .observation_finished
                        .store(true, Ordering::Release);
                    worker_shutdown.ready.store(true, Ordering::Release);
                    worker_app.exit(0);
                })
            {
                eprintln!("Native shutdown: cannot start cleanup observer ({error}); observing during final teardown.");
                // The final Exit event below still waits for supervision.
                shutdown.ready.store(true, Ordering::Release);
                app.exit(0);
            }
        }
        RunEvent::Exit => {
            // Cocoa Dock/OS termination can bypass ExitRequested entirely. This
            // is final event-loop teardown: no more interaction is possible.
            // Only here, wait up to the observation deadline for the off-path
            // supervisor; never perform process I/O or search on this thread.
            signal_shutdown(app, &shutdown);
            // All routes share the deadline established by the first shutdown
            // signal. Final teardown never adds another observation interval.
            if !shutdown.observation_finished.swap(true, Ordering::AcqRel) {
                observe_shutdown(app, &shutdown);
            }
            shutdown.ready.store(true, Ordering::Release);
        }
        _ => {}
    });
}
