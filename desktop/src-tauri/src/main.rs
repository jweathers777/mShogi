#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use mshogi_desktop_session::{
    ApplicationLifecycle, DesktopCommand, DesktopController, IoPhase, OpenToken,
    OperationReservation, SharedOpenToken, SharedSnapshot, Snapshot,
};
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
    with_controller(controller, shutdown, |controller| {
        if controller.lifecycle().close_requested() {
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
    } else if verification.fixture.starts_with("shogi-")
        || verification.fixture.starts_with("xiangqi-")
    {
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
fn game_inspection(
    controller: State<'_, Mutex<DesktopController>>,
    shutdown: State<'_, Arc<ShutdownState>>,
    query_json: String,
) -> Result<mshogi_desktop_session::MovementInspection, String> {
    inspect_game(&controller, &shutdown, &query_json)
}
fn inspect_game(
    controller: &Mutex<DesktopController>,
    shutdown: &ShutdownState,
    query_json: &str,
) -> Result<mshogi_desktop_session::MovementInspection, String> {
    let query = mshogi_desktop_session::MovementInspectionQuery::from_json(query_json)?;
    with_controller(controller, shutdown, |controller| {
        controller.shared_inspection(query)
    })?
}
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
            let reservation = controller.lifecycle().reserve_command()?;
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
        if app_lifecycle(&app).close_requested() && action == "open" {
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
                    .set_file_name(match snapshot.game {
                        mshogi_desktop_session::GameKind::Chess => "chess.mshogi",
                        mshogi_desktop_session::GameKind::Shogi => "shogi.mshogi",
                        mshogi_desktop_session::GameKind::Xiangqi => "xiangqi.mshogi",
                    })
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
            |c| queue_game_file(c, &action, path, expected),
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

// Called after native picker completion under controller serialization. The
// picker itself grants no authority to replace a document or bypass close.
fn queue_game_file(
    controller: &mut DesktopController,
    action: &str,
    path: PathBuf,
    expected: SharedOpenToken,
) -> Result<u64, String> {
    match action {
        "open" => {
            if controller.lifecycle().close_requested() {
                return Err("Finish or cancel the close request before opening.".into());
            }
            controller.queue_product_open(path, expected)
        }
        "save" | "save_as" => controller.queue_shared_save(path),
        _ => Err("Unknown native file operation.".into()),
    }
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
            let reservation = controller.lifecycle().reserve_command()?;
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
    if shutdown.started.load(Ordering::Acquire) {
        return Err("The application is closing; no further game commands are accepted.".into());
    }
    let mut controller = controller
        .lock()
        .map_err(|_| "Native session lock unavailable".to_string())?;
    if shutdown.started.load(Ordering::Acquire) || controller.lifecycle().close_approved() {
        return Err("The application is closing; no further game commands are accepted.".into());
    }
    Ok(operation(&mut controller))
}
#[tauri::command]
fn desktop_snapshot(
    controller: State<'_, Mutex<DesktopController>>,
    shutdown: State<'_, Arc<ShutdownState>>,
) -> Result<Snapshot, String> {
    with_controller(&controller, &shutdown, legacy_snapshot)?
}
// Current native lifecycle owns all document-operation reservations. The host
// retains only technical event-loop/cleanup state.
fn app_lifecycle(app: &AppHandle) -> Arc<ApplicationLifecycle> {
    app.state::<Arc<ApplicationLifecycle>>().inner().clone()
}
fn legacy_snapshot(controller: &mut DesktopController) -> Result<Snapshot, String> {
    controller
        .shared_snapshot()
        .chess
        .ok_or_else(|| "The active native game uses the current application contract.".into())
}
#[tauri::command]
async fn desktop_command(app: AppHandle, command: DesktopCommand) -> Result<Snapshot, String> {
    ensure_chess_mode(&app)?;
    let shutdown = app.state::<Arc<ShutdownState>>().inner().clone();
    let (initial, receipt, reservation) = with_controller(
        &app.state::<Mutex<DesktopController>>(),
        &shutdown,
        |controller| {
            let reservation = controller.lifecycle().reserve_command()?;
            let (snapshot, receipt) = controller.command_with_receipt(command)?;
            Ok::<_, String>((snapshot, receipt, reservation))
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

/// The native lifecycle reserves both picker and file completion. Dialogs and
/// I/O run off the serialized controller lock.
fn reserve_file_operation(app: &AppHandle) -> Result<OperationReservation, String> {
    ensure_chess_mode(app)?;
    let shutdown = app.state::<Arc<ShutdownState>>();
    with_controller(
        &app.state::<Mutex<DesktopController>>(),
        &shutdown,
        |controller| controller.lifecycle().reserve_file(),
    )?
}
fn current_snapshot(app: &AppHandle) -> Result<Snapshot, String> {
    with_controller(
        &app.state::<Mutex<DesktopController>>(),
        &app.state::<Arc<ShutdownState>>(),
        legacy_snapshot,
    )?
}
fn wait_file_operation(app: &AppHandle, id: u64) -> Result<Option<Snapshot>, String> {
    loop {
        let snapshot = current_contract_snapshot(app)?;
        if snapshot.io.id != id {
            return Err("The file operation was superseded; no completion was accepted.".into());
        }
        match snapshot.io.phase {
            IoPhase::Complete => return Ok(snapshot.chess),
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
async fn save_document(app: AppHandle, save_as: bool) -> Result<Option<Snapshot>, String> {
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
            return current_snapshot(&app).map(Some);
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
async fn desktop_save(app: AppHandle) -> Result<Option<Snapshot>, String> {
    save_document(app, false).await
}
#[tauri::command]
async fn desktop_save_as(app: AppHandle) -> Result<Option<Snapshot>, String> {
    save_document(app, true).await
}
#[tauri::command]
async fn desktop_open(
    app: AppHandle,
    expected_version: u64,
    expected_document_revision: u64,
) -> Result<Option<Snapshot>, String> {
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
                if controller.lifecycle().close_requested() {
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
            return current_snapshot(&app).map(Some);
        };
        let id = with_controller(
            &app.state::<Mutex<DesktopController>>(),
            &app.state::<Arc<ShutdownState>>(),
            |controller| {
                if controller.lifecycle().close_requested() {
                    Err("Finish or cancel the close request before opening a game.".to_string())
                } else {
                    controller.queue_product_open(path, actual)
                }
            },
        )??;
        wait_file_operation(&app, id)
    })
    .await
    .map_err(|error| format!("Native open task failed: {error}"))?
}
// Current native close contract. The old desktop_* routes/events below are
// the sole frontend forwarder for App/useNativeSession and verification views;
// MSH-119 deletes them with those callers. They own no native document state.
fn request_close(app: &AppHandle) -> Result<(), String> {
    let shutdown = app.state::<Arc<ShutdownState>>();
    let snapshot = with_controller(
        &app.state::<Mutex<DesktopController>>(),
        &shutdown,
        |controller| -> Result<SharedSnapshot, String> {
            if controller.lifecycle().request_close()? {
                Ok(controller.application_pause_for_close())
            } else {
                Ok(controller.shared_snapshot())
            }
        },
    )??;
    if let Err(error) = app.emit("game-close-requested", &snapshot) {
        app_lifecycle(app).cancel_close()?;
        return Err(format!(
            "Could not request unsaved-change confirmation: {error}"
        ));
    }
    if let Some(chess) = snapshot.chess {
        app.emit("desktop-close-requested", chess)
            .map_err(|error| {
                let _ = app_lifecycle(app).cancel_close();
                format!("Could not forward Chess close confirmation: {error}")
            })?;
    }
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let _ = window.set_focus();
    }
    Ok(())
}
#[tauri::command]
fn game_request_close(app: AppHandle) -> Result<(), String> {
    request_close(&app)
}
#[tauri::command]
fn game_cancel_close(
    controller: State<'_, Mutex<DesktopController>>,
    shutdown: State<'_, Arc<ShutdownState>>,
) -> Result<SharedSnapshot, String> {
    with_controller(&controller, &shutdown, |controller| {
        controller.lifecycle().cancel_close()?;
        Ok(controller.shared_snapshot())
    })?
}
#[tauri::command]
fn game_finish_close(app: AppHandle, expected: SharedOpenToken) -> Result<(), String> {
    let shutdown = app.state::<Arc<ShutdownState>>();
    with_controller(
        &app.state::<Mutex<DesktopController>>(),
        &shutdown,
        |controller| approve_close(controller, expected),
    )??;
    app.exit(0);
    Ok(())
}
fn approve_close(
    controller: &mut DesktopController,
    expected: SharedOpenToken,
) -> Result<(), String> {
    // Caller holds the same controller lock as all accepted mutations.
    controller
        .lifecycle()
        .approve_close(expected, controller.shared_open_token())
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
        controller.lifecycle().cancel_close()?;
        legacy_snapshot(controller)
    })?
}
#[tauri::command]
fn desktop_finish_close(app: AppHandle, expected_document_revision: u64) -> Result<(), String> {
    let shutdown = app.state::<Arc<ShutdownState>>();
    with_controller(
        &app.state::<Mutex<DesktopController>>(),
        &shutdown,
        |controller| {
            let expected = controller.shared_open_token();
            if expected.document_revision != expected_document_revision {
                return Err("The game changed. Review unsaved changes before closing.".into());
            }
            approve_close(controller, expected)
        },
    )??;
    app.exit(0);
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
    // Only the separately labeled QA host can select an external delay/fault
    // wrapper. The wrapper still execs the real bundled engine and supplies no
    // telemetry or responses. Normal distributables have no override.
    let engine = std::env::var_os("MSHOGI_VERIFICATION_ENGINE")
        .map(std::path::PathBuf::from)
        .unwrap_or(engine);
    let engine = if matches!(fixture, "shogi-failure" | "xiangqi-failure") {
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
                let controller = initial_controller(&verification.fixture)?;
                app.manage(controller.lifecycle());
                app.manage(Mutex::new(controller));
                app.manage(verification);
                update_verification_window_title(app.handle())?;
            }
            #[cfg(not(feature = "desktop-verification"))]
            {
                let controller = initial_controller()?;
                app.manage(controller.lifecycle());
                app.manage(Mutex::new(controller));
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            game_inspection,
            game_snapshot,
            game_command,
            game_file,
            game_new,
            game_request_close,
            game_cancel_close,
            game_finish_close,
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
            if !app_lifecycle(app).close_approved() {
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
        Mutex::new(DesktopController::new(PathBuf::from("unused-test-engine")).unwrap())
    }
    #[test]
    fn normal_movement_inspection_is_strict_atomic_and_does_not_poll() {
        let controller = controller();
        let shutdown = ShutdownState::default();
        let token = controller.lock().unwrap().shared_token();
        let query = format!(
            r#"{{"expected":{{"document":{},"version":{},"revision":{},"selection":{}}},"origin":"b8"}}"#,
            token.document, token.version, token.revision, token.selection
        );
        let result = inspect_game(&controller, &shutdown, &query).unwrap();
        assert_eq!(result.label, "Black knight");
        assert!(result.read_only);
        assert_eq!(result.token, token);
        assert_eq!(controller.lock().unwrap().shared_token(), token);
        assert!(
            inspect_game(
                &controller,
                &shutdown,
                &query.replace("\"b8\"", "\"b8\",\"origin\":\"a8\"")
            )
            .is_err()
        );
        assert_eq!(controller.lock().unwrap().shared_token(), token);
        shutdown.started.store(true, Ordering::Release);
        assert!(inspect_game(&controller, &shutdown, &query).is_err());
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

    #[test]
    fn stale_or_unrequested_close_cannot_authorize_shutdown() {
        let controller = controller();
        let shutdown = ShutdownState::default();
        let token = controller.lock().unwrap().shared_open_token();
        assert!(
            with_controller(&controller, &shutdown, |c| approve_close(c, token))
                .unwrap()
                .is_err()
        );
        controller
            .lock()
            .unwrap()
            .lifecycle()
            .request_close()
            .unwrap();
        for stale in [
            SharedOpenToken {
                document: token.document + 1,
                ..token
            },
            SharedOpenToken {
                version: token.version + 1,
                ..token
            },
            SharedOpenToken {
                document_revision: token.document_revision + 1,
                ..token
            },
        ] {
            assert!(
                with_controller(&controller, &shutdown, |c| approve_close(c, stale))
                    .unwrap()
                    .is_err()
            );
        }
        assert!(!controller.lock().unwrap().lifecycle().close_approved());
        assert!(with_controller(&controller, &shutdown, |_| ()).is_ok());
    }

    #[test]
    fn native_file_reservation_blocks_close_then_approval_blocks_every_host_entry() {
        let controller = controller();
        let shutdown = ShutdownState::default();
        let (token, lifecycle) = {
            let c = controller.lock().unwrap();
            (c.shared_open_token(), c.lifecycle())
        };
        lifecycle.request_close().unwrap();
        let file = lifecycle.reserve_file().unwrap();
        assert!(
            with_controller(&controller, &shutdown, |c| approve_close(c, token))
                .unwrap()
                .is_err()
        );
        drop(file);
        with_controller(&controller, &shutdown, |c| approve_close(c, token))
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
            "native approval must leave technical cleanup available"
        );
    }

    #[test]
    fn native_command_reservation_releases_after_rejection_and_excludes_file_work() {
        let controller = controller();
        let lifecycle = controller.lock().unwrap().lifecycle();
        let first = lifecycle.reserve_command().unwrap();
        assert!(lifecycle.reserve_command().is_err());
        assert!(lifecycle.reserve_file().is_err());
        drop(first);
        assert!(!lifecycle.operation_active());
        lifecycle.request_close().unwrap();
        assert!(lifecycle.reserve_command().is_err());
        lifecycle.cancel_close().unwrap();
        assert!(lifecycle.reserve_command().is_ok());
    }

    #[test]
    fn picker_result_cannot_open_during_same_token_close_confirmation() {
        let mut controller = DesktopController::new(PathBuf::from("unused-test-engine")).unwrap();
        let expected = controller.shared_open_token();
        let before_io = controller.shared_snapshot().io.id;
        controller.lifecycle().request_close().unwrap();
        assert_eq!(controller.shared_open_token(), expected);
        let error = queue_game_file(
            &mut controller,
            "open",
            PathBuf::from("unread.mshogi"),
            expected,
        )
        .unwrap_err();
        assert!(error.contains("close request"));
        assert_eq!(controller.shared_open_token(), expected);
        assert_eq!(
            controller.shared_snapshot().io.id,
            before_io,
            "the picker result never queues file work"
        );
        controller.lifecycle().cancel_close().unwrap();
        assert!(
            queue_game_file(
                &mut controller,
                "open",
                PathBuf::from("unread.mshogi"),
                expected
            )
            .is_ok()
        );
    }

    #[test]
    fn shogi_product_close_uses_current_document_identity_and_stops_execution() {
        use mshogi_desktop_session::{GameKind, NativeSide, ShogiSide};
        let controller = controller();
        let shutdown = ShutdownState::default();
        let initial = controller.lock().unwrap().shared_open_token();
        let receipt = with_controller(&controller, &shutdown, |c| {
            c.shared_new_game(
                c.shared_open_token(),
                GameKind::Shogi,
                NativeSide::Shogi(ShogiSide::Sente),
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
        assert!(
            native.chess.is_none(),
            "no Chess projection or dormant game for Shogi"
        );
        assert!(native.document.dirty);
        assert!(legacy_snapshot(&mut controller.lock().unwrap()).is_err());
        let paused = with_controller(&controller, &shutdown, |c| {
            c.lifecycle().request_close().unwrap();
            c.application_pause_for_close()
        })
        .unwrap();
        assert_eq!(paused.game, GameKind::Shogi);
        assert!(paused.paused);
        assert!(
            with_controller(&controller, &shutdown, |c| approve_close(c, initial))
                .unwrap()
                .is_err()
        );
        with_controller(&controller, &shutdown, |c| {
            approve_close(c, paused.open_token)
        })
        .unwrap()
        .unwrap();
        assert!(controller.lock().unwrap().lifecycle().close_approved());
    }

    #[cfg(feature = "desktop-verification")]
    #[test]
    fn synthetic_mutations_share_native_close_ownership_without_a_product_game_alias() {
        let controller = controller();
        let shutdown = ShutdownState::default();
        let verification = VerificationState::new("synthetic".into());
        assert!(verification.require_chess().is_err());
        let initial =
            with_synthetic(&controller, &shutdown, &verification, |s| s.snapshot()).unwrap();
        let lifecycle = controller.lock().unwrap().lifecycle();
        lifecycle.request_close().unwrap();
        assert!(
            with_synthetic(&controller, &shutdown, &verification, |_| panic!(
                "close entered fixture"
            ))
            .is_err()
        );
        lifecycle.cancel_close().unwrap();
        let reset = with_synthetic(&controller, &shutdown, &verification, |s| {
            s.command(SyntheticCommand::Reset {
                version: initial.version,
            })
        })
        .unwrap()
        .unwrap();
        assert!(reset.version > initial.version);
        let snapshot =
            with_controller(&controller, &shutdown, DesktopController::shared_snapshot).unwrap();
        lifecycle.request_close().unwrap();
        with_controller(&controller, &shutdown, |c| {
            approve_close(c, snapshot.open_token)
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
}
