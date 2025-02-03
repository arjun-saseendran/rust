fn main() {
    // Display unsigned integer
    let mut value = 0b1111_0101u8;
    println!("Value is: {} ",value);
    
    // Display binary number
    println!("The binary number of the value is: {:08b} ",value);
    
    // Display not oparator
    value = !value;
    
    // Display not oparator value
    println!("The bitwise not oparator is: {:08b} ",value);
    
    // Display and oparator
    value = value & 0b1111_0111;
    
    // Display and oparator value
    println!("The bitwise and oparator is: {:08b} ",value);
    
    // Display bit position
    println!("bit 6 is: {} ",value & 0b0100_0000);
    
    // Display or oparator
    value = value | 0b0100_0000;
    
    // Display or oparator value
    println!("Or oparator value is: {:08b} ", value);
    
    // Display xor oparator
    value = value ^ 0b0101_0101;
    
    // Display value fo xor oparator
    println!("The value of xor oparator is: {:08b} ", value);
    
    // Left shift oparator
    value = value << 4;
    
    // Display left shift result
    println!("The left shift value is: {:08b}", value);
    
    // Right shift oparator
    value = value >> 2;
    
    // Display right shift result
    println!("The right shift value is: {:08b} ",value);
    
    
}
