# Set up mShogi on Google Cloud

Use this guide to configure the project. These steps do **not** start an experiment.

- Project name: **mShogi**
- Project ID: **mshogi**
- Project number: **216675633801**
- Date: **2026-11-11**
- Billing: connected to the project
- Pilot budget: **US$30 approved by John**

The runner has not created cloud resources. You can stop after each numbered step.
If a step fails, give Codex the step number and the error message.
Do not send passwords, access tokens or private keys.

## 1. Read the region and cost information

Use **Iowa (`us-central1`)**. These jobs operate without your input.
The network delay between Iowa and your computer is not important for this work.

C3 machines are available in more than one Iowa zone. Google Batch can select a zone in this region.
The pilot must confirm that Google has sufficient machines available.
See [Google's region table](https://docs.cloud.google.com/compute/docs/regions-zones).

Each worker uses one **c3-standard-4** Linux virtual machine (VM).
Each VM has four virtual CPUs (vCPUs) and 16 GiB of memory.

The Iowa price checked on 2026-11-11 was **US$0.201608 per VM-hour**.
Eight workers cost approximately **US$1.61 per hour** for VM operation.
Disk, external IP address, storage, log and network charges are additional.
See the [official VM price table](https://cloud.google.com/products/compute/pricing/general-purpose?hl=en).
This price is for VMs. Cloud Build has different prices.

John approved **US$30 for the pilot**. This approval does not include the full 200-game study.
The pilot sequence is:

1. Run one opening pair and its analysis.
2. Review the results.
3. Run eight opening pairs at the same time.

Each task has a six-hour time limit and a maximum of one retry.
Together, the two pilot submissions permit a maximum of 108 active task-hours.
The estimated VM cost for those hours is **US$21.77**.
VM start and cleanup time, and the other charges listed above, are additional.
Actual GCP costs are not yet measured.

Codex must stop and ask before additional submissions or an increased budget.
If a task exceeds its time limit, the runner records an incomplete job.
An incomplete game is not a valid result.

**Budget alerts do not stop spending.** Alerts can arrive late.
The runner limits task count, worker count, retries and operating time.
These limits do not provide an exact billing cutoff.
Stored data can cause charges after the jobs stop.
Codex will report actual costs separately from estimates.

**Complete and verify the setup before you upload the worker image or start a Batch job.**
The steps below create empty resources.
Data storage, image uploads, Pub/Sub messages and VM operation can cause charges.

## 2. Open the project and Cloud Shell

1. Open [the mShogi Cloud Console](https://console.cloud.google.com/home/dashboard?project=mshogi).
2. Make sure the project selector shows **mShogi**.
3. Make sure the project ID is `mshogi`.
4. Click the terminal icon near the top right. Its label is **Activate Cloud Shell**.
5. If Google asks, give Cloud Shell permission to use your signed-in account.
6. Paste the command below. Press Enter.

```sh
gcloud config set project mshogi
```

Cloud Shell is a terminal in your browser.
You do not need to install software on your Mac to use these setup commands.
Run one command block at a time. Wait for it to finish before you continue.
Do not copy the lines with three backtick characters around a command block.

## 3. Add a budget alert for this project

1. Open [Billing](https://console.cloud.google.com/billing?project=mshogi).
2. Select the billing account connected to this project.
3. Open **Budgets & alerts**.
4. Select **Create budget**.
5. Enter the name `mShogi experiment pilot`.
6. Under scope, select **only the mshogi project**. Include all services.
7. Set the monthly amount to **US$30**.
8. Add actual-spend alerts at **50%, 90% and 100%**.
9. Enable email notifications.
10. Save the budget.
11. Make sure the budget applies only to `mshogi`.

This budget does not change billing for your other projects.
If the account uses a different currency, use the equivalent approved amount. Tell Codex which currency you used.

## 4. Enable the required Google APIs

An API lets software use a Google service.
Paste this command into Cloud Shell:

```sh
gcloud services enable compute.googleapis.com batch.googleapis.com artifactregistry.googleapis.com pubsub.googleapis.com storage.googleapis.com logging.googleapis.com --project=mshogi
```

Wait for `Operation ... finished successfully`.
This command enables the services. It does not create or start a VM.
Run this command to check the result:

```sh
gcloud services list --enabled --project=mshogi --filter='config.name:(compute.googleapis.com batch.googleapis.com artifactregistry.googleapis.com pubsub.googleapis.com storage.googleapis.com logging.googleapis.com)' --format='value(config.name)'
```

Make sure the output contains all six service names.
If Google reports a permission error, stop. Send the error message to Codex.
Do not add more access rights or change organization policy to bypass the error.

## 5. Check the CPU quota

A quota is the maximum resource amount that Google permits for the project.

1. Open [Quotas & System Limits](https://console.cloud.google.com/iam-admin/quotas?project=mshogi).
2. Set the service filter to **Compute Engine API**.
3. Set the region filter to **us-central1**.
4. Find **C3 CPUs**. Also check any total CPU quota for this region.
5. Make sure the available quota is at least **32 vCPUs** for eight workers.
6. If the quota is lower, request 32 vCPUs.
7. Use this description: “Eight on-demand Batch workers with operating limits for chess experiments.”

Do not request 64 vCPUs. Sixteen workers require a later decision based on pilot measurements.

Quota approval does not cause VM charges.
Quota approval does not reserve machines or guarantee that machines are available.
We can test one worker while a quota increase is pending, if at least four C3 vCPUs are available.

## 6. Create the worker service account

A service account is an identity for software.
The experiment VMs use this identity to access Google services.
No password or downloaded key is necessary.

Create the account:

```sh
gcloud iam service-accounts create mshogi-batch-worker --display-name='mShogi Batch worker' --project=mshogi
```

Give the account these two roles:

```sh
gcloud projects add-iam-policy-binding mshogi --member='serviceAccount:mshogi-batch-worker@mshogi.iam.gserviceaccount.com' --role='roles/batch.agentReporter'
gcloud projects add-iam-policy-binding mshogi --member='serviceAccount:mshogi-batch-worker@mshogi.iam.gserviceaccount.com' --role='roles/logging.logWriter'
```

Do **not** give the worker the Owner or Editor role.
Do **not** create a service-account key.

The Batch service agent is a different identity. Google manages that identity and its normal role.

## 7. Create the evidence bucket

A bucket stores files in Google Cloud Storage.
The name below includes your project number to help prevent name conflicts.

Create the bucket:

```sh
gcloud storage buckets create gs://mshogi-experiments-216675633801 --project=mshogi --location=us-central1 --uniform-bucket-level-access --public-access-prevention
```

Give the worker permission to read, list and create evidence files:

```sh
gcloud storage buckets add-iam-policy-binding gs://mshogi-experiments-216675633801 --member='serviceAccount:mshogi-batch-worker@mshogi.iam.gserviceaccount.com' --role='roles/storage.objectViewer'
gcloud storage buckets add-iam-policy-binding gs://mshogi-experiments-216675633801 --member='serviceAccount:mshogi-batch-worker@mshogi.iam.gserviceaccount.com' --role='roles/storage.objectCreator'
```

The worker uses claim files to prevent duplicate job execution.
It must be able to remove its claim when a job ends.
Give delete and update permission only for files with names that end in `/claim.json`:

```sh
gcloud storage buckets add-iam-policy-binding gs://mshogi-experiments-216675633801 --member='serviceAccount:mshogi-batch-worker@mshogi.iam.gserviceaccount.com' --role='roles/storage.objectAdmin' --condition="expression=resource.name.endsWith('/claim.json'),title=execution-claims-only"
```

Successful result files do not have this name.
The runner does not replace successful results. It checks that an object does not exist before it creates that object.

Do not make the bucket public.
Do not add a retention lock or automatic evidence deletion.
If the bucket name is already in use by another owner, stop and tell Codex.
Do not use a bucket that you do not own.

## 8. Create the private image repository

A worker image contains the Linux software and engine executables.
The image repository stores that image.

Create the repository and give the worker permission to read it:

```sh
gcloud artifacts repositories create mshogi-experiments --repository-format=docker --location=us-central1 --description='Fixed mShogi experiment workers' --project=mshogi
gcloud artifacts repositories add-iam-policy-binding mshogi-experiments --location=us-central1 --project=mshogi --member='serviceAccount:mshogi-batch-worker@mshogi.iam.gserviceaccount.com' --role='roles/artifactregistry.reader'
```

Codex has built and checked the image locally.
You do not need to build the image or upload engine executables.
Codex will upload the checked image after the project setup is complete and verified.
The upload is part of the approved pilot budget.

## 9. Create the network

Create the network and its subnet:

```sh
gcloud compute networks create mshogi-batch --subnet-mode=custom --project=mshogi
gcloud compute networks subnets create mshogi-batch-us-central1 --network=mshogi-batch --region=us-central1 --range=10.42.0.0/24 --enable-private-ip-google-access --project=mshogi
```

Do not add SSH, HTTP or other incoming firewall rules.
The workers start outgoing connections to Google APIs and the image repository.

The initial setup uses temporary external IP addresses for outgoing access.
These addresses cause an additional charge while allocated.
A separate Cloud NAT service is not necessary.
Static IP addresses, a load balancer, a website and a public server are not necessary.

## 10. Create job-completion notifications

These notifications send an update to the owning Codex chat when a job ends.
Codex does not need to check the job repeatedly.

Create the topic and subscription. Give the worker permission to send notifications:

```sh
gcloud pubsub topics create mshogi-batch-events --project=mshogi
gcloud pubsub subscriptions create mshogi-pilot-completion --topic=mshogi-batch-events --ack-deadline=60 --message-retention-duration=7d --project=mshogi
gcloud pubsub topics add-iam-policy-binding mshogi-batch-events --project=mshogi --member='serviceAccount:mshogi-batch-worker@mshogi.iam.gserviceaccount.com' --role='roles/pubsub.publisher'
```

A topic receives messages. A subscription holds messages for the software that reads them.
Use this subscription for only one completion listener at a time.
Codex will start the local listener before it leaves a submitted job unattended.

Create the topic and subscription **before** job submission.
Messages then remain available if the listener restarts.
You do not need an HTTP endpoint or incoming internet access to your Mac.

## 11. Give the local tools access

The `gcloud` command on your Mac can already list the `mshogi` project.
The Python storage and notification tools also need **Application Default Credentials**.
These credentials require a separate local login.

Open **Terminal on your Mac**. Do not use Cloud Shell for this step.
Run these commands:

```sh
gcloud auth application-default login
gcloud auth application-default set-quota-project mshogi
gcloud auth configure-docker us-central1-docker.pkg.dev
```

Complete the browser sign-in yourself. Use the Google account that owns this project.
Accept the Docker credential-helper configuration for this registry.

Do not send the credential file to Codex.
If a command is unavailable or a permission request is unclear, tell Codex.
Do not download a service-account key to bypass a problem.

## 12. Tell Codex that setup is complete

Give Codex the number of the last successful step.
Tell Codex if a quota request is still pending.
Do not send credentials.

Your **US$30 pilot approval is already recorded**. You do not need to approve the same scope again.
Codex will check the configuration without changing it.
Codex will then record the exact image and build identities before the pilot starts.
If the required setup has an important difference, Codex will ask you about the change.

The prepared configuration is [mshogi-pilot-config.json](mshogi-pilot-config.json).
The first pilot runs one complete opening pair, native replay, screening and confirmation analysis.
Codex must review those results before the eight-worker run.

The full 200-game study, sixteen workers and additional spending are **not approved** by this guide.
The coordinator for MSH-161 (Competitive Gameplay Diagnosis) must review the pilot evidence before the main study starts.

## Stop a pilot

For a normal stop, tell Codex to cancel the pilot.
The runner finishes the current job and does not start the next job.

For an urgent stop:

1. Open [Batch jobs](https://console.cloud.google.com/batch/jobs?project=mshogi).
2. Select the job.
3. Select its delete or cancel action.
4. Wait for the operation to finish.
5. Check [VM instances](https://console.cloud.google.com/compute/instances?project=mshogi) for instances created by Batch.

A deletion request alone does not prove that all VMs have stopped.
Do not delete the evidence bucket or image repository to stop computation.
Billing and usage records can arrive later.

For developer commands and recovery instructions, see [the runner operating guide](parallel-runner.md).

## Check pilot progress

Open Terminal on your Mac. Run these commands:

```sh
cd /Users/johnweathers/.codex/worktrees/parallel-experiment-runner/mShogi
GOOGLE_CLOUD_PROJECT=mshogi /private/tmp/msh162-controller/bin/python -m tools.experiments.cli quick-status --manifest docs/experiments/pilot/eight-pair-manifest.json --store gs://mshogi-experiments-216675633801/msh161 --job-name msh162-pilot-eight-v1
```

The report shows saved progress and the current Batch state. It does not start or
stop work. It does not check all result files. Confirmation totals can increase
while analysis continues. A saved claim can refer to a running or interrupted job.

The Python path above is the environment prepared for this pilot. If it is removed,
create a Python environment and install `tools/experiments/requirements-controller.txt`.
Use that environment's Python command instead. For a later experiment, change the
manifest path and Batch job name. Add `--json` if you need JSON output.
