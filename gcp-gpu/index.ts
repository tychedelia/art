import * as pulumi from "@pulumi/pulumi";
import * as gcp from "@pulumi/gcp";

const zones = [
	"us-central1-c", 
	"us-central1-f",
       	"us-east1-b",
	"us-east1-c",
	"us-west1-a",
	"us-west1-b",
];

//const region = "us-central1" 
//const zone = `${region}-f` 

const zone = process.env["zone"] || zones[0] 
const region = `${zone.split('-')[0]}-${zone.split('-')[1]}`

// allocate a public ipv4 address in our region
const addr = new gcp.compute.Address("addr", {
    region,
});

// create a new virtual network for our vm
const network = new gcp.compute.Network("network");

// attach network to firewall to ensure only ssh is exposed to internet
const firewall = new gcp.compute.Firewall("firewall", {
    network: network.id,
    allows: [{
        protocol: "tcp",
        ports: [ "22" ],
    }],
});

// our boot disk for the compute instance
const disk = new gcp.compute.Disk("disk", {
    // specified in GB
    size: 1000,
    // what distro is imaged onto the disk for startup
    image: "stylegan-image", // our custom image
    // disks are network attached and should live in same zone
    zone,
})

// our main vm for doing work
const vm = new gcp.compute.Instance("vm", {
    // what disk we boot from
    bootDisk: {
        source: disk.id
    },
    // where this machine lives
    zone,
    // this machine is probably a little over-provisioned, but we run into issues with only 1 core
    machineType: "n1-standard-4",
    // attach our machine to the vlan
    networkInterfaces: [{
        network: network.id,
        accessConfigs: [{
            natIp: addr.address
        }],
    }],
    // this is the GPU
    guestAccelerators: [{
        count: 1,
        type: "nvidia-tesla-p100",
    }],
    // tags for this vm
    metadata: {
        // allow use of project wide ssh keys to access this instance
        "block-project-ssh-keys": "FALSE",
    },
    // gpus can't be live migrated
    scheduling: {
        onHostMaintenance: "TERMINATE"
    },
    // kill our instance if we want to resize
    allowStoppingForUpdate: true,
});

export const instanceName = vm.name;
export const instanceIP = addr.address;
