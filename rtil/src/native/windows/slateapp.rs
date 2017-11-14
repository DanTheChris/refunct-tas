use winapi::minwindef::BOOL;

use native::SLATEAPP;
use super::{
    FSLATEAPPLICATION,
    FSLATEAPPLICATION_TICK,
    FSLATEAPPLICATION_ONKEYDOWN,
    FSLATEAPPLICATION_ONKEYUP,
    FSLATEAPPLICATION_ONRAWMOUSEMOVE,
};

pub struct FSlateApplication;

impl FSlateApplication {
    pub unsafe fn on_key_down(key_code: i32, character_code: u32, is_repeat: bool) {
        // set arguments
        asm!("push ecx" :: "{ecx}"(is_repeat as BOOL) :: "intel","volatile");
        asm!("push ecx" :: "{ecx}"(character_code) :: "intel","volatile");
        asm!("push ecx" :: "{ecx}"(key_code) :: "intel","volatile");
        // call function with thiscall
        asm!("call eax" :: "{ecx}"(FSLATEAPPLICATION), "{eax}"(FSLATEAPPLICATION_ONKEYDOWN as usize) :: "intel","volatile");
    }
    pub unsafe fn on_key_up(key_code: i32, character_code: u32, is_repeat: bool) {
        // set arguments
        asm!("push ecx" :: "{ecx}"(is_repeat as BOOL) :: "intel","volatile");
        asm!("push ecx" :: "{ecx}"(character_code) :: "intel","volatile");
        asm!("push ecx" :: "{ecx}"(key_code) :: "intel","volatile");
        // call function with thiscall
        asm!("call eax" :: "{ecx}"(FSLATEAPPLICATION), "{eax}"(FSLATEAPPLICATION_ONKEYUP as usize) :: "intel","volatile");
    }

    pub unsafe fn on_raw_mouse_move(x: i32, y: i32) {
        // set arguments
        asm!("push ecx" :: "{ecx}"(y) :: "intel","volatile");
        asm!("push ecx" :: "{ecx}"(x) :: "intel","volatile");
        // call function with thiscall
        asm!("call eax" :: "{ecx}"(FSLATEAPPLICATION), "{eax}"(FSLATEAPPLICATION_ONRAWMOUSEMOVE as usize) :: "intel","volatile");
    }
}

hook! {
    "FSlateApplication::Tick",
    FSLATEAPPLICATION_TICK,
    hook_slateapp,
    unhook_slateapp,
    get_slateapp,
    true,
}

hook_fn_once! {
    get_slateapp,
    save_slateapp,
    unhook_slateapp,
    FSLATEAPPLICATION_TICK,
}

#[inline(never)]
extern "thiscall" fn save_slateapp(this: usize) {
    SLATEAPP.set(this + 0x3c);
    unsafe { FSLATEAPPLICATION = this + 0x3c };
    log!("Got FSlateApplication: {:#x}", this);
}