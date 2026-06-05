use tauri::{AppHandle, Manager, Position, Size};

pub fn init(app_handle: &tauri::AppHandle) -> tauri::Result<()> {
    let window = app_handle.get_webview_window("main").unwrap();

    // Set skip taskbar so it doesn't clutter the task list
    let _ = window.set_skip_taskbar(true);

    // Close/Hide window when it loses focus (mimics resignation behavior)
    let handle = app_handle.clone();
    window.on_window_event(move |event| {
        if let tauri::WindowEvent::Focused(focused) = event {
            if !*focused {
                log::debug!("Panel lost focus: hiding window");
                if let Some(w) = handle.get_webview_window("main") {
                    let _ = w.hide();
                }
            }
        }
    });

    Ok(())
}

pub fn show_panel(app_handle: &AppHandle, click_coords: Option<(f64, f64)>) {
    if let Some(window) = app_handle.get_webview_window("main") {
        if let Some((cx, cy)) = click_coords {
            position_panel_near_click(app_handle, cx, cy);
        } else {
            position_panel_fallback(app_handle);
        }
        let _ = window.show();
        let _ = window.set_focus();
        if let Some((cx, cy)) = click_coords {
            position_panel_near_click(app_handle, cx, cy);
            spawn_delayed_position_near_click(app_handle.clone(), cx, cy);
        } else {
            position_panel_fallback(app_handle);
            spawn_delayed_position_fallback(app_handle.clone());
        }
    }
}

pub fn hide_panel(app_handle: &AppHandle) {
    if let Some(window) = app_handle.get_webview_window("main") {
        let _ = window.hide();
    }
}

pub fn toggle_panel(app_handle: &AppHandle, click_coords: Option<(f64, f64)>) {
    if let Some(window) = app_handle.get_webview_window("main") {
        let is_visible = window.is_visible().unwrap_or(false);
        if is_visible {
            log::debug!("toggle_panel: hiding window");
            let _ = window.hide();
        } else {
            log::debug!("toggle_panel: showing window");
            if let Some((cx, cy)) = click_coords {
                position_panel_near_click(app_handle, cx, cy);
            } else {
                position_panel_fallback(app_handle);
            }
            let _ = window.show();
            let _ = window.set_focus();
            if let Some((cx, cy)) = click_coords {
                position_panel_near_click(app_handle, cx, cy);
                spawn_delayed_position_near_click(app_handle.clone(), cx, cy);
            } else {
                position_panel_fallback(app_handle);
                spawn_delayed_position_fallback(app_handle.clone());
            }
        }
    }
}

pub fn handle_tray_click(app_handle: &AppHandle, position: Position, size: Size) {
    if let Some(window) = app_handle.get_webview_window("main") {
        let is_visible = window.is_visible().unwrap_or(false);
        if is_visible {
            log::debug!("tray click: hiding panel");
            let _ = window.hide();
        } else {
            log::debug!("tray click: showing panel");
            position_panel_at_tray_icon(app_handle, position.clone(), size.clone());
            let _ = window.show();
            let _ = window.set_focus();
            position_panel_at_tray_icon(app_handle, position.clone(), size.clone());
            spawn_delayed_position_at_tray_icon(app_handle.clone(), position, size);
        }
    }
}

fn spawn_delayed_position_fallback(app_handle: AppHandle) {
    let handle = app_handle.clone();
    std::thread::spawn(move || {
        for delay in [10, 30, 80, 150, 250, 400] {
            std::thread::sleep(std::time::Duration::from_millis(delay));
            let h = handle.clone();
            let _ = handle.run_on_main_thread(move || {
                position_panel_fallback(&h);
            });
        }
    });
}

fn spawn_delayed_position_at_tray_icon(app_handle: AppHandle, position: Position, size: Size) {
    let handle = app_handle.clone();
    std::thread::spawn(move || {
        for delay in [10, 30, 80, 150, 250, 400] {
            std::thread::sleep(std::time::Duration::from_millis(delay));
            let h = handle.clone();
            let pos = position.clone();
            let sz = size.clone();
            let _ = handle.run_on_main_thread(move || {
                position_panel_at_tray_icon(&h, pos, sz);
            });
        }
    });
}

fn spawn_delayed_position_near_click(app_handle: AppHandle, click_x: f64, click_y: f64) {
    let handle = app_handle.clone();
    std::thread::spawn(move || {
        for delay in [10, 30, 80, 150, 250, 400] {
            std::thread::sleep(std::time::Duration::from_millis(delay));
            let h = handle.clone();
            let _ = handle.run_on_main_thread(move || {
                position_panel_near_click(&h, click_x, click_y);
            });
        }
    });
}

pub fn position_panel_at_tray_icon(
    app_handle: &tauri::AppHandle,
    icon_position: Position,
    icon_size: Size,
) {
    let (icon_x, icon_y) = match &icon_position {
        Position::Physical(pos) => (pos.x as f64, pos.y as f64),
        Position::Logical(pos) => (pos.x, pos.y),
    };

    if icon_x == 0.0 && icon_y == 0.0 {
        position_panel_fallback(app_handle);
        return;
    }

    let window = app_handle.get_webview_window("main").unwrap();
    let (icon_w, icon_h) = match &icon_size {
        Size::Physical(s) => (s.width as f64, s.height as f64),
        Size::Logical(s) => (s.width, s.height),
    };

    let scale = window.scale_factor().unwrap_or(1.0);
    let icon_logical_x = icon_x / scale;
    let icon_logical_y = icon_y / scale;
    let icon_logical_w = icon_w / scale;
    let icon_logical_h = icon_h / scale;

    let panel_width = match (window.outer_size(), window.scale_factor()) {
        (Ok(s), Ok(win_scale)) if s.width > 0 => s.width as f64 / win_scale,
        _ => 400.0,
    };

    let target_x = icon_logical_x + (icon_logical_w / 2.0) - (panel_width / 2.0);
    let target_y = icon_logical_y + icon_logical_h + 6.0;

    let _ = window.set_position(tauri::Position::Logical(tauri::LogicalPosition::new(target_x, target_y)));
}

fn position_panel_fallback(app_handle: &tauri::AppHandle) {
    let window = app_handle.get_webview_window("main").unwrap();
    let monitor = window
        .current_monitor()
        .ok()
        .flatten()
        .or_else(|| window.primary_monitor().ok().flatten());

    if let Some(monitor) = monitor {
        let scale = monitor.scale_factor();
        let mon_size = monitor.size();
        let mon_pos = monitor.position();

        let mon_w = mon_size.width as f64 / scale;
        let mon_x = mon_pos.x as f64 / scale;
        let mon_y = mon_pos.y as f64 / scale;

        let panel_width = match (window.outer_size(), window.scale_factor()) {
            (Ok(s), Ok(win_scale)) if s.width > 0 => s.width as f64 / win_scale,
            _ => 400.0,
        };

        let margin_right = 16.0;
        let margin_top = 40.0;

        let target_x = mon_x + mon_w - panel_width - margin_right;
        let target_y = mon_y + margin_top;

        let _ = window.set_position(tauri::Position::Logical(tauri::LogicalPosition::new(target_x, target_y)));
    }
}

pub fn position_panel_near_click(app_handle: &tauri::AppHandle, click_x: f64, click_y: f64) {
    let window = app_handle.get_webview_window("main").unwrap();
    let monitors = window.available_monitors().ok().unwrap_or_default();
    
    // Find the monitor containing (click_x, click_y) in logical coordinates
    let mut target_monitor = None;
    for monitor in monitors {
        let scale = monitor.scale_factor();
        let pos = monitor.position();
        let size = monitor.size();
        
        let mon_x = pos.x as f64 / scale;
        let mon_y = pos.y as f64 / scale;
        let mon_w = size.width as f64 / scale;
        let mon_h = size.height as f64 / scale;
        
        if click_x >= mon_x && click_x <= mon_x + mon_w &&
           click_y >= mon_y && click_y <= mon_y + mon_h {
            target_monitor = Some(monitor);
            break;
        }
    }
    
    let monitor = target_monitor.or_else(|| {
        window.current_monitor().ok().flatten().or_else(|| window.primary_monitor().ok().flatten())
    });
    
    if let Some(monitor) = monitor {
        let scale = monitor.scale_factor();
        let mon_size = monitor.size();
        let mon_pos = monitor.position();

        let mon_x = mon_pos.x as f64 / scale;
        let mon_y = mon_pos.y as f64 / scale;
        let mon_w = mon_size.width as f64 / scale;
        let mon_h = mon_size.height as f64 / scale;

        let panel_width = match (window.outer_size(), window.scale_factor()) {
            (Ok(s), Ok(win_scale)) if s.width > 0 => s.width as f64 / win_scale,
            _ => 400.0,
        };
        let panel_height = match (window.outer_size(), window.scale_factor()) {
            (Ok(s), Ok(win_scale)) if s.height > 0 => s.height as f64 / win_scale,
            _ => 600.0,
        };

        // Determine target coordinates.
        // Horizontal: center the panel on the click, but keep it within monitor bounds
        let mut target_x = click_x - (panel_width / 2.0);
        if target_x < mon_x + 8.0 {
            target_x = mon_x + 8.0;
        } else if target_x + panel_width > mon_x + mon_w - 8.0 {
            target_x = mon_x + mon_w - panel_width - 8.0;
        }
        
        // Vertical: check if click is in top half or bottom half of the monitor
        let target_y = if click_y - mon_y < mon_h / 2.0 {
            // Top half: place panel below the click
            click_y + 12.0
        } else {
            // Bottom half: place panel above the click
            click_y - panel_height - 12.0
        };

        let _ = window.set_position(tauri::Position::Logical(tauri::LogicalPosition::new(target_x, target_y)));
    }
}
