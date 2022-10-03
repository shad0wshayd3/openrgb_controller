# openrgb_controller

Rust app that applies [OpenRGB](https://gitlab.com/CalcProgrammer1/OpenRGB) profiles on Windows Sleep/Wake power events.

## Usage

1. Create two new profiles in OpenRGB, ORGBC_Sleep, and ORGBC_Resume
2. Configure OpenRGB to start server on login, and optionally openrgb_controller.exe to start with it
3. Sleep/Wake the system
