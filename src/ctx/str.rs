use {TryFromCtx, TryIntoCtx, Error, Result, assert_len};
use std::str;

#[derive(Debug)]
pub enum StrCtx {
    Delimiter(u8),
    DelimiterUntil(u8, usize),
    Length(usize),
}

pub const NULL: u8 = 0;
pub const SPACE: u8 = 0x20;
pub const RET: u8 = 0x0a;
pub const TAB: u8 = 0x09;

impl<'a> TryFromCtx<'a, StrCtx, str::Utf8Error> for &'a str {
    #[inline]
    fn try_from_ctx(scroll: &'a [u8], ctx: StrCtx) -> Result<(Self, usize), str::Utf8Error> {
        let len = match ctx {
            StrCtx::Length(len) => {
                assert_len(scroll, len)?;
                len
            }
            StrCtx::Delimiter(delimiter) => {
                scroll
                    .iter()
                    .position(|c| *c == delimiter)
                    .ok_or(Error::Incomplete)?
            }
            StrCtx::DelimiterUntil(delimiter, len) => {
                let len = scroll
                    .iter()
                    .take(len)
                    .position(|c| *c == delimiter)
                    .unwrap_or(len);
                assert_len(scroll, len)?;
                len
            }
        };

        str::from_utf8(&scroll[..len])
            .map(|s| (s, len))
            .map_err(Into::into)
    }
}

impl<'a> TryIntoCtx for &'a str {
    #[inline]
    fn try_into_ctx(self, scroll: &mut [u8], _ctx: ()) -> Result<usize, ()> {
        let bytes = self.as_bytes();

        assert_len(scroll, bytes.len())?;

        scroll[..bytes.len()].clone_from_slice(bytes);

        Ok(bytes.len())
    }
}