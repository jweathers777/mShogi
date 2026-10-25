#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use mshogi_desktop_session::{DesktopCommand, DesktopController, IoPhase, OpenToken, Snapshot};
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
    AppHandle, Emitter, Manager, RunEvent, State, WindowEvent,
    menu::{AboutMetadata, Menu, MenuItem, PredefinedMenuItem, Submenu},
};
use tauri_plugin_dialog::DialogExt;

#[derive(Default)]
struct ShutdownState {
    command_operation: AtomicBool,
    close_requested: AtomicBool,
    close_approved: AtomicBool,
    file_operation: AtomicBool,
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
    if shutdown.started.load(Ordering::Acquire) || shutdown.close_approved.load(Ordering::Acquire) {
        return Err("The application is closing; no further game commands are accepted.".into());
    }
    let mut controller = controller
        .lock()
        .map_err(|_| "Native session lock unavailable".to_string())?;
    if shutdown.started.load(Ordering::Acquire) || shutdown.close_approved.load(Ordering::Acquire) {
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
struct CommandOperationGuard(Arc<ShutdownState>);
impl Drop for CommandOperationGuard {
    fn drop(&mut self) {
        self.0.command_operation.store(false, Ordering::Release);
    }
}
#[tauri::command]
async fn desktop_command(app: AppHandle, command: DesktopCommand) -> Result<Snapshot, String> {
    let shutdown = app.state::<Arc<ShutdownState>>().inner().clone();
    let (initial, receipt, reservation) = with_controller(
        &app.state::<Mutex<DesktopController>>(),
        &shutdown,
        |controller| {
            if shutdown.close_requested.load(Ordering::Acquire) {
                return Err(
                    "Finish or cancel the close request before changing the game.".to_string(),
                );
            }
            shutdown
                .command_operation
                .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
                .map_err(|_| "Wait for the current native command to complete.".to_string())?;
            let reservation = CommandOperationGuard(shutdown.clone());
            let (snapshot, receipt) = controller.command_with_receipt(command)?;
            Ok((snapshot, receipt, reservation))
        },
    )??;
    if let Some(completion) = receipt.completion() {
        completion?;
        return Ok(initial);
    }
    // One bounded host waiter. Serialization/validation belongs to the native
    // admission worker; the controller lock is released between polls. A
    // stable receipt survives later engine-result admissions and cancellation.
    tauri::async_runtime::spawn_blocking(move || {
        let _reservation = reservation;
        loop {
            let snapshot = current_snapshot(&app)?;
            if let Some(completion) = receipt.completion() {
                completion?;
                return Ok(snapshot);
            }
            thread::sleep(Duration::from_millis(15));
        }
    })
    .await
    .map_err(|error| format!("Native command admission failed: {error}"))?
}

/// At most one native picker/waiter may exist. Acquisition and close approval
/// are serialized with the controller, while dialogs and I/O never hold it.
struct FileOperationGuard(Arc<ShutdownState>);
impl Drop for FileOperationGuard {
    fn drop(&mut self) {
        self.0.file_operation.store(false, Ordering::Release);
    }
}
fn reserve_file_operation(app: &AppHandle) -> Result<FileOperationGuard, String> {
    let shutdown = app.state::<Arc<ShutdownState>>().inner().clone();
    with_controller(&app.state::<Mutex<DesktopController>>(), &shutdown, |_| {
        shutdown
            .file_operation
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
            .map(|_| ())
            .map_err(|_| "Finish the current file operation first.".to_string())
    })??;
    Ok(FileOperationGuard(shutdown))
}
fn current_snapshot(app: &AppHandle) -> Result<Snapshot, String> {
    with_controller(
        &app.state::<Mutex<DesktopController>>(),
        &app.state::<Arc<ShutdownState>>(),
        DesktopController::snapshot,
    )
}
fn wait_file_operation(app: &AppHandle, id: u64) -> Result<Snapshot, String> {
    loop {
        let snapshot = current_snapshot(app)?;
        if snapshot.io.id != id {
            return Err("The file operation was superseded; no completion was accepted.".into());
        }
        match snapshot.io.phase {
            IoPhase::Complete => return Ok(snapshot),
            IoPhase::Failed => {
                return Err(snapshot
                    .io
                    .message
                    .unwrap_or_else(|| "File operation failed.".into()));
            }
            _ => thread::sleep(Duration::from_millis(15)),
        }
    }
}
async fn save_document(app: AppHandle, save_as: bool) -> Result<Snapshot, String> {
    let reservation = reserve_file_operation(&app)?;
    tauri::async_runtime::spawn_blocking(move || {
        let _reservation = reservation;
        let existing = current_snapshot(&app)?.document.path;
        let path = if !save_as && existing.is_some() {
            existing.map(PathBuf::from)
        } else {
            app.dialog()
                .file()
                .set_title("Save mShogi game")
                .add_filter("mShogi game", &["mshogi"])
                .set_file_name("game.mshogi")
                .blocking_save_file()
                .map(|path| path.into_path().map_err(|error| error.to_string()))
                .transpose()?
        };
        let Some(path) = path else {
            return current_snapshot(&app);
        };
        let id = with_controller(
            &app.state::<Mutex<DesktopController>>(),
            &app.state::<Arc<ShutdownState>>(),
            |controller| controller.queue_save(path),
        )??;
        wait_file_operation(&app, id)
    })
    .await
    .map_err(|error| format!("Native save task failed: {error}"))?
}
#[tauri::command]
async fn desktop_save(app: AppHandle) -> Result<Snapshot, String> {
    save_document(app, false).await
}
#[tauri::command]
async fn desktop_save_as(app: AppHandle) -> Result<Snapshot, String> {
    save_document(app, true).await
}
#[tauri::command]
async fn desktop_open(
    app: AppHandle,
    expected_version: u64,
    expected_document_revision: u64,
) -> Result<Snapshot, String> {
    let reservation = reserve_file_operation(&app)?;
    let token = OpenToken {
        version: expected_version,
        document_revision: expected_document_revision,
    };
    tauri::async_runtime::spawn_blocking(move || {
        let _reservation = reservation;
        // Reject stale guard decisions before showing a picker as well as at
        // queue/commit time. A picker never grants authority to replace a game.
        let actual = with_controller(
            &app.state::<Mutex<DesktopController>>(),
            &app.state::<Arc<ShutdownState>>(),
            |controller| {
                if app
                    .state::<Arc<ShutdownState>>()
                    .close_requested
                    .load(Ordering::Acquire)
                {
                    Err("Finish or cancel the close request before opening a game.".to_string())
                } else {
                    Ok(controller.open_token())
                }
            },
        )??;
        if actual.version != token.version || actual.document_revision != token.document_revision {
            return Err("The game changed. Review unsaved changes and try Open again.".into());
        }
        let path = app
            .dialog()
            .file()
            .set_title("Open mShogi game")
            .add_filter("mShogi game", &["mshogi"])
            .blocking_pick_file()
            .map(|path| path.into_path().map_err(|error| error.to_string()))
            .transpose()?;
        let Some(path) = path else {
            return current_snapshot(&app);
        };
        let id = with_controller(
            &app.state::<Mutex<DesktopController>>(),
            &app.state::<Arc<ShutdownState>>(),
            |controller| {
                if app
                    .state::<Arc<ShutdownState>>()
                    .close_requested
                    .load(Ordering::Acquire)
                {
                    Err("Finish or cancel the close request before opening a game.".to_string())
                } else {
                    controller.queue_open(path, token)
                }
            },
        )??;
        wait_file_operation(&app, id)
    })
    .await
    .map_err(|error| format!("Native open task failed: {error}"))?
}
fn request_close(app: &AppHandle) -> Result<(), String> {
    let shutdown = app.state::<Arc<ShutdownState>>();
    let snapshot = with_controller(
        &app.state::<Mutex<DesktopController>>(),
        &shutdown,
        |controller| {
            if shutdown.close_requested.swap(true, Ordering::AcqRel) {
                controller.snapshot()
            } else {
                controller.pause_for_close()
            }
        },
    )?;
    // Re-emitting on a repeated request also covers a request made before the
    // frontend event listener finished initializing. The UI deduplicates it.
    if let Err(error) = app.emit("desktop-close-requested", snapshot) {
        shutdown.close_requested.store(false, Ordering::Release);
        return Err(format!(
            "Could not request unsaved-change confirmation: {error}"
        ));
    }
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let _ = window.set_focus();
    }
    Ok(())
}
#[tauri::command]
fn desktop_request_close(app: AppHandle) -> Result<(), String> {
    request_close(&app)
}
#[tauri::command]
fn desktop_cancel_close(
    controller: State<'_, Mutex<DesktopController>>,
    shutdown: State<'_, Arc<ShutdownState>>,
) -> Result<Snapshot, String> {
    with_controller(&controller, &shutdown, |controller| {
        shutdown.close_requested.store(false, Ordering::Release);
        controller.snapshot()
    })
}
#[tauri::command]
fn desktop_finish_close(app: AppHandle, expected_document_revision: u64) -> Result<(), String> {
    let shutdown = app.state::<Arc<ShutdownState>>();
    with_controller(
        &app.state::<Mutex<DesktopController>>(),
        &shutdown,
        |controller| approve_close(controller, &shutdown, expected_document_revision),
    )??;
    app.exit(0);
    Ok(())
}
fn approve_close(
    controller: &mut DesktopController,
    shutdown: &ShutdownState,
    expected_document_revision: u64,
) -> Result<(), String> {
    if !shutdown.close_requested.load(Ordering::Acquire) {
        return Err("Request close before confirming it.".to_string());
    }
    if shutdown.file_operation.load(Ordering::Acquire) {
        return Err("Wait for the file operation before closing.".to_string());
    }
    let snapshot = controller.snapshot();
    if snapshot.document.revision != expected_document_revision {
        return Err("The game changed. Review unsaved changes before closing.".to_string());
    }
    // Caller holds the same lock as every native mutation. Shutdown still has
    // its own flag so approval cannot skip child cleanup.
    shutdown.close_approved.store(true, Ordering::Release);
    Ok(())
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
            // Cocoa's predefined item calls NSApplication::terminate:, like
            // Dock Quit. Keep this native-route probe out of normal packages.
            #[cfg(all(feature = "desktop-verification", target_os = "macos"))]
            &PredefinedMenuItem::quit(app, Some("Native Cocoa Quit (verification)"))?,
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
        .plugin(tauri_plugin_dialog::init())
        .menu(desktop_menu)
        .on_menu_event(|app, event| {
            if event.id().as_ref() == "mshogi-quit"
                && let Err(error) = request_close(app)
            {
                eprintln!("Native close request failed: {error}");
            }
        })
        .on_window_event(|window, event| {
            if let WindowEvent::CloseRequested { api, .. } = event {
                // Keep the webview alive until Save/Discard/Cancel resolves.
                api.prevent_close();
                if let Err(error) = request_close(window.app_handle()) {
                    eprintln!("Native close request failed: {error}");
                }
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
        .invoke_handler(tauri::generate_handler![
            desktop_snapshot,
            desktop_command,
            desktop_save,
            desktop_save_as,
            desktop_open,
            desktop_request_close,
            desktop_cancel_close,
            desktop_finish_close,
        ])
        .build(tauri::generate_context!())
        .expect("mShogi desktop host failed");
    app.run(move |app, event| match event {
        RunEvent::ExitRequested { api, .. } => {
            if shutdown.ready.load(Ordering::Acquire) {
                return;
            }
            api.prevent_exit();
            if !shutdown.close_approved.load(Ordering::Acquire) {
                if let Err(error) = request_close(app) {
                    eprintln!("Native close request failed: {error}");
                }
                return;
            }
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

#[cfg(test)]
mod tests {
    use super::*;

    fn controller() -> Mutex<DesktopController> {
        // No search is submitted in these host-gate tests, so no child starts.
        Mutex::new(DesktopController::new(PathBuf::from("unused-test-engine")).unwrap())
    }

    #[test]
    fn stale_or_unrequested_close_cannot_authorize_shutdown() {
        let controller = controller();
        let shutdown = ShutdownState::default();
        let snapshot =
            with_controller(&controller, &shutdown, DesktopController::snapshot).unwrap();
        assert!(
            with_controller(&controller, &shutdown, |controller| {
                approve_close(controller, &shutdown, snapshot.document.revision)
            })
            .unwrap()
            .is_err()
        );
        shutdown.close_requested.store(true, Ordering::Release);
        assert!(
            with_controller(&controller, &shutdown, |controller| {
                approve_close(controller, &shutdown, snapshot.document.revision + 1)
            })
            .unwrap()
            .is_err()
        );
        assert!(!shutdown.close_approved.load(Ordering::Acquire));
        assert!(with_controller(&controller, &shutdown, DesktopController::snapshot).is_ok());
    }

    #[test]
    fn file_operation_blocks_close_then_approval_blocks_every_controller_entry() {
        let controller = controller();
        let shutdown = Arc::new(ShutdownState::default());
        let snapshot =
            with_controller(&controller, &shutdown, DesktopController::snapshot).unwrap();
        shutdown.close_requested.store(true, Ordering::Release);
        shutdown.file_operation.store(true, Ordering::Release);
        let operation = FileOperationGuard(shutdown.clone());
        assert!(
            with_controller(&controller, &shutdown, |controller| {
                approve_close(controller, &shutdown, snapshot.document.revision)
            })
            .unwrap()
            .is_err()
        );
        drop(operation);
        with_controller(&controller, &shutdown, |controller| {
            approve_close(controller, &shutdown, snapshot.document.revision)
        })
        .unwrap()
        .unwrap();
        assert!(
            with_controller(&controller, &shutdown, |_| panic!(
                "closed controller entered"
            ))
            .is_err()
        );
        assert!(
            !shutdown.started.load(Ordering::Acquire),
            "approval must leave cleanup available"
        );
    }
}
