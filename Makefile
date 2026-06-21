.PHONY: build build-all build-linux build-macos build-windows clean

# 当前平台默认构建
build:
	maturin build --release --out dist

# ============================================================
# 交叉编译 — 需要提前安装对应的 Rust target 和工具链
#   rustup target add <triple>
#
# Linux:   需要安装对应架构的 GCC 交叉编译器
# macOS:   需要对应的 macOS SDK（只在 macOS 上交叉编译 macOS）
# Windows: 需要安装 mingw-w64 或 MSVC 工具链
# ============================================================

# Linux 平台
build-linux-x86_64:          ## manylinux x86_64
	maturin build --release --out dist --target x86_64-unknown-linux-gnu

build-linux-aarch64:         ## manylinux aarch64 (ARM64)
	maturin build --release --out dist --target aarch64-unknown-linux-gnu

build-linux-i686:            ## manylinux i686
	maturin build --release --out dist --target i686-unknown-linux-gnu

# macOS 平台 (仅 macOS 宿主机可交叉编译)
build-macos-x86_64:          ## macOS Intel
	maturin build --release --out dist --target x86_64-apple-darwin

build-macos-aarch64:         ## macOS Apple Silicon
	maturin build --release --out dist --target aarch64-apple-darwin

# Windows 平台
build-windows-x86_64:        ## Windows x86_64 (MSVC)
	maturin build --release --out dist --target x86_64-pc-windows-msvc

build-windows-i686:          ## Windows i686 (MSVC)
	maturin build --release --out dist --target i686-pc-windows-msvc

build-windows-aarch64:       ## Windows ARM64
	maturin build --release --out dist --target aarch64-pc-windows-msvc

# 一键构建所有常见平台 (需要所有交叉工具链)
build-all: build-linux-x86_64 build-linux-aarch64 \
           build-macos-x86_64 build-macos-aarch64 \
           build-windows-x86_64

# 清理构建产物
clean:
	rm -rf dist
