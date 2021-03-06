use crate::common::util::*;
#[cfg(target_os = "linux")]
use rust_users::get_effective_uid;

extern crate chown;

#[cfg(test)]
mod test_passgrp {
    use super::chown::entries::{gid2grp, grp2gid, uid2usr, usr2uid};

    #[test]
    fn test_usr2uid() {
        assert_eq!(0, usr2uid("root").unwrap());
        assert!(usr2uid("88888888").is_err());
        assert!(usr2uid("auserthatdoesntexist").is_err());
    }

    #[test]
    fn test_grp2gid() {
        if cfg!(target_os = "linux") || cfg!(target_os = "android") || cfg!(target_os = "windows") {
            assert_eq!(0, grp2gid("root").unwrap())
        } else {
            assert_eq!(0, grp2gid("wheel").unwrap());
        }
        assert!(grp2gid("88888888").is_err());
        assert!(grp2gid("agroupthatdoesntexist").is_err());
    }

    #[test]
    fn test_uid2usr() {
        assert_eq!("root", uid2usr(0).unwrap());
        assert!(uid2usr(88888888).is_err());
    }

    #[test]
    fn test_gid2grp() {
        if cfg!(target_os = "linux") || cfg!(target_os = "android") || cfg!(target_os = "windows") {
            assert_eq!("root", gid2grp(0).unwrap());
        } else {
            assert_eq!("wheel", gid2grp(0).unwrap());
        }
        assert!(gid2grp(88888888).is_err());
    }
}

#[test]
fn test_invalid_option() {
    new_ucmd!().arg("-w").arg("-q").arg("/").fails();
}

#[test]
fn test_chown_myself() {
    // test chown username file.txt
    let scene = TestScenario::new(util_name!());
    let result = scene.cmd("whoami").run();
    if is_ci() && result.stderr_str().contains("No such user/group") {
        // In the CI, some server are failing to return whoami.
        // As seems to be a configuration issue, ignoring it
        return;
    }
    println!("results {}", result.stdout_str());
    let username = result.stdout_str().trim_end();

    let (at, mut ucmd) = at_and_ucmd!();
    let file1 = "test_install_target_dir_file_a1";

    at.touch(file1);
    let result = ucmd.arg(username).arg(file1).run();
    println!("results stdout {}", result.stdout_str());
    println!("results stderr {}", result.stderr_str());
    if is_ci() && result.stderr_str().contains("invalid user") {
        // In the CI, some server are failing to return id.
        // As seems to be a configuration issue, ignoring it
        return;
    }
    assert!(result.success);
}

#[test]
fn test_chown_myself_second() {
    // test chown username: file.txt
    let scene = TestScenario::new(util_name!());
    let result = scene.cmd("whoami").run();
    if is_ci() && result.stderr_str().contains("No such user/group") {
        // In the CI, some server are failing to return whoami.
        // As seems to be a configuration issue, ignoring it
        return;
    }
    println!("results {}", result.stdout_str());

    let (at, mut ucmd) = at_and_ucmd!();
    let file1 = "test_install_target_dir_file_a1";

    at.touch(file1);
    let result = ucmd
        .arg(result.stdout_str().trim_end().to_owned() + ":")
        .arg(file1)
        .run();

    println!("result.stdout = {}", result.stdout_str());
    println!("result.stderr = {}", result.stderr_str());
    assert!(result.success);
}

#[test]
fn test_chown_myself_group() {
    // test chown username:group file.txt
    let scene = TestScenario::new(util_name!());
    let result = scene.cmd("whoami").run();
    if is_ci() && result.stderr_str().contains("No such user/group") {
        // In the CI, some server are failing to return whoami.
        // As seems to be a configuration issue, ignoring it
        return;
    }
    println!("user name = {}", result.stdout_str());
    let username = result.stdout_str().trim_end();

    let result = scene.cmd("id").arg("-gn").run();
    if is_ci() && result.stderr_str().contains("No such user/group") {
        // In the CI, some server are failing to return whoami.
        // As seems to be a configuration issue, ignoring it
        return;
    }
    println!("group name = {}", result.stdout_str());
    let group = result.stdout_str().trim_end();

    let (at, mut ucmd) = at_and_ucmd!();
    let file1 = "test_install_target_dir_file_a1";
    let perm = username.to_owned() + ":" + group;
    at.touch(file1);
    let result = ucmd.arg(perm).arg(file1).run();
    println!("result.stdout = {}", result.stdout_str());
    println!("result.stderr = {}", result.stderr_str());
    if is_ci() && result.stderr_str().contains("chown: invalid group:") {
        // With some Ubuntu into the CI, we can get this answer
        return;
    }
    assert!(result.success);
}

#[test]
fn test_chown_only_group() {
    // test chown :group file.txt
    let scene = TestScenario::new(util_name!());
    let result = scene.cmd("whoami").run();
    if is_ci() && result.stderr_str().contains("No such user/group") {
        // In the CI, some server are failing to return whoami.
        // As seems to be a configuration issue, ignoring it
        return;
    }
    println!("results {}", result.stdout_str());

    let (at, mut ucmd) = at_and_ucmd!();
    let file1 = "test_install_target_dir_file_a1";
    let perm = ":".to_owned() + result.stdout_str().trim_end();
    at.touch(file1);
    let result = ucmd.arg(perm).arg(file1).run();

    println!("result.stdout = {}", result.stdout_str());
    println!("result.stderr = {}", result.stderr_str());

    if is_ci() && result.stderr_str().contains("Operation not permitted") {
        // With ubuntu with old Rust in the CI, we can get an error
        return;
    }
    if is_ci() && result.stderr_str().contains("chown: invalid group:") {
        // With mac into the CI, we can get this answer
        return;
    }
    assert!(result.success);
}

#[test]
fn test_chown_only_id() {
    // test chown 1111 file.txt
    let result = TestScenario::new("id").ucmd_keepenv().arg("-u").run();
    if is_ci() && result.stderr_str().contains("No such user/group") {
        // In the CI, some server are failing to return whoami.
        // As seems to be a configuration issue, ignoring it
        return;
    }
    println!("result.stdout = {}", result.stdout_str());
    println!("result.stderr = {}", result.stderr_str());
    let id = String::from(result.stdout_str().trim());

    let (at, mut ucmd) = at_and_ucmd!();
    let file1 = "test_install_target_dir_file_a1";

    at.touch(file1);
    let result = ucmd.arg(id).arg(file1).run();

    println!("result.stdout = {}", result.stdout_str());
    println!("result.stderr = {}", result.stderr_str());
    if is_ci() && result.stderr_str().contains("chown: invalid user:") {
        // With some Ubuntu into the CI, we can get this answer
        return;
    }
    assert!(result.success);
}

#[test]
fn test_chown_only_group_id() {
    // test chown :1111 file.txt
    let result = TestScenario::new("id").ucmd_keepenv().arg("-g").run();
    if is_ci() && result.stderr_str().contains("No such user/group") {
        // In the CI, some server are failing to return whoami.
        // As seems to be a configuration issue, ignoring it
        return;
    }
    println!("result.stdout = {}", result.stdout_str());
    println!("result.stderr = {}", result.stderr_str());
    let id = String::from(result.stdout_str().trim());

    let (at, mut ucmd) = at_and_ucmd!();
    let file1 = "test_install_target_dir_file_a1";

    at.touch(file1);
    let perm = ":".to_owned() + &id;

    let result = ucmd.arg(perm).arg(file1).run();

    println!("result.stdout = {}", result.stdout_str());
    println!("result.stderr = {}", result.stderr_str());
    if is_ci() && result.stderr_str().contains("chown: invalid group:") {
        // With mac into the CI, we can get this answer
        return;
    }
    assert!(result.success);
}

#[test]
fn test_chown_both_id() {
    // test chown 1111:1111 file.txt
    let result = TestScenario::new("id").ucmd_keepenv().arg("-u").run();
    if is_ci() && result.stderr_str().contains("No such user/group") {
        // In the CI, some server are failing to return whoami.
        // As seems to be a configuration issue, ignoring it
        return;
    }
    println!("result.stdout = {}", result.stdout_str());
    println!("result.stderr = {}", result.stderr_str());
    let id_user = String::from(result.stdout_str().trim());

    let result = TestScenario::new("id").ucmd_keepenv().arg("-g").run();
    if is_ci() && result.stderr_str().contains("No such user/group") {
        // In the CI, some server are failing to return whoami.
        // As seems to be a configuration issue, ignoring it
        return;
    }
    println!("result.stdout = {}", result.stdout_str());
    println!("result.stderr = {}", result.stderr_str());
    let id_group = String::from(result.stdout_str().trim());

    let (at, mut ucmd) = at_and_ucmd!();
    let file1 = "test_install_target_dir_file_a1";

    at.touch(file1);
    let perm = id_user + &":".to_owned() + &id_group;

    let result = ucmd.arg(perm).arg(file1).run();
    println!("result.stdout = {}", result.stdout_str());
    println!("result.stderr = {}", result.stderr_str());

    if is_ci() && result.stderr_str().contains("invalid user") {
        // In the CI, some server are failing to return id.
        // As seems to be a configuration issue, ignoring it
        return;
    }

    assert!(result.success);
}

#[test]
fn test_chown_both_mix() {
    // test chown 1111:1111 file.txt
    let result = TestScenario::new("id").ucmd_keepenv().arg("-u").run();
    if is_ci() && result.stderr_str().contains("No such user/group") {
        // In the CI, some server are failing to return whoami.
        // As seems to be a configuration issue, ignoring it
        return;
    }
    println!("result.stdout = {}", result.stdout_str());
    println!("result.stderr = {}", result.stderr_str());
    let id_user = String::from(result.stdout_str().trim());

    let result = TestScenario::new("id").ucmd_keepenv().arg("-gn").run();
    if is_ci() && result.stderr_str().contains("No such user/group") {
        // In the CI, some server are failing to return whoami.
        // As seems to be a configuration issue, ignoring it
        return;
    }
    println!("result.stdout = {}", result.stdout_str());
    println!("result.stderr = {}", result.stderr_str());
    let group_name = String::from(result.stdout_str().trim());

    let (at, mut ucmd) = at_and_ucmd!();
    let file1 = "test_install_target_dir_file_a1";

    at.touch(file1);
    let perm = id_user + &":".to_owned() + &group_name;

    let result = ucmd.arg(perm).arg(file1).run();

    if is_ci() && result.stderr_str().contains("invalid user") {
        // In the CI, some server are failing to return id.
        // As seems to be a configuration issue, ignoring it
        return;
    }
    assert!(result.success);
}

#[test]
fn test_chown_recursive() {
    let scene = TestScenario::new(util_name!());
    let result = scene.cmd("whoami").run();
    if is_ci() && result.stderr_str().contains("No such user/group") {
        // In the CI, some server are failing to return whoami.
        // As seems to be a configuration issue, ignoring it
        return;
    }
    println!("result.stdout = {}", result.stdout_str());
    println!("result.stderr = {}", result.stderr_str());
    let username = result.stdout_str().trim_end();

    let (at, mut ucmd) = at_and_ucmd!();
    at.mkdir("a");
    at.mkdir("a/b");
    at.mkdir("a/b/c");
    at.mkdir("z");
    at.touch(&at.plus_as_string("a/a"));
    at.touch(&at.plus_as_string("a/b/b"));
    at.touch(&at.plus_as_string("a/b/c/c"));
    at.touch(&at.plus_as_string("z/y"));

    let result = ucmd
        .arg("-R")
        .arg("--verbose")
        .arg(username)
        .arg("a")
        .arg("z")
        .run();
    println!("result.stdout = {}", result.stdout_str());
    println!("result.stderr = {}", result.stderr_str());
    if is_ci() && result.stderr_str().contains("invalid user") {
        // In the CI, some server are failing to return id.
        // As seems to be a configuration issue, ignoring it
        return;
    }

    result
        .stderr_contains(&"ownership of 'a/a' retained as")
        .stderr_contains(&"ownership of 'z/y' retained as")
        .success();
}

#[test]
fn test_root_preserve() {
    let scene = TestScenario::new(util_name!());
    let result = scene.cmd("whoami").run();
    if is_ci() && result.stderr_str().contains("No such user/group") {
        // In the CI, some server are failing to return whoami.
        // As seems to be a configuration issue, ignoring it
        return;
    }
    println!("result.stdout = {}", result.stdout_str());
    println!("result.stderr = {}", result.stderr_str());
    let username = result.stdout_str().trim_end();

    let result = new_ucmd!()
        .arg("--preserve-root")
        .arg("-R")
        .arg(username)
        .arg("/")
        .fails();
    println!("result.stdout = {}", result.stdout_str());
    println!("result.stderr = {}", result.stderr_str());
    if is_ci() && result.stderr_str().contains("invalid user") {
        // In the CI, some server are failing to return id.
        // As seems to be a configuration issue, ignoring it
        return;
    }
    assert!(result
        .stderr
        .contains("chown: it is dangerous to operate recursively"));
}

#[cfg(target_os = "linux")]
#[test]
fn test_big_p() {
    if get_effective_uid() != 0 {
        new_ucmd!()
            .arg("-RP")
            .arg("bin")
            .arg("/proc/self/cwd")
            .fails()
            .stderr_is(
                "chown: changing ownership of '/proc/self/cwd': Operation not permitted (os error 1)\n",
            );
    }
}
