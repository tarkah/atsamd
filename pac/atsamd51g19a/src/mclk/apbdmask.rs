#[doc = "Reader of register APBDMASK"]
pub type R = crate::R<u32, super::APBDMASK>;
#[doc = "Writer for register APBDMASK"]
pub type W = crate::W<u32, super::APBDMASK>;
#[doc = "Register APBDMASK `reset()`'s with value 0"]
impl crate::ResetValue for super::APBDMASK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SERCOM4_`"]
pub type SERCOM4__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SERCOM4_`"]
pub struct SERCOM4__W<'a> {
    w: &'a mut W,
}
impl<'a> SERCOM4__W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `SERCOM5_`"]
pub type SERCOM5__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SERCOM5_`"]
pub struct SERCOM5__W<'a> {
    w: &'a mut W,
}
impl<'a> SERCOM5__W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `ADC0_`"]
pub type ADC0__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC0_`"]
pub struct ADC0__W<'a> {
    w: &'a mut W,
}
impl<'a> ADC0__W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `ADC1_`"]
pub type ADC1__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC1_`"]
pub struct ADC1__W<'a> {
    w: &'a mut W,
}
impl<'a> ADC1__W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `DAC_`"]
pub type DAC__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DAC_`"]
pub struct DAC__W<'a> {
    w: &'a mut W,
}
impl<'a> DAC__W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `PCC_`"]
pub type PCC__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCC_`"]
pub struct PCC__W<'a> {
    w: &'a mut W,
}
impl<'a> PCC__W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - SERCOM4 APB Clock Enable"]
    #[inline(always)]
    pub fn sercom4_(&self) -> SERCOM4__R {
        SERCOM4__R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SERCOM5 APB Clock Enable"]
    #[inline(always)]
    pub fn sercom5_(&self) -> SERCOM5__R {
        SERCOM5__R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 7 - ADC0 APB Clock Enable"]
    #[inline(always)]
    pub fn adc0_(&self) -> ADC0__R {
        ADC0__R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - ADC1 APB Clock Enable"]
    #[inline(always)]
    pub fn adc1_(&self) -> ADC1__R {
        ADC1__R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - DAC APB Clock Enable"]
    #[inline(always)]
    pub fn dac_(&self) -> DAC__R {
        DAC__R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 11 - PCC APB Clock Enable"]
    #[inline(always)]
    pub fn pcc_(&self) -> PCC__R {
        PCC__R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SERCOM4 APB Clock Enable"]
    #[inline(always)]
    pub fn sercom4_(&mut self) -> SERCOM4__W {
        SERCOM4__W { w: self }
    }
    #[doc = "Bit 1 - SERCOM5 APB Clock Enable"]
    #[inline(always)]
    pub fn sercom5_(&mut self) -> SERCOM5__W {
        SERCOM5__W { w: self }
    }
    #[doc = "Bit 7 - ADC0 APB Clock Enable"]
    #[inline(always)]
    pub fn adc0_(&mut self) -> ADC0__W {
        ADC0__W { w: self }
    }
    #[doc = "Bit 8 - ADC1 APB Clock Enable"]
    #[inline(always)]
    pub fn adc1_(&mut self) -> ADC1__W {
        ADC1__W { w: self }
    }
    #[doc = "Bit 9 - DAC APB Clock Enable"]
    #[inline(always)]
    pub fn dac_(&mut self) -> DAC__W {
        DAC__W { w: self }
    }
    #[doc = "Bit 11 - PCC APB Clock Enable"]
    #[inline(always)]
    pub fn pcc_(&mut self) -> PCC__W {
        PCC__W { w: self }
    }
}
