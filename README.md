# 🌌 Ray Tracer (`rt`)

## 📖 Project Overview

This project implements a **ray tracer** in Rust that renders 3D objects into 2D images by simulating rays of light.  
It computes light interactions (shadows, diffuse lighting, simple reflections) per ray and saves results as **PPM (`.ppm`)** images.

---

## 🎯 Features

- Primitives: **Sphere**, **Cube**, **Plane**, **Cylinder**
- Moveable **camera** (position, look-at, FOV)
- **Point light** with shadows
- **Per‑object color** (albedo) and **reflectivity**
- Configurable **image size** and **samples per pixel**

---

## 🖼️ Required Scenarios (ready to run)

```bash
# 1) Sphere only
cargo run --release -- --scene=sphere --res=800x600 --out=sphere.ppm

# 2) Plane + Cube (dimmer than sphere image)
cargo run --release -- --scene=cube_plane_dim --res=800x600  --out=cube_plane_dim.ppm

# 3) All objects (sphere + cube + cylinder + plane)
cargo run --release -- --scene=all --res=800x600 --out=all_objects.ppm

# 4) Same scene, different camera
cargo run --release -- --scene=all_alt_cam --res=800x600 --out=all_objects_alt_cam.ppm
```

# === Cube + Cylinder (default camera) ===

cargo run --release -- --scene=custom --res=1280x720 --spp=32 --out=cube_cyl.ppm \
 --add-plane="0,-0.5,0;0,1,0;0.82,0.82,0.82;0.05" \
 --add-cube="0.0,-0.2,-1.2;0.6;0.35,0.42,0.65;0.00" \
 --add-cylinder="0.8,-0.1,-1.6;0.3;0.4;0.20,0.70,0.40;0.05"

# === Same scene, alt cam (right side view) ===

cargo run --release -- --scene=custom --res=1280x720 --spp=32 --out=cube_cyl_cam1.ppm \
 --lookfrom=1.2,0.4,1.4 --lookat=0.2,-0.2,-1.4 --fov=60 \
 --add-plane="0,-0.5,0;0,1,0;0.82,0.82,0.82;0.05" \
 --add-cube="0.0,-0.2,-1.2;0.6;0.35,0.42,0.65;0.00" \
 --add-cylinder="0.8,-0.1,-1.6;0.3;0.4;0.20,0.70,0.40;0.05"

# === Same scene, alt cam (slightly top-down, narrower FOV) ===

cargo run --release -- --scene=custom --res=1280x720 --spp=32 --out=cube_cyl_cam2.ppm \
 --lookfrom=0.0,1.0,1.8 --lookat=0.2,-0.2,-1.4 --fov=45 \
 --add-plane="0,-0.5,0;0,1,0;0.82,0.82,0.82;0.05" \
 --add-cube="0.0,-0.2,-1.2;0.6;0.35,0.42,0.65;0.00" \
 --add-cylinder="0.8,-0.1,-1.6;0.3;0.4;0.20,0.70,0.40;0.05"

# === Sphere + Cylinder (default camera) ===

cargo run --release -- --scene=custom --res=1280x720 --spp=32 --out=sphere_cyl.ppm \
 --add-plane="0,-0.5,0;0,1,0;0.82,0.82,0.82;0.05" \
 --add-sphere="-0.6,0.0,-1.2;0.45;0.90,0.25,0.25;0.10" \
 --add-cylinder="0.5,-0.1,-1.5;0.28;0.38;0.20,0.70,0.40;0.06"

# === Sphere + Cylinder, alt cam (left side view) ===

cargo run --release -- --scene=custom --res=1280x720 --spp=32 --out=sphere_cyl_cam1.ppm \
 --lookfrom=-1.1,0.3,1.3 --lookat=-0.2,-0.1,-1.3 --fov=55 \
 --add-plane="0,-0.5,0;0,1,0;0.82,0.82,0.82;0.05" \
 --add-sphere="-0.6,0.0,-1.2;0.45;0.90,0.25,0.25;0.10" \
 --add-cylinder="0.5,-0.1,-1.5;0.28;0.38;0.20,0.70,0.40;0.06"

# === Sphere + Cube (with subtle reflections), alt cam ===

cargo run --release -- --scene=custom --res=1280x720 --spp=32 --out=sphere_cube_cam.ppm \
 --lookfrom=1.4,0.5,1.0 --lookat=0.1,-0.1,-1.3 --fov=60 \
 --add-plane="0,-0.5,0;0,1,0;0.82,0.82,0.82;0.05" \
 --add-sphere="-0.8,0.0,-1.3;0.5;0.90,0.20,0.20;0.10" \
 --add-cube="0.3,-0.2,-1.4;0.6;0.35,0.42,0.65;0.00"

# === Cube + Cylinder with blue floor ===

cargo run --release -- --scene=custom --res=1280x720 --spp=32 --out=cube_cyl_bluefloor.ppm \
 --lookfrom=0.8,0.3,1.4 --lookat=0.3,-0.2,-1.4 --fov=55 \
 --add-plane="0,-0.5,0;0,1,0;0.10,0.10,0.90;0.05" \
 --add-cube="0.0,-0.2,-1.2;0.6;0.30,0.35,0.50;0.00" \
 --add-cylinder="0.9,-0.1,-1.6;0.30;0.40;0.20,0.70,0.40;0.05"

# === Same as above, dimmer light (global brightness change) ===

cargo run --release -- --scene=custom --res=1280x720 --spp=32 --out=cube_cyl_bluefloor_dim.ppm \
 --lookfrom=0.8,0.3,1.4 --lookat=0.3,-0.2,-1.4 --fov=55 \
 --light-int=0.6,0.6,0.6 \
 --add-plane="0,-0.5,0;0,1,0;0.10,0.10,0.90;0.05" \
 --add-cube="0.0,-0.2,-1.2;0.6;0.30,0.35,0.50;0.00" \
 --add-cylinder="0.9,-0.1,-1.6;0.30;0.40;0.20,0.70,0.40;0.05"

# === Cylinder close-up (wider FOV) ===

cargo run --release -- --scene=custom --res=1280x720 --spp=32 --out=cyl_closeup.ppm \
 --lookfrom=0.7,0.3,1.0 --lookat=0.8,-0.1,-1.6 --fov=70 \
 --add-plane="0,-0.5,0;0,1,0;0.82,0.82,0.82;0.05" \
 --add-cylinder="0.8,-0.1,-1.6;0.32;0.42;0.25,0.75,0.45;0.06"

---

## ⚙️ Build

```bash
cargo build --release
```

Default output size is set via the CLI (`--res=WxH`).  
Use higher `--spp` for smoother images.

Example:

```bash
cargo run --release -- --scene=all --res=800x600 --spp=1000 --out=all_smooth.ppm
```

---

## 🧱 Creating Objects (code examples)

Each object takes **position + size + color + reflectivity**:

```rust
use crate::math::{Point3, Vec3, Color};
use crate::sphere::Sphere;
use crate::plane::Plane;
use crate::cube::Cube;
use crate::cylinder::Cylinder;

// Sphere: center, radius, color, reflectivity
let sphere = Sphere::new(Point3::new(0.0, 0.0, -1.3), 0.5, Color::new(0.9, 0.2, 0.2), 0.10);

// Plane: point on plane, normal, color, reflectivity
let plane  = Plane::new(Point3::new(0.0, -0.5, 0.0), Vec3::new(0.0, 1.0, 0.0),
                        Color::new(0.82, 0.82, 0.82), 0.05);

// Cube: center, edge size, color, reflectivity
let cube   = Cube::from_center_size(Point3::new(0.3, -0.2, -1.4), 0.6,
                                    Color::new(0.35, 0.42, 0.65), 0.00);

// Cylinder: center (mid‑height), radius, half_height, color, reflectivity
let cyl    = Cylinder::new(Point3::new(1.4, -0.1, -1.6), 0.3, 0.4,
                           Color::new(0.2, 0.7, 0.4), 0.05);
```

Add objects to the world via `HittableList::add(Box::new(obj))` in `build_scene`.

---

## 💡 Correct Ways to Change **Brightness**

There are **three** levers in this codebase. Use whichever matches your goal:

### 1) **Light intensity** (global scene brightness)

The point light’s intensity is a **color multiplier**. Increasing it brightens everything.

```rust
use crate::light::PointLight;
use crate::math::{Point3, Color};

// Bright white light
let light = PointLight::new(Point3::new(5.0, 5.0, -2.0), Color::new(1.0, 1.0, 1.0));

// Dimmer light (used in --scene=cube_plane_dim)
let dim   = PointLight::new(Point3::new(5.0, 5.0, -2.0), Color::new(0.6, 0.6, 0.6));
```

> In the dim cube scene, **only** the light intensity is reduced → the whole scene renders darker than the sphere scene.

### 2) **Object albedo** (per‑object brightness)

Albedo is the base color that receives light. Darker albedo ⇒ darker object under the same light.

```rust
// Brighter red sphere vs. darker blue‑grey cube
let sphere_albedo = Color::new(0.9, 0.2, 0.2);
let cube_albedo   = Color::new(0.25, 0.28, 0.35);
```

### 3) **Ambient term** (global “base light” in shadows)

Ambient is added everywhere (even in shadow). It appears in `shade_lambert_with_shadow`:

```rust
fn shade_lambert_with_shadow(/* … */) -> Color {
    let ambient = 0.12; // <- increase to lift dark areas globally; decrease for punchier shadows
    // ...
}
```

> **Tip:** Prefer changing **light intensity** or **albedo** first. Tuning ambient changes the whole contrast of the image (shadows vs. lit areas).

**Not brightness:** `reflectivity` affects how reflective a surface looks, but it is **not** an exposure control.

---

## 🎥 Camera: position, direction, field of view

Create a camera by passing **look‑from**, **look‑at**, **up**, **vertical FOV (degrees)**, and **aspect**:

```rust
use crate::camera::Camera;
use crate::math::{Point3, Vec3};

let aspect = width as f64 / height as f64;

// Default view
let cam = Camera::new(Point3::new(0.0, 0.0, 0.0),  // look‑from
                      Point3::new(0.0, 0.0, -1.0), // look‑at
                      Vec3::new(0.0, 1.0, 0.0),    // up
                      90.0,                        // FOV
                      aspect);

// Alternate perspective (used in --scene=all_alt_cam)
let alt = Camera::new(Point3::new(1.6, 0.5, 1.2),
                      Point3::new(0.1, -0.2, -1.5),
                      Vec3::new(0.0, 1.0, 0.0),
                      75.0,
                      aspect);
```

---

## 🖨️ PPM Output (P3, ASCII)

```
P3
800 600
255
r g b
r g b
...
```

- `P3` = ASCII full‑color
- `800 600` = width × height
- `255` = max color value

---

## 📌 Notes for Auditors

- **Shadows** are computed via a shadow ray to the point light.
- **Reflection** is per‑object with recursive bounces (depth limited).
- The “dimmer cube” requirement is satisfied by using **lower light intensity** in `--scene=cube_plane_dim`.
- All four scenes are reproducible using the commands at the top.

---

## 🛠️ Tech Stack

- **Rust**, Cargo
- Basic linear algebra (Vec3, dot/cross)
- CPU ray tracing
- PPM image output

---

## ✅ Learning Outcomes

- Ray–object intersections (sphere, plane, cube (AABB), cylinder)
- Shading: ambient + Lambert diffuse + hard shadows
- Reflections with recursion limits
- Camera modeling and image synthesis

### Custom camera

- `--lookfrom "x,y,z"` set camera position
- `--lookat "x,y,z"` set camera target
- `--vup "x,y,z"` (optional) camera up vector (default 0,1,0)
- `--fov <deg>` vertical field of view

### Light

- `--light-pos "x,y,z"`
- `--light-int "r,g,b"` (acts as intensity; try 0.6,0.6,0.6 to dim)

### Build-your-own scene

Use `--scene=custom` and any number of:

- `--add-plane "px,py,pz; nx,ny,nz; r,g,b; reflectivity"`
- `--add-sphere "cx,cy,cz; radius; r,g,b; reflectivity"`
- `--add-cube "cx,cy,cz; size; r,g,b; reflectivity"`
- `--add-cylinder "cx,cy,cz; radius; half_h; r,g,b; reflectivity"`

> If any `--add-*` flag is provided, custom mode is assumed automatically.
