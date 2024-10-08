package main

import (
	"log"

	"github.com/egginabucket/openmsr/pkg/libmsr"
)

func main() {
	device, err := openMSR605X()
	if err != nil {
		log.Fatalf("%v", err)
	}

	pos := posApplication{
		cardReader: libmsr.NewDevice(device),
	}

	newPosGui(pos)
}
