use bitflags::bitflags;
use std::fmt::{Display, Formatter, Result};

bitflags! {
    /// MyFlags is a set of flags.
    ///
    /// # Example
    ///
    /// ```
    /// use data_structures::MyFlags;
    ///
    /// let e1 = MyFlags::FLAG_A | MyFlags::FLAG_C;
    /// println!("{:?}", e1); // MyFlags(FLAG_A | FLAG_C)
    /// ```
    #[derive(Clone, Copy, PartialEq, Debug)]
    pub struct MyFlags: u32 {
        const FLAG_A       = 0b00000001;
        const FLAG_B       = 0b00000010;
        const FLAG_C       = 0b00000100;
        const FLAG_ABC = Self::FLAG_A.bits()
                           | Self::FLAG_B.bits()
                           | Self::FLAG_C.bits();
    }
}

impl MyFlags {
    /// Clears all flags.
    ///
    /// # Example
    ///
    /// ```
    /// use data_structures::MyFlags;
    ///
    /// let mut flags = MyFlags::FLAG_ABC;
    /// flags.clear();
    /// println!("{:?}", flags); // MyFlags(0)
    /// ```
    pub fn clear(&mut self) -> &mut MyFlags {
        *self = MyFlags::empty();
        self
    }
}

impl Display for MyFlags {
    /// Formats the flag as a 32-bit binary number.
    ///
    /// # Example
    ///
    /// ```
    /// use data_structures::MyFlags;
    ///
    /// let flags = MyFlags::FLAG_ABC;
    /// println!("{}", flags); // 00000000000000000000000000000111
    /// ```
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:032b}", self.bits())
    }
}
