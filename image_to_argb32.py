import sys
from PIL import Image

i = Image.open(sys.argv[1])
try:
    data = [v for r, g, b in i.getdata() for v in (b, g, r, 0)]
except:
    data = [v for r, g, b, a in i.getdata() for v in (b, g, r, 0)]

try:
    out_path = sys.argv[2]
except:
    out_path = 'res/image.rgb'

with open(out_path, 'wb') as f:
    f.write(bytearray(data))
