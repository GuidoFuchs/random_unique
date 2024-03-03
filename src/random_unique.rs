//! Produces a sequence of unique numbers, that look like random


pub struct RandomSequenceOfUniqueU32 {
    // https://preshing.com/20121224/how-to-generate-a-sequence-of-unique-random-integers/
    index: u32,
    offset: u32,
}

impl RandomSequenceOfUniqueU32 {
    /// Bijective permutate function using complete range of u32
    fn permutate(&self, x: u32) -> u32 {
        const PRIME: u32 = 4294967291u32;
        const PRIME64: u64 = PRIME as u64;
        const PRIME_HALF: u32 = PRIME / 2;

        if x >= PRIME {
            return x; // The 5 integers out of range are mapped to themselves
        }
        let x64: u64 = x.into();
        let remainder: u32 = ((x64 * x64) % PRIME64) as u32;

        return if x <= PRIME_HALF {remainder} else {PRIME - remainder};
    }

    pub fn new(seed_base: u32, seed_offset: u32) -> Self {
        let mut s: Self = Self {
            index: 0u32,
            offset: 0u32,
        };
        // actual numbers don't matter to much
        const SHIFT_INDEX: u32 = 0x682f0161;
        const SHIFT_OFFSET: u32 = 0x46790905;

        s.index =  s.permutate(s.permutate(seed_base).wrapping_add(SHIFT_INDEX));
        s.offset = s.permutate(s.permutate(seed_offset).wrapping_add(SHIFT_OFFSET));
        return s
    }

    /// Bijective function using complete range of u32
    /// projecting index element of u32 into u32
    pub fn current(&self) -> u32 {
        // addition and xor is a bijective function on integers
        // permutate is also a bijective function on integers
        // with that it just projects the index (which is element of i32)
        // into a new number bijectively (also in i32)
        const XOR_CONST: u32 = 0x5bf03635;
        return self.permutate((self.permutate(self.index).wrapping_add(self.offset)) ^ XOR_CONST);
    }

    pub fn next(&mut self) -> u32{
        // Linearly counting up through all the random numbers
        self.index = self.index.wrapping_add(1);
        return self.current();
    }

}





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_for_no_repeat(){
        fn progress_report(n: u64)
        {
            if n % 0xFFFFFFu64 == 0 {
                println!("Progress: {:3}/256", n/0xFFFFFFu64)
            }
        }
        let mut a =  Box::new(BitArray::default());
        let mut r = RandomSequenceOfUniqueU32::new(1, 3);

        // This loop procuces as many random numbers as there are integers in u32
        // The bitfield is used to check that none of the random numbers are repeated
        for i in 0..(0xFFFFFFFFu64+1u64) {
            progress_report(i);

            let x = r.next();
            // if this assert fails a number was repeated
            assert!(a.set_bit_if_not_already_set(x));
        }

        let x = r.next();
        // The next line should evaluate to false, since the loop before filled up the complete bitfield
        if a.set_bit_if_not_already_set(x) {
            assert!(false);
        }
    }


    // Below is only implementation of the bit array that is used in above test
    // If the random numbers are actually not repeating in u32
    // Essentially only test supporting code from here on out.
    const BIT_ARRAY_SIZE: usize = 0x20000000;

    struct BitArray {
        data: Vec<u8>,
    }
    
    impl Default for BitArray {
        fn default() -> Self {
            Self {
                data: vec![0u8; BIT_ARRAY_SIZE],
            }
        }
    }
    impl BitArray {
        fn set_bit_if_not_already_set(&mut self, n: u32) -> bool {
          const BITS_PER_BYTE: u32 = 8;
          let array_index: usize = (n / BITS_PER_BYTE).try_into().unwrap();
          let array_sub_index = n % BITS_PER_BYTE;
    
          let already_set: bool = ((self.data[array_index] >> array_sub_index) & 1) == 1;
          if already_set {
            return false;
          }
          else {
            self.data[array_index] |= 1 << array_sub_index;
            return true;
          }
        }
    }

    impl BitArray {
        fn get_element(&self, n: usize) -> u8 {
            return self.data[n];
        }
        fn count_bits(&self) -> u64
        {
            fn progress_report(n: u64)
            {
                if n % 0xFFFFFFu64 == 0 {
                    println!("Bit count progress: {:3}/32", n/0xFFFFFFu64)
                }
            }
            let mut counter = 0u64;
            let mut i = 0u64;
            for v in self.data.iter() {
                let mut bits_in_byte = 0;
                for i in 0..8 {
                    let set = (v >> i) & 1;
                    if set == 1 {
                        bits_in_byte += 1;
                    }
                }
                counter += bits_in_byte;
                progress_report( { i += 1; i } );
            }
            return counter;
        }
    }

    #[test]
    fn test_set_bit() {
        let mut a =  Box::new(BitArray::default());
        a.set_bit_if_not_already_set(0);  // 0 bit in first byte
        a.set_bit_if_not_already_set(3);  // 3 bit in first byte
        a.set_bit_if_not_already_set(8);  // 0 bit in second byte
        a.set_bit_if_not_already_set(23); // 7 bit in third byte
        a.set_bit_if_not_already_set(0xFFFFFFFF); // last bit in array
        assert_eq!(a.get_element(0), 9); // 1 + 8 == 9
        assert_eq!(a.get_element(1), 1);
        assert_eq!(a.get_element(2), 128);
        assert_eq!(a.get_element(0x1FFFFFFF), 128);
    }

    #[test]
    #[ignore]
    fn test_set_bit_long() {
        let mut a =  Box::new(BitArray::default());
        a.set_bit_if_not_already_set(0);  // 0 bit in first byte
        a.set_bit_if_not_already_set(3);  // 3 bit in first byte
        a.set_bit_if_not_already_set(8);  // 0 bit in second byte
        a.set_bit_if_not_already_set(23); // 7 bit in third byte
        a.set_bit_if_not_already_set(0xFFFFFFFF); // last bit in array
        assert_eq!(a.get_element(0), 9); // 1 + 8 == 9
        assert_eq!(a.get_element(1), 1);
        assert_eq!(a.get_element(2), 128);
        assert_eq!(a.get_element(0x1FFFFFFF), 128);
        // This line takes quite long
        assert_eq!(a.count_bits(), 5);
    }

    #[test]
    fn test_set_bit_if_not_set() {
        let mut a =  Box::new(BitArray::default());
        assert_eq!(a.set_bit_if_not_already_set(3), true);
        assert_eq!(a.set_bit_if_not_already_set(3), false);
        assert_eq!(a.get_element(0), 8);
    }
}
