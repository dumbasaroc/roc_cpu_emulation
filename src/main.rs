use roc_cpu::*;

fn main() {
    println!("Roc CPU Interpreter");
    println!("===================");
    println!();

    // x = 19, x = 20
    // y = 15, y = 16

    let _windowed_program = roc_asm! {

        // Red Square Top Left
        PUTMEM 0x87, 0x4D, 0xFF;
        PUTMEM 0x87, 0x4E, 0x00;
        PUTMEM 0x87, 0x4F, 0x00;
        
        // Blue Square Bottom Left
        PUTMEM 0x87, 0xB0, 0x00;
        PUTMEM 0x87, 0xB1, 0x00;
        PUTMEM 0x87, 0xB2, 0xFF;
        
        // Green Square Top Right
        PUTMEM 0x87, 0xAD, 0x00;
        PUTMEM 0x87, 0xAE, 0xFF;
        PUTMEM 0x87, 0xAF, 0x00;        

        // Yellow Square Bottom Right
        PUTMEM 0x87, 0x50, 0xFF;
        PUTMEM 0x87, 0x51, 0xFF;
        PUTMEM 0x87, 0x52, 0x00;

        JUMP @lbl; // <= JUMPS TO THE RETURN STATEMENT

        RENDER; // <= SKIPS THIS LINE
        WAIT 3; // <= SKIPS THIS LINE
        
        @lbl
        EXIT;
    
    };


    let program = roc_asm! {
        PUT $f1, 5;
        PUT $f2, 7;
        CALL @multiply;
        MOV $ret, $fret;
        EXIT;

        // FUNCTION MULTIPLY
        // 
        // Calculates $f1 x $f2
        // Internally uses $f1, $f2, $ax-$dx
        // Returns all registers to their original states
        // Returns value goes to $fret
        @multiply
            PUSH $f1;
            PUSH $f2;
            PUSH $ax;
            PUSH $bx;
            PUSH $cx;

            // Put in default values
            PUT $ax, 1;
            MOV $bx, $f1;
            MOV $cx, $f2;
            PUT $dx, 0;
            PUT $fret, 0;

        @multiplyloop
            // Check if bx (ctr) == 0
            CMP $bx, $dx;
            JZ @multiplyfinish;

            ADD $fret, $cx;
            SUB $bx, $ax;
            JUMP @multiplyloop;

        @multiplyfinish
            POP $cx;
            POP $bx;
            POP $ax;
            POP $f2;
            POP $f1;
            RETURN;
    };


    let mut runner = RocCPURunner::new(Some(&program));
    let retval = runner.execute();

    println!("Execution completed with exit code {}", retval);
}

//WAIT 5;