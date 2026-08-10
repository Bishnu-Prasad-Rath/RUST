use std::collections::HashMap;
use std::time::{Duration, Instant};
use windows_sys::Win32::Foundation::{HWND, BOOL};
use windows_sys::Win32::UI::WindowsAndMessaging::{
  EnumWindows, GetForegroundWindow, GetWindowThreadProcessId, IsHungAppWindow, IsWindowVisible,
};
use windows_sys::Win32::System::Threading::{OpenProcess, TerminateProcess, PROCESS_TERMINATE,};
use windows_sys::Win32::Foundation::CloseHandle;

// Needed Struct to track how long a PID has been unresponsive

pub struct FreezeTracker{
  frozen_since : HashMap<u32, Instant>,
}

