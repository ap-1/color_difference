# color_difference

Command line tool to calculate color differences on local images using the CIEDE2000 algorithm

## Usage

```md
USAGE:
    color_difference [OPTIONS]

OPTIONS:
    -c, --color-file <COLOR_FILE>    Path to file containing RGBs separated by line
    -h, --help                       Print help information
    -i, --image-file <IMAGE_FILE>    Path to image to compare colors against
    -s, --squared-dist               Calculate squared RGB distances as well
    -V, --version                    Print version information
```

After running the command (specifying valid file paths for both `--color-file` and `--image-file`),
the CIEDE2000 color differences will be output in the same order as the RGBs in `--color-file`. The
color file should be a list of RGB values, with one value on every line. For example:

```js
10, 20, 30
30, 20, 10
```

## Installation

Releases for x86_64-apple-darwin (macOS), x86_64-pc-windows-msvc (Windows), and
x86_64-unknown-linux-gnu (Linux) can be found in the
[Releases](https://github.com/ap-1/color_difference/releases/) tab. If you wish to use the tool on
other operating systems or architectures, you will have to install the dependencies and build the
project yourself using [cargo](https://www.rust-lang.org/tools/install).

### Notes

- If you have trouble running the Linux executable due to permission issues, try running
`chmod u+x x86_64-unknown-linux-gnu` before you run the executable using
`./x86_64-unknown-linux-gnu`.

- On Windows machines, Microsoft Defender Smartscreen may warn you when you try to run the app.
However, this tool is safe to use and the warnings may safely be ignored. For further validation,
you can find the checksums yourself and compare them to the ones provided in the release.
