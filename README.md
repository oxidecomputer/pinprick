# pinprick

This is a trivial wrapper around the `miniz_oxide` library that
compresses data to stdout using the DEFLATE algorithm.  The
output of this program is suitable for inflating in `bldb`,
`nanobl-rs` or `phbl`.

## Usage

`pinprick [file] > output`

If `file` is specified, data to be compressed will be read from
the named file, otherwise, standard input is read to EOF.  The
program writes to standard output unconditionally.

## Colophon

It takes but a pinprick to deflate a balloon.

## License

MPL 2.0
