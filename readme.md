# bucky

bucky is the result of developing paranoia of my workstation dying in front of me. So I wanted to have a continuously running backup to my server via ssh.
"The" way to do it is to use `rsync (1)` for this. I started with `crontab (1)` to schedule the backup, but it quickly became cumbersome, so bucky is an attempt to provide a unified, sane way of configuring, running and monitoring my backups.

## config (draft)
```json
{
   "interval" : 60,
   "backups" : [
      {
         "dst" : "/bar/baz",
         "src" : "/foo/bar"
         }
   ]
}
```
