import numpy as np
from PIL import Image

NUM_COLORS = 16
WIDTH = 80
PIXELS_PER_CHAR = 9
PIXELS_PER_COLOR = int((WIDTH * PIXELS_PER_CHAR) / NUM_COLORS)

def ppm_hexdump(IMG_NAME = "dump.ppm"):
    img = np.array(Image.open(IMG_NAME), dtype=int)
    cols = img[0][::PIXELS_PER_COLOR]
    return cols
    


if __name__ == "__main__":
    print(ppm_hexdump())
