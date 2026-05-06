## TODO — Polypus

Service monitor & deployment drift detection for home server Docker services

---

## Phase 1 — CLI Core

* [x] Build CLI-only version first

* [x] No daemon at startup

* [x] Register services manually
  * need to update container id automatically

* [x] A service = one deployed application

* [x] Initially based on one `docker-compose.yml`

* [x] List registered services

* [x] Show global service overview and statuses, sync call

---

## Phase 2 — Service Health & Status

* [ ] Service-level monitoring (not container-level)
* [ ] async check for future daemon

* [ ] Detect and display:

  * running
  * stopped
  * healthy
  * degraded
  * unhealthy
  * unknown
  * outdated
  * healthy but outdated

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

---

## Phase 3 — Detailed Diagnostics

* [ ] Show containers inside each service

* [ ] Identify:

  * which container is failing
  * why it is failing

* [ ] Quick CLI access to:

  * container logs
  * recent failures
  * restart history

---

## Phase 4 — Service Actions

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

---

## Phase 5 — Deployment Drift Detection

* [ ] Compare deployed state vs local `docker-compose.yml`

* [ ] Detect differences between:

  * current deployment
  * local compose definition

* [ ] Mark service as:

  * outdated
  * healthy but outdated

* [ ] CLI command to sync outdated services

---

## Phase 6 — Directory Registry

* [ ] Register a full project directory instead of only `docker-compose.yml`

Track:

* root folder

* subfolders

* config files

* mounted files

* `.env`

* reverse proxy configs

* service-specific configuration

* [ ] If tracked files change:

  * mark service as outdated

---

## Phase 7 — Change Detection Engine

* [ ] Use content hash
* [ ] Do NOT rely on `mtime`

Use hashes for:

* compose files
* config files
* mounted files
* environment files

Avoid:

* false positives
* false negatives
* git checkout issues
* backup restore issues

---

## Phase 8 — Persistent State Storage

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

---

## Phase 9 — Safe Restart Strategy

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

---

## Phase 10 — Standalone Daemon

* [ ] Separate standalone daemon

Daemon responsibilities:

* monitor continuously
* detect failures
* detect outdated services

Optional actions:

* auto-restart unhealthy services

* auto-restart healthy but outdated services

* [ ] Fully configurable from CLI

---

## Phase 11 — Optional API Layer

* [ ] Daemon can optionally expose an API

Use cases:

* remote visualization
* dashboard integration
* external monitoring
* automation hooks

---

## Future Ideas

* [ ] Notifications (Discord / Email / Telegram)
* [ ] Web dashboard
* [ ] Service dependency graph
* [ ] Failure metrics/history
* [ ] Deployment history
* [ ] Backup integration
* [ ] Remote node support
* [ ] Multi-host monitoring
