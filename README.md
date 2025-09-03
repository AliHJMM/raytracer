# Ray Tracer (`rt`)

## 📖 Project Overview

This project implements a **ray tracer** in Rust that generates 3D scenes into 2D images using the **ray tracing** technique rather than rasterization.  
Ray tracing works by simulating rays from the camera through each pixel, calculating light interactions such as brightness, shadows, reflections, and refractions.

The output is saved as a **PPM image file** (`.ppm`), which can be easily viewed with most image viewers.

---

## 🎯 Objectives

The ray tracer supports:

- Rendering of **4 basic objects**:
  - Sphere
  - Cube
  - Plane
  - Cylinder
- **Object transformations** (change position in space).
- **Camera movement** to view the same scene from different angles.
- **Lighting system** with brightness and shadows.
- Output of **.ppm images** in configurable resolution (default: `800x600`).

---

## 🖼️ Required Scenarios

The auditor will validate your implementation using 4 `.ppm` images:

1. A scene with a **sphere**.
2. A scene with a **plane** and a **cube**, with reduced brightness compared to the first image.
3. A scene with **all four objects** (sphere, cube, cylinder, plane).
4. The same as (3) but viewed from a **different camera angle**.

---

## ⚙️ Installation & Usage

### 1. Clone the repository

```bash
git clone https://learn.reboot01.com/git/alihasan6/rt
cd rt
```

### 2. Build & Run

```bash
cargo run > output.ppm
```

This will generate `output.ppm` which can be opened with an image viewer.

To test with smaller resolutions:

```bash
cargo run -- --width 320 --height 240 > test.ppm
```

---

## 🧱 Features

### Objects

Each object can be created and positioned:

```rust
let sphere = Sphere::new(Vec3::new(1.0, 1.0, 1.0), 1.0);  // center (1,1,1), radius 1
let plane = Plane::new(Vec3::new(0.0, -1.0, 0.0), Vec3::new(0.0, 1.0, 0.0)); // ground plane
let cube = Cube::new(Vec3::new(-1.0, 0.0, -3.0), 2.0);  // center and side length
let cylinder = Cylinder::new(Vec3::new(0.0, 0.0, -2.0), 1.0, 3.0); // position, radius, height
```

### Lighting

Brightness can be adjusted globally or per object:

```rust
scene.set_light(Light::new(Vec3::new(5.0, 10.0, -5.0), 0.8)); // light position + intensity
```

### Camera

You can move the camera to change perspective:

```rust
let mut camera = Camera::new(Vec3::new(0.0, 0.0, -5.0));
camera.look_at(Vec3::new(0.0, 0.0, 0.0));  // target point
camera.set_fov(60.0);                      // field of view
```

---

## 📂 PPM Format

Output is in **Portable PixMap (P3)** format:

```
P3
800 600
255
r g b
r g b
...
```

- Header:
  - `P3` = ASCII full color
  - `800 600` = width and height
  - `255` = max color intensity
- Body: RGB triplets for each pixel.

---

## 🔥 Bonus Features (optional)

- **Textures** for object surfaces.
- **Reflections & Refractions** (shiny glass-like surfaces).
- **Particles** and **fluids**.
- Command-line flags (e.g., `-t` for textures).

---

## 📑 Documentation

The project provides:

- Step-by-step **examples** for creating objects.
- Instructions to **adjust brightness** and **camera view**.
- Clear guidance for generating `.ppm` images.

---

## 🛠️ Tech Stack

- **Language:** Rust
- **Build Tool:** Cargo
- **Output Format:** PPM (`.ppm`)

---

## ✅ Learning Outcomes

Through this project, you will learn:

- Fundamentals of **ray tracing**.
- Applying **geometry & math** for object rendering.
- Implementing **lighting models** and camera perspectives.
- Generating and handling **custom image formats**.
