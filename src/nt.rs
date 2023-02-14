#[link(name = "ntoskrnl")]
extern "system" {
    pub fn KeBugCheck(bug_check_code: u32) -> !;
}