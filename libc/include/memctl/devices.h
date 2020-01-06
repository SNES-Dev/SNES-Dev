//
// Created by chorm on 2020-01-03.
//

#ifndef SNES_DEV_DEVICES_H
#define SNES_DEV_DEVICES_H

#include <stdint.h>

typedef struct _DeviceRequest{
    uint8_t devno;
    uint16_t properties;
}__attribute__((packed)) _DeviceRequest;

#endif //SNES_DEV_DEVICES_H
