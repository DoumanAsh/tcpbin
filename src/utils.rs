use core::{fmt, cmp, net};

pub struct FmtBuffer<'a> {
    inner: &'a mut [u8],
    len: usize,
}

impl<'a> FmtBuffer<'a> {
    #[inline(always)]
    pub const fn new(inner: &'a mut [u8]) -> Self {
        Self {
            inner,
            len: 0,
        }
    }

    ///Returns number of bytes copied from `data`
    pub fn extend_from_slice(&mut self, data: &[u8]) -> usize {
        let copy_len = cmp::min(self.inner.len() - self.len, data.len());
        self.inner[self.len..self.len+copy_len].copy_from_slice(&data[..copy_len]);
        self.len += copy_len;
        copy_len
    }

    #[inline(always)]
    pub fn format_addr(&mut self, addr: net::SocketAddr) {
        let _ = fmt::Write::write_fmt(self, format_args!("{}", addr.ip()));
    }

    #[inline(always)]
    pub fn written_data(&'a self) -> &'a [u8] {
        &self.inner[..self.len]
    }
}

impl fmt::Write for FmtBuffer<'_> {
    #[inline(always)]
    fn write_str(&mut self, text: &str) -> fmt::Result {
        self.extend_from_slice(text.as_bytes());
        Ok(())
    }
}
