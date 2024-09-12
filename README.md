# Tauri + React + TypeScript + Vite + Tailwind 

This template provides a setup to get React running on Vite and Tauri. Tailwind is installed in the project and includes some ESLint rules to help in the development process.

## Getting Started

### Prerequisites

#### For Windows:
- Install `Microsoft Visual Studio C++ Build Tools` from [here](https://visualstudio.microsoft.com/visual-cpp-build-tools/). Select `C++ build tools` and `Windows 10 SDK` in the installer.
- Download and run the `Evergreen Bootstrapper` from [here](https://developer.microsoft.com/en-us/microsoft-edge/webview2/#download-section).
- Install `Rust` with `rustup` from [here](https://www.rust-lang.org/tools/install).
- Install `Node.js` from [here](https://nodejs.org/en/download/).
- Install `pnpm` from [here](https://pnpm.io/installation). (Optional)

#### For Linux(Debian/Ubuntu):

- Run the following commands in your terminal:

   ```bash
   sudo apt update
   sudo apt install libwebkit2gtk-4.0-dev \
      build-essential \
      curl \
      wget \
      file \
      libssl-dev \
      libgtk-3-dev \
      libayatana-appindicator3-dev \
      librsvg2-dev
   ```

- To install Rust on Linux, open a terminal and enter the following command:

   ```bash
   curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
   ```

#### For MacOS:

- To install Rust on macOS, open a terminal and enter the following command:

   ```bash
   curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
   ```

To find more detailed information about prerequisites check [docs](https://tauri.app/v1/guides/getting-started/prerequisites).

### Installation

1. Clone the repo to your local machine using a terminal
   ```bash
   git clone https://github.com/emrecancorapci/tauri-react-typescript-tailwind
   ```
2. Change directory
   ```bash
   cd tauri-react-typescript-tailwind
   ```
3. Install NPM packages
   ```bash
   pnpm install
   ```

## Usage

### Development

To start the development server `using Tauri`, run the following command:

```bash
pnpm dev:tauri
```

You can start the development server `without using Tauri`. To do this, run the following command:

```bash
pnpm dev
```

### Build

To build the project, you must change the bundle identifier in `tauri.conf.json > tauri > bundle > identifier`. The default value `com.tauri.dev` is not allowed as it must be unique across applications.

To build the project, run the following command:

```bash
pnpm build:tauri
```
For more details about build, check [docs](https://tauri.app/v1/guides/building).
