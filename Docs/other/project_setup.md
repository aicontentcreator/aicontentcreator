

global is a cross-platform application, 


---

## Compiling from source: 
To compile from source you need to install [Prerequisites] (https://v2.tauri.app/start/prerequisites/)

### Clone the Repository

```bash
git clone https://github.com/evaturing/global.git
cd global
```

---

### Install Frontend Dependencies

> This step depends on your frontend (React, Svelte, Vue, etc.)

```bash
npm install
  npm run tauri android init

```

---

### Run in Development Mode

```bash
npm run tauri dev
  npm install

For Desktop development, run:
  npm run tauri dev

For Android development, run:
  npm run tauri android dev

```

This launches the app with hot-reloading support for frontend and Rust backend.

---

## 🛠 Build for Production

```bash
npm run tauri build
```

The output binary will be located in `src-tauri/target/release/` or inside the `/target/` folder.

---

## 📁 Project Structure

```
my-tauri-app/
├── src/               # Frontend source code
├── src-tauri/         # Rust backend
│   ├── main.rs        # Tauri entry point
│   └── tauri.conf.json
├── dist/              # Frontend production build
├── package.json
└── README.md
```

---

## 🧪 Testing

You can run tests using:

```bash
cargo test        # Rust tests
npm test          # Frontend tests (if applicable)
```

---

## 📦 Packaging & Distribution

To generate installers and distribute your app:

```bash
npm run tauri build
```

Use the output `.dmg`, `.msi`, `.AppImage`, etc., from the `/src-tauri/target/release/bundle/` directory.

---



---

## 📃 License


