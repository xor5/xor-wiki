use std::arch::asm;

#[no_mangle]
fn test(a: u64, b: u64, c: u64) {
    println!("First param: {}\nSecond param: {}\nThird param: {}", a, b, c);
}

fn shellcode() {
    unsafe {
        asm!("
        /*
            Example of calling 'test' with x arguments
            Make sure to invert the order of the arguments ex. test(1, 2, 3) would be
            mov exd, 3
            mov esi, 2
            mov edi, 1
        */
        
        /*
            Example of defining a variable at (runtime?)
            push rbp # get the stack frame
            mov rbp, rsp # set/get the stack pointer
            sub rsp, 16 # allocate x bytes
            mov dword ptr [rbp-8], 0x0A # set rbp-x to 0x0A (10)
            add rsp, 8 # free up our allocated memory
        */
        
        push rbp
        mov rbp, rsp
        sub rsp, 32
        
        mov dword ptr [rbp-8], 0x0A
        mov dword ptr [rbp-16], 0x0B
        mov dword ptr [rbp-24], 0x0C
        
        // pop rbp
        // push rbp
        // mov rbp, rsp
        // add rsp, 16
        
        add rsp, 24 # release our allocated memory, for some reason
        
        // mov edx, 0x0A # 10
        // mov esi, 0x262d # 9773
        // mov edi, 0x539 # 1337
        
        mov edx, dword ptr [rbp-8]
        mov esi, dword ptr [rbp-16]
        mov edi, dword ptr [rbp-24]

        xor eax, eax # reset eax, return code
        call test # call the test function"
        
    );
    }
}

fn main() {
    shellcode();
    // test(1337, 9773, 10);
}
