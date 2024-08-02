#!/bin/bash
magick -size 200x200 xc:transparent -fill red \
  -stroke black \
  -draw "circle 100,100 50,50" \
  assets/red_circle.png
magick -size 200x200 xc:transparent -fill green \
  -stroke black \
  -draw "circle 100,100 50,50" \
  assets/green_circle.png
magick -size 200x200 xc:transparent -fill blue \
  -stroke black \
  -draw "circle 100,100 50,50" \
  assets/blue_circle.png