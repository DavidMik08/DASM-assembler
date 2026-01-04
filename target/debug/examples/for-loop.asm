; basically for (int i = 0; i<10; i++) {} but in DASM
loop_init:
  ;int i = 0;
  add 0 0 r1

loop_start:
  ; i < 10
  sub r1 10 r0
  add_igf loop_end.Low 0 r3
  add_igf loop_end.Mid 0 r4
  add_igf loop_end.High 0 r5
  biz 0 0 r0

  ; i++
  add r1 1 r1

  sub 0 0 r0
  add_igf loop_start.Low 0 r3
  add_igf loop_start.Mid 0 r4
  add_igf loop_start.High 0 r5
  biz 0 0 r0

loop_end:
  brk 0 0 r0
