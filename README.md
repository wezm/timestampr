# Timestampr

Tool for writing timestamps to a file to make video editing easier.

Timestampr appends a timestamp, duration pair to `~/Documents/timestamps.tsv`
each time it is run. The duration is the time from the last "start" entry,
which is an entry with a duration of `00:00:00`. Start entries can be created
by calling Timestampr with `start`.

Each time an entry is added a notification is shown via `notify-send`.

## Sample Output

```tsv
Sat, 13 Aug 2022 15:04:32 +1000	00:00:00
Sat, 13 Aug 2022 16:05:03 +1000	01:00:31
Sat, 13 Aug 2022 16:08:05 +1000	00:00:00
Sat, 13 Aug 2022 16:10:18 +1000	00:02:13
Sat, 13 Aug 2022 16:11:35 +1000	00:03:30
```

Timestamps are in RFC2822 format. Durations are `hh:mm::ss`.

## Usage

```
Usage: timestampr [cmd]

Writes a tab-separated timestamp and duration to ~/Documents/timestamps.tsv
A notification is shown when an entry is added successfully via `notify-send`.

COMMANDS:

start      Add a new entry with the duration set to 00:00:00

[default]  If no command is supplied the default behaviour is to append a
           new timestamp to the file with the duration since the start
           entry. I.e. the last entry with duration 00:00:00

FILES:

        ~/Documents/timestamps.tsv      Where the records are written
```

