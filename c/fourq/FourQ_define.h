

#ifndef __FOURQ_DEFINE_H__
#define __FOURQ_DEFINE_H__

// For C++
#ifdef __cplusplus
extern "C" {
#endif

#ifndef __WINDOWS__
/* #undef __WINDOWS__ */
#endif
#ifndef __LINUX__
#define __LINUX__
#endif

#ifndef _MSC_VER
/* #undef _MSC_VER */
#endif
#ifndef __GNUC__
/* #undef __GNUC__ */
#endif
#ifndef __clang__
#define __clang__
#endif

#ifndef _AMD64_
/* #undef _AMD64_ */
#endif
#ifndef _X86_
/* #undef _X86_ */
#endif
#ifndef _ARM_
/* #undef _ARM_ */
#endif
#ifndef _ARM64_
#define _ARM64_
#endif

#ifndef _AVX2_
/* #undef _AVX2_ */
#endif
#ifndef _AVX_
/* #undef _AVX_ */
#endif
#ifndef _ASM_
/* #undef _ASM_ */
#endif
#ifndef _GENERIC_
/* #undef _GENERIC_ */
#endif

#ifdef __cplusplus
}
#endif

#endif
