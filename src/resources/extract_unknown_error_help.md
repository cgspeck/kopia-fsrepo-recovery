# Unknown Errors Found

The log has the following unknown errors:

----
```
{}
```
----

You need to investigate the unknown errors listed above or use the --continue-on-unknown-error argument.

The following commands may help to identify what blobs belong to what object files:

```shell
kopia show <object_id>
kopia snapshot list --path /path/to/source 
kopia blob show <blob_id>
```
