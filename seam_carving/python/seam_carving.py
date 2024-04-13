from PIL import Image, ImageFilter
from os import mkdir, path, getcwd
import math


Gx = ((1, 0, -1),
      (2, 0, -2),
      (1, 0, -1))
Gy = ((1,   2,  1),
      (0,   0,  0),
      (-1, -2, -1))


def get_temp_dir() -> str:
    dir = path.join(getcwd(), f"../images/tmp")
    if not path.isdir(dir):
        mkdir(dir)
    return dir


def generate_light_map(img: Image.Image) -> list[float]:
    data = img.getdata()
    width, height = img.size

    lightness_data = [0.0] * (width*height)
    for x in range(width * height):
        R = data[x][0]
        G = data[x][1]
        B = data[x][2]
        lightness = 0.2126*R + 0.7152*G + (0.0722*B)
        lightness_normalized = lightness / 100
        lightness_data[x] = lightness_normalized

    light_map_image = Image.new("L", (width, height))
    light_map_image.putdata([int(i * 100) for i in lightness_data])
    light_map_image.save(f"{get_temp_dir()}/lightmap.png")

    return lightness_data


def generate_energy_map(light_map: list[float], width: int, height: int) -> list[float]:
    energy_data = light_map.copy()
    for x in range(1, width-1):
        for y in range(1, height-1):
            p = x + width * y
            A = (tuple(light_map[x+width*(y-1)-1:x+width*(y-1)+1]),
                 tuple(light_map[x+width*y-1:  x+width*y+1]),
                 tuple(light_map[x+width*(y+1)-1:x+width*(y+1)+1]))

            cx, cy = 0, 0
            for i in range(2):
                for j in range(2):
                     cx += Gx[-1*j][-1*i] * A[i][j]
                     cy += Gy[-1*j][-1*i] * A[i][j]

            G = math.sqrt(cx**2 + cy**2)
            energy_data[p] = G

    energy_map_image = Image.new("L", (width, height))
    energy_map_image.putdata([int(i * 100) for i in energy_data])
    energy_map_image.save(f"{get_temp_dir()}/energy_map.png")
    return energy_data


def find_seam_vertical(img: Image.Image, energy_map: list[float]):
    widht, height = img.size

    seam = []
    for i in range(height):
        pass

    image = img.copy()
    for i in seam:
        image.putpixel(i, (255,0,0))
    image.save(f"{get_temp_dir()}/image_with_seam.png")


with Image.open("../images/Broadway_tower_edit.jpg") as img:
    width, height = img.size
    light_map = generate_light_map(img)
    energy_map = generate_energy_map(light_map, width, height)

    find_seam_vertical(img, energy_map)
