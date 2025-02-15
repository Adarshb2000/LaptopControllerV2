# Gamepad Keyboard

🚀 **Control your PC with a Gamepad!**  
LaptopControllerV2 is a lightweight application that transforms your game controller into a **mouse, keyboard, and navigation tool** for your PC. Built with **Rust & Tauri**, it offers a seamless, low-latency experience.

![editted_ss](https://github.com/user-attachments/assets/b4650f65-87de-42cb-b2fb-05159b16d7f8)

---

## 🔥 Features

🎮 **Full PC Control via Gamepad**
- Move the mouse with the joystick 🎯
- Click & scroll using triggers and buttons 🔘
- Type using an intuitive radial keyboard ⌨️

🕹️ **Two Modes: Mouse & Keyboard**
- **Mouse Mode:** Use joysticks to move the cursor and triggers to click.
- **Keyboard Mode:** Enter text using a radial keyboard layout.

⚡ **Quick Toggles & Customization**
- Press **Start** to activate/deactivate the controller.
- Switch keyboard layouts on the fly 🔄
- Non-intrusive **toast notifications** for feedback.

📦 **Lightweight & Fast**
- Runs as an **overlay** with minimal CPU usage.
- Built with **Tauri**, offering a native-like experience.

---

## 🚀 Installation

### Windows (64-bit)
1. **Download the `.msi` installer** from [Releases](https://github.com/Adarshb2000/LaptopControllerV2/releases).
2. Run the installer and follow the setup.
3. Launch the application and connect your gamepad.

### Linux (Ubuntu/Debian)
1. **Download the `.deb` package** from [Releases](https://github.com/Adarshb2000/LaptopControllerV2/releases).
2. Open a terminal and run:
   ```sh
   sudo dpkg -i LaptopControllerV2_x86_64.deb
   ```
3. Start the app and connect your gamepad.

> 🛠️ **Note:** Compatibility with other Linux distributions & Windows versions is untested. Contributions are welcome!

Here's the updated **macOS (Experimental)** section with a bit of personality:  

### macOS (Experimental)  
I **don't have a Mac**, so macOS support is purely theoretical—**but I trust my code** (probably more than I should). If you’re brave enough to try it, here’s how:  

1. Install Rust & Tauri dependencies:  
2. Clone the repository:  
   ```sh
   git clone https://github.com/Adarshb2000/LaptopControllerV2.git  
   cd LaptopControllerV2  
   ```
3. Build and run:  
   ```
      npm i
      npm run tauri dev  
   ```

> ⚠️ **Disclaimer:** If something breaks, it's not a bug—it’s just an *undocumented feature*. Contributions to improve macOS compatibility are **highly encouraged!**  


---

## 🛠️ How It Works

LaptopControllerV2 reads **gamepad inputs** and translates them into **mouse and keyboard actions**.

- Uses **Enigo** for simulating mouse/keyboard input.
- Handles gamepad events via **Gilrs**.
- Uses **Tauri** to create a cross-platform desktop app.

---

## 🎯 Future Scope

🛠️ **Customization**
- Custom radial keyboard layouts 🎨
- Save & load different configurations
- Profiles for gaming, productivity, etc.

⚙️ **Input Enhancements**
- Support for advanced key combos (e.g., **Ctrl+C, Alt+Tab**)
- Macro recording & playback
- Custom **quick action zones** in the radial menu

🎛️ **Fine-Tuned Controls**
- Adjustable mouse movement & scroll sensitivity
- Custom **joystick dead zones**
- **Haptic feedback** (if supported by hardware)

🔮 **Long-Term Goal**
- Text prediction & word completion (not planned for now).

---

## 💡 Contributing

Contributions are welcome! If you'd like to help improve the project, feel free to:
- **Report issues** via [GitHub Issues](https://github.com/Adarshb2000/LaptopControllerV2/issues).
- **Submit pull requests** with bug fixes or enhancements.
- **Test on different platforms** and share feedback.

---

## 📜 License

This project is licensed under the **[MIT License](LICENSE)**.

---

## 🙌 Acknowledgments

- Built with ❤️ using **Rust, Tauri, and Enigo**.
- Inspired by the idea of using **gamepads as full-fledged input devices**.

---
