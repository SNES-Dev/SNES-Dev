//
// Created by chorm on 2020-01-03.
//

#ifndef SNES_DEV_STDNORETURN_H
#define SNES_DEV_STDNORETURN_H

#ifdef __STDC__
#if __STDC_VERSION__>=201112L
#define noreturn _Noreturn
#endif
#endif

#endif //SNES_DEV_STDNORETURN_H
