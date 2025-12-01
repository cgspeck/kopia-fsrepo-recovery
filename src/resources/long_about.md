# kopia-fsrepo-recovery

Tool to automate recovery of a kopia filesystem repository from a backup/synced version of the repository.

Run a snapshot verify on the defective primary repository, allow errors and capture logs.

```shell
time sudo kopia snapshot verify --verify-files-percent=100 --file-parallelism=10 --parallel=10 --max-errors=999999999 2>&1 | tee kopia-snapshot-verify-log.txt
```

Run `extract-from-log` to extract list of missing blobs:

```shell
kopia-fsrepo-recovery extract-from-log kopia-snapshot-verify-log.txt
```

It will create a file called `missing-blobs.json` and warn if there were any errors not associated with missing blobs.
