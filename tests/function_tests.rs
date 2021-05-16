mod common;

#[test]
fn test_fun() {
    common::run(r##"
    function f(x) {
        print("pas"+x)
    }
    f("s")
    "##,"pass");

    common::run(r##"
    function f(x) {
        return "pas"+x
    }
    println("1")
    println(f("s"))
    "##,"1\npass\n");
}
