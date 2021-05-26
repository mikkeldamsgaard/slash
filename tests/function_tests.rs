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

    common::run(r##"

    let x = "pass"

    if lookup_env_var("x") != "pass" {
        print("fail")
    }

    export x

    if lookup_env_var("x") != "pass" {
        print("fail")
    }

    print("pass")
    "##,"pass");

    common::run(r##"
    function add(x,y)
    {
        return x+y
    }

    print(add(1,2)) # 3
    "##,"3");
}

#[test]
fn test_builtin() {
    common::run(r##"
    print()
    "##,"");

    common::run(r##"
    print(1,2,[1]) # prints "1 2 [1]" to stdout
    "##,"1 2 [1]");

    common::run(r##"
    if is_number(2) { print("pass") }
    if is_number("1") { print("fail") }
    "##,"pass");

    common::run(r##"
    if is_string("2") { print("pass") }
    if is_string(1) { print("fail") }
    "##,"pass");

    common::run(r##"
    if is_list([]) { print("pass") }
    if is_list("1") { print("fail") }
    "##,"pass");

    common::run(r##"
    if is_table({}) { print("pass") }
    if is_table("1") { print("fail") }
    "##,"pass");

    common::run(r##"
    if is_function(|| {}) { print("pass") }
    if is_function("1") { print("fail") }
    "##,"pass");

    common::run(r##"
    echo $> pr
    if is_process_result(pr) { print("pass") }
    if is_process_result("1") { print("fail") }
    "##,"pass");
}