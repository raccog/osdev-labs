//! An implementation of the ARM PrimeCell PL011 UART.

use core::fmt::{self, Write};
use tock_registers::{
    interfaces::{ReadWriteable, Readable, Writeable},
    register_bitfields, register_structs,
    registers::{ReadOnly, ReadWrite, WriteOnly},
};

pub const UART0_BASE_ADDRESS: usize = 0x0900_0000;

register_structs! {
    pub Pl011Uart {
        // Data Register, UARTDR
        (0x000 => pub dr: ReadWrite<u16, Data::Register>),
        (0x002 => _reserved0),
        // Receive Status Register/Error Clear Register, UARTRSR/UARTECR
        (0x004 => pub rsr_ecr: ReadWrite<u8, ReceiveStatus::Register>),
        (0x005 => _reserved1),
        // Flag Register, UARTFR
        (0x018 => pub fr: ReadOnly<u16, Flag::Register>),
        (0x01a => _reserved2),
        // IrDA Low-Power Counter Register, UARTILPR
        (0x020 => pub ilpr: ReadWrite<u8>),
        (0x021 => _reserved3),
        // Integer Baud Rate Register, UARTIBRD
        (0x024 => pub ibrd: ReadWrite<u16>),
        (0x026 => _reserved4),
        // Fractional Baud Rate Register, UARTFBRD
        (0x028 => pub fbrd: ReadWrite<u8, FractionalBaudRate::Register>),
        (0x029 => _reserved5),
        // Line Control Register, UARTLCR_H
        (0x02c => pub lcr_h: ReadWrite<u8, LineControl::Register>),
        (0x02d => _reserved6),
        // Control Register, UARTCR
        (0x030 => pub cr: ReadWrite<u16, Control::Register>),
        (0x032 => _reserved7),
        // Interrupt FIFO Level Select Register, UARTIFLS
        (0x034 => pub ifls: ReadWrite<u8, InterruptFifoLevelSelect::Register>),
        (0x035 => _reserved8),
        // Interrupt Mask Set/Clear Register, UARTIMSC
        (0x038 => pub imsc: ReadWrite<u16, InterruptMask::Register>),
        (0x03a => _reserved9),
        // Raw Interrupt Status Register, UARTRIS
        (0x03c => pub ris: ReadOnly<u16, RawInterruptStatus::Register>),
        (0x03e => _reserved10),
        // Masked Interrupt Status Register, UARTMIS
        (0x040 => pub mis: ReadOnly<u16, MaskedInterruptStatus::Register>),
        (0x042 => _reserved11),
        // Interrupt Clear Register, UARTICR
        (0x044 => pub icr: WriteOnly<u16, InterruptClear::Register>),
        (0x046 => _reserved12),
        // DMA Control Register, UARTDMACR
        (0x048 => pub dmacr: ReadWrite<u8, DmaControl::Register>),
        (0x049 => _reserved13),
        // Peripheral Identification Registers, UARTPeriphID0
        (0xfe0 => pub periph_id0: ReadOnly<u8, PeripheralId0::Register>),
        (0xfe1 => _reserved14),
        // Peripheral Identification Registers, UARTPeriphID1
        (0xfe4 => pub periph_id1: ReadOnly<u8, PeripheralId1::Register>),
        (0xfe5 => _reserved15),
        // Peripheral Identification Registers, UARTPeriphID2
        (0xfe8 => pub periph_id2: ReadOnly<u8, PeripheralId2::Register>),
        (0xfe9 => _reserved16),
        // Peripheral Identification Registers, UARTPeriphID3
        (0xfec => pub periph_id3: ReadOnly<u8, PeripheralId3::Register>),
        (0xfed => _reserved17),
        // PrimeCell Identification Registers, UARTPCellID0
        (0xff0 => pub pcell_id0: ReadOnly<u8, PCellId0::Register>),
        (0xff1 => _reserved18),
        // PrimeCell Identification Registers, UARTPCellID1
        (0xff4 => pub pcell_id1: ReadOnly<u8, PCellId1::Register>),
        (0xff5 => _reserved19),
        // PrimeCell Identification Registers, UARTPCellID2
        (0xff8 => pub pcell_id2: ReadOnly<u8, PCellId2::Register>),
        (0xff9 => _reserved20),
        // PrimeCell Identification Registers, UARTPCellID3
        (0xffc => pub pcell_id3: ReadOnly<u8, PCellId3::Register>),
        (0xffd => _reserved21),
        (0x1000 => @END),
    }
}

register_bitfields! [
    u8,
    pub ReceiveStatus [
        // See `Data` for descriptions of these fields
        OE 3,
        BE 2,
        PE 1,
        FE 0
    ],
    pub FractionalBaudRate [
        // The fractional baud rate divisor
        DIVFRAC OFFSET(0) NUMBITS(5) [],
    ],
    pub LineControl [
        // Stick parity select
        SPS OFFSET(7) NUMBITS(1) [],
        // Word length
        WLEN OFFSET(5) NUMBITS(2) [],
        // Enable FIFOs
        FEN OFFSET(4) NUMBITS(1) [],
        // Two stop bits select
        STP2 OFFSET(3) NUMBITS(1) [],
        // Even parity select
        EPS OFFSET(2) NUMBITS(1) [],
        // Parity enable
        PEN OFFSET(1) NUMBITS(1) [],
        // Send break
        BRK OFFSET(0) NUMBITS(1) []
    ],
    pub InterruptFifoLevelSelect [
        // Receive interrupt FIFO level select
        RXIFLSEL OFFSET(3) NUMBITS(3) [
            // Each of these correspond to how full a FIFO can get before an interrupt is triggered
            ONE_EIGHTH = 0b000,
            ONE_FOURTH = 0b001,
            ONE_HALF = 0b010,
            THREE_FOURTHS = 0b011,
            SEVEN_EIGHTHS = 0b100
        ],
        // Transmit interrupt FIFO level select
        TXIFLSEL OFFSET(0) NUMBITS(3) [
            // Each of these correspond to how empty a FIFO can get before an interrupt is triggered
            ONE_EIGHTH = 0b000,
            ONE_FOURTH = 0b001,
            ONE_HALF = 0b010,
            THREE_FOURTHS = 0b011,
            SEVEN_EIGHTHS = 0b100
        ],
    ],
    pub DmaControl [
        // DMA on error
        DMAONERR 2,
        // Transmit DMA enable
        TXDMAE 1,
        // Receive DMA enable
        RXDMAE 0
    ],
    pub PeripheralId0 [
        // First 8 bits of the part number
        PartNumber0 OFFSET(0) NUMBITS(8) [
            VALID_UART = 0x011
        ],
    ],
    pub PeripheralId1 [
        // First 4 bits of the designer ID
        Designer0 OFFSET(4) NUMBITS(4) [],
        // Last 4 bits of the part number
        PartNumber1 OFFSET(0) NUMBITS(4) [
            VALID_UART = 0x0
        ],
    ],
    pub PeripheralId2 [
        // Revision number
        Revision OFFSET(4) NUMBITS(4) [
            R1P0 = 0x0,
            R1P1 = 0x1,
            R1P3 = 0x2,
            R1P5 = 0x3,
        ],
        // Last 4 bits of the designer ID
        Designer1 OFFSET(0) NUMBITS(4) [],
    ],
    pub PeripheralId3 [
        // Not sure what this configuration is used for
        Configuration OFFSET(0) NUMBITS(8) [
            VALID_UART = 0x00
        ]
    ],
    pub PCellId0 [
        UARTPCellID0 OFFSET(0) NUMBITS(8) [
            VALID_PCELL = 0x0d
        ]
    ],
    pub PCellId1 [
        UARTPCellID1 OFFSET(0) NUMBITS(8) [
            VALID_PCELL = 0xf0
        ]
    ],
    pub PCellId2 [
        UARTPCellID2 OFFSET(0) NUMBITS(8) [
            VALID_PCELL = 0x05
        ]
    ],
    pub PCellId3 [
        UARTPCellID3 OFFSET(0) NUMBITS(8) [
            VALID_PCELL = 0xb1
        ]
    ],
];

register_bitfields! [
    u16,
    pub Data [
        // Overrun Error
        OE OFFSET(11) NUMBITS(1) [],
        // Break Error
        BE OFFSET(10) NUMBITS(1) [],
        // Parity Error
        PE OFFSET(9) NUMBITS(1) [],
        // Framing Error
        FE OFFSET(8) NUMBITS(1) [],
        // Receive/transmit data
        DATA OFFSET(0) NUMBITS(8) [],
    ],
    pub Flag [
        // Ring indicator
        RI 8,
        // Transmit FIFO empty
        TXFE 7,
        // Receive FIFO full
        RXFF 6,
        // Transmit FIFO full
        TXFF 5,
        // Receive FIFO empty
        RXFE 4,
        // UART busy
        BUSY 3,
        // Data carrier detect
        DCD 2,
        // Data set ready
        DSR 1,
        // Clear to send
        CTS 0
    ],
    pub Control [
        // CTS hardware flow control enable
        CTSEn 15,
        // RTS hardware flow control enable
        RTSEn 14,
        // This bit is the complement of the UART Out2 (nUARTOut2) modem status output
        Out2 13,
        // This bit is the complement of the UART Out1 (nUARTOut1) modem status output
        Out1 12,
        // Request to send
        RTS 11,
        // Data transmit ready
        DTR 10,
        // Receive enable
        RXE 9,
        // Transmit enable
        TXE 8,
        // Loopback enable
        LBE 7,
        // SIR low-power IrDA mode
        SIRLP 2,
        // SIR enable
        SIREN 1,
        // UART enable
        UARTEN 0
    ],
    pub InterruptMask [
        // Overrun error interrupt mask
        OEIM 10,
        // Break error interrupt mask
        BEIM 9,
        // Parity error interrupt mask
        PEIM 8,
        // Framing error interrupt mask
        FEIM 7,
        // Receive timeout interrupt mask
        RTIM 6,
        // Transmit interrupt mask
        TXIM 5,
        // Receive interrupt mask
        RXIM 4,
        // nUARTDSR modem interrupt mask
        DSRMIM 3,
        // nUARTDCD modem interrupt mask
        DCDMIM 2,
        // nUARTCTS modem interrupt mask
        CTSMIM 1,
        // nUARTRI modem interrupt mask
        RIMIM 0
    ],
    pub RawInterruptStatus [
        // Overrun error interrupt status
        OERIS 10,
        // Break error interrupt status
        BERIS 9,
        // Parity error interrupt status
        PERIS 8,
        // Framing error interrupt status
        FERIS 7,
        // Receive timeout interrupt status
        RTRIS 6,
        // Transmit interrupt status
        TXRIS 5,
        // Receive interrupt status
        RXRIS 4,
        // nUARTDSR modem interrupt status
        DSRRMIS 3,
        // nUARTDCD modem interrupt status
        DCDRMIS 2,
        // nUARTCTS modem interrupt status
        CTSRMIS 1,
        // nUARTRI modem interrupt status
        RIRMIS 0
    ],
    pub MaskedInterruptStatus [
        // Overrun error masked interrupt mask
        OEMIS 10,
        // Break error masked interrupt mask
        BEMIS 9,
        // Parity error masked interrupt mask
        PEMIS 8,
        // Framing error masked interrupt mask
        FEMIS 7,
        // Receive timeout masked interrupt mask
        RTMIS 6,
        // Transmit masked interrupt mask
        TXMIS 5,
        // Receive masked interrupt mask
        RXMIS 4,
        // nUARTDSR modem masked interrupt mask
        DSRMMIS 3,
        // nUARTDCD modem masked interrupt mask
        DCDMMIS 2,
        // nUARTCTS modem masked interrupt mask
        CTSMMIS 1,
        // nUARTRI modem masked interrupt mask
        RIMMIS 0
    ],
    pub InterruptClear [
        // Overrun error interrupt clear
        OEIC 10,
        // Break error interrupt clear
        BEIC 9,
        // Parity error interrupt clear
        PEIC 8,
        // Framing error interrupt clear
        FEIC 7,
        // Receive timeout interrupt clear
        RTIC 6,
        // Transmit interrupt clear
        TXIC 5,
        // Receive interrupt clear
        RXIC 4,
        // nUARTDSR modem interrupt clear
        DSRMIC 3,
        // nUARTDCD modem interrupt clear
        DCDMIC 2,
        // nUARTCTS modem interrupt clear
        CTSMIC 1,
        // nUARTRI modem interrupt clear
        RIMIC 0
    ],
];

impl Pl011Uart {
    pub unsafe fn new(base_addr: usize) -> Result<&'static mut Self, ()> {
        let uart = &mut *(base_addr as *mut Self);

        uart.init()?;

        Ok(uart)
    }

    fn init(&mut self) -> Result<(), ()> {
        // These steps are done according to the ARM PrimeCell UART Manual. They can be found in the UARTCR Register
        // summary section.
        //
        // 1. Disable the UART
        self.cr.modify(Control::UARTEN::CLEAR);

        // 2. Wait for end of transmission or reception of the current character
        'not_busy: {
            // Don't block for too long; return error if transmission/reception does not end
            for _ in 0..0xff {
                if !self.fr.is_set(Flag::BUSY) {
                    break 'not_busy;
                }
            }

            return Err(());
        }

        // 3. Flush the transmit FIFO
        self.lcr_h.modify(LineControl::FEN::CLEAR);

        // 4. Reprogram the UARTCR Register (control register)
        {
            // Enable transmit and receive. Disable loopback
            self.cr
                .modify(Control::RXE::SET + Control::TXE::SET + Control::LBE::CLEAR);

            // Enable FIFOs
            self.lcr_h.modify(LineControl::FEN::SET);

            // TODO: Calculate baudrate at runtime
            // Set baudrate to 115200
            // Calculated using a clock input of 48 MHz
            // Baud Rate Divisor = (48 * 10^6) / (16 * 115200) = 26.0417
            // Integer Divisor = 26
            // Fractional Divisor = integer((0.0417 * 64) + 0.5) = 3
            // Generated Baud Rate Divisor = 26 + (3 / 64) = 0.046875
            // Generated Baud Rate = (48 * 10^6) / (16 * 26.046875) = 115199.8525
            // Error Rate = (115200 - 115176) / 115200 * 100 = 0.0217%
            self.ibrd.set(26);
            self.fbrd.set(3);
        }

        // 5. Enable the UART
        self.cr.modify(Control::UARTEN::SET);

        Ok(())
    }
}

impl Write for Pl011Uart {
    fn write_str(&mut self, out_string: &str) -> fmt::Result {
        for out_byte in out_string.bytes() {
            // Don't block; just return an error if the FIFO is full
            if self.dr.is_set(Data::OE) {
                return Err(fmt::Error);
            }

            self.dr.set(out_byte as u16);
        }
        Ok(())
    }
}
