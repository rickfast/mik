# mik

Mik is a simple tool that renders mustache template files on the file system using environment variables as the model. The primary use case for Mik is to easily containerize applications that require configuration files. Mik allows you to create the configuration file using `-e` with `docker run`.

## Running mik

Mik only takes two options, both optional. The first is `-f` for the target file or directory to render. If the argument to `-f` is a single file, then only the single file will be rendered. If the argument is a directory, then mik will recurse the directory hierarchy and render all files.

The second option is `-t` which allows you to specify a file type (extension) to render. This option can be used in conjunction with `-f` to specify the file extension to render. This will cause all other file types to be ignored.

Sample:

File: `hello.hcl`

```hcl
hello {
  home_dir: {{ HOME }}
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
