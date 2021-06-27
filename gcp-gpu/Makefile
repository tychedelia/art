pulumi_instance_var:=instanceName
instance_name:=$(shell pulumi stack output $(pulumi_instance_var))

.PHONY: pulumi-up
pulumi-up:
	pulumi up --yes

.PHONY: pulumi-destroy
pulumi-destroy:
	pulumi destroy --yes

.PHONY: gcloud-ssh
gcloud-ssh:
	gcloud compute ssh $(instance_name)

.PHONY: gcloud-start
gcloud-start:
	gcloud compute instances start $(instance_name)

.PHONY: gcloud-stop
gcloud-stop:
	gcloud compute instances start $(instance_name)

.PHONY: gcloud-scp-push
gcloud-scp-push:
	 $(if $(from),,$(error Must set from))
	 $(if $(to),,$(error Must set to))
	gcloud compute scp $(from) $(instance_name):$(to)

.PHONY: gcloud-scp-pull
gcloud-scp-pull:
	 $(if $(from),,$(error Must set from))
	 $(if $(to),,$(error Must set to))
	gcloud compute scp --recurse $(instance_name):$(from) $(to)