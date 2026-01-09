; this program calculates the fibonacci sequence untill one of the numbers is grater than 255.
; r1 and r2 hold the last two fibonacci numbers
; the c equivalent would look something like this 
;
; int r1 = 0, r2 = 1;
; 
; while (true) {
;   r1 += r2;
;   if (r1 > 255)
;     break;
;   
;   r2 += r1;
;   if (r2 > 255)
;     break;
; }


; initialise r1 and r2
add 0 0 r1
add 1 0 r2

while_loop_start:
  ; r1 += r2;
  add r1 r2 r1
  
  ; branch to the loop end if the carry flag is on (r1 > 255)
  add_igf while_loop_end.Low 0 r3
  add_igf while_loop_end.Mid 0 r4
  add_igf while_loop_end.High 0 r5
  bic 0 0 r0

  ; r2 += r1;
  add r2 r1 r2

  ; branch to the loop end if the carry flag is on (r2 > 255)
  add_igf while_loop_end.Low 0 r3
  add_igf while_loop_end.Mid 0 r4
  add_igf while_loop_end.High 0 r5
  bic 0 0 r0

  ; branch to the start of the loop
  add 0 0 r0
  add_igf while_loop_start.Low 0 r3
  add_igf while_loop_start.Mid 0 r4
  add_igf while_loop_start.High 0 r5
  biz 0 0 r0


while_loop_end:
  ; this is the code that runs after the while loop
  brk 0 0 r0
