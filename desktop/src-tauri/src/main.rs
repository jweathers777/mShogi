#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use mshogi_desktop_session::{DesktopCommand, DesktopController, IoPhase, OpenToken, Snapshot};
#[cfg(feature = "desktop-verification")]
use mshogi_desktop_session::{SyntheticCommand, SyntheticController, SyntheticSnapshot};
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
#[cfg(feature = "desktop-verification")]
struct VerificationState {
    fixture: String,
    synthetic: Mutex<Option<SyntheticController>>,
}
#[cfg(feature = "desktop-verification")]
impl VerificationState {
    fn new(fixture: String) -> Self {
        let synthetic = (fixture == "synthetic").then(SyntheticController::new);
        Self {
            fixture,
            synthetic: Mutex::new(synthetic),
        }
    }
    fn is_synthetic(&self) -> bool {
        self.fixture == "synthetic"
    }
    fn require_chess(&self) -> Result<(), String> {
        if self.is_synthetic() {
            Err("Synthetic fixture actions and records are separate from chess documents.".into())
        } else {
            Ok(())
        }
    }
}
#[cfg(feature = "desktop-verification")]
const VERIFICATION_SIZE_MINIMUM: &str = "mshogi-verification-window-size-minimum";
#[cfg(feature = "desktop-verification")]
const VERIFICATION_SIZE_DEFAULT: &str = "mshogi-verification-window-size-default";
#[cfg(feature = "desktop-verification")]
fn verification_window_size(menu_id: &str) -> Option<tauri::LogicalSize<f64>> {
    match menu_id {
        VERIFICATION_SIZE_MINIMUM => Some(tauri::LogicalSize::new(760.0, 650.0)),
        VERIFICATION_SIZE_DEFAULT => Some(tauri::LogicalSize::new(1180.0, 820.0)),
        _ => None,
    }
}
#[cfg(feature = "desktop-verification")]
fn verification_window_title(fixture: &str, size: tauri::LogicalSize<f64>) -> String {
    format!(
        "mShogi Verification — {fixture} (test state) — content {:.1} × {:.1} logical",
        size.width, size.height
    )
}
#[cfg(feature = "desktop-verification")]
fn update_verification_window_title(app: &AppHandle) -> tauri::Result<()> {
    if let (Some(verification), Some(window)) = (
        app.try_state::<VerificationState>(),
        app.get_webview_window("main"),
    ) {
        // Report actual content dimensions, including OS constraints and DPI.
        let size = window
            .inner_size()?
            .to_logical::<f64>(window.scale_factor()?);
        window.set_title(&verification_window_title(&verification.fixture, size))?;
    }
    Ok(())
}
#[cfg(feature = "desktop-verification")]
fn resize_verification_window(
    app: &AppHandle,
    size: tauri::LogicalSize<f64>,
) -> Result<(), String> {
    let window = app
        .get_webview_window("main")
        .ok_or_else(|| "Verification window unavailable".to_string())?;
    window
        .set_fullscreen(false)
        .map_err(|error| error.to_string())?;
    window.unmaximize().map_err(|error| error.to_string())?;
    window.set_size(size).map_err(|error| error.to_string())?;
    // A subsequent native Resized event updates this after deferred OS changes.
    update_verification_window_title(app).map_err(|error| error.to_string())
}
fn ensure_chess_mode(_app: &AppHandle) -> Result<(), String> {
    #[cfg(feature = "desktop-verification")]
    _app.state::<VerificationState>().require_chess()?;
    Ok(())
}
#[cfg(feature = "desktop-verification")]
fn with_synthetic<T>(
    controller: &Mutex<DesktopController>,
    shutdown: &ShutdownState,
    verification: &VerificationState,
    operation: impl FnOnce(&mut SyntheticController) -> T,
) -> Result<T, String> {
    // Use the same outer lock as close approval; never acquire these in reverse.
    with_controller(controller, shutdown, |_| {
        if shutdown.close_requested.load(Ordering::Acquire) {
            return Err("Finish or cancel the close request before changing the fixture.".into());
        }
        let mut synthetic = verification
            .synthetic
            .lock()
            .map_err(|_| "Synthetic session lock unavailable".to_string())?;
        let synthetic = synthetic
            .as_mut()
            .ok_or_else(|| "The synthetic fixture is not active.".to_string())?;
        Ok(operation(synthetic))
    })?
}
#[cfg(feature = "desktop-verification")]
#[tauri::command]
fn desktop_verification_mode(verification: State<'_, VerificationState>) -> &'static str {
    if verification.is_synthetic() {
        "synthetic"
    } else if verification.fixture == "shared-contract" {
        "shared-contract"
    } else if verification.fixture.starts_with("shogi-") {
        "product"
    } else {
        "chess"
    }
}
#[cfg(feature = "desktop-verification")]
#[tauri::command]
fn synthetic_snapshot(
    controller: State<'_, Mutex<DesktopController>>,
    shutdown: State<'_, Arc<ShutdownState>>,
    verification: State<'_, VerificationState>,
) -> Result<SyntheticSnapshot, String> {
    with_synthetic(&controller, &shutdown, &verification, |synthetic| {
        synthetic.snapshot()
    })
}
#[cfg(feature = "desktop-verification")]
#[tauri::command]
fn synthetic_command(
    controller: State<'_, Mutex<DesktopController>>,
    shutdown: State<'_, Arc<ShutdownState>>,
    verification: State<'_, VerificationState>,
    command: SyntheticCommand,
) -> Result<SyntheticSnapshot, String> {
    with_synthetic(&controller, &shutdown, &verification, |synthetic| {
        synthetic.command(command)
    })?
}
// Normal product entry points. Raw commands retain duplicate fields through
// the authoritative parsers rather than a Tauri intermediate object map.
#[tauri::command]
fn game_snapshot(
    controller: State<'_, Mutex<DesktopController>>,
    shutdown: State<'_, Arc<ShutdownState>>,
) -> Result<mshogi_desktop_session::SharedSnapshot, String> {
    with_controller(&controller, &shutdown, DesktopController::shared_snapshot)
}
#[tauri::command]
async fn game_command(
    app: AppHandle,
    command_json: String,
) -> Result<mshogi_desktop_session::SharedSnapshot, String> {
    ensure_chess_mode(&app)?;
    let command = mshogi_desktop_session::SharedDesktopCommand::from_json(&command_json)?;
    let shutdown = app.state::<Arc<ShutdownState>>().inner().clone();
    let (initial, receipt, reservation) = with_controller(
        &app.state::<Mutex<DesktopController>>(),
        &shutdown,
        |controller| {
            let reservation = reserve_game_command(&shutdown)?;
            let (initial, receipt) = controller.shared_command_with_receipt(command)?;
            Ok::<_, String>((initial, receipt, reservation))
        },
    )??;
    if let Some(result) = receipt.completion() {
        result?;
        return Ok(initial);
    }
    tauri::async_runtime::spawn_blocking(move || {
        let _reservation = reservation;
        loop {
            let snapshot = current_contract_snapshot(&app)?;
            if let Some(result) = receipt.completion() {
                result?;
                return Ok(snapshot);
            }
            thread::sleep(Duration::from_millis(15));
        }
    })
    .await
    .map_err(|e| format!("Native admission waiter failed: {e}"))?
}
fn current_contract_snapshot(
    app: &AppHandle,
) -> Result<mshogi_desktop_session::SharedSnapshot, String> {
    with_controller(
        &app.state::<Mutex<DesktopController>>(),
        &app.state::<Arc<ShutdownState>>(),
        DesktopController::shared_snapshot,
    )
}
// Both games share native pickers, the bounded archive worker and atomic writer.
#[tauri::command]
async fn game_file(
    app: AppHandle,
    action: String,
    expected: mshogi_desktop_session::SharedOpenToken,
) -> Result<mshogi_desktop_session::SharedSnapshot, String> {
    if !matches!(action.as_str(), "save" | "save_as" | "open") {
        return Err("Unknown native file operation.".into());
    }
    let reservation = reserve_file_operation(&app)?;
    tauri::async_runtime::spawn_blocking(move || {
        let _reservation = reservation;
        let actual = with_controller(
            &app.state::<Mutex<DesktopController>>(),
            &app.state::<Arc<ShutdownState>>(),
            |c| c.shared_open_token(),
        )?;
        if actual != expected {
            return Err("The document changed. Review unsaved changes again.".into());
        }
        if app
            .state::<Arc<ShutdownState>>()
            .close_requested
            .load(Ordering::Acquire)
            && action == "open"
        {
            return Err("Finish or cancel the close request before opening.".into());
        }
        let snapshot = current_contract_snapshot(&app)?;
        let path = if action == "save" && snapshot.document.path.is_some() {
            snapshot.document.path.map(PathBuf::from)
        } else {
            let picker = app.dialog().file().add_filter("mShogi game", &["mshogi"]);
            let selected = if action == "open" {
                picker.set_title("Open mShogi game").blocking_pick_file()
            } else {
                picker
                    .set_title("Save mShogi game")
                    .set_file_name(
                        if snapshot.game == mshogi_desktop_session::GameKind::Shogi {
                            "shogi.mshogi"
                        } else {
                            "chess.mshogi"
                        },
                    )
                    .blocking_save_file()
            };
            selected
                .map(|p| p.into_path().map_err(|e| e.to_string()))
                .transpose()?
        };
        let Some(path) = path else {
            return current_contract_snapshot(&app);
        };
        let id = with_controller(
            &app.state::<Mutex<DesktopController>>(),
            &app.state::<Arc<ShutdownState>>(),
            |c| {
                if action == "open" {
                    c.queue_shared_open(path, expected)
                } else {
                    c.queue_shared_save(path)
                }
            },
        )??;
        loop {
            let snapshot = current_contract_snapshot(&app)?;
            if snapshot.io.id != id {
                return Err("Native file operation superseded.".into());
            }
            match snapshot.io.phase {
                IoPhase::Complete => return Ok(snapshot),
                IoPhase::Failed => {
                    return Err(snapshot
                        .io
                        .message
                        .unwrap_or_else(|| "Native file operation failed.".into()));
                }
                _ => thread::sleep(Duration::from_millis(15)),
            }
        }
    })
    .await
    .map_err(|e| format!("Native document waiter failed: {e}"))?
}

// Raw native commands preserve strict duplicate-field rejection all the way to
// the authoritative session parser. The development harness is only an adapter.
#[cfg(feature = "desktop-verification")]
#[tauri::command]
fn native_contract_snapshot(
    controller: State<'_, Mutex<DesktopController>>,
    shutdown: State<'_, Arc<ShutdownState>>,
) -> Result<mshogi_desktop_session::SharedSnapshot, String> {
    game_snapshot(controller, shutdown)
}
#[cfg(feature = "desktop-verification")]
#[tauri::command]
async fn native_contract_command(
    app: AppHandle,
    command_json: String,
) -> Result<mshogi_desktop_session::SharedSnapshot, String> {
    game_command(app, command_json).await
}
#[cfg(feature = "desktop-verification")]
#[tauri::command]
async fn native_contract_file(
    app: AppHandle,
    action: String,
    expected: mshogi_desktop_session::SharedOpenToken,
) -> Result<mshogi_desktop_session::SharedSnapshot, String> {
    game_file(app, action, expected).await
}
#[tauri::command]
async fn game_new(
    app: AppHandle,
    game: mshogi_desktop_session::GameKind,
    human: mshogi_desktop_session::NativeSide,
    expected: mshogi_desktop_session::SharedOpenToken,
) -> Result<mshogi_desktop_session::SharedSnapshot, String> {
    ensure_chess_mode(&app)?;
    let shutdown = app.state::<Arc<ShutdownState>>().inner().clone();
    let (initial, receipt, reservation) = with_controller(
        &app.state::<Mutex<DesktopController>>(),
        &shutdown,
        |controller| {
            let reservation = reserve_game_command(&shutdown)?;
            let (initial, receipt) = controller.shared_new_game(expected, game, human)?;
            Ok::<_, String>((initial, receipt, reservation))
        },
    )??;
    if let Some(result) = receipt.completion() {
        result?;
        return Ok(initial);
    }
    tauri::async_runtime::spawn_blocking(move || {
        let _reservation = reservation;
        loop {
            let snapshot = current_contract_snapshot(&app)?;
            if let Some(result) = receipt.completion() {
                result?;
                return Ok(snapshot);
            }
            thread::sleep(Duration::from_millis(15));
        }
    })
    .await
    .map_err(|e| format!("New game admission waiter failed: {e}"))?
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
// Call while holding the controller lock, like file and close ownership.
fn reserve_game_command(shutdown: &Arc<ShutdownState>) -> Result<CommandOperationGuard, String> {
    if shutdown.close_requested.load(Ordering::Acquire)
        || shutdown.file_operation.load(Ordering::Acquire)
    {
        return Err("Finish the document or close operation first.".into());
    }
    shutdown
        .command_operation
        .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
        .map_err(|_| "Wait for the current game command.".to_string())?;
    Ok(CommandOperationGuard(shutdown.clone()))
}
#[tauri::command]
async fn desktop_command(app: AppHandle, command: DesktopCommand) -> Result<Snapshot, String> {
    ensure_chess_mode(&app)?;
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
    ensure_chess_mode(app)?;
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
            |controller| controller.queue_shared_save(path),
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
                    Ok(controller.shared_open_token())
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
                    controller.queue_shared_open(path, actual)
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
#[cfg(feature = "desktop-verification")]
fn initial_controller(fixture: &str) -> Result<DesktopController, Box<dyn std::error::Error>> {
    let chess_fixture = if fixture == "synthetic" {
        "standard"
    } else {
        fixture
    };
    let engine = engine_path()?;
    let engine = if fixture == "shogi-failure" {
        // Deliberately absent child only in the separately labeled QA package.
        engine.with_file_name("mshogi-verification-missing-engine")
    } else {
        engine
    };
    DesktopController::verification(engine, chess_fixture).map_err(Into::into)
}
#[cfg(not(feature = "desktop-verification"))]
fn initial_controller() -> Result<DesktopController, Box<dyn std::error::Error>> {
    Ok(DesktopController::new(engine_path()?)?)
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
    #[cfg(feature = "desktop-verification")]
    let verification = Submenu::with_items(
        app,
        "Verification window size",
        true,
        &[
            &MenuItem::with_id(
                app,
                VERIFICATION_SIZE_MINIMUM,
                "760 × 650 logical content (minimum)",
                true,
                None::<&str>,
            )?,
            &MenuItem::with_id(
                app,
                VERIFICATION_SIZE_DEFAULT,
                "1180 × 820 logical content (default)",
                true,
                None::<&str>,
            )?,
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
            #[cfg(feature = "desktop-verification")]
            &verification,
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
            #[cfg(feature = "desktop-verification")]
            if let Some(size) = verification_window_size(event.id().as_ref()) {
                if let Err(error) = resize_verification_window(app, size) {
                    eprintln!("Verification window size failed: {error}");
                }
                return;
            }
            if event.id().as_ref() == "mshogi-quit"
                && let Err(error) = request_close(app)
            {
                eprintln!("Native close request failed: {error}");
            }
        })
        .on_window_event(|window, event| {
            #[cfg(feature = "desktop-verification")]
            if matches!(
                event,
                WindowEvent::Resized(_) | WindowEvent::ScaleFactorChanged { .. }
            ) && let Err(error) = update_verification_window_title(window.app_handle())
            {
                eprintln!("Verification window size observation failed: {error}");
            }
            if let WindowEvent::CloseRequested { api, .. } = event {
                // Keep the webview alive until Save/Discard/Cancel resolves.
                api.prevent_close();
                if let Err(error) = request_close(window.app_handle()) {
                    eprintln!("Native close request failed: {error}");
                }
            }
        })
        .setup(|app| {
            #[cfg(feature = "desktop-verification")]
            {
                let verification = VerificationState::new(
                    std::env::var("MSHOGI_DESKTOP_FIXTURE").unwrap_or_else(|_| "standard".into()),
                );
                app.manage(Mutex::new(initial_controller(&verification.fixture)?));
                app.manage(verification);
                update_verification_window_title(app.handle())?;
            }
            #[cfg(not(feature = "desktop-verification"))]
            app.manage(Mutex::new(initial_controller()?));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            game_snapshot,
            game_command,
            game_file,
            game_new,
            desktop_snapshot,
            desktop_command,
            desktop_save,
            desktop_save_as,
            desktop_open,
            desktop_request_close,
            desktop_cancel_close,
            desktop_finish_close,
            #[cfg(feature = "desktop-verification")]
            desktop_verification_mode,
            #[cfg(feature = "desktop-verification")]
            native_contract_snapshot,
            #[cfg(feature = "desktop-verification")]
            native_contract_command,
            #[cfg(feature = "desktop-verification")]
            native_contract_file,
            #[cfg(feature = "desktop-verification")]
            synthetic_snapshot,
            #[cfg(feature = "desktop-verification")]
            synthetic_command,
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

    #[cfg(feature = "desktop-verification")]
    #[test]
    fn verification_size_title_reports_measured_content_and_preserves_fixture_identity() {
        let physical = tauri::PhysicalSize::new(1520_u32, 1300_u32);
        let measured = physical.to_logical::<f64>(2.0);
        assert_eq!(
            verification_window_size(VERIFICATION_SIZE_MINIMUM),
            Some(measured)
        );
        assert_eq!(
            verification_window_size(VERIFICATION_SIZE_DEFAULT),
            Some(tauri::LogicalSize::new(1180.0, 820.0))
        );
        assert_eq!(verification_window_size("mshogi-quit"), None);
        assert_eq!(
            verification_window_title("synthetic", measured),
            "mShogi Verification — synthetic (test state) — content 760.0 × 650.0 logical"
        );
        let constrained = tauri::LogicalSize::new(1000.0, 700.0);
        assert!(
            verification_window_title("claim", constrained)
                .ends_with("content 1000.0 × 700.0 logical")
        );
    }

    #[cfg(feature = "desktop-verification")]
    #[test]
    fn synthetic_mutations_share_close_gates_and_keep_chess_separate() {
        let controller = controller();
        let shutdown = ShutdownState::default();
        let verification = VerificationState::new("synthetic".into());
        assert!(verification.require_chess().is_err());
        let initial = with_synthetic(&controller, &shutdown, &verification, |synthetic| {
            synthetic.snapshot()
        })
        .unwrap();
        shutdown.close_requested.store(true, Ordering::Release);
        assert!(
            with_synthetic(&controller, &shutdown, &verification, |_| panic!(
                "pending close entered fixture"
            ))
            .is_err()
        );
        shutdown.close_requested.store(false, Ordering::Release);
        let reset = with_synthetic(&controller, &shutdown, &verification, |synthetic| {
            synthetic.command(SyntheticCommand::Reset {
                version: initial.version,
            })
        })
        .unwrap()
        .unwrap();
        assert!(reset.version > initial.version);
        let chess = with_controller(&controller, &shutdown, DesktopController::snapshot).unwrap();
        assert!(chess.history.is_empty());
        assert!(!chess.document.dirty);
        shutdown.close_requested.store(true, Ordering::Release);
        with_controller(&controller, &shutdown, |controller| {
            approve_close(controller, &shutdown, chess.document.revision)
        })
        .unwrap()
        .unwrap();
        assert!(
            with_synthetic(&controller, &shutdown, &verification, |_| panic!(
                "approved close entered fixture"
            ))
            .is_err()
        );
    }

    #[cfg(feature = "desktop-verification")]
    #[test]
    fn chess_verification_mode_has_no_active_synthetic_session() {
        let controller = controller();
        let shutdown = ShutdownState::default();
        let verification = VerificationState::new("standard".into());
        assert!(verification.require_chess().is_ok());
        assert!(
            with_synthetic(&controller, &shutdown, &verification, |_| panic!(
                "inactive fixture entered"
            ))
            .is_err()
        );
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
    #[test]
    fn product_command_reservation_releases_on_failure_and_respects_document_close_ownership() {
        let shutdown = Arc::new(ShutdownState::default());
        let first = reserve_game_command(&shutdown).unwrap();
        assert!(reserve_game_command(&shutdown).is_err());
        drop(first);
        assert!(!shutdown.command_operation.load(Ordering::Acquire));
        shutdown.file_operation.store(true, Ordering::Release);
        assert!(reserve_game_command(&shutdown).is_err());
        shutdown.file_operation.store(false, Ordering::Release);
        shutdown.close_requested.store(true, Ordering::Release);
        assert!(reserve_game_command(&shutdown).is_err());
        shutdown.close_requested.store(false, Ordering::Release);
        assert!(reserve_game_command(&shutdown).is_ok());
    }
    #[test]
    fn shogi_product_close_uses_native_document_revision_and_stops_execution() {
        use mshogi_desktop_session::{GameKind, NativeSide};
        let controller = controller();
        let shutdown = Arc::new(ShutdownState::default());
        let receipt = with_controller(&controller, &shutdown, |c| {
            c.shared_new_game(
                c.shared_open_token(),
                GameKind::Shogi,
                NativeSide::Shogi(mshogi_desktop_session::ShogiSide::Sente),
            )
        })
        .unwrap()
        .unwrap()
        .1;
        let deadline = Instant::now() + Duration::from_secs(20);
        while receipt.completion().is_none() {
            with_controller(&controller, &shutdown, DesktopController::shared_snapshot).unwrap();
            assert!(Instant::now() < deadline);
            thread::sleep(Duration::from_millis(1));
        }
        receipt.completion().unwrap().unwrap();
        let native =
            with_controller(&controller, &shutdown, DesktopController::shared_snapshot).unwrap();
        assert_eq!(native.game, GameKind::Shogi);
        assert!(native.document.dirty);
        let paused =
            with_controller(&controller, &shutdown, DesktopController::pause_for_close).unwrap();
        assert!(paused.document.revision >= native.document.revision);
        shutdown.close_requested.store(true, Ordering::Release);
        assert!(
            with_controller(&controller, &shutdown, |c| approve_close(
                c,
                &shutdown,
                native.document.revision
            ))
            .unwrap()
            .is_err()
        );
        with_controller(&controller, &shutdown, |c| {
            approve_close(c, &shutdown, paused.document.revision)
        })
        .unwrap()
        .unwrap();
        assert!(shutdown.close_approved.load(Ordering::Acquire));
    }
}
