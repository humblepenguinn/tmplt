#[macro_export]
macro_rules! error {
    ($msg:expr) => {{
        eprint!("\x1b[31mError: \x1b[0m{}\n", $msg);
    }};
}
