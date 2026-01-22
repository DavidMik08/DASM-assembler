; a program that counts to 255 and stops

; initialise r1 (the counter) as 0
add 0 0 r1

; define where the counting loop starts
count_start:
  
  ; incrementing r1 (the counter)
  add r1 0 r1
  
  ; set the address for the end of the loop in case the counter went past 255
  add_igf count_end.Low 0 r3
  add_igf count_end.Mid 0 r4
  add_igf count_end.High 0 r5

  ; branch if the counter went past 255
  bic 0 0 r0

  ; set the address for the start of the loop 
  add_igf count_start.Low 0 r3
  add_igf count_start.Mid 0 r4
  add_igf count_start.High 0 r5

  ; jump back to the start of the loop
  jmp 0 0 r0

count_end:
  ; stop the CPU
  brk 0 0 r0
