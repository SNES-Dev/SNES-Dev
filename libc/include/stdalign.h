//
// Created by chorm on 2020-01-03.
//

#ifndef SNES_DEV_STDALIGN_H
#define SNES_DEV_STDALIGN_H

#ifdef __STDC__
#if __STDC_VERSION__>=201112L
#define alignof _Alignof
#define alignas _Alignas
#define __alignas_is_defined 1
#define __alignof_is_defined 1
#endif
#endif

#endif //SNES_DEV_STDALIGN_H
