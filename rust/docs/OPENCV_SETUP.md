# OpenCV Setup Guide for SikuliX Development

SikuliX 3.0 requires OpenCV 4.x for computer vision capabilities. This guide covers installation on Windows and Linux.

## Windows Setup

### Option 1: vcpkg (Recommended)

1. **Install vcpkg** (if not already installed):
```powershell
cd C:\
git clone https://github.com/Microsoft/vcpkg.git
cd vcpkg
.\bootstrap-vcpkg.bat
```

2. **Add vcpkg to PATH** (or set VCPKG_ROOT):
```powershell
# Add to environment variables
setx VCPKG_ROOT "C:\vcpkg"
```

3. **Install OpenCV**:
```powershell
cd C:\vcpkg
.\vcpkg install opencv4:x64-windows
.\vcpkg integrate install
```

4. **Verify installation**:
```powershell
.\vcpkg list | findstr opencv
```

### Option 2: Pre-built Binaries

1. Download OpenCV 4.x from [opencv.org](https://opencv.org/releases/)
2. Extract to `C:\opencv`
3. Set environment variables:
```powershell
setx OPENCV_INCLUDE_PATHS "C:\opencv\build\include"
setx OPENCV_LINK_PATHS "C:\opencv\build\x64\vc16\lib"
setx OPENCV_LINK_LIBS "opencv_world4xx"
```

## Linux Setup

### Ubuntu/Debian

```bash
sudo apt-get update
sudo apt-get install -y \
    libopencv-dev \
    libopencv-core-dev \
    libopencv-imgproc-dev \
    libopencv-imgcodecs-dev \
    pkg-config
```

### Arch Linux

```bash
sudo pacman -S opencv
```

### From Source (Advanced)

```bash
git clone https://github.com/opencv/opencv.git
cd opencv
mkdir build && cd build
cmake -DCMAKE_BUILD_TYPE=Release ..
make -j$(nproc)
sudo make install
```

## Verification

Test that opencv-rust can find your installation:

```bash
cd rust/crates/sikulix-vision
cargo build
```

If successful, you'll see OpenCV linking messages. If it fails, check the probe output for hints.

## Troubleshooting

### Windows: "Failed to find installed OpenCV package"

- Ensure `VCPKG_ROOT` is set: `echo %VCPKG_ROOT%`
- Run `vcpkg integrate install` again
- Verify OpenCV is installed: `vcpkg list | findstr opencv`

### Linux: "pkg-config --libs opencv4" failed

```bash
# Check if pkg-config can find OpenCV
pkg-config --modversion opencv4

# If not, ensure PKG_CONFIG_PATH includes OpenCV
export PKG_CONFIG_PATH=/usr/local/lib/pkgconfig:$PKG_CONFIG_PATH
```

### Specifying OpenCV Location Manually

Set these environment variables before building:

```bash
# Unix/Linux
export OPENCV_INCLUDE_PATHS=/path/to/opencv/include
export OPENCV_LINK_PATHS=/path/to/opencv/lib
export OPENCV_LINK_LIBS=opencv_core,opencv_imgproc,opencv_imgcodecs

# Windows PowerShell
$env:OPENCV_INCLUDE_PATHS="C:\path\to\opencv\include"
$env:OPENCV_LINK_PATHS="C:\path\to\opencv\lib"
$env:OPENCV_LINK_LIBS="opencv_world4xx"
```

## CI/CD Considerations

For automated builds:

**Windows GitHub Actions:**
```yaml
- name: Install OpenCV
  run: |
    vcpkg install opencv4:x64-windows
    vcpkg integrate install
```

**Linux GitHub Actions:**
```yaml
- name: Install OpenCV
  run: |
    sudo apt-get update
    sudo apt-get install -y libopencv-dev pkg-config
```

## Version Requirements

- **Minimum**: OpenCV 4.0
- **Recommended**: OpenCV 4.5.x or later
- **Tested with**: OpenCV 4.5.1, 4.5.5, 4.8.0

## Related Links

- [opencv-rust documentation](https://docs.rs/opencv/)
- [opencv-rust GitHub](https://github.com/twistedfall/opencv-rust)
- [OpenCV official site](https://opencv.org/)
- [vcpkg packages](https://github.com/microsoft/vcpkg)
