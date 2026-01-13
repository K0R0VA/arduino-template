#!/usr/bin/env bash
# -*- coding: utf-8 -*-

cargo build --release

ELF=$1
HEX=$(echo $ELF | sed 's/.elf$/.hex/')
avr-objcopy -j .text -j .data -O ihex target/avr-none/release/*.elf output.hex