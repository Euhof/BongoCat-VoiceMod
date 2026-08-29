use serde::Serialize;
use serde_json::{json, Value};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use tauri::{AppHandle, Emitter, Runtime, command};

#[cfg(target_os = "linux")]
use std::time::Duration;

#[derive(Debug, Clone, Serialize)]
pub enum DeviceEventKind {
    MousePress,
    MouseRelease,
    MouseMove,
    KeyboardPress,
    KeyboardRelease,
}

#[derive(Debug, Clone, Serialize)]
pub struct DeviceEvent {
    kind: DeviceEventKind,
    value: Value,
}

static IS_LISTENING: AtomicBool = AtomicBool::new(false);

#[cfg(target_os = "linux")]
fn is_wayland_session() -> bool {
    std::env::var_os("WAYLAND_DISPLAY").is_some()
        || std::env::var("XDG_SESSION_TYPE")
            .map(|value| value.eq_ignore_ascii_case("wayland"))
            .unwrap_or(false)
}

// rdev is the portable backend. Its Key names already match the frontend
// convention for most keys, so only the keys whose model names differ are
// normalized here.
fn rdev_key_to_frontend(key: rdev::Key) -> String {
    use rdev::Key;

    match key {
        Key::F1
        | Key::F2
        | Key::F3
        | Key::F4
        | Key::F5
        | Key::F6
        | Key::F7
        | Key::F8
        | Key::F9
        | Key::F10
        | Key::F11
        | Key::F12
        | Key::Function => "Fn".into(),
        Key::MetaLeft | Key::MetaRight => "Meta".into(),
        Key::BackSlash | Key::IntlBackslash => "Slash".into(),
        other => format!("{:?}", other),
    }
}

#[cfg(target_os = "linux")]
fn evdev_key_to_frontend(key: evdev::Key) -> String {
    use evdev::Key;

    match key {
        Key::KEY_A => "KeyA".into(),
        Key::KEY_B => "KeyB".into(),
        Key::KEY_C => "KeyC".into(),
        Key::KEY_D => "KeyD".into(),
        Key::KEY_E => "KeyE".into(),
        Key::KEY_F => "KeyF".into(),
        Key::KEY_G => "KeyG".into(),
        Key::KEY_H => "KeyH".into(),
        Key::KEY_I => "KeyI".into(),
        Key::KEY_J => "KeyJ".into(),
        Key::KEY_K => "KeyK".into(),
        Key::KEY_L => "KeyL".into(),
        Key::KEY_M => "KeyM".into(),
        Key::KEY_N => "KeyN".into(),
        Key::KEY_O => "KeyO".into(),
        Key::KEY_P => "KeyP".into(),
        Key::KEY_Q => "KeyQ".into(),
        Key::KEY_R => "KeyR".into(),
        Key::KEY_S => "KeyS".into(),
        Key::KEY_T => "KeyT".into(),
        Key::KEY_U => "KeyU".into(),
        Key::KEY_V => "KeyV".into(),
        Key::KEY_W => "KeyW".into(),
        Key::KEY_X => "KeyX".into(),
        Key::KEY_Y => "KeyY".into(),
        Key::KEY_Z => "KeyZ".into(),
        Key::KEY_1 => "Num1".into(),
        Key::KEY_2 => "Num2".into(),
        Key::KEY_3 => "Num3".into(),
        Key::KEY_4 => "Num4".into(),
        Key::KEY_5 => "Num5".into(),
        Key::KEY_6 => "Num6".into(),
        Key::KEY_7 => "Num7".into(),
        Key::KEY_8 => "Num8".into(),
        Key::KEY_9 => "Num9".into(),
        Key::KEY_0 => "Num0".into(),
        Key::KEY_LEFTCTRL => "ControlLeft".into(),
        Key::KEY_RIGHTCTRL => "ControlRight".into(),
        Key::KEY_LEFTSHIFT => "ShiftLeft".into(),
        Key::KEY_RIGHTSHIFT => "ShiftRight".into(),
        Key::KEY_LEFTALT => "Alt".into(),
        Key::KEY_RIGHTALT => "AltGr".into(),
        Key::KEY_LEFTMETA | Key::KEY_RIGHTMETA => "Meta".into(),
        Key::KEY_SPACE => "Space".into(),
        Key::KEY_ENTER => "Return".into(),
        Key::KEY_ESC => "Escape".into(),
        Key::KEY_TAB => "Tab".into(),
        Key::KEY_BACKSPACE => "Backspace".into(),
        Key::KEY_CAPSLOCK => "CapsLock".into(),
        Key::KEY_DELETE => "Delete".into(),
        Key::KEY_UP => "UpArrow".into(),
        Key::KEY_DOWN => "DownArrow".into(),
        Key::KEY_LEFT => "LeftArrow".into(),
        Key::KEY_RIGHT => "RightArrow".into(),
        Key::KEY_GRAVE => "BackQuote".into(),
        Key::KEY_SLASH | Key::KEY_BACKSLASH | Key::KEY_102ND => "Slash".into(),
        Key::KEY_F1
        | Key::KEY_F2
        | Key::KEY_F3
        | Key::KEY_F4
        | Key::KEY_F5
        | Key::KEY_F6
        | Key::KEY_F7
        | Key::KEY_F8
        | Key::KEY_F9
        | Key::KEY_F10
        | Key::KEY_F11
        | Key::KEY_F12 => "Fn".into(),
        Key::BTN_LEFT => "Left".into(),
        Key::BTN_RIGHT => "Right".into(),
        Key::BTN_MIDDLE => "Middle".into(),
        other => {
            let value = format!("{:?}", other);
            value.strip_prefix("KEY_").unwrap_or(&value).to_string()
        }
    }
}

#[cfg(target_os = "linux")]
fn is_evdev_mouse_button(key: evdev::Key) -> bool {
    use evdev::Key;

    matches!(
        key,
        Key::BTN_LEFT
            | Key::BTN_RIGHT
            | Key::BTN_MIDDLE
            | Key::BTN_SIDE
            | Key::BTN_EXTRA
            | Key::BTN_FORWARD
            | Key::BTN_BACK
    )
}

fn start_rdev_listener<R: Runtime>(app_handle: AppHandle<R>) -> Result<(), String> {
    use rdev::{listen, Event, EventType};

    let callback = move |event: Event| {
        let device_event = match event.event_type {
            EventType::ButtonPress(button) => DeviceEvent {
                kind: DeviceEventKind::MousePress,
                value: json!(format!("{:?}", button)),
            },
            EventType::ButtonRelease(button) => DeviceEvent {
                kind: DeviceEventKind::MouseRelease,
                value: json!(format!("{:?}", button)),
            },
            EventType::MouseMove { x, y } => DeviceEvent {
                kind: DeviceEventKind::MouseMove,
                value: json!({ "x": x, "y": y }),
            },
            EventType::KeyPress(key) => DeviceEvent {
                kind: DeviceEventKind::KeyboardPress,
                value: json!(rdev_key_to_frontend(key)),
            },
            EventType::KeyRelease(key) => DeviceEvent {
                kind: DeviceEventKind::KeyboardRelease,
                value: json!(rdev_key_to_frontend(key)),
            },
            _ => return,
        };

        let _ = app_handle.emit("device-changed", device_event);
    };

    listen(callback).map_err(|error| format!("Failed to listen device: {error:?}"))
}

#[cfg(target_os = "linux")]
fn start_wayland_listener<R: Runtime>(app_handle: AppHandle<R>) -> Result<(), String> {
    use evdev::InputEventKind;

    for (path, device) in evdev::enumerate() {
        let has_keys = device
            .supported_keys()
            .map(|keys| keys.iter().next().is_some())
            .unwrap_or(false);

        if !has_keys {
            continue;
        }

        let name = device.name().unwrap_or("unknown").to_string();
        println!("[device] Opening keyboard/mouse: {} ({:?})", name, path);

        let app = app_handle.clone();

        thread::spawn(move || {
            let mut device = device;

            loop {
                if !IS_LISTENING.load(Ordering::SeqCst) {
                    break;
                }

                match device.fetch_events() {
                    Ok(events) => {
                        for event in events {
                            if let InputEventKind::Key(key) = event.kind() {
                                let value = event.value();
                                let pressed = value == 1 || value == 2;
                                let released = value == 0;
                                let frontend_key = evdev_key_to_frontend(key);

                                let kind = if is_evdev_mouse_button(key) {
                                    if pressed {
                                        Some(DeviceEventKind::MousePress)
                                    } else if released {
                                        Some(DeviceEventKind::MouseRelease)
                                    } else {
                                        None
                                    }
                                } else if pressed {
                                    Some(DeviceEventKind::KeyboardPress)
                                } else if released {
                                    Some(DeviceEventKind::KeyboardRelease)
                                } else {
                                    None
                                };

                                if let Some(kind) = kind {
                                    let _ = app.emit(
                                        "device-changed",
                                        DeviceEvent {
                                            kind,
                                            value: json!(frontend_key),
                                        },
                                    );
                                }
                            }
                        }
                    }
                    Err(error) => {
                        eprintln!("[device/evdev] Error in {}: {error}", name);
                        thread::sleep(Duration::from_millis(100));
                    }
                }
            }
        });
    }

    // evdev reports relative movement. Hyprland gives us the absolute cursor
    // position, preserving the same MouseMove payload used by rdev.
    let app_mouse = app_handle.clone();
    thread::spawn(move || {
        let mut last_x = -1.0;
        let mut last_y = -1.0;

        loop {
            if !IS_LISTENING.load(Ordering::SeqCst) {
                break;
            }

            if let Ok(output) = std::process::Command::new("hyprctl")
                .args(["cursorpos"])
                .output()
            {
                if output.status.success() {
                    let position = String::from_utf8_lossy(&output.stdout);
                    let position = position.trim();

                    if let Some((x_str, y_str)) = position.split_once(',') {
                        if let (Ok(x), Ok(y)) = (
                            x_str.trim().parse::<f64>(),
                            y_str.trim().parse::<f64>(),
                        ) {
                            if (x - last_x).abs() > 0.5 || (y - last_y).abs() > 0.5 {
                                last_x = x;
                                last_y = y;

                                let _ = app_mouse.emit(
                                    "device-changed",
                                    DeviceEvent {
                                        kind: DeviceEventKind::MouseMove,
                                        value: json!({ "x": x, "y": y }),
                                    },
                                );
                            }
                        }
                    }
                }
            }

            thread::sleep(Duration::from_millis(16));
        }
    });

    Ok(())
}

#[command]
pub async fn start_device_listening<R: Runtime>(app_handle: AppHandle<R>) -> Result<(), String> {
    if IS_LISTENING.load(Ordering::SeqCst) {
        return Ok(());
    }

    IS_LISTENING.store(true, Ordering::SeqCst);

    #[cfg(target_os = "linux")]
    {
        if is_wayland_session() {
            println!("[device] Wayland session detected; using evdev backend");
            return start_wayland_listener(app_handle);
        }

        println!("[device] Non-Wayland Linux session detected; using rdev backend");
        return start_rdev_listener(app_handle);
    }

    #[cfg(not(target_os = "linux"))]
    {
        println!("[device] Using rdev backend");
        start_rdev_listener(app_handle)
    }
}