# mik

[![Build Status](https://travis-ci.org/rickfast/mik.svg?branch=master)](https://travis-ci.org/rickfast/mik)

Mik is a simple tool that renders mustache template files on the file system using environment variables as the model. The primary use case for Mik is to easily containerize applications that require configuration files. Mik allows you to create the configuration file using `-e` with `docker run`.

## Get mik

Download the Linux binary here: [https://bintray.com/artifact/download/rickfast/mik/linux_amd64/mik]

## Compiling for Linux using Docker

You can build Mik for linux (for instance on your Mac) using a dockerized build environment.  Here's how:

```
docker run -it --rm -v $(pwd):/source jimmycuadra/rust cargo build
```

The resulting binary lives at: `target/debug/mik`

Add the `--release` flag to create an optimized version for release.  Its also smaller :)

```
docker run -it --rm -v $(pwd):/source jimmycuadra/rust cargo build --release
```

This optimized version can be found at `target/release/mik`

## Running mik

Mik only takes two options, both optional. The first is `-f` for the target file or directory to render. If the argument to `-f` is a single file, then only the single file will be rendered. If the argument is a directory, then mik will recurse the directory hierarchy and render all files.

The second option is `-t` which allows you to specify a file type (extension) to render. This option can be used in conjunction with `-f` to specify the file extension to render. This will cause all other file types to be ignored.

Sample:

File: `hello.hcl`

```hcl
hello {
  home_dir: {{ HOME_DIR }}
}
```

To render the mustache template with environment variables, run:

```sh
$ mik -f hello.hcl
```

Will produce:

File: `hello.hcl`

```hcl
hello {
  home_dir: /Users/rickfast
}
```

## Using in Docker

File `entrypoint.sh`:

```sh
#! /usr/bin/env bash
mik -f hello.hcl
exec your-program -config hello.hcl
```

File `hello.hcl`:

```hcl
hello {
  some_value: {{ SOME_VALUE }}
}
```

File `Dockerfile`:

```Dockerfile
FROM alpine
RUN some crap here
ENTRYPOINT ["entrypoint.sh"]
```

To run the image:

```sh
$ docker build -t my-image .
$ docker run -e SOME_VALUE=foo my-image
```

# Development

* Github: [https://github.com/rickfast/mik](https://github.com/rickfast/mik)
* Slack: [mik-template.slack.com](https://mik-template.slack.com/)
* Bintray: [https://bintray.com/rickfast/mik/mik/view](https://bintray.com/rickfast/mik/mik/view)