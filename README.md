# simple-scoreboard
A simple scoreboard for CTF and workshop

## Quick start
### Install these:
```
git
docker
docker-buildx
```

### Run this:
```
git clone https://github.com/zuyoutoki/simple-scoreboard
cd simple-scoreboard
docker buildx build --tag simple-scoreboard:latest .
```

### Edit `db/init.sql` to add your flags
This needs to be valid SQL. Some flags are already there to show the syntax.

### Run this:
```
docker run --rm -p 8000:8000 -v ./db:/app/db simple-scoreboard:latest
```

### :tada: You got a `simple-scoreboard` running on port 8000 :tada:
