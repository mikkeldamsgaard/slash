mod common;

#[test]
fn test_123() {
    common::run("let j=1;print(j)","1");
    common::run("print(42*7/6+1)","50");
    common::run("print(1,2)","1 2");
    common::run(r##"print("str " ) "##,"str ");
    common::run(r##"print([1,  2, 3]  ) "##,"[1, 2, 3]");
    //common::run(r##"1<2"##,"[1, 2, 3]");
}

#[test]
fn test_arg_list() {
    common::run("print(len([1,2]))","2");
}

#[test]
fn test_index_assignment() {
    common::run(r##"
    let j = ["fail"]
    j[0] = "pass"
    print(j)
    "##, r##"["pass"]"##);

    common::run(r##"
    let j = {}
    j["pass"] = 1
    print(j)
    "##, r##"{"pass": 1}"##);

}


#[test]
fn test_index_eval() {
    common::run(r##"
    let j = [1,2]
    print(j[0]+41)
    "##, "42");

    common::run(r##"
    function f() { return ["pass"] }
    print(f()[0])
    "##, "pass");


}

#[test]
fn test_table_literal() {
    common::run(r##"
    let f1 = "f1"
    let j = { f1 : "pass"}
    print(j["f1"])
    "##, "pass");

    common::run(r##"
    let j = { "f1" : "pass"}
    print(j["f1"])
    "##, "pass");

    common::run(r##"
    let j = { "f1" : "pass",
    "f2": "fail"}
    print(j["f1"])
    "##, "pass");

    common::run(r##"
    let j = { "f1" : "pass",
    "f2": "fail"}
    print(j.f1)
    "##, "pass");

    common::run(r##"
    let j = { "a": 41 }
    print(j.a+1)
    "##, "42");


}

#[test]
fn test_list() {
    common::run(r##"
    print(join(["p","a"] + ["ss"], ""))
    "##, "pass");
}

#[test]
fn test_functional() {
    common::run(r##"
    function f() { return "pass" }
    function g() { return f }
    print(g()())
    "##, "pass");

    common::run(r##"
    function f() { return "pass" }
    let t = { "f": f }
    print(t["f"]())
    "##, "pass");

    common::run(r##"
    let f = || { return "pass" }
    let t = { "f": f }
    print(t["f"]())
    "##, "pass");

    common::run(r##"
    let t = { "f": || { return "pass" }  }
    print(t["f"]())
    "##, "pass");

    common::run(r##"
    let t = { "f": |x| { return x }  }
    print(t["f"]("pass"))
    "##, "pass");

    common::run(r##"
    let t = [ |x| { return x } ]
    print(t[0]("pass"))
    "##, "pass");

}