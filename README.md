# RustB
Just some Rust code demo using docker.

# Run
`source goin.sh`

# How to use
```bash
docker run -it --rm --volume $(pwd):/src --volume $(pwd)/bash_profile:/.bash_profile rust bash 
cd /src
```

# View binary details
- On Mac
	otool -L binary
	strings main | head -1

- On Debian 9 (Linux)
	ldd binary

# More
[Link](https://bxbrenden.github.io/)
