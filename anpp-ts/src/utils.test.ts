import { calculateCRC16 } from "./utils";

describe("crc-16", function () {
    it("crc of empty buffer is 0xffff", function () {
        const buffer = new Uint8Array(0);

        const got = calculateCRC16(buffer);

        expect(got).toEqual(0xffff);
    })

    it("Random data never generates crc out of bounds", function () {
        const buffer = new Uint8Array(1024);
        for (let i = 0; i < buffer.length; i++) {
            buffer[i] = Math.floor(Math.random() * 256);
        }

        const got = calculateCRC16(buffer);

        expect(got).toBeGreaterThanOrEqual(0);
        expect(got).toBeLessThanOrEqual(0xffff);
    })
})