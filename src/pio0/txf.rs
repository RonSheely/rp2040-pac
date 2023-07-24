#[doc = "Register `TXF%s` writer"]
pub struct W(crate::W<TXF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<TXF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXF_SPEC>) -> Self {
        W(writer)
    }
}
impl core::fmt::Debug for crate::generic::Reg<TXF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Direct write access to the TX FIFO for this state machine. Each write pushes one word to the FIFO. Attempting to write to a full FIFO has no effect on the FIFO state or contents, and sets the sticky FDEBUG_TXOVER error flag for this FIFO.  

This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [txf](index.html) module"]
pub struct TXF_SPEC;
impl crate::RegisterSpec for TXF_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [txf::W](W) writer structure"]
impl crate::Writable for TXF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXF%s to value 0"]
impl crate::Resettable for TXF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
