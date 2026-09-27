use docpack::progress::create_progress;

#[test]
fn creates_progress_bar() {
    let bar = create_progress(10, "testing");

    bar.inc(1);
    bar.finish();

    assert!(true);
}
