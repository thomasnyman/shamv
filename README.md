# `shamv`

The `shamv` ("SHA move") utility renames files, making their names "almost"
unique.  It uses a cryptographic hash function, SHA-256 by default, to generate
a 256-bit (32-byte) digest based on the content of the file that is used to
derive the filename.

Because the cryptographic hash function used to generate the name is
deterministic, if two files have exactly the same content (bit-by-bit), then
their names will also be the same. This way, if you find two files with the same
name, you can determine (with a high-degree of certainty) that they are copies
of the same file.

The `shamv` utility is prototype software for exploring different use case of
files named after their cryptographic hash values using a general purpose
command line interface. It is not intended to be used in production
environments.

One use case for `shamv` is managing screenshots and other attachments imported
into note-taking software such as [Obsidian][1]. Screenshot software provided by
some operating systems may use naming conventions for screenshots that derive
the filename, e.g., from the date the screenshot is taken (`Screenshot
2021-04-22.png`).  This can lead to conflicts with files which have the same
name. While there are [plugins and scripts available for this purpose][2],
`shamv` aims to be a general purpose tool for manipulating files named after
their hash values and uses modern, cryptographic primitives based on the SHA-2
NIST standard.

Another use case for `shamv` is file archival where files indexed by their
cryptographic hash can be immediately located from a filesystem based on their
hash value. A property of `shamv`-named files is that the integrity of the file
contents can be verified using the name of the file. Similarly digitally signing
a manifest of filenames is sufficient to verify not only the list of files, but
their content.

Further suggestions for use cases and feature requests can be submitted as
[Github Issues][3].

## Usage

    shamv [OPTION...] FILE[...]

    The shamv utility renames the file named by the FILE operand to a
    destination path that is formed from the SHA-2 hash of the content
    of the file.

    If the FILE operand includes a filename extension that consists of one or
    more suffixes, each separated by a dot (.) character, the destination path
    is formed by the concatenation of the SHA-2 hash and the filename extension
    suffixes of the original FILE name.

    Mandatory arguments to long options are mandatory for short options too.
     -a, --algorithm      The SHA-2 algorithm to use: sha256 (default) or sha512.
     -n, --dry-run        Display the current and new filenames but do not
                          perform the rename.
     -h, --help           Print this help and exit.
     -V, --version        Print the version of the program and exit.

## License

The code of `shamv` is made available under a dual-license. This means that you
can decide which license you wish to use when using the software. The two
options are:

 - a) You can use the [Apache License, Version 2.0][4] or any later version
      published by the Apache Software Foundation

 - b) You can use the [MIT license][5] published by the Massachusetts Institute
      of Technology.

[1]: https://obsidian.md/
[2]: https://forum.obsidian.md/t/17141
[3]: https://github.com/thomasnyman/shamv/issues
[4]: http://www.apache.org/licenses/LICENSE-2.0
[5]: http://opensource.org/licenses/MIT
