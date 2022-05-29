; ModuleID = 'main'
source_filename = "main"

@0 = private unnamed_addr constant [14 x i8] c"hello, world.\00", align 1

declare i32 @puts(i8*)

define i32 @main() {
main:
  %0 = call i32 @puts(i8* getelementptr inbounds ([14 x i8], [14 x i8]* @0, i32 0, i32 0))
  ret i32 0
}

define i32 @test() {
test:
  ret void
}
