#[doc = "Register `SETUP_0` reader"]
pub struct R(crate::R<SETUP_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SETUP_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SETUP_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SETUP_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SETUP_0` writer"]
pub struct W(crate::W<SETUP_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SETUP_0_SPEC>;
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
impl From<crate::W<SETUP_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SETUP_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `YEAR` reader - Year"]
pub struct YEAR_R(crate::FieldReader<u16, u16>);
impl YEAR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        YEAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for YEAR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `YEAR` writer - Year"]
pub struct YEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> YEAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 12)) | ((value as u32 & 0x0fff) << 12);
        self.w
    }
}
#[doc = "Field `MONTH` reader - Month (1..12)"]
pub struct MONTH_R(crate::FieldReader<u8, u8>);
impl MONTH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MONTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MONTH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MONTH` writer - Month (1..12)"]
pub struct MONTH_W<'a> {
    w: &'a mut W,
}
impl<'a> MONTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `DAY` reader - Day of the month (1..31)"]
pub struct DAY_R(crate::FieldReader<u8, u8>);
impl DAY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DAY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAY` writer - Day of the month (1..31)"]
pub struct DAY_W<'a> {
    w: &'a mut W,
}
impl<'a> DAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 12:23 - Year"]
    #[inline(always)]
    pub fn year(&self) -> YEAR_R {
        YEAR_R::new(((self.bits >> 12) & 0x0fff) as u16)
    }
    #[doc = "Bits 8:11 - Month (1..12)"]
    #[inline(always)]
    pub fn month(&self) -> MONTH_R {
        MONTH_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 0:4 - Day of the month (1..31)"]
    #[inline(always)]
    pub fn day(&self) -> DAY_R {
        DAY_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 12:23 - Year"]
    #[inline(always)]
    pub fn year(&mut self) -> YEAR_W {
        YEAR_W { w: self }
    }
    #[doc = "Bits 8:11 - Month (1..12)"]
    #[inline(always)]
    pub fn month(&mut self) -> MONTH_W {
        MONTH_W { w: self }
    }
    #[doc = "Bits 0:4 - Day of the month (1..31)"]
    #[inline(always)]
    pub fn day(&mut self) -> DAY_W {
        DAY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC setup register 0  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [setup_0](index.html) module"]
pub struct SETUP_0_SPEC;
impl crate::RegisterSpec for SETUP_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [setup_0::R](R) reader structure"]
impl crate::Readable for SETUP_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [setup_0::W](W) writer structure"]
impl crate::Writable for SETUP_0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SETUP_0 to value 0"]
impl crate::Resettable for SETUP_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
