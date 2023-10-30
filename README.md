# Pink

`pink` is a command-line tool inspired by the Unix `man` command. It displays custom-formatted text pages in the terminal using a subset of HTML-like tags.

## Current Working Tested OS's ⚙️

- **macOS** Ventura 13.6
- **FreeBSD** 13.2

## Features 🎨

- **Custom Formatting**: Apply styles such as bold, italic, underline, and color to text.
- **Pink Tags**: Use a subset of HTML-like tags tailored for terminal display.
- **Easy to Use**: Simply run `pink <filename>` to display a formatted text page.

## Installation 🛠️

To install `pink`, you need to have Rust and Cargo installed on your system. If you haven't already installed Rust, you can do so by following the instructions on the [official Rust website](https://www.rust-lang.org/tools/install).

Once Rust and Cargo are installed, follow these steps to build, install, and set up `pink`:

1. Clone the repository and navigate to the project directory:
   ```sh
   git clone https://github.com/KatahGii/pink.git
   cd pink
   ```

2. Build the project:
   ```sh
   cargo build --release
   ```

3. Run the installation script (you might need to give it execute permissions first):
   ```sh
   chmod +x install.sh
   ./install.sh
   ```

   This script will:
   - Create the necessary directories.
   - Add the default `pink.pink` file.
   - Move the compiled binary to a directory in your `PATH`.

## Usage 🚀

To display a formatted text page, run:

```sh
pink <filename>
```

## Tag Usage 🏷️

`pink` utilizes a subset of HTML-like tags, referred to as "pink tags", to apply styles to the text. Here is a brief overview:

- `<b>...</b>`: Makes the enclosed text bold.
- `<i>...</i>`: Makes the enclosed text italic.
- `<u>...</u>`: Underlines the enclosed text.
- `<HP>...</HP>`: Changes the text color to pink.
- `<H1>...</H1>`, `<H2>...</H2>`, `<H3>...</H3>`: Apply heading styles. (more info in the pink.pink file)

## Contributing 👥

Contributions are welcome! Feel free to open an issue or submit a pull request.

## License 📜

This project is licensed under the GPLv3 License - see the [LICENSE](LICENSE) file for details.
