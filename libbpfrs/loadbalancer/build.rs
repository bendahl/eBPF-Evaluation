use libbpf_cargo::SkeletonBuilder;
use std::process::Command;

fn main() {
    Command::new("bash")
        .arg("-c")
        .arg("bpftool btf dump file /sys/kernel/btf/vmlinux format c > vmlinux.h")
        .output()
        .expect("failed to execute process");

    SkeletonBuilder::new()
        .source("loadbalancer.bpf.c")
        .debug(true)
        .build_and_generate("loadbalancer.skel.rs")
        .unwrap();
}
