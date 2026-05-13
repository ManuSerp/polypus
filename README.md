#  — Polypus

Small CLI tool to monitor self-hosted services in my Home Lab server.
Most services are deployed from a docker compose file, so this CLI (for the moment only docker compose) registers those services and allows easy monitoring.

A service that Polypus monitors can be composed of multiple containers (services in the docker compose file), but it is registered as one logical service. The CLI will monitor all containers and determine the overall service status.

To register: `polypus register {name_of_your_service}`

Then: `polypus status` to see the registered services and their status.


---

## What is done

* [x] Build CLI-only version first


* [x] Register services manually

* [x] A service = one deployed application

* [x] Initially based on one `docker-compose.yml`

* [x] List registered services

* [x] Show global service overview and statuses, sync call

* [X] monitor service containers to determine status 

---

## Futur improvement
 

* [ ] register folder config option


* [ ] Service-level monitoring (not container-level)
* [ ] async check might be better

* [ ] Do not rely only on Docker healthcheck

* [ ] Combine:

  * container running state
  * Docker healthcheck
  * container failures
  * restart loops
  * missing containers

* [ ] Future support for:

  * custom health checks
  * HTTP endpoint checks
  * DB connectivity checks


* [ ] Quick CLI access to:

  * container logs
  * recent failures
  * restart history


* [ ] Service commands:

  * start
  * stop
  * restart

* [ ] Restart outdated services only

* [ ] Add `--dry-run`

Examples:

```bash
polypus restart --dry-run
polypus sync --dry-run
```

Should show:

* what will restart
* why
* which files changed

 ### — Deployment Drift Detection

* [ ] Compare deployed state vs local `docker-compose.yml`

* [ ] Detect differences between:

  * current deployment
  * local compose definition

* [ ] Mark service as:

  * outdated
  * healthy but outdated

* [ ] CLI command to sync outdated services





## far away futur

* [ ] Store state locally with SQLite

Store:

* registered services

* tracked paths

* file hashes

* last known states

* restart policies

* auto-restart rules

* daemon config

* action history

* failure history

* [ ] Avoid JSON-based persistence

SQLite as default storage backend


* [ ] Safe restart flow:

  * restart service
  * validate health after restart

If restart fails:

* alert user first

Later:

* investigate rollback feasibility

Important constraints:

* volumes
* DB migrations
* removed images
* dependency order
* multi-container services

Do not implement full rollback too early
