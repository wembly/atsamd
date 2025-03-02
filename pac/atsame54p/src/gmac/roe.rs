#[doc = "Register `ROE` reader"]
pub struct R(crate::R<ROE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ROE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ROE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ROE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXOVR` reader - Receive Overruns"]
pub struct RXOVR_R(crate::FieldReader<u16, u16>);
impl RXOVR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        RXOVR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXOVR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:9 - Receive Overruns"]
    #[inline(always)]
    pub fn rxovr(&self) -> RXOVR_R {
        RXOVR_R::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "Receive Overrun Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [roe](index.html) module"]
pub struct ROE_SPEC;
impl crate::RegisterSpec for ROE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [roe::R](R) reader structure"]
impl crate::Readable for ROE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ROE to value 0"]
impl crate::Resettable for ROE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
