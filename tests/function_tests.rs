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

    common::run(r##"
    if cwd() != "" {
        print("pass")
    }
    "##,"pass");


    common::run(r##"
    let p = split("pa ss"," ")
    print(p[0]+p[1])
    "##,"pass");

    common::run(r##"
    if starts_with("ababac", "aba") {
        print("pass")
    }
    "##,"pass");

    common::run(r##"
    let p = join(split("pa ss"," "),"")
    print(p)
    "##,"pass");

}
