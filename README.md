# BPF Use Case Implementation 
This repository contains [eBPF](https://docs.kernel.org/bpf/index.html) example programs that were implemented as part of an eBPF library evaluation.
The evaluation was carried out in the context of a master thesis in computer science.
It is important to note that eBPF is often referred to as BPF.
Within this document, the commonly used term BPF is used to refer to eBPF.
The predecessor of eBPF - [cBPF](https://docs.kernel.org/bpf/classic_vs_extended.html) - is not subject of this evaluation.

## Approach
The set of BPF libraries selected for this thesis was first evaluated theoretically. 
The theoretical model applied here is based on the [OSSPal model](https://www.researchgate.net/publication/316356715_OSSpal_Finding_and_Evaluating_Open_Source_Software).
In addition, since BPF is used to solve a diverse set of practical problems, it is sensible to evaluate the utility of each library in the context of typical [BPF use cases](#use-cases).
To accomplish this, all of the identified use cases were implemented using the selected libraries. 
The resulting BPF programs are contained in this repository, along with some basic documentation.

## Libraries
The list of libraries was determined as part of the evaluation, using the [OSSPal model](https://library.oapen.org/bitstream/handle/20.500.12657/27754/1002251.pdf?sequence=1#page=199).
The following subsections describe each library used.
Versions used for each library were fixed to those publicly available as of June 5th, 2023. Any later releases were not considered for implementation and testing.

### Aya
[Aya](https://github.com/aya-rs/aya) is a [Rust](https://www.rust-lang.org/)-based BPF framework that leverages Rust's ability to compile to BPF bytecode, enabling developers to write kernel and user space code in Rust.
It offers project templates for different types of BPF programs along with facilities to generate a starter project as well as interface types, required for communication between kernel space and user space.
In addition, it does come with integrated logging facilities.
Its [companion website](https://aya-rs.dev/) gives a short introduction into BPF development with Aya.

### Ebpf
The [Ebpf](https://github.com/cilium/ebpf) library is part of the cilium project, a cloud-native networking, observability and security stack for Kubernetes.
The library is implemented in the [Go programming language](https://go.dev/) and eases development of BPF programs by providing a comprehensive set of functions to build and manage BPF programs.
Kernel-Code (the BPF program) is written in C.
Interface types that are required to communicate data between kernel and user space can be generated automatically using [bpf2go](https://pkg.go.dev/github.com/cilium/ebpf/cmd/bpf2go).

### Libbpf
[Libbpf](https://github.com/libbpf/libbpf) is a pure C library that is developed as part of the linux kernel. 
It simplifies the creation and handling of BPF programs by taking care of the general BPF program's lifecycle as well as supporting interface types (skeletons) generated by [bpftool](https://github.com/libbpf/bpftool/releases).
It supports a wide array of program types and works reliably.


### Libbpfgo
[Libbpfgo](https://github.com/aquasecurity/libbpfgo) is a Libbpf-wrapper written in Go.
It leverages the abilities of Libbpf, but does not offer direct support for interface type generation.
This can be alleviated by using [bpf2go](https://pkg.go.dev/github.com/cilium/ebpf/cmd/bpf2go) to generate Go interface types to interact with the kernel part of the program.


### Libbpf-rs
[Libbpf-rs](https://github.com/libbpf/libbpf-rs) is another Libbpf-wrapper.
It is written in [Rust](https://www.rust-lang.org/), but uses C for the BPF code.
Generating Rust code to interface with kernel data types is handled by the [skeleton builder](https://docs.rs/libbpf-cargo/latest/libbpf_cargo/struct.SkeletonBuilder.html).

### RedBPF
[RedBPF](https://github.com/foniod/redbpf) is a [Rust](https://www.rust-lang.org/)-based BPF framework that follows a similar approach to Aya, using Rust for kernel code and user space code.
It is not actively maintained anymore and requires an older Rust- as well as [LLVM](https://llvm.org/)-version.
The Rust compiler version and instructions listed in the repository's README did not yield any successful builds.
Neither did the containerized build environments.
Through testing different compiler combinations a working combination was determined.
However, only some of the examples could be recreated without issues.
 
## BPF Use Cases
The BPF use cases presented here are also based on information publicly available on [ebpf.io](https://www.ebpf.io) as well as [other sources](#sources).
This is not an exhaustive list, but presents a diverse set of use cases in order to test different abilities of the given libraries.
The following sections describe the use cases in more detail.
It also explains the basic functionality of the example programs and how to use them.

### Packet Filtering

Using the [express data path (XDP)](https://docs.cilium.io/en/latest/bpf/), incoming packets can be filtered before they are processed by the regular Linux network stack, saving processing time. 
Depending on the hardware used, it is even possible to offload the processing to the network interface card (nic), instead of relying on the system's CPU.
This is especially useful in the context of DDoS mitigation, where a large number of packets needs to be processed quickly.

The example program implemented as part of the evaluation is called "packetfilter".
It is a very simple XDP program that can be configured using the included yaml file.
Filtering rules consist of an interface, default settings for IPv4 and IPv6 traffic (block/allow), as well as a block- and allow-list for IPv4- and IPv6-addresses.
Packets are filtered solely by their source IP address.

### Load Balancing

High performance software load balancers are a flexible alternative to hardware based load balancing and are especially useful to distribute traffic between internal systems.
BPF can be used in different ways to build programmable software load balancers.

A layer 4 load balancer implementation using XDP is implemented as an example.
To keep things simple, this example implementation makes use of direct server return (DSR) and does not do full NAT.
The included Vagrantfile takes care of the environment setup, including the set up of the main development (load balancer) machine and two backend servers for testing.


### Network Condition Simulation

Distributed systems may suffer from various problems, often related to the network. 
It is therefore good practice to test a system's ability to handle certain network related issues by simulating such conditions.
Within the Linux kernel, [Traffic Control (TC)](https://tldp.org/HOWTO/Traffic-Control-HOWTO/intro.html) and [queueing disciplines (qdiscs)](https://www.linuxjournal.com/content/queueing-linux-network-stack) can be used for this purpose. 

The example TC program created as part of the evaluation is attached to the egress side of the specified interface and simulates network conditions, such as:
- jitter
- packet drops
- packet reordering

The interface to be used as well as the issues to be simulated can be configured in a yaml configuration file.
The example program is called "jittergen".
    
### Connection Tracing

Tracing system state is one of BPF's main use cases.
Dedicated trace points in the kernel offer a stable interface to attach custom tracing logic.
Building on this, a simple tcp connection tracer is implemented to evaluate each library's support for [tracepoints](https://docs.kernel.org/trace/tracepoints.html). 
    
A tcp connection tracer that is heavily inspired by [Brendan Gregg's tcplife](https://github.com/iovisor/bcc/blob/master/tools/tcplife.py) was built as part of this evaluation as well.
It is called "comtrace" and traces TCP connections, recording the communicating process, source and destination addresses as well as transmitted bytes and connection time.

### Implementation of Security Functionality

BPF is used to audit running systems in order to detect suspicious activity.
It can also be used to actively intercept certain actions via [Linux Security Modules (LSM)](https://docs.kernel.org/security/lsm.html).

A simple LSM was created that intercepts chroot calls to the root directory issued by containerized processes.
The program is called "lsmbpf".


## Repository Structure
To ease navigation, the repository structure is organized as follows:

```
repo-root/
    README.md
    library/
        use-case/
            source-file1
            source-file2
            ...
            source-fileN
            Vagrantfile
```

On the top level, you will find this README. 
The second level contains directories for each library tested. 
The lowest level in the hierarchy contains the actual implementation files for each use case based on the given library.
Also, note that each implementation directory contains its own `Vagrantfile`. 
This file can be used to automatically create an environment for development and testing.
In order to use this file, refer to the [prerequisites](#prerequisites)

## Prerequisites
To ease working with the examples, [Vagrant](https://www.vagrantup.com/) can be used to automatically provision a virtual machine for testing and development.
Vagrant will also take care of the installation and configuration of all required tools during the provisioning.
In order to use this setup, the following prerequisites will need to be installed:

- [Vagrant](https://www.vagrantup.com/) 2.2.19+
- [Virtualbox](https://www.virtualbox.org/) 6.1.38+

If any further prerequisites are required to build/run a specific example, the relevant information can be found in the accompanying README file.

### Using the VM
Once the required prerequisites are installed, simply run the following commands to start/stop the development VM within the project directory:

- Starting the VM:
    `vagrant up`

Note that in case of the lsm examples a reload via `vagrant reload` is required as an additional step.

- Tearing down the VM (will delete all allocated resources):
    `vagrant destroy -f`

To simplify working with the development environment, you may also use dedicated make targets instead of the above-mentioned commands. 
The following targets are present in all projects and take care of proper setup and tear down of the VMs:

- Startup:
    `make testenvironment-up`

- Teardown:
    `make testenvironment-down`

Note that these steps may take some time, especially the setup of the VM. For more details on using Vagrant, see the [documentation](https://developer.hashicorp.com/vagrant/docs).

### Building and running the examples
In order to build each of the examples, the included Makefile can be used. 
It is recommended to use the accompanying VM in order to perform a local build.
This will ensure that all required dependencies are met without further user intervention.
The following commands can be used from within each implementation directory, where the appropriate Vagrantfile is located:

- Enter the VM using the command `vagrant ssh` (note that this needs to be executed within the project directory, containing the Vagrant file).
- Change the directory to `/vagrant` using `cd /vagrant`. This directory is shared between the host machine and the VM to allow accessing the source files from the VM.
- Run `make build` in order to build the program.
- Run the example program using `sudo ./[bpf-program-name]` (the sudo is important, since loading BPF programs requires root privileges).
- Unless stated otherwise, the running programs can be stopped by simply pressing `<CTRL>+<C>`

### Cleaning up
Once you're done testing and developing, you may use `make clean` to clean up any generated files.

### Additional Scripts
As part of the evaluation, certain metrics were calculated using different scripts. 
These scripts can be found in the `scripts/` directory.
