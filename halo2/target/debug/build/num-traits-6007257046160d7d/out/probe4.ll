; ModuleID = 'probe4.8aeb6fd20b525f19-cgu.0'
source_filename = "probe4.8aeb6fd20b525f19-cgu.0"
target datalayout = "e-m:o-i64:64-i128:128-n32:64-S128"
target triple = "arm64-apple-macosx11.0.0"

@alloc_2985744981838df235a86f7a748f23cb = private unnamed_addr constant <{ [75 x i8] }> <{ [75 x i8] c"/rustc/fb5ed726f72c6d16c788517c60ec00d4564b9348/library/core/src/num/mod.rs" }>, align 1
@alloc_1144d7f2cfc888dd79a6c289ef38719b = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_2985744981838df235a86f7a748f23cb, [16 x i8] c"K\00\00\00\00\00\00\00y\04\00\00\05\00\00\00" }>, align 8
@str.0 = internal unnamed_addr constant [25 x i8] c"attempt to divide by zero"

; probe4::probe
; Function Attrs: uwtable
define void @_ZN6probe45probe17hd20da5e4820afacdE() unnamed_addr #0 {
start:
  %0 = call i1 @llvm.expect.i1(i1 false, i1 false)
  br i1 %0, label %panic.i, label %"_ZN4core3num21_$LT$impl$u20$u32$GT$10div_euclid17h7a9612fe18c66378E.exit"

panic.i:                                          ; preds = %start
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17h44b8c9fdcfadbec6E(ptr align 1 @str.0, i64 25, ptr align 8 @alloc_1144d7f2cfc888dd79a6c289ef38719b) #3
  unreachable

"_ZN4core3num21_$LT$impl$u20$u32$GT$10div_euclid17h7a9612fe18c66378E.exit": ; preds = %start
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(none)
declare i1 @llvm.expect.i1(i1, i1) #1

; core::panicking::panic
; Function Attrs: cold noinline noreturn uwtable
declare void @_ZN4core9panicking5panic17h44b8c9fdcfadbec6E(ptr align 1, i64, ptr align 8) unnamed_addr #2

attributes #0 = { uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #1 = { nocallback nofree nosync nounwind willreturn memory(none) }
attributes #2 = { cold noinline noreturn uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #3 = { noreturn }

!llvm.module.flags = !{!0}
!llvm.ident = !{!1}

!0 = !{i32 8, !"PIC Level", i32 2}
!1 = !{!"rustc version 1.77.0-nightly (fb5ed726f 2023-12-28)"}
