mod common;

#[test]
fn test_for() {
    common::run(r##"
    let s = ""
    for p in ["1","2","3"] {
        s = s + p
    }
    print(s)
    "##,"123");
    common::run("for p in [1] { print(p) };","1");
    common::run("{ print(1) }","1");
    common::run(r##"
    let s = ""
    for i=0;i<2;i=i+1 {
        s = s + to_str(i)
    }
    print(s)
    "##,"01");
    common::run(r##"
    let s = ""
    for i=0;i<6;i=i+1 {
        if i == 2 { continue }
        if i == 5 { break }

        s = s + to_str(i)
    }
    print(s)
    "##,"0134");

}


#[test]
fn test_if() {
    common::run(r##"if 1 { print("pass") }"##,"pass");
    common::run(r##"if 0 { print("fail") }"##,"");
    common::run(r##"
    if 1 {
      print("pass")
    } else if 1 {
      print("fail")
    } else {
      print("fail2")
    }"##,"pass");
    common::run(r##"
    if 0 {
      print("fail")
    } else if 1 {
      print("pass")
    } else {
      print("fail2")
    }"##,"pass");
    common::run(r##"
    if 0 {
      print("fail")
    } else if 0 {
      print("fail2")
    } else {
      print("pass")
    }"##,"pass");

}
