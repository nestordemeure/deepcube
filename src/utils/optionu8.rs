/// simulate an option under the hypothesize that the maximum value will never be used
pub struct OptionU8
{
    content: u8
}

impl OptionU8
{
    /// value used to represent none
    const NONE_VALUE: u8 = u8::MAX;

    /// creates a new, empty, option
    pub fn none() -> OptionU8
    {
        let content = Self::NONE_VALUE;
        OptionU8 { content }
    }

    /// sets the Option to the given value
    /// returns true if the option was None
    /// does nothing if the option was some
    pub fn set(&mut self, value: u8) -> bool
    {
        debug_assert_ne!(value, Self::NONE_VALUE);
        if self.content == Self::NONE_VALUE
        {
            self.content = value;
            true
        }
        else
        {
            false
        }
    }

    /// unwraps the option into its inner value
    pub fn unwrap(self) -> u8
    {
        if self.content == Self::NONE_VALUE
        {
            panic!("OptionU8: unable to unwrap None!")
        }
        self.content
    }
}
