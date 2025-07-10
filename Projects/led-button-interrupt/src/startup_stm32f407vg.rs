use core::ptr;

unsafe extern "C" {
    fn BusFault_Handler();
    fn MemManage_Handler();
    fn PendSV_Handler();
    fn SVCall_Handler();
    fn SysTick_Handler();
    fn UsageFault_Handler();
    fn ADC_Handler();
    fn CAN1_RX0_Handler();
    fn CAN1_RX1_Handler();
    fn CAN1_SCE_Handler();
    fn CAN1_TX_Handler();
    fn CAN2_RX0_Handler();
    fn CAN2_RX1_Handler();
    fn CAN2_SCE_Handler();
    fn CAN2_TX_Handler();
    fn CRYP_Handler();
    fn DCMI_Handler();
    fn DMA1_Stream0_Handler();
    fn DMA1_Stream1_Handler();
    fn DMA1_Stream2_Handler();
    fn DMA1_Stream3_Handler();
    fn DMA1_Stream4_Handler();
    fn DMA1_Stream5_Handler();
    fn DMA1_Stream6_Handler();
    fn DMA1_Stream7_Handler();
    fn DMA2_Stream0_Handler();
    fn DMA2_Stream1_Handler();
    fn DMA2_Stream2_Handler();
    fn DMA2_Stream3_Handler();
    fn DMA2_Stream4_Handler();
    fn DMA2_Stream5_Handler();
    fn DMA2_Stream6_Handler();
    fn DMA2_Stream7_Handler();
    fn ETH_Handler();
    fn ETH_WKUP_Handler();
    fn EXTI0_Handler();
    fn EXTI15_10_Handler();
    fn EXTI1_Handler();
    fn EXTI2_Handler();
    fn EXTI3_Handler();
    fn EXTI4_Handler();
    fn EXTI9_5_Handler();
    fn FPU_Handler();
    fn FSMC_Handler();
    fn HASH_RNG_Handler();
    fn I2C1_ER_Handler();
    fn I2C1_EV_Handler();
    fn I2C2_ER_Handler();
    fn I2C2_EV_Handler();
    fn I2C3_ER_Handler();
    fn I2C3_EV_Handler();
    fn OTG_FS_Handler();
    fn OTG_FS_WKUP_Handler();
    fn OTG_HS_EP1_IN_Handler();
    fn OTG_HS_EP1_OUT_Handler();
    fn OTG_HS_Handler();
    fn OTG_HS_WKUP_Handler();
    fn PVD_Handler();
    fn RCC_Handler();
    fn RTC_Alarm_Handler();
    fn RTC_WKUP_Handler();
    fn SDIO_Handler();
    fn SPI1_Handler();
    fn SPI2_Handler();
    fn SPI3_Handler();
    fn TAMP_STAMP_Handler();
    fn TIM1_BRK_TIM9_Handler();
    fn TIM1_CC_Handler();
    fn TIM1_TRG_COM_TIM11_Handler();
    fn TIM1_UP_TIM10_Handler();
    fn TIM2_Handler();
    fn TIM3_Handler();
    fn TIM4_Handler();
    fn TIM5_Handler();
    fn TIM6_DAC_Handler();
    fn TIM7_Handler();
    fn TIM8_BRK_TIM12_Handler();
    fn TIM8_CC_Handler();
    fn TIM8_TRG_COM_TIM14_Handler();
    fn TIM8_UP_TIM13_Handler();
    fn UART4_Handler();
    fn UART5_Handler();
    fn USART1_Handler();
    fn USART2_Handler();
    fn USART3_Handler();
    fn USART6_Handler();
    fn WWDG_Handler();
}

#[link_section = ".isr_vector"]
#[used]
static VECTOR_TABLE: [Option<unsafe extern "C" fn()>; 97] = [
    Some(Reset_Handler),
    Some(NMI_Handler),
    Some(HardFault_Handler),
    Some(MemManage_Handler),
    Some(BusFault_Handler),
    Some(UsageFault_Handler),
    None,
    None,
    None,
    None,
    Some(SVCall_Handler),
    None,
    None,
    Some(PendSV_Handler),
    Some(SysTick_Handler),
    Some(WWDG_Handler),
    Some(PVD_Handler),
    Some(TAMP_STAMP_Handler),
    Some(RTC_WKUP_Handler),
    None,
    Some(RCC_Handler),
    Some(EXTI0_Handler),
    Some(EXTI1_Handler),
    Some(EXTI2_Handler),
    Some(EXTI3_Handler),
    Some(EXTI4_Handler),
    Some(DMA1_Stream0_Handler),
    Some(DMA1_Stream1_Handler),
    Some(DMA1_Stream2_Handler),
    Some(DMA1_Stream3_Handler),
    Some(DMA1_Stream4_Handler),
    Some(DMA1_Stream5_Handler),
    Some(DMA1_Stream6_Handler),
    Some(ADC_Handler),
    Some(CAN1_TX_Handler),
    Some(CAN1_RX0_Handler),
    Some(CAN1_RX1_Handler),
    Some(CAN1_SCE_Handler),
    Some(EXTI9_5_Handler),
    Some(TIM1_BRK_TIM9_Handler),
    Some(TIM1_UP_TIM10_Handler),
    Some(TIM1_TRG_COM_TIM11_Handler),
    Some(TIM1_CC_Handler),
    Some(TIM2_Handler),
    Some(TIM3_Handler),
    Some(TIM4_Handler),
    Some(I2C1_EV_Handler),
    Some(I2C1_ER_Handler),
    Some(I2C2_EV_Handler),
    Some(I2C2_ER_Handler),
    Some(SPI1_Handler),
    Some(SPI2_Handler),
    Some(USART1_Handler),
    Some(USART2_Handler),
    Some(USART3_Handler),
    Some(EXTI15_10_Handler),
    Some(RTC_Alarm_Handler),
    Some(OTG_FS_WKUP_Handler),
    Some(TIM8_BRK_TIM12_Handler),
    Some(TIM8_UP_TIM13_Handler),
    Some(TIM8_TRG_COM_TIM14_Handler),
    Some(TIM8_CC_Handler),
    Some(DMA1_Stream7_Handler),
    Some(FSMC_Handler),
    Some(SDIO_Handler),
    Some(TIM5_Handler),
    Some(SPI3_Handler),
    Some(UART4_Handler),
    Some(UART5_Handler),
    Some(TIM6_DAC_Handler),
    Some(TIM7_Handler),
    Some(DMA2_Stream0_Handler),
    Some(DMA2_Stream1_Handler),
    Some(DMA2_Stream2_Handler),
    Some(DMA2_Stream3_Handler),
    Some(DMA2_Stream4_Handler),
    Some(ETH_Handler),
    Some(ETH_WKUP_Handler),
    Some(CAN2_TX_Handler),
    Some(CAN2_RX0_Handler),
    Some(CAN2_RX1_Handler),
    Some(CAN2_SCE_Handler),
    Some(OTG_FS_Handler),
    Some(DMA2_Stream5_Handler),
    Some(DMA2_Stream6_Handler),
    Some(DMA2_Stream7_Handler),
    Some(USART6_Handler),
    Some(I2C3_EV_Handler),
    Some(I2C3_ER_Handler),
    Some(OTG_HS_EP1_OUT_Handler),
    Some(OTG_HS_EP1_IN_Handler),
    Some(OTG_HS_WKUP_Handler),
    Some(OTG_HS_Handler),
    Some(DCMI_Handler),
    Some(CRYP_Handler),
    Some(HASH_RNG_Handler),
    Some(FPU_Handler),
];


unsafe extern "C" {
    unsafe static _sidata: u32; // Start of .data section in FLASH
    unsafe static mut _sdata: u32; // Start of .data section in RAM
    unsafe static mut _edata: u32; // End of .data section in RAM
    unsafe static mut _sbss: u32; // Start of .bss section in RAM
    unsafe static mut _ebss: u32; // End of .bss section in RAM
}

#[unsafe(no_mangle)]
extern "C" fn Default_Handler() {
    loop {}
}

#[unsafe(no_mangle)]
extern "C" fn NMI_Handler() {
    loop {}
}

#[unsafe(no_mangle)]
extern "C" fn HardFault_Handler() {
    loop {}
}

// Define the reset handler
#[unsafe(no_mangle)]
extern "C" fn Reset_Handler() {
    unsafe {
        // Copy the .data section from FLASH to RAM
        let mut src_is_flash = ptr::addr_of!(_sidata);
        let mut dest_is_ram = ptr::addr_of_mut!(_sdata);
        let data_end_in_ram = ptr::addr_of_mut!(_edata);

        while dest_is_ram < data_end_in_ram {
            *dest_is_ram = *src_is_flash;
            dest_is_ram = dest_is_ram.add(1);
            src_is_flash = src_is_flash.add(1);
        }

        // Zero out the .bss section in the RAM
        let mut bss = ptr::addr_of_mut!(_sbss);
        let bss_end = ptr::addr_of_mut!(_ebss);

        while bss < bss_end {
            *bss = 0;
            bss = bss.add(1);
        }
    }

    // Call main function
    crate::main();
}

// Define the exception handlers if needed
