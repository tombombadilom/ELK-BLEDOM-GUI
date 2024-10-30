# ELK-BLEDOM-bluetooth-led-strip-controller
A Rust controller for sending commands to Chinese generic Bluetooth LED strip controllers (ELK-BLEDOM).

## Features
- Control RGB LED strips via Bluetooth
- Set static colors with RGB values
- Adjust brightness
- Built-in animations (Flash, Strobe, Fade, Smooth)
- Warm white mode with adjustable intensity
- Power on/off control

## Building from source

# Development

1. Install npm: https://docs.npmjs.com/downloading-and-installing-node-js-and-npm
2. Install the tailwind css cli: https://tailwindcss.com/docs/installation
3. Run the following command in the root of the project to start the tailwind CSS compiler:

```bash
npx tailwindcss -i ./input.css -o ./assets/tailwind.css --watch
```

Run the following command in the root of the project to start the Dioxus dev server:

```bash
dx serve --hot-reload --platform desktop
```