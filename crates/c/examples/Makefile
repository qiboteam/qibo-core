PREFIX=$(shell realpath ../prefix/)
export PKG_CONFIG_PATH := $(PREFIX)/lib/pkgconfig

CFLAGS=-g -Wall -Wextra $(shell pkg-config --cflags qibo_core_c)
LDFLAGS=$(shell pkg-config --libs qibo_core_c)

SOURCES=$(wildcard *.c)
EXECUTABLES=$(patsubst %.c,%,$(SOURCES))


libmicrosim.so: microsimulator/microsim.c microsimulator/microsim.h
	@cp ./microsimulator/microsim.h $(PREFIX)/include/
	$(CC) $(CFLAGS) -I$(PREFIX)/include $(LDFLAGS) microsimulator/microsim.c -shared -o libmicrosim.so
	cp libmicrosim.so $(PREFIX)/lib/

execute: libmicrosim.so execute.c
	$(CC) $(CFLAGS) -I$(PREFIX)/include $(LDFLAGS) -L$(PREFIX)/lib -lmicrosim execute.c -o execute

all: $(EXECUTABLES)
