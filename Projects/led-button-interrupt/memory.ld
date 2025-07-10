/* Linker script for the STM32F407VG */
/* 1024KByte FLASH, 128KByte RAM, 64KByte CCMRAM */

ENTRY(Reset_Handler)
/* Memory addresses  */
MEMORY
{
  FLASH  (rx)  : ORIGIN = 0x08000000, LENGTH = 1024K
  RAM    (rwx) : ORIGIN = 0x20000000, LENGTH = 128K
  CCMRAM (rwx) : ORIGIN = 0x10000000, LENGTH = 64K
}

_start_of_stack = ORIGIN(RAM) + LENGTH(RAM);
_min_stack_size = 0x400; /* 1KB */
_min_heap_size = 0x400;

/*
1. Define where each section (.text, .data, .bss) should be placed in memory.

2. The linker assigns addresses based on the SECTIONS command, creating an ELF file with exact memory locations.
For example, if FLASH starts at 0x08000000 and the .text is placed in FLASH then the linker
will assign memory address starting from 0x08000000 for the .text section.

3. The ELF file guides the programming tool to correctly place the sections in the microcontroller
ensuring proper execution of the code
*/
SECTIONS
{


    /* 
    Code like functions, interrupt service routines and other compiled code are stored here.
    It contains all of the program's machine instructions that the microcontroller will execute.
    */
    .text :
    {
        /* 
        ALIGN is used in this case to align the next address to a multiple of 4 bytes.
        So if the location counter is 0x0800 0021 for *(.text.*), then it will align the memory address to be 0x0800 0024 (next multiple of 4)
        */
        . = ORIGIN(FLASH);
        LONG(_start_of_stack); /* Initial stack address pointer should come first in FLASH */
        KEEP(*(.isr_vector)); /* KEEP instructs the compiler to retain this section in the final ELF file */
        *(.text)
        *(.text*)
        . = ALIGN(4);
    } > FLASH

    /* Read-only data */
    .rodata :
    {
        . = ALIGN(4);
        *(.rodata)
        *(.rodata*)
        . = ALIGN(4);
    } > FLASH

    /* 
    Initialized data will be in FLASH and it will also be in RAM during runtime.
    This data will be stored initially in FLASH before being copied to RAM during startup.

    First we specify execution memory address and then the load memory address.
    The syntax is `<execution memory address> AT> <load memory address>
    */
    .data :
    {
        _sidata = LOADADDR(.data); /* This return the LMA (FLASH) memory address */
        . = ALIGN(4);
        _sdata = .; /* Start of data section in VMA (virtual memory address which is the RAM) */
        *(.data)
        *(.data*)
        . = ALIGN(4);
        _edata = .; /* Start of data section in LMA (load memory address which is the FLASH) */
    } > RAM AT> FLASH

    /* Uninitialized data */
    .bss :
    {
        . = ALIGN(4);
        _sbss = .;
        *(.bss)
        *(.bss*)
        . = ALIGN(4);
        _ebss = .;
    } > RAM

    .ram_usage_check :
    {
        . = ALIGN(8);
        . = . + _min_stack_size;
        . = . + _min_heap_size;
        . = ALIGN(8);
    } > RAM
}

PROVIDE(MemManage_Handler = Default_Handler);
PROVIDE(BusFault_Handler = Default_Handler);
PROVIDE(UsageFault_Handler = Default_Handler);
PROVIDE(SVCall_Handler = Default_Handler);
PROVIDE(DebugMon_Handler = Default_Handler);
PROVIDE(PendSV_Handler = Default_Handler);
PROVIDE(SysTick_Handler = Default_Handler);
PROVIDE(ADC_Handler = Default_Handler);
PROVIDE(CAN1_RX0_Handler = Default_Handler);
PROVIDE(CAN1_RX1_Handler = Default_Handler);
PROVIDE(CAN1_SCE_Handler = Default_Handler);
PROVIDE(CAN1_TX_Handler = Default_Handler);
PROVIDE(CAN2_RX0_Handler = Default_Handler);
PROVIDE(CAN2_RX1_Handler = Default_Handler);
PROVIDE(CAN2_SCE_Handler = Default_Handler);
PROVIDE(CAN2_TX_Handler = Default_Handler);
PROVIDE(CRYP_Handler = Default_Handler);
PROVIDE(DCMI_Handler = Default_Handler);
PROVIDE(DMA1_Stream0_Handler = Default_Handler);
PROVIDE(DMA1_Stream1_Handler = Default_Handler);
PROVIDE(DMA1_Stream2_Handler = Default_Handler);
PROVIDE(DMA1_Stream3_Handler = Default_Handler);
PROVIDE(DMA1_Stream4_Handler = Default_Handler);
PROVIDE(DMA1_Stream5_Handler = Default_Handler);
PROVIDE(DMA1_Stream6_Handler = Default_Handler);
PROVIDE(DMA1_Stream7_Handler = Default_Handler);
PROVIDE(DMA2_Stream0_Handler = Default_Handler);
PROVIDE(DMA2_Stream1_Handler = Default_Handler);
PROVIDE(DMA2_Stream2_Handler = Default_Handler);
PROVIDE(DMA2_Stream3_Handler = Default_Handler);
PROVIDE(DMA2_Stream4_Handler = Default_Handler);
PROVIDE(DMA2_Stream5_Handler = Default_Handler);
PROVIDE(DMA2_Stream6_Handler = Default_Handler);
PROVIDE(DMA2_Stream7_Handler = Default_Handler);
PROVIDE(ETH_Handler = Default_Handler);
PROVIDE(ETH_WKUP_Handler = Default_Handler);
PROVIDE(EXTI0_Handler = Default_Handler);
PROVIDE(EXTI15_10_Handler = Default_Handler);
PROVIDE(EXTI1_Handler = Default_Handler);
PROVIDE(EXTI2_Handler = Default_Handler);
PROVIDE(EXTI3_Handler = Default_Handler);
PROVIDE(EXTI4_Handler = Default_Handler);
PROVIDE(EXTI9_5_Handler = Default_Handler);
PROVIDE(FPU_Handler = Default_Handler);
PROVIDE(FSMC_Handler = Default_Handler);
PROVIDE(HASH_RNG_Handler = Default_Handler);
PROVIDE(I2C1_ER_Handler = Default_Handler);
PROVIDE(I2C1_EV_Handler = Default_Handler);
PROVIDE(I2C2_ER_Handler = Default_Handler);
PROVIDE(I2C2_EV_Handler = Default_Handler);
PROVIDE(I2C3_ER_Handler = Default_Handler);
PROVIDE(I2C3_EV_Handler = Default_Handler);
PROVIDE(OTG_FS_Handler = Default_Handler);
PROVIDE(OTG_FS_WKUP_Handler = Default_Handler);
PROVIDE(OTG_HS_EP1_IN_Handler = Default_Handler);
PROVIDE(OTG_HS_EP1_OUT_Handler = Default_Handler);
PROVIDE(OTG_HS_Handler = Default_Handler);
PROVIDE(OTG_HS_WKUP_Handler = Default_Handler);
PROVIDE(PVD_Handler = Default_Handler);
PROVIDE(RCC_Handler = Default_Handler);
PROVIDE(RTC_Alarm_Handler = Default_Handler);
PROVIDE(RTC_WKUP_Handler = Default_Handler);
PROVIDE(SDIO_Handler = Default_Handler);
PROVIDE(SPI1_Handler = Default_Handler);
PROVIDE(SPI2_Handler = Default_Handler);
PROVIDE(SPI3_Handler = Default_Handler);
PROVIDE(TAMP_STAMP_Handler = Default_Handler);
PROVIDE(TIM1_BRK_TIM9_Handler = Default_Handler);
PROVIDE(TIM1_CC_Handler = Default_Handler);
PROVIDE(TIM1_TRG_COM_TIM11_Handler = Default_Handler);
PROVIDE(TIM1_UP_TIM10_Handler = Default_Handler);
PROVIDE(TIM2_Handler = Default_Handler);
PROVIDE(TIM3_Handler = Default_Handler);
PROVIDE(TIM4_Handler = Default_Handler);
PROVIDE(TIM5_Handler = Default_Handler);
PROVIDE(TIM6_DAC_Handler = Default_Handler);
PROVIDE(TIM7_Handler = Default_Handler);
PROVIDE(TIM8_BRK_TIM12_Handler = Default_Handler);
PROVIDE(TIM8_CC_Handler = Default_Handler);
PROVIDE(TIM8_TRG_COM_TIM14_Handler = Default_Handler);
PROVIDE(TIM8_UP_TIM13_Handler = Default_Handler);
PROVIDE(UART4_Handler = Default_Handler);
PROVIDE(UART5_Handler = Default_Handler);
PROVIDE(USART1_Handler = Default_Handler);
PROVIDE(USART2_Handler = Default_Handler);
PROVIDE(USART3_Handler = Default_Handler);
PROVIDE(USART6_Handler = Default_Handler);
PROVIDE(WWDG_Handler = Default_Handler);