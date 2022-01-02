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
fn test_while() {
    common::run(r##"
    let p = ["p","a","s","s"]
    let s = ""
    let i = 0
    while i<4 {
        s = s + p[i]
        i = i + 1
    }
    print(s)
    "##,"pass");
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

    common::run(r##"
    if (
    1+


    2 ==

    3) {
      print("pass")
    } "##,"pass");

}

#[test]
fn test_match() {
    common::run(r##"
    match 1 {
      1 => {
      print("pass")
      }
    }"##,"pass");

    common::run(r##"
    match 1 {
      1 -> 3 => {
      print("pass")
      }
    }"##,"pass");
    common::run(r##"
    match 2 {
      1 -> 5 => {
      print("pass")
      }
    }"##,"pass");
    common::run(r##"
    match 5 {
      1 -> 5 => {
      print("pass")
      }
    }"##,"pass");
    common::run(r##"
    match 7 {
      1 -> 5 => {
      print("fail")
      }
      _ => { print("pass") }
    }"##,"pass");
    common::run(r##"
    match 7 {
      1 -> 5; 7 => {
      print("pass")
      }
    }"##,"pass");
    common::run(r##"
    match -7 {
      -20 -> -5; 7 => {
      print("pass")
      }
    }"##,"pass");

}

#[test]
fn test_index_assignment() {
    common::run(r##"
    let l = [1,2]
    l[0] = "pass"
    print(l[0])
    "##,"pass");

    common::run(r##"
    let t = {"a":"fail", "b":"fail"}
    t["c"] = "pass"
    print(t["c"])
    "##,"pass");

    common::run(r##"
    let t = {"a":"fail", "b":"fail"}
    t.c = "pass"
    print(t.c)
    "##,"pass");

}

#[test]
fn test_white_space() {
    common::run(r##"
    let
    j
    =
    "pass"

    print
    (
    j
    )
    "##,"pass");

}