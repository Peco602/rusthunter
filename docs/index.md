<h1>What is RustHunter?</h1>

RustHunter is a modular incident response framework to build and compare environmental baselines. It is written in [`Rust`](https://www.rust-lang.org/) and uses [`Ansible`](https://www.ansible.com/) to collect data across multiple hosts. 

<img src="images/logo.png" style="display: block; margin-left: auto; margin-right: auto; width: 75%;" alt="RustHunter Logo">

Due to the following features it can be also employed to perform threat hunting and incident handling: 

* **Multi-Platform**: it is able to collect data from both `Windows`, `Linux` and `macOS` machines;
* **Agentless**: the usage of the Ansible technology based on SSH and WinRM allows to overcome the requirement of a local agent waiting for a task to be accomplished;
* **Easily Deployable**: it is cross-platform and can be deployed both on premises and in a Cloud-based environment. A *Bash* and a *PowerShell* scripts have been developed to execute the tool respectively from a Linux and Windows machine;
* **Easily Expandable**: it provides developer-ready Rust specifications offering an easy way to expand the product features by writing custom modules to collect additional machine data.
