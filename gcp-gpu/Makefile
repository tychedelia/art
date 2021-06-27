.PHONY: pulumi-up
pulumi-up:
	pulumi up --yes

.PHONY: pulumi-destroy
pulumi-destroy:
	pulumi destroy --yes

.PHONY: gcloud-ssh
gcloud-ssh:
	gcloud compute ssh $$(pulumi stack output instanceName)

.PHONY: gcloud-start
gcloud-start:
	gcloud compute instances start $$(pulumi stack output instanceName)

.PHONY: gcloud-start
gcloud-start:
	gcloud compute instances start $$(pulumi stack output instanceName)

