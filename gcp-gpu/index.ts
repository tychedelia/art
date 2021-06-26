import * as pulumi from "@pulumi/pulumi";
import * as gcp from "@pulumi/gcp";
import * as fs from "fs";

const zone = "us-east1-b"
const region = "us-east1"

const addr = new gcp.compute.Address("addr", {
    region,
});
const network = new gcp.compute.Network("network");
const firewall = new gcp.compute.Firewall("firewall", {
    network: network.id,
    allows: [{
        protocol: "tcp",
        ports: [ "22" ],
    }],
});

// our boot disk for the compute instance
// TODO: attach additional disk so that model can survive instance getting killed for maintenance
const disk = new gcp.compute.Disk("disk", {
    // specified in GB
    size: 200,
    image: "ubuntu-os-cloud/ubuntu-1804-lts",
    zone,
})

const metadataStartupScript = fs.readFileSync("setup.sh", "utf-8")
const vm = new gcp.compute.Instance("vm", {
    bootDisk: {
        source: disk.id
    },
    zone,
    machineType: "n1-standard-1",
    networkInterfaces: [{
        network: network.id,
        accessConfigs: [{
            natIp: addr.address
        }],
    }],
    guestAccelerators: [{
        count: 1,
        type: "nvidia-tesla-p100",
    }],
    metadata: {
        // allow use of project wide ssh keys to access this instance
        "block-project-ssh-keys": "FALSE",
    },
    // gpus can't be live migrated
    scheduling: {
        onHostMaintenance: "TERMINATE"
    },
    // setup.sh
    metadataStartupScript,
});

export const instanceName = vm.name;
export const instanceIP = addr.address