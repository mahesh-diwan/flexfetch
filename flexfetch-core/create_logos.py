#!/usr/bin/env python3
"""Generate simple PNG logo files for modules and distros"""

from PIL import Image, ImageDraw
import os

# Create directories
os.makedirs("assets/logos/modules", exist_ok=True)
os.makedirs("assets/logos/distros", exist_ok=True)

# Module logos - simple colored icons
MODULE_COLORS = {
    "title": (255, 215, 0),      # Gold
    "os": (0, 170, 255),         # Blue
    "host": (0, 200, 100),       # Green
    "kernel": (180, 80, 220),    # Purple
    "uptime": (0, 180, 200),     # Cyan
    "locale": (255, 165, 0),     # Orange
    "shell": (100, 220, 100),    # Light Green
    "terminal": (0, 150, 220),   # Sky Blue
    "de": (240, 130, 0),         # Amber
    "wm": (0, 180, 180),         # Teal
    "packages": (60, 200, 80),   # Green
    "cpu": (255, 140, 0),        # Dark Orange
    "memory": (0, 160, 240),     # Azure
    "disk": (50, 200, 100),      # Emerald
    "gpu": (200, 50, 200),       # Magenta
    "network": (0, 140, 255),    # Bright Blue
    "battery": (255, 180, 0),    # Yellow-Orange
    "processes": (100, 100, 255), # Periwinkle
    "resolution": (255, 100, 100), # Coral
    "colors": (200, 100, 200),   # Orchid
    "custom": (180, 180, 180),   # Gray
}

# Distro logos
DISTRO_COLORS = {
    "arch": (23, 147, 209),      # Arch Blue
    "ubuntu": (225, 71, 44),     # Ubuntu Orange
    "debian": (163, 0, 42),      # Debian Red
    "fedora": (46, 159, 255),    # Fedora Blue
    "nixos": (75, 135, 222),     # NixOS Blue
    "gentoo": (105, 24, 155),    # Gentoo Purple
    "alpine": (0, 135, 62),      # Alpine Green
    "void": (255, 69, 69),       # Void Red
    "centos": (136, 176, 75),    # CentOS Green
    "opensuse": (102, 189, 73),  # openSUSE Green
    "kali": (27, 27, 38),        # Kali Dark
    "macos": (160, 160, 165),    # macOS Gray
    "generic": (100, 100, 100),  # Gray
}

def create_module_logo(name, color):
    """Create a simple module logo - a colored square with a simple pattern"""
    img = Image.new('RGBA', (64, 64), (0, 0, 0, 0))
    draw = ImageDraw.Draw(img)
    
    # Draw a rounded rectangle background
    r, g, b = color
    # Main square
    draw.rounded_rectangle([8, 8, 56, 56], radius=8, fill=(r, g, b, 255))
    
    # Add a subtle inner pattern
    for i in range(3):
        alpha = 80 - i * 20
        draw.rounded_rectangle([14+i*4, 14+i*4, 50-i*4, 50-i*4], radius=4, 
                               outline=(255, 255, 255, alpha), width=2)
    
    # Add module initial in center
    initial = name[0].upper()
    # Use a simple approach - draw a white circle with the letter
    center = (32, 32)
    draw.ellipse([24, 24, 40, 40], fill=(255, 255, 255, 60))
    
    img.save(f"assets/logos/modules/{name}.png")

def create_distro_logo(name, color):
    """Create a distro logo"""
    img = Image.new('RGBA', (128, 128), (0, 0, 0, 0))
    draw = ImageDraw.Draw(img)
    
    r, g, b = color
    
    # Draw a circle with the distro color
    draw.ellipse([16, 16, 112, 112], fill=(r, g, b, 255))
    
    # Add a white inner circle
    draw.ellipse([28, 28, 100, 100], fill=(255, 255, 255, 30))
    
    # Add a smaller colored circle
    draw.ellipse([36, 36, 92, 92], fill=(r, g, b, 200))
    
    # Add distro name initial
    initial = name[0].upper()
    # Draw as text-like shape
    draw.ellipse([48, 48, 80, 80], fill=(255, 255, 255, 100))
    
    img.save(f"assets/logos/distros/{name}.png")

# Generate all logos
for name, color in MODULE_COLORS.items():
    create_module_logo(name, color)

for name, color in DISTRO_COLORS.items():
    create_distro_logo(name, color)

print("Generated all logos!")
