"""Render header stills and transparent fframes layers (pip install resvg-py)."""
from copy import deepcopy
from pathlib import Path
import xml.etree.ElementTree as ET
import resvg_py

HERE = Path(__file__).resolve().parent
ASSETS = HERE.parent.parent
MEDIA = HERE.parent / 'v5/animation/media'
NS = '{http://www.w3.org/2000/svg}'
ET.register_namespace('', NS[1:-1])

def render(root, path):
    path.write_bytes(resvg_py.svg_to_bytes(svg_string=ET.tostring(root, encoding='unicode'), zoom=2))

for mobile in (False, True):
    suffix = '-mobile' if mobile else ''
    key = 'mobile' if mobile else 'desktop'
    root = ET.parse(HERE / f'header{suffix}.svg').getroot()
    render(root, ASSETS / f'profile-banner{suffix}.png')
    foreground = next(e for e in root if e.tag == NS+'g' and 'font-family' in e.attrib)
    # Keep the name and handle separate from independently animated badges.
    names = deepcopy(root)
    for e in list(names):
        if e.tag != NS+'defs':
            names.remove(e)
    text = deepcopy(foreground)
    for e in list(text):
        if 'data-tech' in e.attrib:
            text.remove(e)
    names.append(text)
    render(names, MEDIA / f'header_{key}_name.png')
    for index, badge in enumerate(e for e in foreground if 'data-tech' in e.attrib):
        x, y = map(float, badge.attrib['transform'][10:-1].split())
        width = 278 if mobile else 216
        layer = ET.Element(NS+'svg', width=str(width+8), height='80',
                           viewBox=f'{x-4:g} {y-4:g} {width+8} 80')
        group = ET.SubElement(layer, NS+'g', {'font-family':'Segoe UI,Arial,sans-serif'})
        group.append(deepcopy(badge))
        render(layer, MEDIA / f'header_{key}_{index}.png')
