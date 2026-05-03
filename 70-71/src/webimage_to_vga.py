import numpy as np
import requests as r
import sys
import binascii
from io import BytesIO
from PIL import Image as im
from ppm_hexdump import ppm_hexdump

# Constants
ROWS = 25
COLS = 80

#Functions
def image_to_rs_vga_buffer(file):
    file = "https://cd-public.github.io/ai101/images/photo-cat.jpg" if sys.argv[-1] == "src/webimage_to_vga.py" else file
    try:
        req = r.get(file)
        if req.status_code == 200:
            img_data = BytesIO(req.content)
            img = np.array(im.open(img_data), dtype=int)
        else:
            FileNotFoundError(f"No web file {file} could be found.")
    except:
        try:
            img = np.array(im.open(file), dtype=int)
            print(img)
            print(len(img))
            print(len(img[0]))
        except:
            FileNotFoundError(f"No local file {file} could be found.")

    vga_img = coerce_vga(img)
    try:
        f = open("src/img.rs", "x")
    except:
        f = open("src/img.rs", "w")
    f.write(f"pub const VGA_BUFFER: [[u16;80]; 25] = [\n\t{', \n\t'.join(['[%s]' % ', '.join(x) for x in vga_img])}\n];")


def coerce_vga(array):
    row_iter = [int(len(array)*x/ROWS) for x in range(ROWS)]
    col_iter = [int(len(array[0])*x/COLS) for x in range(COLS)]
    raw_pixels = [[array[row][col] for col in col_iter] for row in row_iter]
    coerced_pixels = [[0 for col in range(COLS)] for row in range(ROWS)]
    colors = ppm_hexdump()
    for row in range(len(raw_pixels)):
        for col in range(len(raw_pixels[row])):
            pixel = raw_pixels[row][col]
            coerced_pixels[row][col] = "0x{:01X}000".format(np.argmin([np.linalg.norm(pixel[:3] - color) for color in colors]))

    return coerced_pixels    
            
                
        

if __name__ == "__main__":
    image_to_rs_vga_buffer(sys.argv[-1])
