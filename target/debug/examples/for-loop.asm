; this is an example of an for loop in DASM
; the c equivalent would be something like this:
; int i;
; for(i = 0; i<10; i = i+1) {}

add 0 0 r1

loop_start:
  ; compare r1 to 10
  sub r1 10 r0
  ; set the address
  add_igf loop_end.Low 0 r3
  add_igf loop_end.Mid 0 r4
  add_igf loop_end.High 0 r5
  ; branch to the end if r1 == 10
  biz 0 0 r0

  ; this is where you can put code that will run inside of the loop

  ; increment r1 (i)
  add r1 1 r1

  ; branch back to the start of the loop
  add 0 0 r0
  add_igf loop_start.Low 0 r3
  add_igf loop_start.Mid 0 r4
  add_igf loop_start.High 0 r5
  biz 0 0 r0
