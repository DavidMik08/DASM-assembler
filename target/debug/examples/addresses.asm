; the point of this example is to shou how addresses work
; addresses are used when working with ram and when branching
;
; here are some examples of how to use the address with ram
;
; lets load 26 into the ram address 1234

add 1234.Low 0 r3
add 1234.Mid 0 r4
add 1234.High 0 r5

add 26 0 ram

; the .Low, .Mid and .High suffixes at the end of an 24 bit number or a label 
; specify what byte to use
; we can use them to extract the bytes from the address and put them 
; into the correct registers
;
; a more generic example would be something like this:
; add address.Low 0 r3
; add address.Mid 0 r4
; add address.High 0 r5
; add data 0 ram
;
; we dont have to use add to store to ram,
; this way we can store the result of any instruction into ram
; for example instead of putting 'add data 0 ram' we can use
; 'and r1 16 ram' to save the bottom 4 bits of r1 to ram


; we can also read from ram almost the same way as we write
; we just use ram as one of the arguments 
; and anything that we want to use as the other arguments
; for example lets read ram xor 5 into r1 from the address 2143

add 2143.Low 0 r3
add 2143.Mid 0 r4
add 2143.High 0 r5
xor ram 5 r1

; this also allows us to quickly modify a single byte of ram,
; lets say we want to increment ram at address 9054 by 3 
; and we dont care about owerflow
; we could just do something like this:
add 9054.Low 0 r3
add 9054.Mid 0 r4
add 9054.High 0 r5
add ram 3 ram

; this example uses the same ram address as both input and output 
; so it takes the value from ram, increments it by 3 
; and stores it back at the same address
