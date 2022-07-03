<h1>Architecture</h1>

RustHunter is able to take a **snapshot** of a complex network environment. It can be installed on a Windows or Linus machine (**Master Node**) and leverages an Ansible docker to upload and execute a **collector** artifact on multiple machines (**Target Nodes**) to collect data. An **inventory** and a **config file** are respectively responsible for the selection of the target nodes and the configuration of the collector. Snapshots can then be compared to identify eventual changes in the environment, such as users, administrators, autoruns or listening ports.

<img src="architecture.png" style="display: block; margin-left: auto; margin-right: auto; width: 100%;" alt="RustHunter Architecture">

The main elements are listed and described hereafter:

- **Snapshot**: A JSON file representing an image of the enterprise environment;
- **Master Node**: A `Windows` or `Linux` machine from which RustHunter can be executed;
- **Target Nodes**: Multi-platform machines (`Windows`, `Linux` and `macOS`) that can be included in the environmental snapshot;
- **Collector**: Multi-platform artifact able to collect data from the target nodes;
- **Plugin**: A single generic constituting the collector with the capability to capture specific information in a snapshot;
- **Inventory File**: An INI file representing the Ansible inventory for the target nodes to be included in a snapshot;
- **Config File**: An INI file to enable/disable or configure plugins for a snapshot.
