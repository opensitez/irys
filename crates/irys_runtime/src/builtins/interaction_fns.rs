use crate::value::{RuntimeError, Value};
use std::process::Command;

/// Beep() - Makes a beep sound
pub fn beep_fn(args: &[Value]) -> Result<Value, RuntimeError> {
    #[cfg(target_os = "macos")]
    {
        // Use afplay to play system beep
        let _ = Command::new("afplay")
            .arg("/System/Library/Sounds/Ping.aiff")
            .spawn();
    }
    
    #[cfg(target_os = "linux")]
    {
        // Use beep command or fall back to terminal bell
        if Command::new("beep").spawn().is_err() {
            print!("\x07"); // Terminal bell
        }
    }
    
    #[cfg(target_os = "windows")]
    {
        // Use Windows beep API via PowerShell
        let _ = Command::new("powershell")
            .arg("-Command")
            .arg("[console]::beep(800, 200)")
            .spawn();
    }
    
    Ok(Value::Nothing)
}

/// Shell(pathname[, windowstyle]) - Executes an external program
pub fn shell_fn(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.is_empty() || args.len() > 2 {
        return Err(RuntimeError::Custom("Shell requires 1 or 2 arguments".to_string()));
    }
    
    let command = args[0].as_string();
    let _window_style = if args.len() == 2 {
        args[1].as_integer().unwrap_or(1)
    } else {
        1 // vbNormalFocus
    };
    
    // Parse command and arguments
    let parts: Vec<&str> = command.split_whitespace().collect();
    if parts.is_empty() {
        return Err(RuntimeError::Custom("Shell: empty command".to_string()));
    }
    
    let program = parts[0];
    let args = &parts[1..];
    
    match Command::new(program).args(args).spawn() {
        Ok(child) => {
            // Return process ID
            Ok(Value::Integer(child.id() as i32))
        }
        Err(e) => Err(RuntimeError::Custom(format!("Shell error: {}", e))),
    }
}

/// Environ(envstring | number) - Returns environment variable value
pub fn environ_fn(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::Custom("Environ requires 1 argument".to_string()));
    }
    
    match &args[0] {
        Value::String(var_name) => {
            // Get specific environment variable
            match std::env::var(var_name) {
                Ok(value) => Ok(Value::String(value)),
                Err(_) => Ok(Value::String(String::new())),
            }
        }
        Value::Integer(index) => {
            // Get environment variable by index
            let vars: Vec<(String, String)> = std::env::vars().collect();
            if *index >= 1 && (*index as usize) <= vars.len() {
                let (key, value) = &vars[(*index - 1) as usize];
                Ok(Value::String(format!("{}={}", key, value)))
            } else {
                Ok(Value::String(String::new()))
            }
        }
        _ => Err(RuntimeError::Custom("Environ requires string or integer argument".to_string())),
    }
}

/// Command() - Returns command line arguments
pub fn command_fn(args: &[Value]) -> Result<Value, RuntimeError> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    Ok(Value::String(args.join(" ")))
}

/// SendKeys(keys[, wait]) - Sends keystrokes to the active window (stub - platform specific)
pub fn sendkeys_fn(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.is_empty() || args.len() > 2 {
        return Err(RuntimeError::Custom("SendKeys requires 1 or 2 arguments".to_string()));
    }
    
    let _keys = args[0].as_string();
    let _wait = if args.len() == 2 {
        args[1].as_bool().unwrap_or(false)
    } else {
        false
    };
    
    // SendKeys is highly platform-specific and requires accessibility permissions
    // For now, just return Nothing (real implementation would need platform-specific code)
    Ok(Value::Nothing)
}

/// AppActivate(title | pid) - Activates an application window (stub - platform specific)
pub fn appactivate_fn(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.is_empty() || args.len() > 2 {
        return Err(RuntimeError::Custom("AppActivate requires 1 or 2 arguments".to_string()));
    }
    
    // AppActivate is platform-specific and requires window management APIs
    // Real implementation would use platform-specific code
    Ok(Value::Nothing)
}
