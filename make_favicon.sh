#!/usr/bin/env bash

# requires imagemagick
convert -background none -density 384 data/logo.svg -define icon:auto-resize=64,32,16 data/favicon.ico
