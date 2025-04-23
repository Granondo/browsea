# Browsea

## Installation

### Installing the app

To install the app, run the following script in the terminal: 

```
.\build.bat
```

### Extract the ZIP File

Extract the downloaded ZIP file to a folder of your choice.

### Run the Installer

1.  Navigate to the extracted folder.
2.  Right-click on `install.bat` and select **Run as administrator**.

### Set as Default Browser

1.  Open Windows Settings.
2.  Go to **Apps** > **Default Apps**.
3.  Set Browsea as your default browser.

## Building from Source

### Prerequisites

Make sure you have Rust and Cargo installed. If not, you can download them from [https://www.rust-lang.org/](https://www.rust-lang.org/).

### Build the Cargo project

To build the project, run:

```
cargo build
```

### Run the Project

To run the project with a specific URL, use the following command:

```
cargo run -- "https://example.com"
```

## Additional Information

### Uninstallation
To uninstall Browsea:
1. Navigate to the installation folder
2. Right-click on `uninstall.bat` and select **Run as administrator**
3. Follow the prompts to complete uninstallation

### Development
- Built with Rust and eframe
- Uses Windows Registry for browser detection


### Contributing
Contributions are welcome! Please feel free to submit pull requests.

### License
[Add license information here]

