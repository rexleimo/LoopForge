# Config Health Check

**Goal:** verify config + doctor status and produce an actionable health note.

## Run

1) Execute health commands:

=== "macOS/Linux"
    ```bash
    loopforge config validate | tee notes/config-validate.txt
    loopforge doctor | tee notes/doctor.txt
    ```

=== "Windows (PowerShell)"
    ```powershell
    loopforge config validate | Tee-Object -FilePath notes/config-validate.txt
    loopforge doctor | Tee-Object -FilePath notes/doctor.txt
    ```

2) Generate summary note:

=== "macOS/Linux"
    ```bash
    loopforge agent run --workspace . --prompt "Read notes/config-validate.txt and notes/doctor.txt. Write notes/config-health-check.md with: 1) current status 2) blocking errors 3) non-blocking warnings 4) exact next commands to fix top 3 issues."
    ```

=== "Windows (PowerShell)"
    ```powershell
    loopforge agent run --workspace . --prompt "Read notes/config-validate.txt and notes/doctor.txt. Write notes/config-health-check.md with: 1) current status 2) blocking errors 3) non-blocking warnings 4) exact next commands to fix top 3 issues."
    ```

## What to expect

- `notes/config-validate.txt`
- `notes/doctor.txt`
- `notes/config-health-check.md`
