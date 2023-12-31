use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;
use assert_fs::assert::PathAssert;
use assert_fs::fixture::PathChild;
use assert_fs::fixture::FileWriteStr;

#[test]
fn file_does_not_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(env!("CARGO_CRATE_NAME"))?;

    cmd.arg("-a").arg("sha256").arg("test/file/doesnt/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("file not found"));

    Ok(())
}

#[test]
fn arg_is_directory() -> Result<(), Box<dyn std::error::Error>> {
    let tmp_dir = assert_fs::TempDir::new()?;

    let mut cmd = Command::cargo_bin(env!("CARGO_CRATE_NAME"))?;
    cmd.arg(tmp_dir.path());
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Is a directory"));

    Ok(())
}

#[test]
fn default_algorithm() -> Result<(), Box<dyn std::error::Error>> {
    let old_file_name = "file_with_extension.txt";
    let new_file_name = "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad.txt";

    let tmp_dir = assert_fs::TempDir::new()?;
    let file = tmp_dir.child(old_file_name);
    file.write_str("abc")?;

    let mut cmd = Command::cargo_bin(env!("CARGO_CRATE_NAME"))?;
    cmd.arg("--dry-run").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(new_file_name));

    let mut cmd = Command::cargo_bin(env!("CARGO_CRATE_NAME"))?;
    cmd.arg(file.path());
    cmd.assert()
        .success();

    tmp_dir
        .child(new_file_name)
        .assert(predicate::path::exists());

    Ok(())
}

#[test]
fn file_with_extension() -> Result<(), Box<dyn std::error::Error>> {
    let old_file_name = "file_with_extension.txt";
    let new_file_name = "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad.txt";

    let tmp_dir = assert_fs::TempDir::new()?;
    let file = tmp_dir.child(old_file_name);
    file.write_str("abc")?;

    let mut cmd = Command::cargo_bin(env!("CARGO_CRATE_NAME"))?;
    cmd.arg("--dry-run").arg("--algorithm").arg("sha256").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(new_file_name));

    let mut cmd = Command::cargo_bin(env!("CARGO_CRATE_NAME"))?;
    cmd.arg("--algorithm").arg("sha256").arg(file.path());
    cmd.assert()
        .success();

    tmp_dir
        .child(new_file_name)
        .assert(predicate::path::exists());

    Ok(())
}

#[test]
fn file_without_extension() -> Result<(), Box<dyn std::error::Error>> {
    let old_file_name = "file_with_extension";
    let new_file_name = "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad";

    let tmp_dir = assert_fs::TempDir::new()?;
    let file = tmp_dir.child(old_file_name);
    file.write_str("abc")?;

    let mut cmd = Command::cargo_bin(env!("CARGO_CRATE_NAME"))?;
    cmd.arg("--dry-run").arg("--algorithm").arg("sha256").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(new_file_name));

    let mut cmd = Command::cargo_bin(env!("CARGO_CRATE_NAME"))?;
    cmd.arg("--algorithm").arg("sha256").arg(file.path());
    cmd.assert()
        .success();

    tmp_dir
        .child(new_file_name)
        .assert(predicate::path::exists());

    Ok(())
}

#[test]
fn sha224_of_empty_file() -> Result<(), Box<dyn std::error::Error>> {
    let old_file_name = "empty.txt";
    let new_file_name = "d14a028c2a3a2bc9476102bb288234c415a2b01f828ea62ac5b3e42f.txt";

    let tmp_dir = assert_fs::TempDir::new()?;
    let file = tmp_dir.child(old_file_name);
    file.write_str("")?;

    let mut cmd = Command::cargo_bin(env!("CARGO_CRATE_NAME"))?;
    cmd.arg("--dry-run").arg("--algorithm").arg("sha224").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(new_file_name));

    let mut cmd = Command::cargo_bin(env!("CARGO_CRATE_NAME"))?;
    cmd.arg("--algorithm").arg("sha224").arg(file.path());
    cmd.assert()
        .success();

    tmp_dir
        .child(new_file_name)
        .assert(predicate::path::exists());

    Ok(())
}

#[test]
fn sha224_of_nist_1_test_vector() -> Result<(), Box<dyn std::error::Error>> {
    let old_file_name = "NIST.1.txt";
    let new_file_name = "23097d223405d8228642a477bda255b32aadbce4bda0b3f7e36c9da7.txt";

    let tmp_dir = assert_fs::TempDir::new()?;
    let file = tmp_dir.child(old_file_name);
    file.write_str("abc")?;

    let mut cmd = Command::cargo_bin(env!("CARGO_CRATE_NAME"))?;
    cmd.arg("--dry-run").arg("--algorithm").arg("sha224").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(new_file_name));

    let mut cmd = Command::cargo_bin(env!("CARGO_CRATE_NAME"))?;
    cmd.arg("--algorithm").arg("sha224").arg(file.path());
    cmd.assert()
        .success();

    tmp_dir
        .child(new_file_name)
        .assert(predicate::path::exists());

    Ok(())
}

#[test]
fn sha224_of_nist_3_test_vector() -> Result<(), Box<dyn std::error::Error>> {
    let old_file_name = "NIST.3.txt";
    let new_file_name = "20794655980c91d8bbb4c1ea97618a4bf03f42581948b2ee4ee7ad67.txt";

    let tmp_dir = assert_fs::TempDir::new()?;
    let file = tmp_dir.child(old_file_name);
    file.write_str(&"a".repeat(1000000))?;

    let mut cmd = Command::cargo_bin(env!("CARGO_CRATE_NAME"))?;
    cmd.arg("--dry-run").arg("--algorithm").arg("sha224").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(new_file_name));

    let mut cmd = Command::cargo_bin(env!("CARGO_CRATE_NAME"))?;
    cmd.arg("--algorithm").arg("sha224").arg(file.path());
    cmd.assert()
        .success();

    tmp_dir
        .child(new_file_name)
        .assert(predicate::path::exists());

    Ok(())
}

#[test]
fn sha224_of_multiple_input_files() -> Result<(), Box<dyn std::error::Error>> {
    let old_empty_file_name = "empty.txt";
    let new_empty_file_name = "d14a028c2a3a2bc9476102bb288234c415a2b01f828ea62ac5b3e42f.txt";
    let old_nist_1_file_name = "NIST.1.txt";
    let new_nist_1_file_name = "23097d223405d8228642a477bda255b32aadbce4bda0b3f7e36c9da7.txt";
    let old_nist_3_file_name = "NIST.3.txt";
    let new_nist_3_file_name = "20794655980c91d8bbb4c1ea97618a4bf03f42581948b2ee4ee7ad67.txt";

    let tmp_dir = assert_fs::TempDir::new()?;
    let empty_file = tmp_dir.child(old_empty_file_name);
    empty_file.write_str("")?;
    let nist_1_file = tmp_dir.child(old_nist_1_file_name);
    nist_1_file.write_str("abc")?;
    let nist_3_file = tmp_dir.child(old_nist_3_file_name);
    nist_3_file.write_str(&"a".repeat(1000000))?;

    let mut cmd = Command::cargo_bin(env!("CARGO_CRATE_NAME"))?;
    cmd.arg("--dry-run").arg("--algorithm").arg("sha224")
        .arg(empty_file.path())
        .arg(nist_1_file.path())
        .arg(nist_3_file.path());

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(new_empty_file_name))
        .stdout(predicate::str::contains(new_nist_1_file_name))
        .stdout(predicate::str::contains(new_nist_3_file_name));

    let mut cmd = Command::cargo_bin(env!("CARGO_CRATE_NAME"))?;
    cmd.arg("--algorithm").arg("sha224")
        .arg(empty_file.path())
        .arg(nist_1_file.path())
        .arg(nist_3_file.path());

    cmd.assert()
        .success();

    tmp_dir
        .child(new_empty_file_name)
        .assert(predicate::path::exists());

    tmp_dir
        .child(new_nist_1_file_name)
        .assert(predicate::path::exists());

    tmp_dir
        .child(new_nist_3_file_name)
        .assert(predicate::path::exists());

    Ok(())
}

#[test]
fn sha256_of_empty_file() -> Result<(), Box<dyn std::error::Error>> {
    let old_file_name = "empty.txt";
    let new_file_name = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855.txt";

    let tmp_dir = assert_fs::TempDir::new()?;
    let file = tmp_dir.child(old_file_name);
    file.write_str("")?;

    let mut cmd = Command::cargo_bin(env!("CARGO_CRATE_NAME"))?;
    cmd.arg("--dry-run").arg("--algorithm").arg("sha256").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(new_file_name));

    let mut cmd = Command::cargo_bin(env!("CARGO_CRATE_NAME"))?;
    cmd.arg("--algorithm").arg("sha256").arg(file.path());
    cmd.assert()
        .success();

    tmp_dir
        .child(new_file_name)
        .assert(predicate::path::exists());

    Ok(())
}

#[test]
fn sha256_of_nist_1_test_vector() -> Result<(), Box<dyn std::error::Error>> {
    let old_file_name = "NIST.1.txt";
    let new_file_name = "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad.txt";

    let tmp_dir = assert_fs::TempDir::new()?;
    let file = tmp_dir.child(old_file_name);
    file.write_str("abc")?;

    let mut cmd = Command::cargo_bin(env!("CARGO_CRATE_NAME"))?;
    cmd.arg("--dry-run").arg("--algorithm").arg("sha256").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(new_file_name));

    let mut cmd = Command::cargo_bin(env!("CARGO_CRATE_NAME"))?;
    cmd.arg("--algorithm").arg("sha256").arg(file.path());
    cmd.assert()
        .success();

    tmp_dir
        .child(new_file_name)
        .assert(predicate::path::exists());

    Ok(())
}

#[test]
fn sha256_of_nist_3_test_vector() -> Result<(), Box<dyn std::error::Error>> {
    let old_file_name = "NIST.3.txt";
    let new_file_name = "cdc76e5c9914fb9281a1c7e284d73e67f1809a48a497200e046d39ccc7112cd0.txt";

    let tmp_dir = assert_fs::TempDir::new()?;
    let file = tmp_dir.child(old_file_name);
    file.write_str(&"a".repeat(1000000))?;

    let mut cmd = Command::cargo_bin(env!("CARGO_CRATE_NAME"))?;
    cmd.arg("--dry-run").arg("--algorithm").arg("sha256").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(new_file_name));

    let mut cmd = Command::cargo_bin(env!("CARGO_CRATE_NAME"))?;
    cmd.arg("--algorithm").arg("sha256").arg(file.path());
    cmd.assert()
        .success();

    tmp_dir
        .child(new_file_name)
        .assert(predicate::path::exists());

    Ok(())
}

#[test]
fn sha256_of_multiple_input_files() -> Result<(), Box<dyn std::error::Error>> {
    let old_empty_file_name = "empty.txt";
    let new_empty_file_name = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855.txt";
    let old_nist_1_file_name = "NIST.1.txt";
    let new_nist_1_file_name = "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad.txt";
    let old_nist_3_file_name = "NIST.3.txt";
    let new_nist_3_file_name = "cdc76e5c9914fb9281a1c7e284d73e67f1809a48a497200e046d39ccc7112cd0.txt";

    let tmp_dir = assert_fs::TempDir::new()?;
    let empty_file = tmp_dir.child(old_empty_file_name);
    empty_file.write_str("")?;
    let nist_1_file = tmp_dir.child(old_nist_1_file_name);
    nist_1_file.write_str("abc")?;
    let nist_3_file = tmp_dir.child(old_nist_3_file_name);
    nist_3_file.write_str(&"a".repeat(1000000))?;

    let mut cmd = Command::cargo_bin(env!("CARGO_CRATE_NAME"))?;
    cmd.arg("--dry-run").arg("--algorithm").arg("sha256")
        .arg(empty_file.path())
        .arg(nist_1_file.path())
        .arg(nist_3_file.path());

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(new_empty_file_name))
        .stdout(predicate::str::contains(new_nist_1_file_name))
        .stdout(predicate::str::contains(new_nist_3_file_name));

    let mut cmd = Command::cargo_bin(env!("CARGO_CRATE_NAME"))?;
    cmd.arg("--algorithm").arg("sha256")
        .arg(empty_file.path())
        .arg(nist_1_file.path())
        .arg(nist_3_file.path());

    cmd.assert()
        .success();

    tmp_dir
        .child(new_empty_file_name)
        .assert(predicate::path::exists());

    tmp_dir
        .child(new_nist_1_file_name)
        .assert(predicate::path::exists());

    tmp_dir
        .child(new_nist_3_file_name)
        .assert(predicate::path::exists());

    Ok(())
}

#[test]
fn sha384_of_empty_file() -> Result<(), Box<dyn std::error::Error>> {
    let old_file_name = "empty.txt";
    let new_file_name = "38b060a751ac96384cd9327eb1b1e36a21fdb71114be07434c0cc7bf63f6e1da274edebfe76f65fbd51ad2f14898b95b.txt";

    let tmp_dir = assert_fs::TempDir::new()?;
    let file = tmp_dir.child(old_file_name);
    file.write_str("")?;

    let mut cmd = Command::cargo_bin(env!("CARGO_CRATE_NAME"))?;
    cmd.arg("--dry-run").arg("--algorithm").arg("sha384").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(new_file_name));

    let mut cmd = Command::cargo_bin(env!("CARGO_CRATE_NAME"))?;
    cmd.arg("--algorithm").arg("sha384").arg(file.path());
    cmd.assert()
        .success();

    tmp_dir
        .child(new_file_name)
        .assert(predicate::path::exists());

    Ok(())
}

#[test]
fn sha384_of_nist_1_test_vector() -> Result<(), Box<dyn std::error::Error>> {
    let old_file_name = "NIST.1.txt";
    let new_file_name = "cb00753f45a35e8bb5a03d699ac65007272c32ab0eded1631a8b605a43ff5bed8086072ba1e7cc2358baeca134c825a7.txt";

    let tmp_dir = assert_fs::TempDir::new()?;
    let file = tmp_dir.child(old_file_name);
    file.write_str("abc")?;

    let mut cmd = Command::cargo_bin(env!("CARGO_CRATE_NAME"))?;
    cmd.arg("--dry-run").arg("--algorithm").arg("sha384").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(new_file_name));

    let mut cmd = Command::cargo_bin(env!("CARGO_CRATE_NAME"))?;
    cmd.arg("--algorithm").arg("sha384").arg(file.path());
    cmd.assert()
        .success();

    tmp_dir
        .child(new_file_name)
        .assert(predicate::path::exists());

    Ok(())
}

#[test]
fn sha384_of_nist_3_test_vector() -> Result<(), Box<dyn std::error::Error>> {
    let old_file_name = "NIST.3.txt";
    let new_file_name = "9d0e1809716474cb086e834e310a4a1ced149e9c00f248527972cec5704c2a5b07b8b3dc38ecc4ebae97ddd87f3d8985.txt";

    let tmp_dir = assert_fs::TempDir::new()?;
    let file = tmp_dir.child(old_file_name);
    file.write_str(&"a".repeat(1000000))?;

    let mut cmd = Command::cargo_bin(env!("CARGO_CRATE_NAME"))?;
    cmd.arg("--dry-run").arg("--algorithm").arg("sha384").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(new_file_name));

    let mut cmd = Command::cargo_bin(env!("CARGO_CRATE_NAME"))?;
    cmd.arg("--algorithm").arg("sha384").arg(file.path());
    cmd.assert()
        .success();

    tmp_dir
        .child(new_file_name)
        .assert(predicate::path::exists());

    Ok(())
}

#[test]
fn sha384_of_multiple_input_files() -> Result<(), Box<dyn std::error::Error>> {
    let old_empty_file_name = "empty.txt";
    let new_empty_file_name = "38b060a751ac96384cd9327eb1b1e36a21fdb71114be07434c0cc7bf63f6e1da274edebfe76f65fbd51ad2f14898b95b.txt";
    let old_nist_1_file_name = "NIST.1.txt";
    let new_nist_1_file_name = "cb00753f45a35e8bb5a03d699ac65007272c32ab0eded1631a8b605a43ff5bed8086072ba1e7cc2358baeca134c825a7.txt";
    let old_nist_3_file_name = "NIST.3.txt";
    let new_nist_3_file_name = "9d0e1809716474cb086e834e310a4a1ced149e9c00f248527972cec5704c2a5b07b8b3dc38ecc4ebae97ddd87f3d8985.txt";

    let tmp_dir = assert_fs::TempDir::new()?;
    let empty_file = tmp_dir.child(old_empty_file_name);
    empty_file.write_str("")?;
    let nist_1_file = tmp_dir.child(old_nist_1_file_name);
    nist_1_file.write_str("abc")?;
    let nist_3_file = tmp_dir.child(old_nist_3_file_name);
    nist_3_file.write_str(&"a".repeat(1000000))?;

    let mut cmd = Command::cargo_bin(env!("CARGO_CRATE_NAME"))?;
    cmd.arg("--dry-run").arg("--algorithm").arg("sha384")
        .arg(empty_file.path())
        .arg(nist_1_file.path())
        .arg(nist_3_file.path());

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(new_empty_file_name))
        .stdout(predicate::str::contains(new_nist_1_file_name))
        .stdout(predicate::str::contains(new_nist_3_file_name));

    let mut cmd = Command::cargo_bin(env!("CARGO_CRATE_NAME"))?;
    cmd.arg("--algorithm").arg("sha384")
        .arg(empty_file.path())
        .arg(nist_1_file.path())
        .arg(nist_3_file.path());

    cmd.assert()
        .success();

    tmp_dir
        .child(new_empty_file_name)
        .assert(predicate::path::exists());

    tmp_dir
        .child(new_nist_1_file_name)
        .assert(predicate::path::exists());

    tmp_dir
        .child(new_nist_3_file_name)
        .assert(predicate::path::exists());

    Ok(())
}

#[test]
fn sha512_of_empty_file() -> Result<(), Box<dyn std::error::Error>> {
    let old_file_name = "empty.txt";
    let new_file_name = "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e.txt";

    let tmp_dir = assert_fs::TempDir::new()?;
    let file = tmp_dir.child(old_file_name);
    file.write_str("")?;

    let mut cmd = Command::cargo_bin(env!("CARGO_CRATE_NAME"))?;
    cmd.arg("--dry-run").arg("--algorithm").arg("sha512").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(new_file_name));

    let mut cmd = Command::cargo_bin(env!("CARGO_CRATE_NAME"))?;
    cmd.arg("--algorithm").arg("sha512").arg(file.path());
    cmd.assert()
        .success();

    tmp_dir
        .child(new_file_name)
        .assert(predicate::path::exists());

    Ok(())
}

#[test]
fn sha512_of_nist_1_test_vector() -> Result<(), Box<dyn std::error::Error>> {
    let old_file_name = "NIST.1.txt";
    let new_file_name = "ddaf35a193617abacc417349ae20413112e6fa4e89a97ea20a9eeee64b55d39a2192992a274fc1a836ba3c23a3feebbd454d4423643ce80e2a9ac94fa54ca49f.txt";

    let tmp_dir = assert_fs::TempDir::new()?;
    let file = tmp_dir.child(old_file_name);
    file.write_str("abc")?;

    let mut cmd = Command::cargo_bin(env!("CARGO_CRATE_NAME"))?;
    cmd.arg("--dry-run").arg("--algorithm").arg("sha512").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(new_file_name));

    let mut cmd = Command::cargo_bin(env!("CARGO_CRATE_NAME"))?;
    cmd.arg("--algorithm").arg("sha512").arg(file.path());
    cmd.assert().success();

    tmp_dir
        .child(&new_file_name)
        .assert(predicate::path::exists());

    Ok(())
}

#[test]
fn sha512_of_nist_3_test_vector() -> Result<(), Box<dyn std::error::Error>> {
    let old_file_name = "NIST.3.txt";
    let new_file_name = "e718483d0ce769644e2e42c7bc15b4638e1f98b13b2044285632a803afa973ebde0ff244877ea60a4cb0432ce577c31beb009c5c2c49aa2e4eadb217ad8cc09b.txt";

    let tmp_dir = assert_fs::TempDir::new()?;
    let file = tmp_dir.child(old_file_name);
    file.write_str(&"a".repeat(1000000))?;

    let mut cmd = Command::cargo_bin(env!("CARGO_CRATE_NAME"))?;
    cmd.arg("--dry-run").arg("--algorithm").arg("sha512").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(new_file_name));

    let mut cmd = Command::cargo_bin(env!("CARGO_CRATE_NAME"))?;
    cmd.arg("--algorithm").arg("sha512").arg(file.path());
    cmd.assert()
        .success();

    tmp_dir
        .child(new_file_name)
        .assert(predicate::path::exists());

    Ok(())
}

#[test]
fn sha512_of_multiple_input_files() -> Result<(), Box<dyn std::error::Error>> {
    let old_empty_file_name = "empty.txt";
    let new_empty_file_name = "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e.txt";
    let old_nist_1_file_name = "NIST.1.txt";
    let new_nist_1_file_name = "ddaf35a193617abacc417349ae20413112e6fa4e89a97ea20a9eeee64b55d39a2192992a274fc1a836ba3c23a3feebbd454d4423643ce80e2a9ac94fa54ca49f.txt";
    let old_nist_3_file_name = "NIST.3.txt";
    let new_nist_3_file_name = "e718483d0ce769644e2e42c7bc15b4638e1f98b13b2044285632a803afa973ebde0ff244877ea60a4cb0432ce577c31beb009c5c2c49aa2e4eadb217ad8cc09b.txt";

    let tmp_dir = assert_fs::TempDir::new()?;
    let empty_file = tmp_dir.child(old_empty_file_name);
    empty_file.write_str("")?;
    let nist_1_file = tmp_dir.child(old_nist_1_file_name);
    nist_1_file.write_str("abc")?;
    let nist_3_file = tmp_dir.child(old_nist_3_file_name);
    nist_3_file.write_str(&"a".repeat(1000000))?;

    let mut cmd = Command::cargo_bin(env!("CARGO_CRATE_NAME"))?;
    cmd.arg("--dry-run").arg("--algorithm").arg("sha512")
        .arg(empty_file.path())
        .arg(nist_1_file.path())
        .arg(nist_3_file.path());

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(new_empty_file_name))
        .stdout(predicate::str::contains(new_nist_1_file_name))
        .stdout(predicate::str::contains(new_nist_3_file_name));

    let mut cmd = Command::cargo_bin(env!("CARGO_CRATE_NAME"))?;
    cmd.arg("--algorithm").arg("sha512")
        .arg(empty_file.path())
        .arg(nist_1_file.path())
        .arg(nist_3_file.path());

    cmd.assert()
        .success();

    tmp_dir
        .child(new_empty_file_name)
        .assert(predicate::path::exists());

    tmp_dir
        .child(new_nist_1_file_name)
        .assert(predicate::path::exists());

    tmp_dir
        .child(new_nist_3_file_name)
        .assert(predicate::path::exists());

    Ok(())
}
