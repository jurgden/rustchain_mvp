mod ui;

fn main() {
    if let Err(err) = ui::run_tui() {
        eprintln!("Error: {:?}", err);
    }
}
