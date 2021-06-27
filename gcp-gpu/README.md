# Using Pulumi to manage GCP compute  for stylegan2 training

Pulumi is a cloud infrastructure management framework, similar to Terraform, that allows us to describe infrastructure
as code (IAC). Most cloud providers provide UIs to manage the creation of new infrastructure, but these UIs are often buggy,
poorly documented, and difficult to use. IAC not only allows us to keep track of infrastructure in source control, but
also enables us to easily create and tear down infrastructure as needed. Because we don't want to leave expensive GPU
compute instances lying around, the ability to repeatedly build infrastructure is immensely helpful.

## Required accounts

First, sign up for a new Pulumi account: https://app.pulumi.com. Pulumi is free to use for individual projects.

Next, sign up for GCP: https://console.cloud.google.com/. You'll need to create some billing details. You'll also
need to create a project that will contain all our resources. You can name this whatever you want.

When working with cloud providers as an individual, it can be helpful to create billing limits in order to prevent
accidental cost overrun. While providers are used to people accidentally running up a bill that needs to be written off,
you don't want to rely on their good will. You can create a budget by going to billing > budgets & alerts and create
a global budget for all of your projects.

You'll also likely need to increase the global GPU quota for your account, which is set to 0 by default. This can be
done by going to iam & admin > quotas. Filter for "GPU" and select the quota named "GPUs (all regions)" and click edit
quotas. Increase the quota to one and provide a brief description of what your intended use is.

There are several ways to manage accessing created resources. I use SSH, which requires adding a project wide ssh
public key. GCP recommends against this, but it's totally fine for personal projects.

## Project dependencies (Windows)

Install the following tools using Scoop:

```shell
scoop install gcloud pulumi nodejs
```

One `gcloud` is installed, you'll need to run `gcloud init` and `gcloud auth application-default login`. Credentials
can also be managed by providing a raw credential string, but it's easier to let the `gcloud` CLI manage authentication
for us. Using credentials as secrets can be helpful if you want to set up automated deployments using CI on GitHub,
which isn't necessary for most art projects. Additionally, we're not using a service account, which is generally a
best practice to isolate permissions from our main admin account. If you end up creating many projects, you may want
to explore using a service account.

Next, create a new project using `pulumi`. This should be run inside the folder you're intending to use:

```shell
pulumi new gcp-typescript
```

This will use the `gcp-typescript` template to create a new project from the template. As part of setup, it will
ask you what gcp project you want to use. Provide the unique project ID from the cloud console rather than just the
project name.

## Create the stack

Next, run Pulumi in order to create [our new stack as defined in index.ts](./index.ts):

```shell
pulumi up
```

Select "yes" and if everything is set up correctly, you should see a few resources being created. In order to view
the IP of our new compute VM, you can use `pulumi stack output instanceIP`.

You can always tear down your project by using `pulumi destroy`.

To confirm that everything was creating, use `gcloud`:

```shell
gcloud compute instances list
```

Use either `gcloud` or `ssh` to access your instance:
```shell
gcloud compute ssh $(pulumi stack output instanceName) 
# or
ssh $YOUR_SSH_PUB_USER@$(pulumi stack output instanceIP)
```

## Install required dependencies

The directory [startup-scripts](./setup-scripts) has steps for installing the dependencies necessary for running
stylegan2 training. Additional packages may be needed as necessary.

## Workflow

Make sure your instance is started:
```shell
gcloud compute instances start $(pulumi stack output instanceName) 
```

SSH in and do some work:
```shell
gcloud compute ssh $(pulumi stack output instanceName) 
```

Shutdown instance when done (will still be billed for disk):
```shell
gcloud compute instances stop $(pulumi stack output instanceName) 
```


