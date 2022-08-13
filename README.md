# Timestampr

Tool for writing timestamps to a file to make video editing easier.

Timestampr appends a timestamp, duration pair to `~/Documents/timestamps.tsv`
each time it is run. The duration is is the time from the last "start" entry,
which is an entry with a duration of 00:00:00. Start entries can be created by
calling Timestampr with `start`.

Each time an entry is added a notification is shown via `notify-send`.
