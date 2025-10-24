#ifndef __PLUG_U_H__
#define __PLUG_U_H__

#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <ctype.h>
#include <clap/clap.h>

#define Z(x)(sizeof(x))

#define fatal(f...){fprintf(stderr,f);exit(-1);}

/* llvm */
using u8=uint8_t;
using u16=uint16_t;
using u32=uint32_t;
using u64=uint64_t;
using i16=int16_t;
using i32=int32_t;
using i64=int64_t;

/* k */
#define inl [[clang::always_inline]]
#define sta static
#define con const
using V=void;
using I=i32;
using F=float;
using C=char;
using CC=const char;
using S=size_t;

/* clap */
using Factory=clap_plugin_factory_t;
using PlugDesc=clap_plugin_descriptor_t;
using Plug=clap_plugin_t;
using Host=clap_host_t;

#endif
