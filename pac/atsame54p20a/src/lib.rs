#![doc = "Peripheral access API for ATSAME54P20A microcontrollers (generated using svd2rust v0.16.1)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.16.1/svd2rust/#peripheral-api"]
#![deny(const_err)]
#![deny(dead_code)]
#![deny(improper_ctypes)]
#![deny(legacy_directory_ownership)]
#![deny(missing_docs)]
#![deny(no_mangle_generic_items)]
#![deny(non_shorthand_field_patterns)]
#![deny(overflowing_literals)]
#![deny(path_statements)]
#![deny(patterns_in_fns_without_body)]
#![deny(plugin_as_library)]
#![deny(private_in_public)]
#![deny(safe_extern_statics)]
#![deny(unconditional_recursion)]
#![deny(unions_with_drop_fields)]
#![deny(unused_allocation)]
#![deny(unused_comparisons)]
#![deny(unused_parens)]
#![deny(while_true)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
extern crate bare_metal;
extern crate cortex_m;
#[cfg(feature = "rt")]
extern crate cortex_m_rt;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r"Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 3;
#[cfg(feature = "rt")]
extern "C" {
    fn PM_INTREQ();
    fn MCLK_INTREQ();
    fn OSCCTRL_INTREQ_0();
    fn OSCCTRL_INTREQ_1();
    fn OSCCTRL_INTREQ_2();
    fn OSCCTRL_INTREQ_3();
    fn OSCCTRL_INTREQ_4();
    fn OSC32KCTRL_INTREQ();
    fn SUPC_INTREQ_0();
    fn SUPC_INTREQ_1();
    fn WDT_INTREQ();
    fn RTC_INTREQ();
    fn EIC_INTREQ_0();
    fn EIC_INTREQ_1();
    fn EIC_INTREQ_2();
    fn EIC_INTREQ_3();
    fn EIC_INTREQ_4();
    fn EIC_INTREQ_5();
    fn EIC_INTREQ_6();
    fn EIC_INTREQ_7();
    fn EIC_INTREQ_8();
    fn EIC_INTREQ_9();
    fn EIC_INTREQ_10();
    fn EIC_INTREQ_11();
    fn EIC_INTREQ_12();
    fn EIC_INTREQ_13();
    fn EIC_INTREQ_14();
    fn EIC_INTREQ_15();
    fn FREQM_INTREQ();
    fn NVMCTRL_INTREQ_0();
    fn NVMCTRL_INTREQ_1();
    fn DMAC_INTREQ_0();
    fn DMAC_INTREQ_1();
    fn DMAC_INTREQ_2();
    fn DMAC_INTREQ_3();
    fn DMAC_INTREQ_4();
    fn EVSYS_INTREQ_0();
    fn EVSYS_INTREQ_1();
    fn EVSYS_INTREQ_2();
    fn EVSYS_INTREQ_3();
    fn EVSYS_INTREQ_4();
    fn PAC_INTREQ();
    fn TAL_INTREQ_0();
    fn TAL_INTREQ_1();
    fn RAMECC_INTREQ();
    fn SERCOM0_INTREQ_0();
    fn SERCOM0_INTREQ_1();
    fn SERCOM0_INTREQ_2();
    fn SERCOM0_INTREQ_3();
    fn SERCOM1_INTREQ_0();
    fn SERCOM1_INTREQ_1();
    fn SERCOM1_INTREQ_2();
    fn SERCOM1_INTREQ_3();
    fn SERCOM2_INTREQ_0();
    fn SERCOM2_INTREQ_1();
    fn SERCOM2_INTREQ_2();
    fn SERCOM2_INTREQ_3();
    fn SERCOM3_INTREQ_0();
    fn SERCOM3_INTREQ_1();
    fn SERCOM3_INTREQ_2();
    fn SERCOM3_INTREQ_3();
    fn SERCOM4_INTREQ_0();
    fn SERCOM4_INTREQ_1();
    fn SERCOM4_INTREQ_2();
    fn SERCOM4_INTREQ_3();
    fn SERCOM5_INTREQ_0();
    fn SERCOM5_INTREQ_1();
    fn SERCOM5_INTREQ_2();
    fn SERCOM5_INTREQ_3();
    fn SERCOM6_INTREQ_0();
    fn SERCOM6_INTREQ_1();
    fn SERCOM6_INTREQ_2();
    fn SERCOM6_INTREQ_3();
    fn SERCOM7_INTREQ_0();
    fn SERCOM7_INTREQ_1();
    fn SERCOM7_INTREQ_2();
    fn SERCOM7_INTREQ_3();
    fn CAN0_INTREQ();
    fn CAN1_INTREQ();
    fn USB_INTREQ_0();
    fn USB_INTREQ_1();
    fn USB_INTREQ_2();
    fn USB_INTREQ_3();
    fn GMAC_INTREQ();
    fn TCC0_INTREQ_0();
    fn TCC0_INTREQ_1();
    fn TCC0_INTREQ_2();
    fn TCC0_INTREQ_3();
    fn TCC0_INTREQ_4();
    fn TCC0_INTREQ_5();
    fn TCC0_INTREQ_6();
    fn TCC1_INTREQ_0();
    fn TCC1_INTREQ_1();
    fn TCC1_INTREQ_2();
    fn TCC1_INTREQ_3();
    fn TCC1_INTREQ_4();
    fn TCC2_INTREQ_0();
    fn TCC2_INTREQ_1();
    fn TCC2_INTREQ_2();
    fn TCC2_INTREQ_3();
    fn TCC3_INTREQ_0();
    fn TCC3_INTREQ_1();
    fn TCC3_INTREQ_2();
    fn TCC4_INTREQ_0();
    fn TCC4_INTREQ_1();
    fn TCC4_INTREQ_2();
    fn TC0_INTREQ();
    fn TC1_INTREQ();
    fn TC2_INTREQ();
    fn TC3_INTREQ();
    fn TC4_INTREQ();
    fn TC5_INTREQ();
    fn TC6_INTREQ();
    fn TC7_INTREQ();
    fn PDEC_INTREQ_0();
    fn PDEC_INTREQ_1();
    fn PDEC_INTREQ_2();
    fn ADC0_INTREQ_0();
    fn ADC0_INTREQ_1();
    fn ADC1_INTREQ_0();
    fn ADC1_INTREQ_1();
    fn AC_INTREQ();
    fn DAC_INTREQ_0();
    fn DAC_INTREQ_1();
    fn DAC_INTREQ_2();
    fn DAC_INTREQ_3();
    fn DAC_INTREQ_4();
    fn I2S_INTREQ();
    fn PCC_INTREQ();
    fn AES_INTREQ();
    fn TRNG_INTREQ();
    fn ICM_INTREQ();
    fn QSPI_INTREQ();
    fn SDHC0_INTREQ();
    fn SDHC1_INTREQ();
}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 137] = [
    Vector {
        _handler: PM_INTREQ,
    },
    Vector {
        _handler: MCLK_INTREQ,
    },
    Vector {
        _handler: OSCCTRL_INTREQ_0,
    },
    Vector {
        _handler: OSCCTRL_INTREQ_1,
    },
    Vector {
        _handler: OSCCTRL_INTREQ_2,
    },
    Vector {
        _handler: OSCCTRL_INTREQ_3,
    },
    Vector {
        _handler: OSCCTRL_INTREQ_4,
    },
    Vector {
        _handler: OSC32KCTRL_INTREQ,
    },
    Vector {
        _handler: SUPC_INTREQ_0,
    },
    Vector {
        _handler: SUPC_INTREQ_1,
    },
    Vector {
        _handler: WDT_INTREQ,
    },
    Vector {
        _handler: RTC_INTREQ,
    },
    Vector {
        _handler: EIC_INTREQ_0,
    },
    Vector {
        _handler: EIC_INTREQ_1,
    },
    Vector {
        _handler: EIC_INTREQ_2,
    },
    Vector {
        _handler: EIC_INTREQ_3,
    },
    Vector {
        _handler: EIC_INTREQ_4,
    },
    Vector {
        _handler: EIC_INTREQ_5,
    },
    Vector {
        _handler: EIC_INTREQ_6,
    },
    Vector {
        _handler: EIC_INTREQ_7,
    },
    Vector {
        _handler: EIC_INTREQ_8,
    },
    Vector {
        _handler: EIC_INTREQ_9,
    },
    Vector {
        _handler: EIC_INTREQ_10,
    },
    Vector {
        _handler: EIC_INTREQ_11,
    },
    Vector {
        _handler: EIC_INTREQ_12,
    },
    Vector {
        _handler: EIC_INTREQ_13,
    },
    Vector {
        _handler: EIC_INTREQ_14,
    },
    Vector {
        _handler: EIC_INTREQ_15,
    },
    Vector {
        _handler: FREQM_INTREQ,
    },
    Vector {
        _handler: NVMCTRL_INTREQ_0,
    },
    Vector {
        _handler: NVMCTRL_INTREQ_1,
    },
    Vector {
        _handler: DMAC_INTREQ_0,
    },
    Vector {
        _handler: DMAC_INTREQ_1,
    },
    Vector {
        _handler: DMAC_INTREQ_2,
    },
    Vector {
        _handler: DMAC_INTREQ_3,
    },
    Vector {
        _handler: DMAC_INTREQ_4,
    },
    Vector {
        _handler: EVSYS_INTREQ_0,
    },
    Vector {
        _handler: EVSYS_INTREQ_1,
    },
    Vector {
        _handler: EVSYS_INTREQ_2,
    },
    Vector {
        _handler: EVSYS_INTREQ_3,
    },
    Vector {
        _handler: EVSYS_INTREQ_4,
    },
    Vector {
        _handler: PAC_INTREQ,
    },
    Vector {
        _handler: TAL_INTREQ_0,
    },
    Vector {
        _handler: TAL_INTREQ_1,
    },
    Vector { _reserved: 0 },
    Vector {
        _handler: RAMECC_INTREQ,
    },
    Vector {
        _handler: SERCOM0_INTREQ_0,
    },
    Vector {
        _handler: SERCOM0_INTREQ_1,
    },
    Vector {
        _handler: SERCOM0_INTREQ_2,
    },
    Vector {
        _handler: SERCOM0_INTREQ_3,
    },
    Vector {
        _handler: SERCOM1_INTREQ_0,
    },
    Vector {
        _handler: SERCOM1_INTREQ_1,
    },
    Vector {
        _handler: SERCOM1_INTREQ_2,
    },
    Vector {
        _handler: SERCOM1_INTREQ_3,
    },
    Vector {
        _handler: SERCOM2_INTREQ_0,
    },
    Vector {
        _handler: SERCOM2_INTREQ_1,
    },
    Vector {
        _handler: SERCOM2_INTREQ_2,
    },
    Vector {
        _handler: SERCOM2_INTREQ_3,
    },
    Vector {
        _handler: SERCOM3_INTREQ_0,
    },
    Vector {
        _handler: SERCOM3_INTREQ_1,
    },
    Vector {
        _handler: SERCOM3_INTREQ_2,
    },
    Vector {
        _handler: SERCOM3_INTREQ_3,
    },
    Vector {
        _handler: SERCOM4_INTREQ_0,
    },
    Vector {
        _handler: SERCOM4_INTREQ_1,
    },
    Vector {
        _handler: SERCOM4_INTREQ_2,
    },
    Vector {
        _handler: SERCOM4_INTREQ_3,
    },
    Vector {
        _handler: SERCOM5_INTREQ_0,
    },
    Vector {
        _handler: SERCOM5_INTREQ_1,
    },
    Vector {
        _handler: SERCOM5_INTREQ_2,
    },
    Vector {
        _handler: SERCOM5_INTREQ_3,
    },
    Vector {
        _handler: SERCOM6_INTREQ_0,
    },
    Vector {
        _handler: SERCOM6_INTREQ_1,
    },
    Vector {
        _handler: SERCOM6_INTREQ_2,
    },
    Vector {
        _handler: SERCOM6_INTREQ_3,
    },
    Vector {
        _handler: SERCOM7_INTREQ_0,
    },
    Vector {
        _handler: SERCOM7_INTREQ_1,
    },
    Vector {
        _handler: SERCOM7_INTREQ_2,
    },
    Vector {
        _handler: SERCOM7_INTREQ_3,
    },
    Vector {
        _handler: CAN0_INTREQ,
    },
    Vector {
        _handler: CAN1_INTREQ,
    },
    Vector {
        _handler: USB_INTREQ_0,
    },
    Vector {
        _handler: USB_INTREQ_1,
    },
    Vector {
        _handler: USB_INTREQ_2,
    },
    Vector {
        _handler: USB_INTREQ_3,
    },
    Vector {
        _handler: GMAC_INTREQ,
    },
    Vector {
        _handler: TCC0_INTREQ_0,
    },
    Vector {
        _handler: TCC0_INTREQ_1,
    },
    Vector {
        _handler: TCC0_INTREQ_2,
    },
    Vector {
        _handler: TCC0_INTREQ_3,
    },
    Vector {
        _handler: TCC0_INTREQ_4,
    },
    Vector {
        _handler: TCC0_INTREQ_5,
    },
    Vector {
        _handler: TCC0_INTREQ_6,
    },
    Vector {
        _handler: TCC1_INTREQ_0,
    },
    Vector {
        _handler: TCC1_INTREQ_1,
    },
    Vector {
        _handler: TCC1_INTREQ_2,
    },
    Vector {
        _handler: TCC1_INTREQ_3,
    },
    Vector {
        _handler: TCC1_INTREQ_4,
    },
    Vector {
        _handler: TCC2_INTREQ_0,
    },
    Vector {
        _handler: TCC2_INTREQ_1,
    },
    Vector {
        _handler: TCC2_INTREQ_2,
    },
    Vector {
        _handler: TCC2_INTREQ_3,
    },
    Vector {
        _handler: TCC3_INTREQ_0,
    },
    Vector {
        _handler: TCC3_INTREQ_1,
    },
    Vector {
        _handler: TCC3_INTREQ_2,
    },
    Vector {
        _handler: TCC4_INTREQ_0,
    },
    Vector {
        _handler: TCC4_INTREQ_1,
    },
    Vector {
        _handler: TCC4_INTREQ_2,
    },
    Vector {
        _handler: TC0_INTREQ,
    },
    Vector {
        _handler: TC1_INTREQ,
    },
    Vector {
        _handler: TC2_INTREQ,
    },
    Vector {
        _handler: TC3_INTREQ,
    },
    Vector {
        _handler: TC4_INTREQ,
    },
    Vector {
        _handler: TC5_INTREQ,
    },
    Vector {
        _handler: TC6_INTREQ,
    },
    Vector {
        _handler: TC7_INTREQ,
    },
    Vector {
        _handler: PDEC_INTREQ_0,
    },
    Vector {
        _handler: PDEC_INTREQ_1,
    },
    Vector {
        _handler: PDEC_INTREQ_2,
    },
    Vector {
        _handler: ADC0_INTREQ_0,
    },
    Vector {
        _handler: ADC0_INTREQ_1,
    },
    Vector {
        _handler: ADC1_INTREQ_0,
    },
    Vector {
        _handler: ADC1_INTREQ_1,
    },
    Vector {
        _handler: AC_INTREQ,
    },
    Vector {
        _handler: DAC_INTREQ_0,
    },
    Vector {
        _handler: DAC_INTREQ_1,
    },
    Vector {
        _handler: DAC_INTREQ_2,
    },
    Vector {
        _handler: DAC_INTREQ_3,
    },
    Vector {
        _handler: DAC_INTREQ_4,
    },
    Vector {
        _handler: I2S_INTREQ,
    },
    Vector {
        _handler: PCC_INTREQ,
    },
    Vector {
        _handler: AES_INTREQ,
    },
    Vector {
        _handler: TRNG_INTREQ,
    },
    Vector {
        _handler: ICM_INTREQ,
    },
    Vector { _reserved: 0 },
    Vector {
        _handler: QSPI_INTREQ,
    },
    Vector {
        _handler: SDHC0_INTREQ,
    },
    Vector {
        _handler: SDHC1_INTREQ,
    },
];
#[doc = r"Enumeration of all the interrupts"]
#[derive(Copy, Clone, Debug)]
pub enum Interrupt {
    #[doc = "0 - PM_INTREQ"]
    PM_INTREQ,
    #[doc = "1 - MCLK_INTREQ"]
    MCLK_INTREQ,
    #[doc = "2 - OSCCTRL_INTREQ_0"]
    OSCCTRL_INTREQ_0,
    #[doc = "3 - OSCCTRL_INTREQ_1"]
    OSCCTRL_INTREQ_1,
    #[doc = "4 - OSCCTRL_INTREQ_2"]
    OSCCTRL_INTREQ_2,
    #[doc = "5 - OSCCTRL_INTREQ_3"]
    OSCCTRL_INTREQ_3,
    #[doc = "6 - OSCCTRL_INTREQ_4"]
    OSCCTRL_INTREQ_4,
    #[doc = "7 - OSC32KCTRL_INTREQ"]
    OSC32KCTRL_INTREQ,
    #[doc = "8 - SUPC_INTREQ_0"]
    SUPC_INTREQ_0,
    #[doc = "9 - SUPC_INTREQ_1"]
    SUPC_INTREQ_1,
    #[doc = "10 - WDT_INTREQ"]
    WDT_INTREQ,
    #[doc = "11 - RTC_INTREQ"]
    RTC_INTREQ,
    #[doc = "12 - EIC_INTREQ_0"]
    EIC_INTREQ_0,
    #[doc = "13 - EIC_INTREQ_1"]
    EIC_INTREQ_1,
    #[doc = "14 - EIC_INTREQ_2"]
    EIC_INTREQ_2,
    #[doc = "15 - EIC_INTREQ_3"]
    EIC_INTREQ_3,
    #[doc = "16 - EIC_INTREQ_4"]
    EIC_INTREQ_4,
    #[doc = "17 - EIC_INTREQ_5"]
    EIC_INTREQ_5,
    #[doc = "18 - EIC_INTREQ_6"]
    EIC_INTREQ_6,
    #[doc = "19 - EIC_INTREQ_7"]
    EIC_INTREQ_7,
    #[doc = "20 - EIC_INTREQ_8"]
    EIC_INTREQ_8,
    #[doc = "21 - EIC_INTREQ_9"]
    EIC_INTREQ_9,
    #[doc = "22 - EIC_INTREQ_10"]
    EIC_INTREQ_10,
    #[doc = "23 - EIC_INTREQ_11"]
    EIC_INTREQ_11,
    #[doc = "24 - EIC_INTREQ_12"]
    EIC_INTREQ_12,
    #[doc = "25 - EIC_INTREQ_13"]
    EIC_INTREQ_13,
    #[doc = "26 - EIC_INTREQ_14"]
    EIC_INTREQ_14,
    #[doc = "27 - EIC_INTREQ_15"]
    EIC_INTREQ_15,
    #[doc = "28 - FREQM_INTREQ"]
    FREQM_INTREQ,
    #[doc = "29 - NVMCTRL_INTREQ_0"]
    NVMCTRL_INTREQ_0,
    #[doc = "30 - NVMCTRL_INTREQ_1"]
    NVMCTRL_INTREQ_1,
    #[doc = "31 - DMAC_INTREQ_0"]
    DMAC_INTREQ_0,
    #[doc = "32 - DMAC_INTREQ_1"]
    DMAC_INTREQ_1,
    #[doc = "33 - DMAC_INTREQ_2"]
    DMAC_INTREQ_2,
    #[doc = "34 - DMAC_INTREQ_3"]
    DMAC_INTREQ_3,
    #[doc = "35 - DMAC_INTREQ_4"]
    DMAC_INTREQ_4,
    #[doc = "36 - EVSYS_INTREQ_0"]
    EVSYS_INTREQ_0,
    #[doc = "37 - EVSYS_INTREQ_1"]
    EVSYS_INTREQ_1,
    #[doc = "38 - EVSYS_INTREQ_2"]
    EVSYS_INTREQ_2,
    #[doc = "39 - EVSYS_INTREQ_3"]
    EVSYS_INTREQ_3,
    #[doc = "40 - EVSYS_INTREQ_4"]
    EVSYS_INTREQ_4,
    #[doc = "41 - PAC_INTREQ"]
    PAC_INTREQ,
    #[doc = "42 - TAL_INTREQ_0"]
    TAL_INTREQ_0,
    #[doc = "43 - TAL_INTREQ_1"]
    TAL_INTREQ_1,
    #[doc = "45 - RAMECC_INTREQ"]
    RAMECC_INTREQ,
    #[doc = "46 - SERCOM0_INTREQ_0"]
    SERCOM0_INTREQ_0,
    #[doc = "47 - SERCOM0_INTREQ_1"]
    SERCOM0_INTREQ_1,
    #[doc = "48 - SERCOM0_INTREQ_2"]
    SERCOM0_INTREQ_2,
    #[doc = "49 - SERCOM0_INTREQ_3"]
    SERCOM0_INTREQ_3,
    #[doc = "50 - SERCOM1_INTREQ_0"]
    SERCOM1_INTREQ_0,
    #[doc = "51 - SERCOM1_INTREQ_1"]
    SERCOM1_INTREQ_1,
    #[doc = "52 - SERCOM1_INTREQ_2"]
    SERCOM1_INTREQ_2,
    #[doc = "53 - SERCOM1_INTREQ_3"]
    SERCOM1_INTREQ_3,
    #[doc = "54 - SERCOM2_INTREQ_0"]
    SERCOM2_INTREQ_0,
    #[doc = "55 - SERCOM2_INTREQ_1"]
    SERCOM2_INTREQ_1,
    #[doc = "56 - SERCOM2_INTREQ_2"]
    SERCOM2_INTREQ_2,
    #[doc = "57 - SERCOM2_INTREQ_3"]
    SERCOM2_INTREQ_3,
    #[doc = "58 - SERCOM3_INTREQ_0"]
    SERCOM3_INTREQ_0,
    #[doc = "59 - SERCOM3_INTREQ_1"]
    SERCOM3_INTREQ_1,
    #[doc = "60 - SERCOM3_INTREQ_2"]
    SERCOM3_INTREQ_2,
    #[doc = "61 - SERCOM3_INTREQ_3"]
    SERCOM3_INTREQ_3,
    #[doc = "62 - SERCOM4_INTREQ_0"]
    SERCOM4_INTREQ_0,
    #[doc = "63 - SERCOM4_INTREQ_1"]
    SERCOM4_INTREQ_1,
    #[doc = "64 - SERCOM4_INTREQ_2"]
    SERCOM4_INTREQ_2,
    #[doc = "65 - SERCOM4_INTREQ_3"]
    SERCOM4_INTREQ_3,
    #[doc = "66 - SERCOM5_INTREQ_0"]
    SERCOM5_INTREQ_0,
    #[doc = "67 - SERCOM5_INTREQ_1"]
    SERCOM5_INTREQ_1,
    #[doc = "68 - SERCOM5_INTREQ_2"]
    SERCOM5_INTREQ_2,
    #[doc = "69 - SERCOM5_INTREQ_3"]
    SERCOM5_INTREQ_3,
    #[doc = "70 - SERCOM6_INTREQ_0"]
    SERCOM6_INTREQ_0,
    #[doc = "71 - SERCOM6_INTREQ_1"]
    SERCOM6_INTREQ_1,
    #[doc = "72 - SERCOM6_INTREQ_2"]
    SERCOM6_INTREQ_2,
    #[doc = "73 - SERCOM6_INTREQ_3"]
    SERCOM6_INTREQ_3,
    #[doc = "74 - SERCOM7_INTREQ_0"]
    SERCOM7_INTREQ_0,
    #[doc = "75 - SERCOM7_INTREQ_1"]
    SERCOM7_INTREQ_1,
    #[doc = "76 - SERCOM7_INTREQ_2"]
    SERCOM7_INTREQ_2,
    #[doc = "77 - SERCOM7_INTREQ_3"]
    SERCOM7_INTREQ_3,
    #[doc = "78 - CAN0_INTREQ"]
    CAN0_INTREQ,
    #[doc = "79 - CAN1_INTREQ"]
    CAN1_INTREQ,
    #[doc = "80 - USB_INTREQ_0"]
    USB_INTREQ_0,
    #[doc = "81 - USB_INTREQ_1"]
    USB_INTREQ_1,
    #[doc = "82 - USB_INTREQ_2"]
    USB_INTREQ_2,
    #[doc = "83 - USB_INTREQ_3"]
    USB_INTREQ_3,
    #[doc = "84 - GMAC_INTREQ"]
    GMAC_INTREQ,
    #[doc = "85 - TCC0_INTREQ_0"]
    TCC0_INTREQ_0,
    #[doc = "86 - TCC0_INTREQ_1"]
    TCC0_INTREQ_1,
    #[doc = "87 - TCC0_INTREQ_2"]
    TCC0_INTREQ_2,
    #[doc = "88 - TCC0_INTREQ_3"]
    TCC0_INTREQ_3,
    #[doc = "89 - TCC0_INTREQ_4"]
    TCC0_INTREQ_4,
    #[doc = "90 - TCC0_INTREQ_5"]
    TCC0_INTREQ_5,
    #[doc = "91 - TCC0_INTREQ_6"]
    TCC0_INTREQ_6,
    #[doc = "92 - TCC1_INTREQ_0"]
    TCC1_INTREQ_0,
    #[doc = "93 - TCC1_INTREQ_1"]
    TCC1_INTREQ_1,
    #[doc = "94 - TCC1_INTREQ_2"]
    TCC1_INTREQ_2,
    #[doc = "95 - TCC1_INTREQ_3"]
    TCC1_INTREQ_3,
    #[doc = "96 - TCC1_INTREQ_4"]
    TCC1_INTREQ_4,
    #[doc = "97 - TCC2_INTREQ_0"]
    TCC2_INTREQ_0,
    #[doc = "98 - TCC2_INTREQ_1"]
    TCC2_INTREQ_1,
    #[doc = "99 - TCC2_INTREQ_2"]
    TCC2_INTREQ_2,
    #[doc = "100 - TCC2_INTREQ_3"]
    TCC2_INTREQ_3,
    #[doc = "101 - TCC3_INTREQ_0"]
    TCC3_INTREQ_0,
    #[doc = "102 - TCC3_INTREQ_1"]
    TCC3_INTREQ_1,
    #[doc = "103 - TCC3_INTREQ_2"]
    TCC3_INTREQ_2,
    #[doc = "104 - TCC4_INTREQ_0"]
    TCC4_INTREQ_0,
    #[doc = "105 - TCC4_INTREQ_1"]
    TCC4_INTREQ_1,
    #[doc = "106 - TCC4_INTREQ_2"]
    TCC4_INTREQ_2,
    #[doc = "107 - TC0_INTREQ"]
    TC0_INTREQ,
    #[doc = "108 - TC1_INTREQ"]
    TC1_INTREQ,
    #[doc = "109 - TC2_INTREQ"]
    TC2_INTREQ,
    #[doc = "110 - TC3_INTREQ"]
    TC3_INTREQ,
    #[doc = "111 - TC4_INTREQ"]
    TC4_INTREQ,
    #[doc = "112 - TC5_INTREQ"]
    TC5_INTREQ,
    #[doc = "113 - TC6_INTREQ"]
    TC6_INTREQ,
    #[doc = "114 - TC7_INTREQ"]
    TC7_INTREQ,
    #[doc = "115 - PDEC_INTREQ_0"]
    PDEC_INTREQ_0,
    #[doc = "116 - PDEC_INTREQ_1"]
    PDEC_INTREQ_1,
    #[doc = "117 - PDEC_INTREQ_2"]
    PDEC_INTREQ_2,
    #[doc = "118 - ADC0_INTREQ_0"]
    ADC0_INTREQ_0,
    #[doc = "119 - ADC0_INTREQ_1"]
    ADC0_INTREQ_1,
    #[doc = "120 - ADC1_INTREQ_0"]
    ADC1_INTREQ_0,
    #[doc = "121 - ADC1_INTREQ_1"]
    ADC1_INTREQ_1,
    #[doc = "122 - AC_INTREQ"]
    AC_INTREQ,
    #[doc = "123 - DAC_INTREQ_0"]
    DAC_INTREQ_0,
    #[doc = "124 - DAC_INTREQ_1"]
    DAC_INTREQ_1,
    #[doc = "125 - DAC_INTREQ_2"]
    DAC_INTREQ_2,
    #[doc = "126 - DAC_INTREQ_3"]
    DAC_INTREQ_3,
    #[doc = "127 - DAC_INTREQ_4"]
    DAC_INTREQ_4,
    #[doc = "128 - I2S_INTREQ"]
    I2S_INTREQ,
    #[doc = "129 - PCC_INTREQ"]
    PCC_INTREQ,
    #[doc = "130 - AES_INTREQ"]
    AES_INTREQ,
    #[doc = "131 - TRNG_INTREQ"]
    TRNG_INTREQ,
    #[doc = "132 - ICM_INTREQ"]
    ICM_INTREQ,
    #[doc = "134 - QSPI_INTREQ"]
    QSPI_INTREQ,
    #[doc = "135 - SDHC0_INTREQ"]
    SDHC0_INTREQ,
    #[doc = "136 - SDHC1_INTREQ"]
    SDHC1_INTREQ,
}
unsafe impl bare_metal::Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        match *self {
            Interrupt::PM_INTREQ => 0,
            Interrupt::MCLK_INTREQ => 1,
            Interrupt::OSCCTRL_INTREQ_0 => 2,
            Interrupt::OSCCTRL_INTREQ_1 => 3,
            Interrupt::OSCCTRL_INTREQ_2 => 4,
            Interrupt::OSCCTRL_INTREQ_3 => 5,
            Interrupt::OSCCTRL_INTREQ_4 => 6,
            Interrupt::OSC32KCTRL_INTREQ => 7,
            Interrupt::SUPC_INTREQ_0 => 8,
            Interrupt::SUPC_INTREQ_1 => 9,
            Interrupt::WDT_INTREQ => 10,
            Interrupt::RTC_INTREQ => 11,
            Interrupt::EIC_INTREQ_0 => 12,
            Interrupt::EIC_INTREQ_1 => 13,
            Interrupt::EIC_INTREQ_2 => 14,
            Interrupt::EIC_INTREQ_3 => 15,
            Interrupt::EIC_INTREQ_4 => 16,
            Interrupt::EIC_INTREQ_5 => 17,
            Interrupt::EIC_INTREQ_6 => 18,
            Interrupt::EIC_INTREQ_7 => 19,
            Interrupt::EIC_INTREQ_8 => 20,
            Interrupt::EIC_INTREQ_9 => 21,
            Interrupt::EIC_INTREQ_10 => 22,
            Interrupt::EIC_INTREQ_11 => 23,
            Interrupt::EIC_INTREQ_12 => 24,
            Interrupt::EIC_INTREQ_13 => 25,
            Interrupt::EIC_INTREQ_14 => 26,
            Interrupt::EIC_INTREQ_15 => 27,
            Interrupt::FREQM_INTREQ => 28,
            Interrupt::NVMCTRL_INTREQ_0 => 29,
            Interrupt::NVMCTRL_INTREQ_1 => 30,
            Interrupt::DMAC_INTREQ_0 => 31,
            Interrupt::DMAC_INTREQ_1 => 32,
            Interrupt::DMAC_INTREQ_2 => 33,
            Interrupt::DMAC_INTREQ_3 => 34,
            Interrupt::DMAC_INTREQ_4 => 35,
            Interrupt::EVSYS_INTREQ_0 => 36,
            Interrupt::EVSYS_INTREQ_1 => 37,
            Interrupt::EVSYS_INTREQ_2 => 38,
            Interrupt::EVSYS_INTREQ_3 => 39,
            Interrupt::EVSYS_INTREQ_4 => 40,
            Interrupt::PAC_INTREQ => 41,
            Interrupt::TAL_INTREQ_0 => 42,
            Interrupt::TAL_INTREQ_1 => 43,
            Interrupt::RAMECC_INTREQ => 45,
            Interrupt::SERCOM0_INTREQ_0 => 46,
            Interrupt::SERCOM0_INTREQ_1 => 47,
            Interrupt::SERCOM0_INTREQ_2 => 48,
            Interrupt::SERCOM0_INTREQ_3 => 49,
            Interrupt::SERCOM1_INTREQ_0 => 50,
            Interrupt::SERCOM1_INTREQ_1 => 51,
            Interrupt::SERCOM1_INTREQ_2 => 52,
            Interrupt::SERCOM1_INTREQ_3 => 53,
            Interrupt::SERCOM2_INTREQ_0 => 54,
            Interrupt::SERCOM2_INTREQ_1 => 55,
            Interrupt::SERCOM2_INTREQ_2 => 56,
            Interrupt::SERCOM2_INTREQ_3 => 57,
            Interrupt::SERCOM3_INTREQ_0 => 58,
            Interrupt::SERCOM3_INTREQ_1 => 59,
            Interrupt::SERCOM3_INTREQ_2 => 60,
            Interrupt::SERCOM3_INTREQ_3 => 61,
            Interrupt::SERCOM4_INTREQ_0 => 62,
            Interrupt::SERCOM4_INTREQ_1 => 63,
            Interrupt::SERCOM4_INTREQ_2 => 64,
            Interrupt::SERCOM4_INTREQ_3 => 65,
            Interrupt::SERCOM5_INTREQ_0 => 66,
            Interrupt::SERCOM5_INTREQ_1 => 67,
            Interrupt::SERCOM5_INTREQ_2 => 68,
            Interrupt::SERCOM5_INTREQ_3 => 69,
            Interrupt::SERCOM6_INTREQ_0 => 70,
            Interrupt::SERCOM6_INTREQ_1 => 71,
            Interrupt::SERCOM6_INTREQ_2 => 72,
            Interrupt::SERCOM6_INTREQ_3 => 73,
            Interrupt::SERCOM7_INTREQ_0 => 74,
            Interrupt::SERCOM7_INTREQ_1 => 75,
            Interrupt::SERCOM7_INTREQ_2 => 76,
            Interrupt::SERCOM7_INTREQ_3 => 77,
            Interrupt::CAN0_INTREQ => 78,
            Interrupt::CAN1_INTREQ => 79,
            Interrupt::USB_INTREQ_0 => 80,
            Interrupt::USB_INTREQ_1 => 81,
            Interrupt::USB_INTREQ_2 => 82,
            Interrupt::USB_INTREQ_3 => 83,
            Interrupt::GMAC_INTREQ => 84,
            Interrupt::TCC0_INTREQ_0 => 85,
            Interrupt::TCC0_INTREQ_1 => 86,
            Interrupt::TCC0_INTREQ_2 => 87,
            Interrupt::TCC0_INTREQ_3 => 88,
            Interrupt::TCC0_INTREQ_4 => 89,
            Interrupt::TCC0_INTREQ_5 => 90,
            Interrupt::TCC0_INTREQ_6 => 91,
            Interrupt::TCC1_INTREQ_0 => 92,
            Interrupt::TCC1_INTREQ_1 => 93,
            Interrupt::TCC1_INTREQ_2 => 94,
            Interrupt::TCC1_INTREQ_3 => 95,
            Interrupt::TCC1_INTREQ_4 => 96,
            Interrupt::TCC2_INTREQ_0 => 97,
            Interrupt::TCC2_INTREQ_1 => 98,
            Interrupt::TCC2_INTREQ_2 => 99,
            Interrupt::TCC2_INTREQ_3 => 100,
            Interrupt::TCC3_INTREQ_0 => 101,
            Interrupt::TCC3_INTREQ_1 => 102,
            Interrupt::TCC3_INTREQ_2 => 103,
            Interrupt::TCC4_INTREQ_0 => 104,
            Interrupt::TCC4_INTREQ_1 => 105,
            Interrupt::TCC4_INTREQ_2 => 106,
            Interrupt::TC0_INTREQ => 107,
            Interrupt::TC1_INTREQ => 108,
            Interrupt::TC2_INTREQ => 109,
            Interrupt::TC3_INTREQ => 110,
            Interrupt::TC4_INTREQ => 111,
            Interrupt::TC5_INTREQ => 112,
            Interrupt::TC6_INTREQ => 113,
            Interrupt::TC7_INTREQ => 114,
            Interrupt::PDEC_INTREQ_0 => 115,
            Interrupt::PDEC_INTREQ_1 => 116,
            Interrupt::PDEC_INTREQ_2 => 117,
            Interrupt::ADC0_INTREQ_0 => 118,
            Interrupt::ADC0_INTREQ_1 => 119,
            Interrupt::ADC1_INTREQ_0 => 120,
            Interrupt::ADC1_INTREQ_1 => 121,
            Interrupt::AC_INTREQ => 122,
            Interrupt::DAC_INTREQ_0 => 123,
            Interrupt::DAC_INTREQ_1 => 124,
            Interrupt::DAC_INTREQ_2 => 125,
            Interrupt::DAC_INTREQ_3 => 126,
            Interrupt::DAC_INTREQ_4 => 127,
            Interrupt::I2S_INTREQ => 128,
            Interrupt::PCC_INTREQ => 129,
            Interrupt::AES_INTREQ => 130,
            Interrupt::TRNG_INTREQ => 131,
            Interrupt::ICM_INTREQ => 132,
            Interrupt::QSPI_INTREQ => 134,
            Interrupt::SDHC0_INTREQ => 135,
            Interrupt::SDHC1_INTREQ => 136,
        }
    }
}
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, FPU, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[allow(unused_imports)]
use generic::*;
#[doc = r"Common register and bit access and modify traits"]
pub mod generic;
#[doc = "Analog Comparators"]
pub struct AC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AC {}
impl AC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ac::RegisterBlock {
        0x4200_2000 as *const _
    }
}
impl Deref for AC {
    type Target = ac::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*AC::ptr() }
    }
}
#[doc = "Analog Comparators"]
pub mod ac;
#[doc = "Analog Digital Converter 0"]
pub struct ADC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC0 {}
impl ADC0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const adc0::RegisterBlock {
        0x4300_1c00 as *const _
    }
}
impl Deref for ADC0 {
    type Target = adc0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ADC0::ptr() }
    }
}
#[doc = "Analog Digital Converter 0"]
pub mod adc0;
#[doc = "Analog Digital Converter 1"]
pub struct ADC1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC1 {}
impl ADC1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const adc0::RegisterBlock {
        0x4300_2000 as *const _
    }
}
impl Deref for ADC1 {
    type Target = adc0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ADC1::ptr() }
    }
}
#[doc = "Advanced Encryption Standard"]
pub struct AES {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AES {}
impl AES {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aes::RegisterBlock {
        0x4200_2400 as *const _
    }
}
impl Deref for AES {
    type Target = aes::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*AES::ptr() }
    }
}
#[doc = "Advanced Encryption Standard"]
pub mod aes;
#[doc = "Control Area Network 0"]
pub struct CAN0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN0 {}
impl CAN0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can0::RegisterBlock {
        0x4200_0000 as *const _
    }
}
impl Deref for CAN0 {
    type Target = can0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN0::ptr() }
    }
}
#[doc = "Control Area Network 0"]
pub mod can0;
#[doc = "Control Area Network 1"]
pub struct CAN1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN1 {}
impl CAN1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can0::RegisterBlock {
        0x4200_0400 as *const _
    }
}
impl Deref for CAN1 {
    type Target = can0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN1::ptr() }
    }
}
#[doc = "Configurable Custom Logic"]
pub struct CCL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCL {}
impl CCL {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ccl::RegisterBlock {
        0x4200_3800 as *const _
    }
}
impl Deref for CCL {
    type Target = ccl::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CCL::ptr() }
    }
}
#[doc = "Configurable Custom Logic"]
pub mod ccl;
#[doc = "Cortex M Cache Controller"]
pub struct CMCC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CMCC {}
impl CMCC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const cmcc::RegisterBlock {
        0x4100_6000 as *const _
    }
}
impl Deref for CMCC {
    type Target = cmcc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CMCC::ptr() }
    }
}
#[doc = "Cortex M Cache Controller"]
pub mod cmcc;
#[doc = "Digital-to-Analog Converter"]
pub struct DAC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DAC {}
impl DAC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dac::RegisterBlock {
        0x4300_2400 as *const _
    }
}
impl Deref for DAC {
    type Target = dac::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DAC::ptr() }
    }
}
#[doc = "Digital-to-Analog Converter"]
pub mod dac;
#[doc = "Direct Memory Access Controller"]
pub struct DMAC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMAC {}
impl DMAC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dmac::RegisterBlock {
        0x4100_a000 as *const _
    }
}
impl Deref for DMAC {
    type Target = dmac::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DMAC::ptr() }
    }
}
#[doc = "Direct Memory Access Controller"]
pub mod dmac;
#[doc = "Device Service Unit"]
pub struct DSU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DSU {}
impl DSU {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dsu::RegisterBlock {
        0x4100_2000 as *const _
    }
}
impl Deref for DSU {
    type Target = dsu::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DSU::ptr() }
    }
}
#[doc = "Device Service Unit"]
pub mod dsu;
#[doc = "External Interrupt Controller"]
pub struct EIC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EIC {}
impl EIC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const eic::RegisterBlock {
        0x4000_2800 as *const _
    }
}
impl Deref for EIC {
    type Target = eic::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*EIC::ptr() }
    }
}
#[doc = "External Interrupt Controller"]
pub mod eic;
#[doc = "Event System Interface"]
pub struct EVSYS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EVSYS {}
impl EVSYS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const evsys::RegisterBlock {
        0x4100_e000 as *const _
    }
}
impl Deref for EVSYS {
    type Target = evsys::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*EVSYS::ptr() }
    }
}
#[doc = "Event System Interface"]
pub mod evsys;
#[doc = "Frequency Meter"]
pub struct FREQM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FREQM {}
impl FREQM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const freqm::RegisterBlock {
        0x4000_2c00 as *const _
    }
}
impl Deref for FREQM {
    type Target = freqm::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FREQM::ptr() }
    }
}
#[doc = "Frequency Meter"]
pub mod freqm;
#[doc = "Generic Clock Generator"]
pub struct GCLK {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GCLK {}
impl GCLK {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gclk::RegisterBlock {
        0x4000_1c00 as *const _
    }
}
impl Deref for GCLK {
    type Target = gclk::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GCLK::ptr() }
    }
}
#[doc = "Generic Clock Generator"]
pub mod gclk;
#[doc = "Ethernet MAC"]
pub struct GMAC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GMAC {}
impl GMAC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gmac::RegisterBlock {
        0x4200_0800 as *const _
    }
}
impl Deref for GMAC {
    type Target = gmac::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GMAC::ptr() }
    }
}
#[doc = "Ethernet MAC"]
pub mod gmac;
#[doc = "HSB Matrix"]
pub struct HMATRIX {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for HMATRIX {}
impl HMATRIX {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const hmatrix::RegisterBlock {
        0x4100_c000 as *const _
    }
}
impl Deref for HMATRIX {
    type Target = hmatrix::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*HMATRIX::ptr() }
    }
}
#[doc = "HSB Matrix"]
pub mod hmatrix;
#[doc = "Integrity Check Monitor"]
pub struct ICM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ICM {}
impl ICM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const icm::RegisterBlock {
        0x4200_2c00 as *const _
    }
}
impl Deref for ICM {
    type Target = icm::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ICM::ptr() }
    }
}
#[doc = "Integrity Check Monitor"]
pub mod icm;
#[doc = "Inter-IC Sound Interface"]
pub struct I2S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2S {}
impl I2S {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2s::RegisterBlock {
        0x4300_2800 as *const _
    }
}
impl Deref for I2S {
    type Target = i2s::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2S::ptr() }
    }
}
#[doc = "Inter-IC Sound Interface"]
pub mod i2s;
#[doc = "Main Clock"]
pub struct MCLK {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MCLK {}
impl MCLK {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const mclk::RegisterBlock {
        0x4000_0800 as *const _
    }
}
impl Deref for MCLK {
    type Target = mclk::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*MCLK::ptr() }
    }
}
#[doc = "Main Clock"]
pub mod mclk;
#[doc = "Non-Volatile Memory Controller"]
pub struct NVMCTRL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for NVMCTRL {}
impl NVMCTRL {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const nvmctrl::RegisterBlock {
        0x4100_4000 as *const _
    }
}
impl Deref for NVMCTRL {
    type Target = nvmctrl::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*NVMCTRL::ptr() }
    }
}
#[doc = "Non-Volatile Memory Controller"]
pub mod nvmctrl;
#[doc = "Oscillators Control"]
pub struct OSCCTRL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OSCCTRL {}
impl OSCCTRL {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const oscctrl::RegisterBlock {
        0x4000_1000 as *const _
    }
}
impl Deref for OSCCTRL {
    type Target = oscctrl::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*OSCCTRL::ptr() }
    }
}
#[doc = "Oscillators Control"]
pub mod oscctrl;
#[doc = "32kHz Oscillators Control"]
pub struct OSC32KCTRL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OSC32KCTRL {}
impl OSC32KCTRL {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const osc32kctrl::RegisterBlock {
        0x4000_1400 as *const _
    }
}
impl Deref for OSC32KCTRL {
    type Target = osc32kctrl::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*OSC32KCTRL::ptr() }
    }
}
#[doc = "32kHz Oscillators Control"]
pub mod osc32kctrl;
#[doc = "Peripheral Access Controller"]
pub struct PAC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PAC {}
impl PAC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pac::RegisterBlock {
        0x4000_0000 as *const _
    }
}
impl Deref for PAC {
    type Target = pac::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PAC::ptr() }
    }
}
#[doc = "Peripheral Access Controller"]
pub mod pac;
#[doc = "Parallel Capture Controller"]
pub struct PCC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PCC {}
impl PCC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pcc::RegisterBlock {
        0x4300_2c00 as *const _
    }
}
impl Deref for PCC {
    type Target = pcc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PCC::ptr() }
    }
}
#[doc = "Parallel Capture Controller"]
pub mod pcc;
#[doc = "Quadrature Decodeur"]
pub struct PDEC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PDEC {}
impl PDEC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pdec::RegisterBlock {
        0x4200_1c00 as *const _
    }
}
impl Deref for PDEC {
    type Target = pdec::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PDEC::ptr() }
    }
}
#[doc = "Quadrature Decodeur"]
pub mod pdec;
#[doc = "Power Manager"]
pub struct PM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PM {}
impl PM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pm::RegisterBlock {
        0x4000_0400 as *const _
    }
}
impl Deref for PM {
    type Target = pm::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PM::ptr() }
    }
}
#[doc = "Power Manager"]
pub mod pm;
#[doc = "Port Module"]
pub struct PORT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORT {}
impl PORT {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const port::RegisterBlock {
        0x4100_8000 as *const _
    }
}
impl Deref for PORT {
    type Target = port::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PORT::ptr() }
    }
}
#[doc = "Port Module"]
pub mod port;
#[doc = "Quad SPI interface"]
pub struct QSPI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for QSPI {}
impl QSPI {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const qspi::RegisterBlock {
        0x4200_3400 as *const _
    }
}
impl Deref for QSPI {
    type Target = qspi::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*QSPI::ptr() }
    }
}
#[doc = "Quad SPI interface"]
pub mod qspi;
#[doc = "RAM ECC"]
pub struct RAMECC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RAMECC {}
impl RAMECC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ramecc::RegisterBlock {
        0x4102_0000 as *const _
    }
}
impl Deref for RAMECC {
    type Target = ramecc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RAMECC::ptr() }
    }
}
#[doc = "RAM ECC"]
pub mod ramecc;
#[doc = "Reset Controller"]
pub struct RSTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RSTC {}
impl RSTC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rstc::RegisterBlock {
        0x4000_0c00 as *const _
    }
}
impl Deref for RSTC {
    type Target = rstc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RSTC::ptr() }
    }
}
#[doc = "Reset Controller"]
pub mod rstc;
#[doc = "Real-Time Counter"]
pub struct RTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC {}
impl RTC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rtc::RegisterBlock {
        0x4000_2400 as *const _
    }
}
impl Deref for RTC {
    type Target = rtc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RTC::ptr() }
    }
}
#[doc = "Real-Time Counter"]
pub mod rtc;
#[doc = "SD/MMC Host Controller 0"]
pub struct SDHC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SDHC0 {}
impl SDHC0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sdhc0::RegisterBlock {
        0x4500_0000 as *const _
    }
}
impl Deref for SDHC0 {
    type Target = sdhc0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SDHC0::ptr() }
    }
}
#[doc = "SD/MMC Host Controller 0"]
pub mod sdhc0;
#[doc = "SD/MMC Host Controller 1"]
pub struct SDHC1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SDHC1 {}
impl SDHC1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sdhc0::RegisterBlock {
        0x4600_0000 as *const _
    }
}
impl Deref for SDHC1 {
    type Target = sdhc0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SDHC1::ptr() }
    }
}
#[doc = "Serial Communication Interface 0"]
pub struct SERCOM0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SERCOM0 {}
impl SERCOM0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sercom0::RegisterBlock {
        0x4000_3000 as *const _
    }
}
impl Deref for SERCOM0 {
    type Target = sercom0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SERCOM0::ptr() }
    }
}
#[doc = "Serial Communication Interface 0"]
pub mod sercom0;
#[doc = "Serial Communication Interface 1"]
pub struct SERCOM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SERCOM1 {}
impl SERCOM1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sercom0::RegisterBlock {
        0x4000_3400 as *const _
    }
}
impl Deref for SERCOM1 {
    type Target = sercom0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SERCOM1::ptr() }
    }
}
#[doc = "Serial Communication Interface 2"]
pub struct SERCOM2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SERCOM2 {}
impl SERCOM2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sercom0::RegisterBlock {
        0x4101_2000 as *const _
    }
}
impl Deref for SERCOM2 {
    type Target = sercom0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SERCOM2::ptr() }
    }
}
#[doc = "Serial Communication Interface 3"]
pub struct SERCOM3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SERCOM3 {}
impl SERCOM3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sercom0::RegisterBlock {
        0x4101_4000 as *const _
    }
}
impl Deref for SERCOM3 {
    type Target = sercom0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SERCOM3::ptr() }
    }
}
#[doc = "Serial Communication Interface 4"]
pub struct SERCOM4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SERCOM4 {}
impl SERCOM4 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sercom0::RegisterBlock {
        0x4300_0000 as *const _
    }
}
impl Deref for SERCOM4 {
    type Target = sercom0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SERCOM4::ptr() }
    }
}
#[doc = "Serial Communication Interface 5"]
pub struct SERCOM5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SERCOM5 {}
impl SERCOM5 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sercom0::RegisterBlock {
        0x4300_0400 as *const _
    }
}
impl Deref for SERCOM5 {
    type Target = sercom0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SERCOM5::ptr() }
    }
}
#[doc = "Serial Communication Interface 6"]
pub struct SERCOM6 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SERCOM6 {}
impl SERCOM6 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sercom0::RegisterBlock {
        0x4300_0800 as *const _
    }
}
impl Deref for SERCOM6 {
    type Target = sercom0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SERCOM6::ptr() }
    }
}
#[doc = "Serial Communication Interface 7"]
pub struct SERCOM7 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SERCOM7 {}
impl SERCOM7 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sercom0::RegisterBlock {
        0x4300_0c00 as *const _
    }
}
impl Deref for SERCOM7 {
    type Target = sercom0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SERCOM7::ptr() }
    }
}
#[doc = "Supply Controller"]
pub struct SUPC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SUPC {}
impl SUPC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const supc::RegisterBlock {
        0x4000_1800 as *const _
    }
}
impl Deref for SUPC {
    type Target = supc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SUPC::ptr() }
    }
}
#[doc = "Supply Controller"]
pub mod supc;
#[doc = "Trigger Allocator"]
pub struct TAL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TAL {}
impl TAL {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tal::RegisterBlock {
        0x4101_e000 as *const _
    }
}
impl Deref for TAL {
    type Target = tal::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TAL::ptr() }
    }
}
#[doc = "Trigger Allocator"]
pub mod tal;
#[doc = "Basic Timer Counter 0"]
pub struct TC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TC0 {}
impl TC0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tc0::RegisterBlock {
        0x4000_3800 as *const _
    }
}
impl Deref for TC0 {
    type Target = tc0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TC0::ptr() }
    }
}
#[doc = "Basic Timer Counter 0"]
pub mod tc0;
#[doc = "Basic Timer Counter 1"]
pub struct TC1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TC1 {}
impl TC1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tc0::RegisterBlock {
        0x4000_3c00 as *const _
    }
}
impl Deref for TC1 {
    type Target = tc0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TC1::ptr() }
    }
}
#[doc = "Basic Timer Counter 2"]
pub struct TC2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TC2 {}
impl TC2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tc0::RegisterBlock {
        0x4101_a000 as *const _
    }
}
impl Deref for TC2 {
    type Target = tc0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TC2::ptr() }
    }
}
#[doc = "Basic Timer Counter 3"]
pub struct TC3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TC3 {}
impl TC3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tc0::RegisterBlock {
        0x4101_c000 as *const _
    }
}
impl Deref for TC3 {
    type Target = tc0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TC3::ptr() }
    }
}
#[doc = "Basic Timer Counter 4"]
pub struct TC4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TC4 {}
impl TC4 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tc0::RegisterBlock {
        0x4200_1400 as *const _
    }
}
impl Deref for TC4 {
    type Target = tc0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TC4::ptr() }
    }
}
#[doc = "Basic Timer Counter 5"]
pub struct TC5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TC5 {}
impl TC5 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tc0::RegisterBlock {
        0x4200_1800 as *const _
    }
}
impl Deref for TC5 {
    type Target = tc0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TC5::ptr() }
    }
}
#[doc = "Basic Timer Counter 6"]
pub struct TC6 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TC6 {}
impl TC6 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tc0::RegisterBlock {
        0x4300_1400 as *const _
    }
}
impl Deref for TC6 {
    type Target = tc0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TC6::ptr() }
    }
}
#[doc = "Basic Timer Counter 7"]
pub struct TC7 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TC7 {}
impl TC7 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tc0::RegisterBlock {
        0x4300_1800 as *const _
    }
}
impl Deref for TC7 {
    type Target = tc0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TC7::ptr() }
    }
}
#[doc = "Timer Counter Control 0"]
pub struct TCC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TCC0 {}
impl TCC0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tcc0::RegisterBlock {
        0x4101_6000 as *const _
    }
}
impl Deref for TCC0 {
    type Target = tcc0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TCC0::ptr() }
    }
}
#[doc = "Timer Counter Control 0"]
pub mod tcc0;
#[doc = "Timer Counter Control 1"]
pub struct TCC1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TCC1 {}
impl TCC1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tcc0::RegisterBlock {
        0x4101_8000 as *const _
    }
}
impl Deref for TCC1 {
    type Target = tcc0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TCC1::ptr() }
    }
}
#[doc = "Timer Counter Control 2"]
pub struct TCC2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TCC2 {}
impl TCC2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tcc0::RegisterBlock {
        0x4200_0c00 as *const _
    }
}
impl Deref for TCC2 {
    type Target = tcc0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TCC2::ptr() }
    }
}
#[doc = "Timer Counter Control 3"]
pub struct TCC3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TCC3 {}
impl TCC3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tcc0::RegisterBlock {
        0x4200_1000 as *const _
    }
}
impl Deref for TCC3 {
    type Target = tcc0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TCC3::ptr() }
    }
}
#[doc = "Timer Counter Control 4"]
pub struct TCC4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TCC4 {}
impl TCC4 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tcc0::RegisterBlock {
        0x4300_1000 as *const _
    }
}
impl Deref for TCC4 {
    type Target = tcc0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TCC4::ptr() }
    }
}
#[doc = "True Random Generator"]
pub struct TRNG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TRNG {}
impl TRNG {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const trng::RegisterBlock {
        0x4200_2800 as *const _
    }
}
impl Deref for TRNG {
    type Target = trng::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TRNG::ptr() }
    }
}
#[doc = "True Random Generator"]
pub mod trng;
#[doc = "Universal Serial Bus"]
pub struct USB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB {}
impl USB {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usb::RegisterBlock {
        0x4100_0000 as *const _
    }
}
impl Deref for USB {
    type Target = usb::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USB::ptr() }
    }
}
#[doc = "Universal Serial Bus"]
pub mod usb;
#[doc = "Watchdog Timer"]
pub struct WDT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WDT {}
impl WDT {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const wdt::RegisterBlock {
        0x4000_2000 as *const _
    }
}
impl Deref for WDT {
    type Target = wdt::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*WDT::ptr() }
    }
}
#[doc = "Watchdog Timer"]
pub mod wdt;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r"All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "AC"]
    pub AC: AC,
    #[doc = "ADC0"]
    pub ADC0: ADC0,
    #[doc = "ADC1"]
    pub ADC1: ADC1,
    #[doc = "AES"]
    pub AES: AES,
    #[doc = "CAN0"]
    pub CAN0: CAN0,
    #[doc = "CAN1"]
    pub CAN1: CAN1,
    #[doc = "CCL"]
    pub CCL: CCL,
    #[doc = "CMCC"]
    pub CMCC: CMCC,
    #[doc = "DAC"]
    pub DAC: DAC,
    #[doc = "DMAC"]
    pub DMAC: DMAC,
    #[doc = "DSU"]
    pub DSU: DSU,
    #[doc = "EIC"]
    pub EIC: EIC,
    #[doc = "EVSYS"]
    pub EVSYS: EVSYS,
    #[doc = "FREQM"]
    pub FREQM: FREQM,
    #[doc = "GCLK"]
    pub GCLK: GCLK,
    #[doc = "GMAC"]
    pub GMAC: GMAC,
    #[doc = "HMATRIX"]
    pub HMATRIX: HMATRIX,
    #[doc = "ICM"]
    pub ICM: ICM,
    #[doc = "I2S"]
    pub I2S: I2S,
    #[doc = "MCLK"]
    pub MCLK: MCLK,
    #[doc = "NVMCTRL"]
    pub NVMCTRL: NVMCTRL,
    #[doc = "OSCCTRL"]
    pub OSCCTRL: OSCCTRL,
    #[doc = "OSC32KCTRL"]
    pub OSC32KCTRL: OSC32KCTRL,
    #[doc = "PAC"]
    pub PAC: PAC,
    #[doc = "PCC"]
    pub PCC: PCC,
    #[doc = "PDEC"]
    pub PDEC: PDEC,
    #[doc = "PM"]
    pub PM: PM,
    #[doc = "PORT"]
    pub PORT: PORT,
    #[doc = "QSPI"]
    pub QSPI: QSPI,
    #[doc = "RAMECC"]
    pub RAMECC: RAMECC,
    #[doc = "RSTC"]
    pub RSTC: RSTC,
    #[doc = "RTC"]
    pub RTC: RTC,
    #[doc = "SDHC0"]
    pub SDHC0: SDHC0,
    #[doc = "SDHC1"]
    pub SDHC1: SDHC1,
    #[doc = "SERCOM0"]
    pub SERCOM0: SERCOM0,
    #[doc = "SERCOM1"]
    pub SERCOM1: SERCOM1,
    #[doc = "SERCOM2"]
    pub SERCOM2: SERCOM2,
    #[doc = "SERCOM3"]
    pub SERCOM3: SERCOM3,
    #[doc = "SERCOM4"]
    pub SERCOM4: SERCOM4,
    #[doc = "SERCOM5"]
    pub SERCOM5: SERCOM5,
    #[doc = "SERCOM6"]
    pub SERCOM6: SERCOM6,
    #[doc = "SERCOM7"]
    pub SERCOM7: SERCOM7,
    #[doc = "SUPC"]
    pub SUPC: SUPC,
    #[doc = "TAL"]
    pub TAL: TAL,
    #[doc = "TC0"]
    pub TC0: TC0,
    #[doc = "TC1"]
    pub TC1: TC1,
    #[doc = "TC2"]
    pub TC2: TC2,
    #[doc = "TC3"]
    pub TC3: TC3,
    #[doc = "TC4"]
    pub TC4: TC4,
    #[doc = "TC5"]
    pub TC5: TC5,
    #[doc = "TC6"]
    pub TC6: TC6,
    #[doc = "TC7"]
    pub TC7: TC7,
    #[doc = "TCC0"]
    pub TCC0: TCC0,
    #[doc = "TCC1"]
    pub TCC1: TCC1,
    #[doc = "TCC2"]
    pub TCC2: TCC2,
    #[doc = "TCC3"]
    pub TCC3: TCC3,
    #[doc = "TCC4"]
    pub TCC4: TCC4,
    #[doc = "TRNG"]
    pub TRNG: TRNG,
    #[doc = "USB"]
    pub USB: USB,
    #[doc = "WDT"]
    pub WDT: WDT,
}
impl Peripherals {
    #[doc = r"Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r"Unchecked version of `Peripherals::take`"]
    #[inline]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            AC: AC {
                _marker: PhantomData,
            },
            ADC0: ADC0 {
                _marker: PhantomData,
            },
            ADC1: ADC1 {
                _marker: PhantomData,
            },
            AES: AES {
                _marker: PhantomData,
            },
            CAN0: CAN0 {
                _marker: PhantomData,
            },
            CAN1: CAN1 {
                _marker: PhantomData,
            },
            CCL: CCL {
                _marker: PhantomData,
            },
            CMCC: CMCC {
                _marker: PhantomData,
            },
            DAC: DAC {
                _marker: PhantomData,
            },
            DMAC: DMAC {
                _marker: PhantomData,
            },
            DSU: DSU {
                _marker: PhantomData,
            },
            EIC: EIC {
                _marker: PhantomData,
            },
            EVSYS: EVSYS {
                _marker: PhantomData,
            },
            FREQM: FREQM {
                _marker: PhantomData,
            },
            GCLK: GCLK {
                _marker: PhantomData,
            },
            GMAC: GMAC {
                _marker: PhantomData,
            },
            HMATRIX: HMATRIX {
                _marker: PhantomData,
            },
            ICM: ICM {
                _marker: PhantomData,
            },
            I2S: I2S {
                _marker: PhantomData,
            },
            MCLK: MCLK {
                _marker: PhantomData,
            },
            NVMCTRL: NVMCTRL {
                _marker: PhantomData,
            },
            OSCCTRL: OSCCTRL {
                _marker: PhantomData,
            },
            OSC32KCTRL: OSC32KCTRL {
                _marker: PhantomData,
            },
            PAC: PAC {
                _marker: PhantomData,
            },
            PCC: PCC {
                _marker: PhantomData,
            },
            PDEC: PDEC {
                _marker: PhantomData,
            },
            PM: PM {
                _marker: PhantomData,
            },
            PORT: PORT {
                _marker: PhantomData,
            },
            QSPI: QSPI {
                _marker: PhantomData,
            },
            RAMECC: RAMECC {
                _marker: PhantomData,
            },
            RSTC: RSTC {
                _marker: PhantomData,
            },
            RTC: RTC {
                _marker: PhantomData,
            },
            SDHC0: SDHC0 {
                _marker: PhantomData,
            },
            SDHC1: SDHC1 {
                _marker: PhantomData,
            },
            SERCOM0: SERCOM0 {
                _marker: PhantomData,
            },
            SERCOM1: SERCOM1 {
                _marker: PhantomData,
            },
            SERCOM2: SERCOM2 {
                _marker: PhantomData,
            },
            SERCOM3: SERCOM3 {
                _marker: PhantomData,
            },
            SERCOM4: SERCOM4 {
                _marker: PhantomData,
            },
            SERCOM5: SERCOM5 {
                _marker: PhantomData,
            },
            SERCOM6: SERCOM6 {
                _marker: PhantomData,
            },
            SERCOM7: SERCOM7 {
                _marker: PhantomData,
            },
            SUPC: SUPC {
                _marker: PhantomData,
            },
            TAL: TAL {
                _marker: PhantomData,
            },
            TC0: TC0 {
                _marker: PhantomData,
            },
            TC1: TC1 {
                _marker: PhantomData,
            },
            TC2: TC2 {
                _marker: PhantomData,
            },
            TC3: TC3 {
                _marker: PhantomData,
            },
            TC4: TC4 {
                _marker: PhantomData,
            },
            TC5: TC5 {
                _marker: PhantomData,
            },
            TC6: TC6 {
                _marker: PhantomData,
            },
            TC7: TC7 {
                _marker: PhantomData,
            },
            TCC0: TCC0 {
                _marker: PhantomData,
            },
            TCC1: TCC1 {
                _marker: PhantomData,
            },
            TCC2: TCC2 {
                _marker: PhantomData,
            },
            TCC3: TCC3 {
                _marker: PhantomData,
            },
            TCC4: TCC4 {
                _marker: PhantomData,
            },
            TRNG: TRNG {
                _marker: PhantomData,
            },
            USB: USB {
                _marker: PhantomData,
            },
            WDT: WDT {
                _marker: PhantomData,
            },
        }
    }
}
