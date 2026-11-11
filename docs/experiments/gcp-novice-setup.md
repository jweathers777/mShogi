# Set up mShogi on Google Cloud: a step-by-step guide

Prepared for John, 2026-11-11. Project **mShogi**, project ID **mshogi**, project
number **216675633801**. Billing is already linked. No experiment resources have
been created by the runner yet.

These instructions configure the project; they do **not** start experiments.
You can stop after any numbered step. Tell Codex which step failed and its error
message, without pasting passwords, access tokens or private keys.

## 1. Understand the proposed region and pilot cost

Use **Iowa (`us-central1`)**. Latency to your computer is unimportant for these
unattended jobs. C3 machines are offered in multiple Iowa zones, so the scheduler
can choose a zone within that region. This is published availability, not a promise
of immediate spare capacity; the pilot will check actual quota/capacity.
[Google's region table](https://docs.cloud.google.com/compute/docs/regions-zones).

The initial worker is **c3-standard-4**, an on-demand Linux VM with four vCPUs and
16 GiB RAM. Google's Iowa list price checked today is **$0.201608 per VM-hour**,
about **$1.61/hour for eight simultaneous workers**, before disk, external IP,
storage, logs and network charges. This is VM pricing, not Cloud Build pricing.
[Official VM price table](https://cloud.google.com/products/compute/pricing/general-purpose?hl=en).

Proposed approval: **US$30 for the pilot**, not the full 200-game study. First run
one pair and its analysis, review it, then run eight pairs concurrently. With a
six-hour per-task limit and at most one retry, these two submissions allow at most
108 active task-hours: about **$21.77 of VM compute** before startup/cleanup and
other charges. Most successful jobs should use less, but this has not been measured
on GCP. Stop and ask before more submissions or a larger budget. A timeout records
an incomplete job; it never shortens a game into a valid result.

Budget alerts are **not spending caps** and can be delayed. The runner's limited
job count, worker count, retries and runtime prevent an open-ended experiment, but
there is no promise of an exact hard billing cutoff. Storage persists after jobs
finish. We will report measured costs separately from this estimate.

**Do not start a Batch job or upload the worker image until you explicitly approve
the pilot spending.** Configuration below creates small empty resources; data
storage, image uploads, Pub/Sub traffic and running VMs are the chargeable activity.

## 2. Open the correct project and Cloud Shell

1. Open [the mShogi Cloud Console](https://console.cloud.google.com/home/dashboard?project=mshogi).
2. Check the project selector at the top says **mShogi**. Its ID must be `mshogi`.
3. Click the terminal icon near the top right (**Activate Cloud Shell**).
4. If prompted, authorize Cloud Shell to use your signed-in Google account.
5. Paste this command and press Enter:

```sh
gcloud config set project mshogi
```

Cloud Shell is Google's browser terminal; you do not need to install software on
your Mac for these setup commands. Run each block separately and wait for it to
finish. Do not paste the Markdown triple-backtick lines.

## 3. Add a project-specific budget alert

1. Open [Billing](https://console.cloud.google.com/billing?project=mshogi).
2. Select the billing account already linked to this project.
3. Open **Budgets & alerts**, then **Create budget**.
4. Name it `mShogi experiment pilot`.
5. Under scope, select **only the mshogi project**. Keep all services included.
6. Use a monthly specified amount of **US$30** (or the amount you approve).
7. Add actual-spend alerts at **50%, 90% and 100%**, and enable email notifications.
8. Save. Verify the budget's scope still says `mshogi` rather than every project.

This does not limit or change the billing of your other projects. If your account
uses another currency, use the corresponding approved amount and tell Codex.

## 4. Enable the required Google APIs

Paste into Cloud Shell:

```sh
gcloud services enable compute.googleapis.com batch.googleapis.com artifactregistry.googleapis.com pubsub.googleapis.com storage.googleapis.com logging.googleapis.com --project=mshogi
```

Wait for `Operation ... finished successfully`. This enables capabilities; it does
not create or start a VM. Verify:

```sh
gcloud services list --enabled --project=mshogi --filter='config.name:(compute.googleapis.com batch.googleapis.com artifactregistry.googleapis.com pubsub.googleapis.com storage.googleapis.com logging.googleapis.com)' --format='value(config.name)'
```

The six service names should appear. If Google says you need permission, stop and
send the error rather than granting a broad role or changing organization policy.

## 5. Check CPU quota before creating jobs

1. Open [Quotas & System Limits](https://console.cloud.google.com/iam-admin/quotas?project=mshogi).
2. Filter **Service = Compute Engine API**, **Region = us-central1**.
3. Find the quota for **C3 CPUs** and any applicable overall regional CPU quota.
4. Eight workers need **32 vCPUs**. If the available quota is below 32, request 32,
   describing the use as “eight bounded on-demand Batch workers for chess experiments”.
5. Do not request 64 yet; sixteen workers is a later decision based on measurements.

Quota approval does not incur VM charges. Quota is permission to allocate, not a
reservation or guarantee of capacity. We can validate one worker while an increase
is pending if at least four C3 vCPUs are available.

## 6. Create a worker service account

A service account is the identity the experiment VMs use. It does not need a
password or downloadable key.

```sh
gcloud iam service-accounts create mshogi-batch-worker --display-name='mShogi Batch worker' --project=mshogi
```

Grant the two operational roles it needs:

```sh
gcloud projects add-iam-policy-binding mshogi --member='serviceAccount:mshogi-batch-worker@mshogi.iam.gserviceaccount.com' --role='roles/batch.agentReporter'
gcloud projects add-iam-policy-binding mshogi --member='serviceAccount:mshogi-batch-worker@mshogi.iam.gserviceaccount.com' --role='roles/logging.logWriter'
```

Do **not** grant Owner or Editor to the worker. Do **not** create a service-account
key. The Batch service agent is a separate Google-managed identity; the API manages
its normal service-agent role.

## 7. Create the evidence bucket and its permissions

The project number makes this bucket name unlikely to conflict with another owner:

```sh
gcloud storage buckets create gs://mshogi-experiments-216675633801 --project=mshogi --location=us-central1 --uniform-bucket-level-access --public-access-prevention
```

Grant read/list and create-only access for evidence:

```sh
gcloud storage buckets add-iam-policy-binding gs://mshogi-experiments-216675633801 --member='serviceAccount:mshogi-batch-worker@mshogi.iam.gserviceaccount.com' --role='roles/storage.objectViewer'
gcloud storage buckets add-iam-policy-binding gs://mshogi-experiments-216675633801 --member='serviceAccount:mshogi-batch-worker@mshogi.iam.gserviceaccount.com' --role='roles/storage.objectCreator'
```

The worker also needs to release its execution locks. Restrict delete/update rights
to objects ending in `/claim.json`; successful results do not have that name:

```sh
gcloud storage buckets add-iam-policy-binding gs://mshogi-experiments-216675633801 --member='serviceAccount:mshogi-batch-worker@mshogi.iam.gserviceaccount.com' --role='roles/storage.objectAdmin' --condition="expression=resource.name.endsWith('/claim.json'),title=execution-claims-only"
```

Do not make the bucket public, add a retention lock, or add automatic evidence
deletion. The runner writes immutable attempts and uses conditional object creation.
If the name already exists and is not yours, stop and tell Codex; do not reuse an
unknown bucket.

## 8. Create the private image repository

This is where the fixed Linux worker image will be stored:

```sh
gcloud artifacts repositories create mshogi-experiments --repository-format=docker --location=us-central1 --description='Fixed mShogi experiment workers' --project=mshogi
gcloud artifacts repositories add-iam-policy-binding mshogi-experiments --location=us-central1 --project=mshogi --member='serviceAccount:mshogi-batch-worker@mshogi.iam.gserviceaccount.com' --role='roles/artifactregistry.reader'
```

Codex will prepare and verify the image. You do not need to build it manually or
upload arbitrary engine executables. Image uploads wait for spending approval.

## 9. Create a network with no incoming access

```sh
gcloud compute networks create mshogi-batch --subnet-mode=custom --project=mshogi
gcloud compute networks subnets create mshogi-batch-us-central1 --network=mshogi-batch --region=us-central1 --range=10.42.0.0/24 --enable-private-ip-google-access --project=mshogi
```

Do not add SSH, HTTP or other incoming firewall rules. These workers only initiate
outbound connections to Google APIs and the image repository. The initial design
uses temporary external IP addresses for outbound access (a small additional
charge while allocated), avoiding a separate Cloud NAT service. No static IPs,
load balancer, website or public server are needed.

## 10. Create completion notifications

This wakes the owning Codex chat when a job finishes instead of polling it repeatedly:

```sh
gcloud pubsub topics create mshogi-batch-events --project=mshogi
gcloud pubsub subscriptions create mshogi-pilot-completion --topic=mshogi-batch-events --ack-deadline=60 --message-retention-duration=7d --project=mshogi
gcloud pubsub topics add-iam-policy-binding mshogi-batch-events --project=mshogi --member='serviceAccount:mshogi-batch-worker@mshogi.iam.gserviceaccount.com' --role='roles/pubsub.publisher'
```

This subscription belongs to one completion consumer at a time. Codex will attach
the local listener before leaving a submitted job unattended. The topic/subscription
must exist **before** submission, so events survive a listener restart. You do not
need to configure an HTTP endpoint or expose your Mac to the internet.

## 11. Authorize local access on your Mac

The `gcloud` CLI on this Mac already lists `mshogi`. The Python storage/subscriber
clients also need **Application Default Credentials**, a separate local login.
In your Mac Terminal (not Cloud Shell), run:

```sh
gcloud auth application-default login
gcloud auth application-default set-quota-project mshogi
gcloud auth configure-docker us-central1-docker.pkg.dev
```

Complete the browser sign-in yourself using the account that owns this project.
Accept Docker credential-helper configuration for this registry. Do not paste the
resulting credential file into chat. If a command is unavailable or a permission
prompt is unclear, tell Codex; do not download a service-account key as a workaround.

## 12. Tell Codex you are ready

Reply with the last successful step and whether you approve the **US$30 pilot**.
If quotas are pending, include that fact. You do not need to send credentials.
Codex will verify configuration read-only, freeze the exact image/build, and ask
about any material difference before a billable launch.

The prepared configuration is [mshogi-pilot-config.json](mshogi-pilot-config.json).
The initial pilot is one complete pair with replay, screening and confirmations,
then an eight-worker run after the first result is reviewed. The full 200-game
study, sixteen workers and further spending are **not** authorized by following
this setup guide. The MSH-161 coordinator must review validation before the main
study starts.

## If you need to stop a running pilot

Tell Codex to cancel it. A normal stop finishes the current job and starts no next
job. For an urgent stop, open [Batch jobs](https://console.cloud.google.com/batch/jobs?project=mshogi),
select the job and use its delete/cancel action. Wait for the operation to finish;
requesting deletion alone is not proof all VMs have stopped. Then check
[VM instances](https://console.cloud.google.com/compute/instances?project=mshogi)
for Batch-created instances. Do not delete the evidence bucket or image repository
as a way to stop computation. Billing and usage records can arrive later.

For the developer commands and recovery rules, see
[the runner operating guide](parallel-runner.md).
