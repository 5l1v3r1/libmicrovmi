# libmicrovmi

![](https://github.com/Wenzel/libmicrovmi/workflows/Build/badge.svg)
[![Join the chat at https://gitter.im/libmicrovmi/community](https://badges.gitter.im/libmicrovmi/community.svg)](https://gitter.im/libmicrovmi/community?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)
[![tokei](https://tokei.rs/b1/github/Wenzel/libmicrovmi)](https://github.com/Wenzel/libmicrovmi)
[![repo size](https://img.shields.io/github/repo-size/Wenzel/libmicrovmi)](https://github.com/Wenzel/libmicrovmi)
[![standard-readme compliant](https://img.shields.io/badge/readme%20style-standard-brightgreen.svg?style=flat-square)](https://github.com/RichardLitt/standard-readme)

> A cross-platform unified interface on top of hypervisor's VMI APIs

## Table of Contents

- [Overview](#overview)
- [VMI API](#vmi-api)
- [Requirements](#requirements)
- [Build](#build)
- [Example](#example)
- [Maintainers](#maintainers)
- [Contributing](#contributing)
- [License](#license)

## Overview

`libmicrovmi` simply aims to provide a cross-platform unified API and the necessary drivers to be
compatible with the diversity of hypervisors available today.

Its main goal is to solve the problem of [`libvmi`](https://github.com/libvmi/libvmi) being written in `C` and a
Linux-centered library, not yet ready to offer drivers for other platforms such
as Windows and MacOS.

It doesn't aim to **replace** `libvmi`, since it doesn't implement features like rekall profile parsing or operating system knowledge, but rather to provide a foundation for high-level VMI libraries.

The term micro (μ) refers to the library's simplicity as well as the letter `U`
standing for `Unified` interface.

The grand goal is to be the foundation for a VMI abstraction library that will
be
- multi-hypervisor
- multi-emulator
- cross-plaform
- high-level

So that it provides the necessary abstractions and semantic context to let
developers focus on building VMI apps.

![libmicrovmi_image](https://user-images.githubusercontent.com/964610/67619627-51036e80-f7ed-11e9-80f6-2eb15b018108.png)

### Drivers

- [x] Xen
- [x] KVM (based on [KVM-VMI](https://github.com/KVM-VMI/kvm-vmi))
- [x] VirtualBox (based on [icebox](https://github.com/thalium/icebox))
- [ ] Hyper-V (based on [LiveCloudKd](https://github.com/comaeio/LiveCloudKd))
- [ ] QEMU (based on [TCG Plugins](https://github.com/comaeio/LiveCloudKd))

## VMI API

* Query and modify the VM hardware state
    - read/write VCPU registers
    - read/write physical memory
* Subscribe and listen to hardware events
    - mov to/from CR3/CR8
    - mov to/from DRx
    - mov to/from MSR
    - interrupts
    - singlestep (MTF)
    - hypercalls
    - descriptors
    - SLAT (Second Level Address Translation) events
        - `r/w/x` event on a page
        - dynamically switch to multiple memory _views_ using alternate SLAT pointers
- Utilities
    - foreign mapping
    - pagefault injection

For more detailed information, please check the [Wiki](https://github.com/Wenzel/libmicrovmi/wiki)

## Requirements

- `Rust` stable
- `cargo`
- `clang` (bindgen)

## Build

To build the library, simply run

    cargo build

By default, only the `Dummy` driver will be available (it does nothing).

To enable a driver, for example `xen`, enable the corresponding feature
(`Cargo.toml`)

    cargo build --features xen

## Example

### mem-dump

A small binary is available to demonstrate what the `libmicrovmi` can do: `mem-dump`

It will dump the raw memory of the specified domain in a `domain_name.dump`
file.

Example with the `xen` driver:
~~~
$ cargo run --features xen --example mem-dump winxp
~~~

A memory dump should have been written in `winxp.dump`.

### API example

~~~Rust
// select drive type (Dummy, Xen, KVM, ...)
let drv_type = DriverType::Dummy;
// init library
let mut drv: Box<dyn Introspectable> = microvmi::init(drv_type, domain_name);
// pause VM
drv.pause()
    .expect("Failed to pause VM");
// get max physical address
let max_addr = drv.get_max_physical_addr()
                    .expect("Failed to get max physical address");
// read physical memory
let mut buffer: [u8; 4096] = [0; 4096];
let result = drv.read_physical(0x804d7000, &mut buffer);
// resume VM
drv.resume()
    .expect("Failed to resume VM");
~~~

## References

- [LibVMI C library](https://github.com/libvmi/libvmi): Simplified Virtual Machine Introspection

## Maintainers

[@Wenzel](https://github.com/Wenzel)

## Contributing

PRs accepted.

Small note: If editing the Readme, please conform to the [standard-readme](https://github.com/RichardLitt/standard-readme) specification.

## License

[GNU General Public License v3.0](https://github.com/Wenzel/pyvmidbg/blob/master/LICENSE)

