function check_pass(proc) {
    if stdout(proc) != "pass" {
        print("fail: "+stdout(proc));
    }
}

$(path_of_script()+"/sub/print_arg.sl") 2 "pass" $> capture
check_pass(capture)

let pa = path_of_script()+"/sub/print_arg.sl"
$pa 4 fail fail     pass $> capture
check_pass(capture)

$pa 7 f1 f2 f3 f4 \
      f5 pass \
      $> capture
check_pass(capture)

print("pass")