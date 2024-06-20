CFLAGS=-g -Wall -Wextra $(shell pkg-config --cflags qibo_core_c)
LDFLAGS=$(shell pkg-config --libs qibo_core_c)

SOURCES=$(wildcard *.c)
EXECUTABLES=$(patsubst %.c,%,$(SOURCES))

all: $(EXECUTABLES)
