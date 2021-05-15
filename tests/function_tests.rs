mod common;

#[test]
fn test_fun() {
    common::run(r##"
    function f(x) {
        print("pas"+x)
    }
    f("s")
    "##,"pass");
}
