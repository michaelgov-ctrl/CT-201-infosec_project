package main

import (
	"errors"
	"fmt"
	"strings"

	"golang.org/x/text/cases"
	"golang.org/x/text/language"

	"github.com/egginabucket/openmsr/pkg/libmsr"
	"github.com/karalabe/usb"
)

type posApplication struct {
	cardReader *libmsr.Device
}

var ErrInvalidFormat = errors.New("data does not fit expected format")

func openMSR605X() (usb.Device, error) {
	hids, err := usb.EnumerateHid(0x0801, 0x0003)
	if err != nil || len(hids) == 0 {
		return nil, fmt.Errorf("failed to find device")
	}

	device, err := hids[0].Open()
	if err != nil {
		return nil, fmt.Errorf("failed to open device")
	}

	return device, nil
}

func (pos *posApplication) readCard() (string, error) {
	isoTracks, err := pos.cardReader.ReadISOTracks()
	if err != nil {
		return "", err
	}

	return string(isoTracks), nil
}

func (pos *posApplication) parseCardHolderName(ccInfo string) (string, error) {
	parts := strings.Split(ccInfo, "^")
	if len(parts) < 2 {
		return "", ErrInvalidFormat
	}

	splitName := strings.Split(parts[1], "/")
	if len(parts) < 2 {
		return "", ErrInvalidFormat
	}

	caser := cases.Title(language.English)
	return caser.String(fmt.Sprintf("%s %s", splitName[1], splitName[0])), nil
}
