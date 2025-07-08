use winres::WindowsResource;

fn main() {
    if cfg!(target_os = "windows") {
        let mut res = WindowsResource::new();
        res.set_icon("wix/grp.ico");
        res.compile().unwrap();
    }
}