#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use mshogi_desktop_session::{
    ApplicationLifecycle, DesktopController, IoPhase, OperationReservation, SharedOpenToken,
    SharedSnapshot,
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
    fn require_product(&self) -> Result<(), String> {
        if self.is_synthetic() {
            Err("Synthetic fixture actions and records are separate from game documents.".into())
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
fn ensure_product_mode(_app: &AppHandle) -> Result<(), String> {
    #[cfg(feature = "desktop-verification")]
    _app.state::<VerificationState>().require_product()?;
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
    ensure_product_mode(&app)?;
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
// All games share native pickers, the bounded archive worker and atomic writer.
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
            return Err("The document changed. Try the operation again.".into());
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
                picker.set_title("Load mShogi game").blocking_pick_file()
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
            // Cancellation is not a successful Save receipt and cannot authorize
            // a pending close, even if a newer snapshot happens to be clean.
            return Err(if action == "open" {
                "Load cancelled.".into()
            } else {
                "Save cancelled.".into()
            });
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
    // The engine/close callback may change authoritative content while the OS
    // picker is open. A path grants no authority to save that newer document.
    // Check every operation again under the same lock as capture/queueing.
    if expected != controller.shared_open_token() {
        return Err("The document changed. Try the operation again.".into());
    }
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

#[tauri::command]
async fn game_new(
    app: AppHandle,
    game: mshogi_desktop_session::GameKind,
    human: mshogi_desktop_session::NativeSide,
    expected: mshogi_desktop_session::SharedOpenToken,
) -> Result<mshogi_desktop_session::SharedSnapshot, String> {
    ensure_product_mode(&app)?;
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
// Native lifecycle owns document reservations; the host owns only event-loop cleanup.
fn app_lifecycle(app: &AppHandle) -> Arc<ApplicationLifecycle> {
    app.state::<Arc<ApplicationLifecycle>>().inner().clone()
}
/// Picker and completion are reserved under the native application owner.
fn reserve_file_operation(app: &AppHandle) -> Result<OperationReservation, String> {
    ensure_product_mode(app)?;
    let shutdown = app.state::<Arc<ShutdownState>>();
    with_controller(
        &app.state::<Mutex<DesktopController>>(),
        &shutdown,
        |controller| controller.lifecycle().reserve_file(),
    )?
}
// The one current native close contract serves every selected game.
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
            #[cfg(feature = "desktop-verification")]
            desktop_verification_mode,
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

    fn file_path(label: &str) -> PathBuf {
        static NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
        std::env::temp_dir().join(format!(
            "msh125-host-{}-{}-{label}.mshogi",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ))
    }

    fn wait_file(controller: &mut DesktopController, id: u64) -> Result<SharedSnapshot, String> {
        let deadline = Instant::now() + Duration::from_secs(20);
        loop {
            let snapshot = controller.shared_snapshot();
            assert_eq!(snapshot.io.id, id, "unexpected superseding file work");
            match snapshot.io.phase {
                IoPhase::Complete => return Ok(snapshot),
                IoPhase::Failed => return Err(snapshot.io.message.unwrap_or_default()),
                _ => {}
            }
            assert!(Instant::now() < deadline, "host file work did not complete");
            thread::sleep(Duration::from_millis(1));
        }
    }

    fn assert_picker_source_authority(controller: &mut DesktopController) {
        let expected = controller.shared_open_token();
        let before = controller.shared_snapshot();
        // Each component independently rejects: same board/ply is not authority
        // to save or replace an obsolete document or content revision.
        for stale in [
            SharedOpenToken {
                document: expected.document + 1,
                ..expected
            },
            SharedOpenToken {
                version: expected.version + 1,
                ..expected
            },
            SharedOpenToken {
                document_revision: expected.document_revision + 1,
                ..expected
            },
        ] {
            for action in ["save", "save_as", "open"] {
                let path = file_path("obsolete-picker");
                let error = queue_game_file(controller, action, path.clone(), stale).unwrap_err();
                assert!(error.contains("document changed"));
                let after = controller.shared_snapshot();
                assert_eq!(after.open_token, expected);
                assert_eq!(after.io.id, before.io.id, "obsolete work was queued");
                assert_eq!(after.document.dirty, before.document.dirty);
                assert_eq!(after.document.path, before.document.path);
                assert!(!path.exists(), "an obsolete picker wrote a document");
            }
        }
    }

    #[test]
    fn picker_save_and_open_recheck_every_source_identity_before_queuing() {
        let mut controller = DesktopController::new(PathBuf::from("unused-test-engine")).unwrap();
        assert_picker_source_authority(&mut controller);
    }

    fn assert_file_completion_authority(controller: &mut DesktopController) {
        let path = file_path("accepted");
        let token = controller.shared_open_token();
        let before = controller.shared_snapshot();
        let invalid_first = file_path("missing-first-parent").join("game.mshogi");
        let id = queue_game_file(controller, "save", invalid_first, token).unwrap();
        assert!(wait_file(controller, id).is_err());
        let failed = controller.shared_snapshot();
        assert_eq!(failed.open_token, token);
        assert_eq!(failed.document.dirty, before.document.dirty);
        assert_eq!(failed.document.path, before.document.path);
        assert!(!path.exists());

        let id = queue_game_file(controller, "save", path.clone(), token).unwrap();
        let saved = wait_file(controller, id).unwrap();
        assert!(!saved.document.dirty);
        assert_eq!(saved.document.path.as_deref(), path.to_str());
        let bytes = std::fs::read(&path).unwrap();
        assert!(!bytes.is_empty());

        // A subsequent Save uses the established path and remains an actual
        // atomic writer operation, including during an unsaved-close workflow.
        controller.lifecycle().request_close().unwrap();
        let reservation = controller.lifecycle().reserve_file().unwrap();
        let token = controller.shared_open_token();
        let id = queue_game_file(controller, "save", path.clone(), token).unwrap();
        let saved = wait_file(controller, id).unwrap();
        assert!(!saved.document.dirty);
        assert_eq!(std::fs::read(&path).unwrap(), bytes);
        assert!(approve_close(controller, saved.open_token).is_err());
        drop(reservation);
        controller.lifecycle().cancel_close().unwrap();

        let invalid = file_path("invalid-load");
        std::fs::write(&invalid, b"not a native desktop document").unwrap();
        let token = controller.shared_open_token();
        let id = queue_game_file(controller, "open", invalid.clone(), token).unwrap();
        assert!(wait_file(controller, id).is_err());
        assert_eq!(controller.shared_open_token(), token);
        assert!(!controller.shared_snapshot().document.dirty);
        assert_eq!(
            controller.shared_snapshot().document.path.as_deref(),
            path.to_str()
        );
        assert_eq!(std::fs::read(&path).unwrap(), bytes);

        // A filesystem failure cannot replace the valid path or bless a newer
        // revision. Atomic writer fault-injection coverage lives in native tests.
        let missing_parent = file_path("missing-parent").join("game.mshogi");
        let id = queue_game_file(controller, "save", missing_parent, token).unwrap();
        assert!(wait_file(controller, id).is_err());
        let failed = controller.shared_snapshot();
        assert_eq!(failed.open_token, token);
        assert!(!failed.document.dirty);
        assert_eq!(failed.document.path.as_deref(), path.to_str());
        assert_eq!(std::fs::read(&path).unwrap(), bytes);
        std::fs::remove_file(path).unwrap();
        std::fs::remove_file(invalid).unwrap();
    }

    #[test]
    fn accepted_file_work_preserves_path_content_and_close_reservations_on_failure() {
        let mut controller = DesktopController::new(PathBuf::from("unused-test-engine")).unwrap();
        assert_file_completion_authority(&mut controller);
    }

    #[cfg(feature = "desktop-verification")]
    #[test]
    fn all_native_games_preserve_picker_source_and_file_completion_authority() {
        for fixture in ["standard", "shogi-standard", "xiangqi-standard"] {
            let mut controller =
                DesktopController::verification(PathBuf::from("unused-test-engine"), fixture)
                    .unwrap();
            assert_picker_source_authority(&mut controller);
            assert_file_completion_authority(&mut controller);

            // Even an identical new standard diagram has fresh native authority.
            // A delayed Save/Load/close from the old document cannot target it.
            let obsolete = controller.shared_open_token();
            let (_, receipt) = controller
                .shared_new_game(
                    obsolete,
                    mshogi_desktop_session::GameKind::Shogi,
                    mshogi_desktop_session::NativeSide::Shogi(
                        mshogi_desktop_session::ShogiSide::Sente,
                    ),
                )
                .unwrap();
            let deadline = Instant::now() + Duration::from_secs(20);
            while receipt.completion().is_none() {
                controller.shared_snapshot();
                assert!(Instant::now() < deadline);
                thread::sleep(Duration::from_millis(1));
            }
            receipt.completion().unwrap().unwrap();
            let current = controller.shared_open_token();
            assert_ne!(current.document, obsolete.document);
            let id = controller.shared_snapshot().io.id;
            for action in ["save", "open"] {
                assert!(
                    queue_game_file(
                        &mut controller,
                        action,
                        file_path("replaced-document"),
                        obsolete,
                    )
                    .is_err()
                );
                assert_eq!(controller.shared_snapshot().io.id, id);
                assert_eq!(controller.shared_open_token(), current);
            }
            controller.lifecycle().request_close().unwrap();
            assert!(approve_close(&mut controller, obsolete).is_err());
            assert!(!controller.lifecycle().close_approved());
            approve_close(&mut controller, current).unwrap();
        }
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
        assert_eq!(native.game, GameKind::Shogi);
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
        assert!(verification.require_product().is_err());
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
        assert!(verification.require_product().is_ok());
        assert!(
            with_synthetic(&controller, &shutdown, &verification, |_| panic!(
                "inactive fixture entered"
            ))
            .is_err()
        );
    }
}
