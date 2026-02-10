
fn main() {
    #[cfg(windows)]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("wix/grp.ico");
        res.compile().unwrap();
    }
}